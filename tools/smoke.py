#!/usr/bin/env python3
"""The deploy gate: the ten things a user does, run headless against the
freshly built server on a scratch port, cold and then warm (world cache
primed) and then on a throttled network — so a boot-timing race that
passes one pass and fails another is caught before it ships.

  python3 tools/smoke.py            # build dir products/miso/build, port 8140
  python3 tools/smoke.py --port N   # another port
  python3 tools/smoke.py --keep     # leave the server up on failure

Exit 0 = every step passed in every pass; anything else blocks deploy.sh.
Written 2026-08-25 (accounts #p96) after the lozenge went dead on the phone
while every desktop rig said it was fine.
"""
import argparse, asyncio, json, os, pathlib, shutil, signal, subprocess, sys, time

REPO = pathlib.Path(__file__).resolve().parent.parent
BUILD = REPO / "products" / "miso" / "build"
# one scratch per port: two gates on one machine (a deploy's and a rig's) must
# never share a world — a fixed path had them deleting each other's users.json
# mid-pass (2026-09-01, three shape-shifting gate failures). Rebound in main().
SCRATCH = pathlib.Path(os.environ.get("MISO_SMOKE_DIR") or "/tmp/miso-smoke")
USER = {"name": "_smoke", "phone": "+15550000999", "authority": "admin"}


def sh(args, **kw):
    return subprocess.run(args, capture_output=True, text=True, **kw)


def build_on(port: int) -> pathlib.Path:
    """rebuild the debug server with serve_port() at `port`, restore the source"""
    src = BUILD / "server" / "src" / "main.rs"
    orig = src.read_text()
    marker = "fn serve_port() -> u16 {\n        8095\n"
    if marker not in orig:
        sys.exit("smoke: serve_port() seam not found in the emitted main.rs — link first")
    src.write_text(orig.replace(marker, f"fn serve_port() -> u16 {{\n        {port}\n", 1))
    try:
        r = sh(["cargo", "build", "-q"], cwd=BUILD / "server")
        if r.returncode != 0:
            sys.exit("smoke: cargo build failed:\n" + r.stderr[-2000:])
    finally:
        src.write_text(orig)
    return BUILD / "server" / "target" / "debug" / "miso_server"


def start(binary: pathlib.Path, port: int):
    if SCRATCH.exists():
        shutil.rmtree(SCRATCH)
    (SCRATCH / "home" / ".miso-auth").mkdir(parents=True)
    (SCRATCH / "ctx").mkdir(parents=True)
    (SCRATCH / "home" / ".miso-auth" / "users.json").write_text(json.dumps([USER]))
    log = open(SCRATCH / "server.log", "w")
    env = dict(os.environ, HOME=str(SCRATCH / "home"), MISO_CONTEXT_DIR=str(SCRATCH / "ctx"))
    p = subprocess.Popen([str(binary)], cwd=BUILD, env=env, stdout=log, stderr=subprocess.STDOUT)
    for _ in range(40):
        if sh(["curl", "-s", "-m", "1", f"localhost:{port}/version"]).returncode == 0:
            return p
        time.sleep(0.25)
    p.kill()
    sys.exit("smoke: the server did not come up on port %d" % port)


def login(port: int) -> str:
    sh(["curl", "-s", "-X", "POST", f"localhost:{port}/auth/request", "-d", json.dumps({"phone": USER["phone"]})])
    time.sleep(0.4)
    pin = ""
    for line in (SCRATCH / "server.log").read_text().splitlines():
        if f"test user {USER['name']} pin" in line:
            pin = line.split()[-1]
    if not pin:
        sys.exit("smoke: no login code in the server log")
    r = sh(["curl", "-s", "-i", "-X", "POST", f"localhost:{port}/auth/verify", "-d", json.dumps({"phone": USER["phone"], "pin": pin})]).stdout
    for line in r.splitlines():
        if line.lower().startswith("set-cookie:") and "miso_auth=" in line:
            return line.split("miso_auth=")[1].split(";")[0]
    sys.exit("smoke: verify issued no cookie")


STEPS = []


def step(name):
    def deco(fn):
        STEPS.append((name, fn)); return fn
    return deco


async def go_home(pg):
    """‹ goes one level up (/one-level): press it until it is gone.

    Read from the SCREEN, not from `feature_Loop.state`. `open_tool` is
    bridged, /payload republishes it part-way down the update chain, and
    /people, /posts and /projects each write it back at a LATER link — so on
    the tap that means "back to the set" the mirror says "" while the row
    already shows the tool open. This helper used to believe the mirror and
    then click a tool button that /current-only was no longer drawing, which
    is a 30s timeout and a step blamed on the app (misses.md, "the gate's own
    caret" and "navigation from the wrong side")."""
    for _ in range(6):
        if not await pg.evaluate("!!document.querySelector('.toolbar [data-ev=\"tools_home\"]')"): return
        await pg.click('.toolbar [data-ev="tools_home"]'); await pg.wait_for_timeout(900)


async def open_tool(pg, tool):
    if not await pg.evaluate(f"!!document.querySelector('.toolbar .tool-button.sel[data-ev=\"tool_{tool}\"]')"):
        await go_home(pg)
        await pg.click(f'.toolbar [data-ev="tool_{tool}"]')
    await pg.wait_for_timeout(1500)


async def until(pg, js, limit_ms=10000, every=200):
    """poll a condition; return the ms it took, or -1"""
    waited = 0
    while waited <= limit_ms:
        if await pg.evaluate(js): return waited
        await pg.wait_for_timeout(every); waited += every
    return -1


async def pass_gate(pg):
    """/profile-first, at boot and before any housekeeping tap: the smoke user
    is a fresh world, so the first pass lands on the card with no way off — the
    tool buttons are withheld, and a click on one would wait forever. The
    picture goes in through the loop's own event (the chooser is a native
    sheet), the line is typed, the tick saves. /tour then offers itself and is
    ended here so later steps see the plain screen. Later passes find the
    card complete and the gate down."""
    if not await pg.evaluate("typeof feature_ProfileFirst !== 'undefined'"):
        return True
    if await pg.evaluate("feature_ProfileFirst.gated()"):
        for _ in range(20):
            if await pg.evaluate("!!document.querySelector('.card-page .card-text')"): break
            await pg.wait_for_timeout(250)
        # /greetings: the first welcome page stands over the card; let's go
        if await pg.evaluate("!!document.querySelector('#greetSheet .greet-go')"):
            await pg.click("#greetSheet .greet-go"); await pg.wait_for_timeout(600)
        await pg.evaluate("""(() => {
            const s = JSON.parse(feature_Loop.state); const cards = JSON.parse(s.cards || '[]');
            const me = cards.find(c => c.type === 'profile' && !c.from); if (!me) return;
            const i = me.blocks.findIndex(b => b.kind === 'picture');
            const png = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAgAAAAICAYAAADED76LAAAAF0lEQVR4nGP4z8DwHwyBGEIwMKBQMDAAAJ9YD/HG9V3DAAAAAElFTkSuQmCC';
            feature_Loop.send({ type: 'CardPic', data: { id: me.id, i: i, data: png, t: Date.now() } });
        })()""")
        await pg.wait_for_timeout(800)
        await pg.click(".card-page .card-text"); await pg.keyboard.type("here to help")
        await pg.wait_for_timeout(400)
        await pg.click('[data-ctl="card_edit"]')
        took = await until(pg, "!feature_ProfileFirst.gated()")
        print(f"      (profile gate lifted in {took} ms)" if took >= 0 else "      (profile gate never lifted)")
        if took < 0:
            await dump(pg, "profile-first"); return False
        # /greetings: the second page, once the gate is down. /set-up's rows
        # must settle first: each pending row is tried (a headless Chrome
        # refuses the passkey and that counts as settled); then got it
        await pg.wait_for_timeout(600)
        for _ in range(3):
            if not await pg.evaluate("!!document.querySelector('#greetSheet .greet-row:not(.settled) .greet-do')"): break
            await pg.click("#greetSheet .greet-row:not(.settled) .greet-do"); await pg.wait_for_timeout(1500)
        # then every remaining page (/last-word adds a third)
        for _ in range(3):
            if not await pg.evaluate("!!document.querySelector('#greetSheet .greet-go')"): break
            await pg.click("#greetSheet .greet-go"); await pg.wait_for_timeout(600)
    if await pg.evaluate("typeof feature_Tour !== 'undefined' && feature_Tour.at >= 0"):
        await pg.evaluate("feature_Tour.end()"); await pg.wait_for_timeout(600)
    return True


@step("a bare card was gated until a picture and a line went in; the gate is down")
async def s_profile_first(pg):
    if not await pg.evaluate("typeof feature_ProfileFirst !== 'undefined'"):
        return True
    return not await pg.evaluate("feature_ProfileFirst.gated()")


@step("the lozenge opens the system panel on a tap")
async def s_lozenge(pg):
    await pg.click("#build")
    took = await until(pg, "getComputedStyle(document.getElementById('panel')).display === 'block'")
    print(f"      (panel opened in {took} ms)" if took >= 0 else "      (panel never opened)")
    if took < 0 or os.environ.get("SMOKE_DUMP"):
        await dump(pg, "lozenge")
    await pg.evaluate("feature_Panel.close()")
    return took >= 0


async def dump(pg, tag):
    """what is on top: the element under the lozenge, and every large fixed or
    absolute box that is visible — a failure that says 'covered' names the cover"""
    info = await pg.evaluate("""(() => {
      const b = document.getElementById('build'); const r = b ? b.getBoundingClientRect() : {x:0,y:0,width:0,height:0};
      const under = document.elementFromPoint(r.x + r.width/2, r.y + r.height/2);
      const path = []; for (let e = under; e && e !== document.body; e = e.parentElement) path.push(e.tagName.toLowerCase() + (e.id ? '#' + e.id : '') + (e.className && typeof e.className === 'string' ? '.' + e.className.split(' ').join('.') : ''));
      const covers = [];
      for (const e of document.querySelectorAll('body *')) {
        const cs = getComputedStyle(e); if (cs.display === 'none' || cs.visibility === 'hidden' || cs.opacity === '0') continue;
        if (cs.position !== 'fixed' && cs.position !== 'absolute') continue;
        const q = e.getBoundingClientRect(); if (q.width * q.height < 40000) continue;
        covers.push(`${e.tagName.toLowerCase()}${e.id ? '#' + e.id : ''}${typeof e.className === 'string' && e.className ? '.' + e.className.split(' ').join('.') : ''} ${Math.round(q.width)}x${Math.round(q.height)} z=${cs.zIndex} pe=${cs.pointerEvents}`);
      }
      return {under: path.join(' < '), covers, panel: (document.getElementById('panel')||{}).className, shade: (document.getElementById('shade')||{}).className};
    })()""")
    print(f"      (under the lozenge: {info['under']})")
    for c in info["covers"]: print(f"      (cover: {c})")
    print(f"      (panel class: {info['panel']!r}, shade class: {info['shade']!r})")
    more = await pg.evaluate("""(() => ({build: (document.getElementById('build')||{}).className, misoVersion: localStorage.misoVersion, running: typeof feature_Update !== 'undefined' ? feature_Update.running : null, server: typeof feature_Update !== 'undefined' ? feature_Update.server : null, sw: !!(navigator.serviceWorker && navigator.serviceWorker.controller), parked: typeof feature_Account !== 'undefined' ? !!feature_Account.parked : null, tool: JSON.parse(feature_Loop.state||'{}').open_tool, panelOpen: typeof feature_Panel !== 'undefined' && !!feature_Panel.open}))()""")
    print(f"      (state: {more})")
    panel = await pg.evaluate("""(() => { const p=document.getElementById('panel'); const cs=getComputedStyle(p); return {display: cs.display, visibility: cs.visibility, opacity: cs.opacity, cls: p.className, style: p.getAttribute('style'), sheets: [...document.styleSheets].map(s => (s.href||'inline').split('/').pop() + ':' + (() => { try { return s.cssRules.length } catch (e) { return 'x' } })()).join(' ')}; })()""")
    print(f"      (panel: {panel})")
    slow = await pg.evaluate("performance.getEntriesByType('resource').filter(r => r.duration > 800).map(r => r.name.split('/').pop() + ' ' + Math.round(r.duration) + 'ms').slice(0, 12)")
    print(f"      (slow resources: {slow})")
    timing = await pg.evaluate("""(async () => { const out = {}; for (const f of ['changes.json', 'version', 'hashes.json', 'features/tree.json']) { const t0 = performance.now(); try { const r = await Promise.race([fetch(f, {cache: 'no-store'}), new Promise((_, rej) => setTimeout(() => rej(new Error('8s')), 8000))]); out[f] = r.status + ' in ' + Math.round(performance.now() - t0) + 'ms'; } catch (e) { out[f] = 'FAILED ' + e.message + ' after ' + Math.round(performance.now() - t0) + 'ms'; } } return out; })()""")
    print(f"      (fetch now: {timing})")
    hist = await pg.evaluate("performance.getEntriesByType('resource').filter(r => /changes|version|hashes|tree/.test(r.name)).map(r => r.name.split('/').pop() + ' ' + Math.round(r.duration) + 'ms')")
    print(f"      (fetch history: {hist})")
    probes = await pg.evaluate("""(async () => { const out = {}; const t = async (k, f) => { const t0 = performance.now(); try { const r = await Promise.race([f(), new Promise((_, rej) => setTimeout(() => rej(new Error('6s')), 6000))]); out[k] = (r.status || r) + ' in ' + Math.round(performance.now() - t0) + 'ms'; } catch (e) { out[k] = 'FAILED ' + e.message; } };
      await t('tree?bust', () => fetch('features/tree.json?b=' + Math.random(), {cache: 'no-store'}));
      await t('tree default-cache', () => fetch('features/tree.json'));
      await t('tree xhr', () => new Promise((res, rej) => { const x = new XMLHttpRequest(); x.open('GET', 'features/tree.json'); x.onload = () => res(x.status); x.onerror = rej; x.send(); }));
      await t('cache.match tree', async () => { const c = await caches.open('miso'); const m = await c.match('features/tree.json'); return m ? 'hit' : 'miss'; });
      await t('cache keys', async () => { const c = await caches.open('miso'); return (await c.keys()).length; });
      const reg = navigator.serviceWorker && await navigator.serviceWorker.getRegistration();
      out.sw = reg ? {installing: !!reg.installing, waiting: !!reg.waiting, active: !!reg.active, state: reg.active && reg.active.state} : null;
      if (reg) { await reg.unregister(); await t('tree after unregister', () => fetch('features/tree.json?u=1', {cache: 'no-store'})); }
      return out; })()""")
    print(f"      (probes: {probes})")
    print(f"      (navigations so far: {pg._smoke_navs})")
    for l in pg._smoke_logs[-12:]: print(f"      (console: {l})")
    await pg.screenshot(path=str(SCRATCH / f"fail-{tag}.png"))
    print(f"      (screenshot: {SCRATCH / f'fail-{tag}.png'})")


@step("👤 lands on the people map — no picker, no grid, the band holds the set")
async def s_people(pg):
    # /map-only: the map is the only view, the picker is gone, and everyone in
    # the set is in the band whether they are on the ground or not
    await open_tool(pg, "account")
    for _ in range(20):
        if await pg.evaluate("!!document.getElementById('mapData') && !document.querySelector('.browse-picker') && !document.querySelector('.card-tile')"):
            return True
        await pg.wait_for_timeout(500)
    print("      (mapData:", await pg.evaluate("!!document.getElementById('mapData')"),
          "picker:", await pg.evaluate("!!document.querySelector('.browse-picker')"),
          "tiles:", await pg.evaluate("!!document.querySelector('.card-tile')"), ")")
    await dump(pg, "people-map")
    return False


@step("the own card opens from the band and an edit is saved")
async def s_edit(pg):
    # with the grid gone the band is the way in, so this step is also the
    # proof that a placeless person has one (/map-only)
    me = await pg.evaluate("""(() => {
        const cs = JSON.parse(JSON.parse(feature_Loop.state).cards || '[]');
        const m = cs.find(c => c.type === 'profile' && !c.from); return m ? m.id : ''; })()""")
    if not me:
        print("      (no own profile card)"); return False
    sel = f'#mapReel .reel-post[data-ev="browse_open:{me}"]'
    for _ in range(20):
        if await pg.evaluate(f"!!document.querySelector('{sel}')"): break
        await pg.wait_for_timeout(500)
    else:
        print("      (own card has no lozenge in the band — nothing on this map opens it)")
        await dump(pg, "band-own-card"); return False
    await pg.click(sel); await pg.wait_for_timeout(2000)
    # a card opens read-only since /editing: press edit first if it is offered
    # since /editing/toolbar the control is the pencil in the toolbar; the pill before it
    if await pg.evaluate("!!document.querySelector('.toolbar [data-ctl=card_edit]')"):
        await pg.dispatch_event('.toolbar [data-ctl=card_edit]', 'pointerdown'); await pg.wait_for_timeout(600)
    elif await pg.evaluate("(() => { const e=document.getElementById('cardEdit'); return !!e && e.classList.contains('show'); })()"):
        await pg.dispatch_event('#cardEdit', 'pointerdown'); await pg.wait_for_timeout(600)
    if not await pg.evaluate("!!document.querySelector('.card-page .card-text[contenteditable=true]')"):
        print("      (no editable card text after the pencil)"); await dump(pg, "edit-open"); return False
    # the caret goes to the END OF THE TEXT by a collapsed range, not the End
    # key: on this Mac's Chrome End does not move the caret in a contenteditable,
    # so the caret stayed where the click landed — past the end of a short text
    # (cold, warm), inside a longer one (throttled: 'here to help smoke sm|oke',
    # 2026-09-02). It never was a repaint: the caret rig showed no paint between
    # the keys and the caret at 21/24 before the first one.
    await pg.click(".card-text")
    await pg.evaluate("(() => { const el = document.querySelector('.card-text'); const r = document.createRange(); r.selectNodeContents(el); r.collapse(false); const s = getSelection(); s.removeAllRanges(); s.addRange(r); })()")
    await pg.keyboard.type(" smoke"); await pg.click(".card-title"); await pg.wait_for_timeout(1500)
    txt = await pg.evaluate("(document.querySelector('.card-text')||{}).innerText || ''")
    if not txt.endswith("smoke"):
        print(f"      (card text after the edit: {txt[-60:]!r})"); await dump(pg, "edit-save")
    return txt.endswith("smoke")


@step("the pencil survives the phone's tap: press on the glyph, click on the ground")
async def s_pencil_phone(pg):
    # from ash's black box, 2026-08-26 15:08:17 (#p160): pointerdown lands on the
    # pencil's <svg>, the words focus, the keyboard shifts the page, and the
    # click that follows lands on <html>. The card must open for editing, not close.
    if not await pg.evaluate("!!document.querySelector('.card-page')"): return False
    await pg.evaluate("(() => { const b = document.querySelector('.toolbar [data-ctl=card_edit]'); if (b && b.getAttribute('data-face') === 'save') { document.activeElement && document.activeElement.blur && document.activeElement.blur(); } })()")
    await pg.wait_for_timeout(400)
    if await pg.evaluate("(document.querySelector('.toolbar [data-ctl=card_edit]')||{getAttribute:()=>null}).getAttribute('data-face')") != 'edit':
        await pg.evaluate("feature_Editing.lock()"); await pg.wait_for_timeout(300)
    ok = await pg.evaluate("""(() => {
      const path = document.querySelector('.toolbar [data-ctl=card_edit] svg path');
      if (!path) return 'no glyph';
      path.dispatchEvent(new PointerEvent('pointerdown', {bubbles: true, cancelable: true, pointerType: 'touch', clientX: 249, clientY: 754}));
      document.documentElement.dispatchEvent(new MouseEvent('click', {bubbles: true, cancelable: true, clientX: 249, clientY: 754}));
      return 'sent'; })()""")
    if ok != 'sent': print(f"      ({ok})"); return False
    await pg.wait_for_timeout(1200)
    still = await pg.evaluate("!!document.querySelector('.card-page')")
    editable = await pg.evaluate("!!document.querySelector('.card-page .card-text[contenteditable=true]')")
    if not (still and editable): print(f"      (card open: {still}, editable: {editable})")
    return still and editable


@step("undo lights after a card edit")
async def s_undo(pg):
    return not await pg.evaluate("document.querySelector('[data-ev=\"ctx_undo\"]').classList.contains('dim')")


@step("the admin sees the invite plus; a tap puts the two ways in in the row")
async def s_invite(pg):
    await pg.click('[data-ev="tool_account"]'); await pg.wait_for_timeout(1000)   # back to the set
    # the plus appears once users/invited has answered — on a slow network that is later
    for _ in range(10):
        if await pg.evaluate("!!document.querySelector('.toolbar [data-ev=\"tool_invite\"]')"): break
        await pg.wait_for_timeout(500)
    else:
        return False
    await pg.click('.toolbar [data-ev="tool_invite"]'); await pg.wait_for_timeout(1500)
    # since /as-sub-tools (2026-09-02) the two ways in are sub-tool buttons in
    # the control row — a QR code and a keyboard — and the page is empty: no
    # doors, no list, no pencil in the row
    ways = await pg.evaluate(
        "Array.from(document.querySelectorAll('.toolbar .tool-button')).map(b => b.getAttribute('data-ev'))")
    doors = await pg.evaluate("document.querySelectorAll('.door').length")
    pencil = await pg.evaluate("!!document.querySelector('.toolbar [data-ctl=card_edit]')")
    ok = "invite_qr" in ways and "invite_name" in ways and doors == 0 and not pencil
    if not ok:
        print(f"      (row: {ways}, doors on the page: {doors}, pencil in the row: {pencil})")
    await go_home(pg)
    return ok


@step("the QR sheet draws a scannable code and puts itself away")
async def s_qr(pg):
    await pg.click('[data-ev="tool_account"]'); await pg.wait_for_timeout(1000)
    for _ in range(10):
        if await pg.evaluate("!!document.querySelector('.toolbar [data-ev=\"tool_invite\"]')"): break
        await pg.wait_for_timeout(500)
    else:
        return False
    await pg.click('.toolbar [data-ev="tool_invite"]'); await pg.wait_for_timeout(1200)
    # /as-sub-tools: the QR sub-tool in the row asks the rank first (team
    # preselected), then show
    await pg.click('.toolbar [data-ev="invite_qr"]'); await pg.wait_for_timeout(800)
    if not await pg.evaluate("(() => { const s = document.getElementById('doorSheet'); return !!s && getComputedStyle(s).display !== 'none'; })()"):
        print("      (the rank sheet did not open)"); await go_home(pg); return False
    await pg.click('#doorSheet .door-go'); await pg.wait_for_timeout(2000)
    drawn = await pg.evaluate(
        "!!document.querySelector('.qr-sheet .qr-code svg path')")
    # done must close the sheet WITHOUT closing the invite page under it
    await pg.click('[data-qr="done"]'); await pg.wait_for_timeout(1000)
    gone = await pg.evaluate("!document.querySelector('.qr-sheet')")
    # the level is kept: the invite tool is still the open one and its two ways
    # in are still in the row (the page itself is drawn by nothing now)
    still = await pg.evaluate(
        "!!document.querySelector('.toolbar [data-ev=\"invite_qr\"]')"
        " && !!document.querySelector('.toolbar [data-ev=\"invite_name\"]')")
    if not (drawn and gone and still):
        print(f"      (drawn: {drawn}, closed: {gone}, page kept: {still})")
    await go_home(pg)
    return drawn and gone and still


@step("posts: + opens the recording row (or records, or writes) and a post is made")
async def s_post(pg):
    await open_tool(pg, "posts")
    # /armed (2026-09-04): the plus opens a recording row one level down —
    # rec, stop, camera, publish level — and does not film. The row and the
    # level under it are walked with real clicks; the post itself is minted
    # through /new's own event, because a headless browser has no camera.
    if await pg.evaluate("!!document.querySelector('.toolbar [data-ev=\"tool_record\"]')"):
        shape = await pg.evaluate("!document.querySelector('.toolbar [data-ev=\"posts_new\"]') && !document.querySelector('.toolbar [data-ev=\"oneadd_pick\"]')")
        await pg.click('[data-ev="tool_record"]'); await pg.wait_for_timeout(1200)
        row = await pg.evaluate("""(() => {
            const q = (s) => !!document.querySelector('.toolbar ' + s);
            const level = q('[data-ev="tool_level"]') || q('[data-ev="armed_pick"]');
            return q('[data-ev="vid_rec"]') && q('[data-ev="armed_flip"]')
                && level && q('.armed-act.off')
                && !q('[data-ev="vid_stop"]');
        })()""")
        # /in-place (2026-09-04): the sliders pop the list up in the row rather
        # than descending. Either shape is walked the same way — open the list,
        # count the levels, put it away, and climb back to the posts.
        pop = await pg.evaluate("!!document.querySelector('.toolbar [data-ev=\"armed_pick\"]')")
        opener = '[data-ev="armed_pick"]' if pop else '[data-ev="tool_level"]'
        where = '.armed-pop' if pop else '.armed-page'
        await pg.click(opener); await pg.wait_for_timeout(1200)
        page = await pg.evaluate(f"document.querySelectorAll('{where} .armed-pill').length")
        if pop:
            # the row is untouched under the popover — that is the whole ask
            page = page if await pg.evaluate("!!document.querySelector('.toolbar [data-ev=\"vid_rec\"]') && !!document.querySelector('.toolbar [data-ev=\"armed_flip\"]')") else -1
        await pg.click('[data-ev="tools_home"]'); await pg.wait_for_timeout(1000)
        back = await pg.evaluate(f"!document.querySelector('{where}') && !!document.querySelector('.toolbar [data-ev=\"vid_rec\"]')")
        await pg.click('[data-ev="tools_home"]'); await pg.wait_for_timeout(1000)
        home = await pg.evaluate("!!document.querySelector('.toolbar [data-ev=\"tool_record\"]')")
        await pg.evaluate("feature_Loop.send({type:'CardNew', data:{owner:'_smoke', type:'post', title:'a post', t:Date.now()}})"); await pg.wait_for_timeout(2000)
        ok = shape and row and page == 7 and back and home and await pg.evaluate("!!document.querySelector('.card-page')")
        if not ok:
            print(f"      (shape={shape} row={row} pills={page} back={back} home={home})")
        await go_home(pg)
        return ok
    # /video-only (2026-09-03) without /armed: the add button records a video
    # and there is no kind chooser. A headless browser has no camera, so the
    # road is asserted by its shape and a post is minted through /new's own
    # event, as tests/sim/one-level.json does.
    if await pg.evaluate("!!document.querySelector('.toolbar [data-ev=\"vid_rec\"]') && !document.querySelector('.toolbar [data-ev=\"posts_new\"]')"):
        shape = await pg.evaluate("!document.querySelector('.toolbar [data-ev=\"oneadd_pick\"]') && !document.querySelector('.toolbar [data-ev=\"capture_photo\"]') && !document.querySelector('.toolbar [data-ev=\"dict_rec\"]')")
        await pg.evaluate("feature_Loop.send({type:'CardNew', data:{owner:'_smoke', type:'post', title:'a post', t:Date.now()}})"); await pg.wait_for_timeout(2000)
        ok = shape and await pg.evaluate("!!document.querySelector('.card-page')")
        await go_home(pg)
        return ok
    await pg.click('[data-ev="posts_new"]'); await pg.wait_for_timeout(2000)
    ok = await pg.evaluate("!!document.querySelector('.card-page .card-text[contenteditable=true]')")
    await go_home(pg)
    return ok


@step("projects: new makes a project with a people section")
async def s_project(pg):
    await open_tool(pg, "projects")
    await pg.click('[data-proj="new"]'); await pg.wait_for_timeout(2000)
    ok = await pg.evaluate("!!document.querySelector('.card-page') && /people/i.test(document.querySelector('.card-page').innerText)")
    if not ok: print("      (saw:", (await pg.evaluate("(document.querySelector('.card-page')||{innerText:'no card page'}).innerText")).replace('\n',' | ')[:100], ")")
    await go_home(pg)
    return ok


@step("the map is the only view: Leaflet mounts with nothing to switch to")
async def s_map(pg):
    # /map-only: there is no picker to press, so the map is what opening a
    # browse tool gives you — and there is no way to leave it for a grid.
    await open_tool(pg, "account")
    took = await until(pg, "!!document.querySelector('.leaflet-container') && getComputedStyle(document.querySelector('.leaflet-container')).display !== 'none'")
    print(f"      (map mounted in {took} ms)" if took >= 0 else "      (map never mounted)")
    ok = took >= 0
    if await pg.evaluate("!!document.querySelector('[data-ev=\"browse_grid\"]') || !!document.querySelector('[data-ev=\"browse_list\"]') || !!document.querySelector('[data-ev=\"browse_map\"]')"):
        print("      (a view-picker button is still drawn)"); ok = False
    await go_home(pg)
    return ok


@step("the time filter cuts the set: today drops last month's post, all brings it back")
async def s_since(pg):
    # /since. The post is minted through /new's own event and dated through
    # /post-time's CardWhen, as the post step does; the evidence is #mapData's
    # data-ids, which is the set the map draws AND the set the band lists.
    await open_tool(pg, "posts")
    if await pg.evaluate("document.querySelectorAll('.since-pill').length") != 4:
        print("      (the four pills are not on the strip)"); await dump(pg, "since-pills"); return False
    old = int(time.time() * 1000) - 40 * 86400000
    await pg.evaluate("(w) => feature_Loop.send({type:'CardNew', data:{owner:'_smoke', type:'post', title:'an old post', t:w}})", old)
    await pg.wait_for_timeout(1200)
    cid = await pg.evaluate("""(() => { const cs = JSON.parse(JSON.parse(feature_Loop.state).cards || '[]');
        const ps = cs.filter(c => c.type === 'post'); return ps.length ? ps[ps.length - 1].id : ''; })()""")
    await pg.evaluate("([id, w]) => feature_Loop.send({type:'CardWhen', data:{id:id, when:w, source:'photo', t:Date.now()}})", [cid, old])
    await pg.wait_for_timeout(1200)
    await go_home(pg)
    await open_tool(pg, "posts")

    async def ids():
        return await pg.evaluate("(() => { const d = document.getElementById('mapData'); return d ? (d.getAttribute('data-ids') || '') : ''; })()")

    async def pill(which):
        await pg.click(f'.since-pill[data-ev="since_{which}"]'); await pg.wait_for_timeout(1400)
        return await ids()

    under_all, under_today, back = await pill("all"), await pill("today"), await pill("all")
    ok = (cid in under_all.split(",")) and (cid not in under_today.split(",")) and (cid in back.split(","))
    if not ok:
        print(f"      (the old post {cid}: all={cid in under_all.split(',')} "
              f"today={cid in under_today.split(',')} back={cid in back.split(',')}; "
              f"all held {len(under_all.split(',')) if under_all else 0}, today held "
              f"{len(under_today.split(',')) if under_today else 0})")
        await dump(pg, "since")
    await go_home(pg)
    return ok


@step("the lozenge still opens the panel after all of that")
async def s_lozenge_again(pg):
    return await s_lozenge(pg)


async def passes(port: int, cookie: str) -> int:
    from playwright.async_api import async_playwright
    failures = 0
    async with async_playwright() as p:
        browser = await p.chromium.launch(channel="chrome")
        ctx = await browser.new_context(viewport={"width": 390, "height": 844})
        await ctx.add_cookies([{"name": "miso_auth", "value": cookie, "domain": "localhost", "path": "/"}])
        pg = await ctx.new_page()
        errors = []
        pg.on("pageerror", lambda e: errors.append(str(e)[:200]))
        # a page that dies mid-pass says so, by name: a renderer crash and a
        # close are different faults (2026-09-02, when a pass ended in a
        # string of TargetClosed errors and nobody could say which)
        pg.on("crash", lambda: print("  !! the page crashed (renderer)"))
        pg.on("close", lambda: print("  !! the page closed"))
        navs, logs = [], []
        pg.on("framenavigated", lambda f: navs.append(f.url) if f == pg.main_frame else None)
        pg.on("console", lambda m: logs.append(f"{m.type}: {m.text[:160]}"))
        for label in ("cold", "warm (world cache primed)", "throttled"):
            if label.startswith("throttled"):
                async def slow(route):
                    await asyncio.sleep(0.25)
                    await route.continue_()
                await ctx.route("**/*", slow)
            pg._smoke_navs, pg._smoke_logs = navs, logs
            await pg.goto(f"http://localhost:{port}/", wait_until="load")
            # wait for the loop itself, not a timer: under load the debug wasm
            # boots slowly and a fixed wait taps a page that has not started
            booted = False
            for _ in range(120):
                try:
                    if await pg.evaluate("typeof feature_Loop !== 'undefined' && feature_Loop.state !== null"): booted = True; break
                except Exception:
                    pass
                await pg.wait_for_timeout(250)
            if not booted:
                print(f"  [FAIL] the app did not boot within 30s ({label})"); failures += 1; continue
            await pg.wait_for_timeout(1500)
            print(f"== {label}")
            if not await pass_gate(pg):
                print("  [FAIL] the profile gate did not lift"); failures += 1; continue
            await go_home(pg)
            # start every pass unfiltered, whatever the last pass left behind:
            # /since's `period` is a USER var and the world outlives the pass
            await open_tool(pg, "account")
            await pg.evaluate("(() => { const v=document.querySelector('[data-ev=\"since_all\"]'); if (v) v.click(); })()"); await pg.wait_for_timeout(800)
            await go_home(pg)
            for name, fn in STEPS:
                try:
                    ok = await fn(pg)
                except Exception as e:
                    ok = False; print(f"      ({type(e).__name__}: {str(e)[:120]})")
                print(f"  [{'PASS' if ok else 'FAIL'}] {name}")
                failures += 0 if ok else 1
            if errors:
                print(f"  [FAIL] page errors: {errors[:3]}"); failures += 1; errors.clear()
        await browser.close()
    return failures


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", type=int, default=8140)
    ap.add_argument("--keep", action="store_true")
    a = ap.parse_args()
    if not os.environ.get("MISO_SMOKE_DIR"):
        global SCRATCH
        SCRATCH = pathlib.Path(f"/tmp/miso-smoke-{a.port}")
    if sh(["lsof", "-t", "-i", f":{a.port}", "-sTCP:LISTEN"]).stdout.strip():
        sys.exit(f"smoke: port {a.port} is busy — pass --port")
    binary = build_on(a.port)
    server = start(binary, a.port)
    try:
        cookie = login(a.port)
        failures = asyncio.run(passes(a.port, cookie))
    finally:
        if not a.keep or failures == 0:
            server.send_signal(signal.SIGTERM); server.wait(timeout=10)
    if failures:
        print(f"smoke: {failures} failure(s) — NOT shipping")
        sys.exit(1)
    print("smoke: all passes green")


if __name__ == "__main__":
    main()
