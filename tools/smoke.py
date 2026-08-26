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
    """an open tool shows only its own button: tap it until the launcher is back"""
    for _ in range(4):
        cur = await pg.evaluate("JSON.parse(feature_Loop.state).open_tool")
        if not cur: return
        await pg.click(f'[data-ev="tool_{cur}"]'); await pg.wait_for_timeout(900)


async def open_tool(pg, tool):
    if await pg.evaluate("JSON.parse(feature_Loop.state).open_tool") != tool:
        await go_home(pg)
        await pg.click(f'[data-ev="tool_{tool}"]')
    await pg.wait_for_timeout(1500)


async def until(pg, js, limit_ms=10000, every=200):
    """poll a condition; return the ms it took, or -1"""
    waited = 0
    while waited <= limit_ms:
        if await pg.evaluate(js): return waited
        await pg.wait_for_timeout(every); waited += every
    return -1


@step("the lozenge opens the system panel on a tap")
async def s_lozenge(pg):
    await pg.click("#build")
    took = await until(pg, "getComputedStyle(document.getElementById('panel')).display === 'block'")
    print(f"      (panel opened in {took} ms)" if took >= 0 else "      (panel never opened)")
    await pg.evaluate("feature_Panel.close()")
    return took >= 0


@step("👤 lands on the people surface with the picker")
async def s_people(pg):
    await open_tool(pg, "account")
    for _ in range(20):
        if await pg.evaluate("!!document.querySelector('.card-tile') && !!document.querySelector('.browse-picker')"): return True
        await pg.wait_for_timeout(500)
    return False


@step("the own tile opens an editable card and an edit is saved")
async def s_edit(pg):
    await pg.click(".card-tile"); await pg.wait_for_timeout(2000)
    # a card opens read-only since /editing: press edit first if it is offered
    # since /editing/toolbar the control is the pencil in the toolbar; the pill before it
    if await pg.evaluate("!!document.querySelector('.toolbar [data-ctl=card_edit]')"):
        await pg.dispatch_event('.toolbar [data-ctl=card_edit]', 'pointerdown'); await pg.wait_for_timeout(600)
    elif await pg.evaluate("(() => { const e=document.getElementById('cardEdit'); return !!e && e.classList.contains('show'); })()"):
        await pg.dispatch_event('#cardEdit', 'pointerdown'); await pg.wait_for_timeout(600)
    if not await pg.evaluate("!!document.querySelector('.card-page .card-text[contenteditable=true]')"): return False
    await pg.click(".card-text"); await pg.keyboard.press("End"); await pg.keyboard.type(" smoke"); await pg.click(".card-title"); await pg.wait_for_timeout(1500)
    txt = await pg.evaluate("(document.querySelector('.card-text')||{}).innerText || ''")
    return txt.endswith("smoke")


@step("undo lights after a card edit")
async def s_undo(pg):
    return not await pg.evaluate("document.querySelector('[data-ev=\"ctx_undo\"]').classList.contains('dim')")


@step("the admin sees the invite plus; a tap opens the invite page")
async def s_invite(pg):
    await pg.click('[data-ev="tool_account"]'); await pg.wait_for_timeout(1000)   # back to the set
    # the plus appears once users/invited has answered — on a slow network that is later
    for _ in range(10):
        if await pg.evaluate("!!document.querySelector('.toolbar [data-ev=\"tool_invite\"]')"): break
        await pg.wait_for_timeout(500)
    else:
        return False
    await pg.click('.toolbar [data-ev="tool_invite"]'); await pg.wait_for_timeout(1500)
    ok = await pg.evaluate("!!document.querySelector('.invite-page .invite-new')")
    await go_home(pg)
    return ok


@step("posts: + makes a post ready to write")
async def s_post(pg):
    await open_tool(pg, "posts")
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


@step("the map view mounts Leaflet")
async def s_map(pg):
    await open_tool(pg, "account")
    await pg.evaluate("(() => { const v=document.querySelector('[data-ev=\"browse_map\"]'); if (v) v.click(); })()")
    took = await until(pg, "!!document.querySelector('.leaflet-container') && getComputedStyle(document.querySelector('.leaflet-container')).display !== 'none'")
    print(f"      (map mounted in {took} ms)" if took >= 0 else "      (map never mounted)")
    ok = took >= 0
    await pg.evaluate("(() => { const v=document.querySelector('[data-ev=\"browse_grid\"]'); if (v) v.click(); })()"); await pg.wait_for_timeout(1200)
    # the view is a device var the world cache remembers across passes: leave it on the grid, proven
    if await pg.evaluate("!!document.querySelector('.leaflet-container') && getComputedStyle(document.querySelector('.leaflet-container')).display !== 'none'"):
        print("      (map still showing after switching to grid)"); ok = False
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
        for label in ("cold", "warm (world cache primed)", "throttled"):
            if label.startswith("throttled"):
                async def slow(route):
                    await asyncio.sleep(0.25)
                    await route.continue_()
                await ctx.route("**/*", slow)
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
            await go_home(pg)
            # start every pass on the grid, whatever the last pass left behind
            await open_tool(pg, "account")
            await pg.evaluate("(() => { const v=document.querySelector('[data-ev=\"browse_grid\"]'); if (v) v.click(); })()"); await pg.wait_for_timeout(800)
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
