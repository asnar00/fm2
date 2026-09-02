#!/usr/bin/env python3
"""The simulator rig: user-level tests of the installed app on an iPhone
simulator, at speed — real touches through idb, eyes through /readout (with
/rects), hands through /drive, the record through /blackbox — no screenshots
in the loop (one is taken on a failure).

  python3 tools/simrig.py readout                 # the screen as JSON (summary)
  python3 tools/simrig.py find '[ctl=card_edit]'  # where a selector is
  python3 tools/simrig.py tap '[ev=tool_posts]'   # a real finger on it
  python3 tools/simrig.py tapxy 201 814           # a real finger at a point
  python3 tools/simrig.py text 'hello'            # type on the sim keyboard
  python3 tools/simrig.py drive '{"tap":"#build"}' # a /drive command (synthetic)
  python3 tools/simrig.py js 'return feature_Loop.state.length' # run JS on the page, get its value
  python3 tools/simrig.py login _bob              # through the login page, by /drive
  python3 tools/simrig.py run tests/sim/pencil.json
  python3 tools/simrig.py shot name               # a screenshot to the evidence dir
  python3 tools/simrig.py native                  # is a native alert/sheet over the page?
  python3 tools/simrig.py press Allow             # press a native button by label

Selectors (a mini-language over the readout tree, first match depth-first):
  #id  .cls  [ev=…]  [ctl=…]  [face=…]  tag  text=…  — joined with spaces
  for descent: '.toolbar [ctl=card_edit]'. `hidden` nodes never match.

Environment: SIM_UDID (the device), MISO_PORT (the rig server, default 8099),
SIM_RIG_LOG (the rig server's log, for login codes), SIM_RIG_HOME (its HOME,
for the user list), SIM_EVIDENCE (dir).
Written 2026-08-26 (#p164a) after the pencil bug that three desktop rigs
missed and one phone black box explained.
"""
import json, os, pathlib, subprocess, sys, time, urllib.request

UDID = os.environ.get("SIM_UDID", "")
PORT = os.environ.get("MISO_PORT", "8099")
BASE = f"http://localhost:{PORT}"
LOG = os.environ.get("SIM_RIG_LOG", "")
EVIDENCE = pathlib.Path(os.environ.get("SIM_EVIDENCE", "/tmp/miso-simrig"))


def sh(args, **kw):
    return subprocess.run(args, capture_output=True, text=True, **kw)


def udid():
    if UDID:
        return UDID
    out = sh(["xcrun", "simctl", "list", "devices", "booted"]).stdout
    for line in out.splitlines():
        if "iPhone" in line and "Booted" in line:
            return line.split("(")[1].split(")")[0]
    sys.exit("simrig: no booted iPhone simulator (set SIM_UDID)")


# ---- eyes -------------------------------------------------------------------

def readout():
    try:
        with urllib.request.urlopen(f"{BASE}/diag/readout", timeout=5) as r:
            return json.loads(r.read().decode())
    except Exception as e:
        sys.exit(f"simrig: no readout from {BASE}: {e}")


def parse_sel(sel):
    steps = []
    for part in sel.split():
        want = {}
        rest = part
        while rest:
            if rest.startswith("#"):
                rest = rest[1:]; i = _end(rest); want["id"] = rest[:i]; rest = rest[i:]
            elif rest.startswith("."):
                rest = rest[1:]; i = _end(rest); want.setdefault("cls", []).append(rest[:i]); rest = rest[i:]
            elif rest.startswith("["):
                j = rest.index("]"); k, v = rest[1:j].split("=", 1); want[k] = v; rest = rest[j + 1:]
            elif rest.startswith("text="):
                want["text"] = rest[5:]; rest = ""
            else:
                i = _end(rest); want["tag"] = rest[:i]; rest = rest[i:]
        steps.append(want)
    return steps


def _end(s):
    for i, c in enumerate(s):
        if c in "#.[":
            return i
    return len(s)


def matches(node, want):
    if node.get("hidden"):
        return False
    for k, v in want.items():
        if k == "cls":
            have = set((node.get("cls") or "").split())
            if not all(c in have for c in v):
                return False
        elif k == "text":
            if v.lower() not in (node.get("text") or "").lower():
                return False
        elif str(node.get(k, "")) != v:
            return False
    return True


def find_all(tree, steps):
    """depth-first; each step must match a descendant of the previous match"""
    def walk(node, i, out):
        if node is None:
            return
        if matches(node, steps[i]):
            if i == len(steps) - 1:
                out.append(node)
            else:
                for k in node.get("kids", []):
                    walk(k, i + 1, out)
        for k in node.get("kids", []):
            walk(k, i, out)
    out = []
    walk(tree, 0, out)
    return out


def find(sel, snap=None):
    snap = snap or readout()
    body = snap.get("body") or {}
    hits = find_all(body, parse_sel(sel))
    return hits[0] if hits else None, body


def summary(node, depth=0, out=None, limit=400):
    out = [] if out is None else out
    if len(out) >= limit:
        return out
    bits = [node.get("tag", "?")]
    if node.get("id"): bits.append("#" + node["id"])
    if node.get("cls"): bits.append("." + ".".join(node["cls"].split()[:3]))
    if node.get("ev"): bits.append(f"[ev={node['ev']}]")
    if node.get("ctl"): bits.append(f"[ctl={node['ctl']}]")
    if node.get("face"): bits.append(f"[face={node['face']}]")
    if node.get("ce"): bits.append("[ce]")
    if node.get("hidden"): bits.append("(hidden)")
    if node.get("r"): bits.append(str(node["r"]))
    if node.get("text"): bits.append(repr(node["text"][:50]))
    if node.get("tag") == "body": bits.append(f"vv={node.get('vv')} sy={node.get('sy')} screen={node.get('screen')} focus={node.get('focus')!r}")
    out.append("  " * depth + " ".join(bits))
    for k in node.get("kids", []):
        summary(k, depth + 1, out, limit)
    return out


# ---- hands ------------------------------------------------------------------

def tapxy(x, y):
    r = sh(["idb", "ui", "tap", "--udid", udid(), str(int(x)), str(int(y))])
    if r.returncode != 0:
        sys.exit(f"simrig: idb tap failed: {r.stderr.strip()[:200]}")


def tap(sel, snap=None):
    node, body = find(sel, snap)
    if not node or not node.get("r"):
        return False
    x, y, w, h = node["r"]
    # the rectangle is layout-viewport; if the keyboard has moved the screen
    # (visual viewport offset) the point on glass is lower by that much
    vv = body.get("vv") or [0, 0]
    sc = body.get("screen") or [0, 0, 0, 0]
    inset = max(0, (sc[1] or 0) - (sc[3] or 0)) if sc[1] and sc[3] else 0   # the status bar above the web view
    # a target the size of the screen (the shade behind a sheet) has its
    # centre under whatever sits on it — the panel — so the finger goes near
    # the bottom-left corner instead, where only the target is (the
    # credits-button review, 2026-09-02: a tap on #shade landed on the panel)
    if w >= 0.9 * (sc[2] or w) and h >= 0.6 * (sc[3] or h):
        px, py = x + 24, y + h - 40 - (vv[0] or 0) + inset
    else:
        px, py = x + w / 2, y + h / 2 - (vv[0] or 0) + inset
    print(f"      (finger at {int(px)},{int(py)}: rect {node['r']} vv {vv} inset {inset})")
    tapxy(px, py)
    return True


# ---- the native eye ---------------------------------------------------------
# the readout sees the DOM; a permission alert, the passkey sheet, a share
# sheet are native and invisible to it. `idb ui describe-point` names the
# native element under a point, so the rig can see them without a screenshot.

def describe(x, y):
    r = sh(["idb", "ui", "describe-point", "--udid", udid(), str(int(x)), str(int(y))])
    try:
        return json.loads(r.stdout)
    except Exception:
        return {}


PREFER = ["Allow While Using App", "Allow", "Cancel", "Not Now", "OK", "Don\u2019t Allow", "Don't Allow", "Continue"]


def native_alert():
    """is something native covering the page? the element at the screen's
    centre is the web view unless an alert is up"""
    d = describe(201, 437)
    if not d:
        return None
    if (d.get("type") == "Application") or (d.get("AXLabel") == "Web"):
        return None
    if "WebContent" in (d.get("traits") or []):     # the page itself
        return None
    return d


def press_native(labels=PREFER, ys=range(300, 860, 32), xs=(70, 140, 201, 262, 332)):
    """scan the alert band for a button with one of the labels; press the first found"""
    seen = {}
    for y in ys:
        for x in xs:
            d = describe(x, y)
            lab = d.get("AXLabel") or ""
            if d.get("type") == "Button" and lab and lab not in seen:
                seen[lab] = d.get("frame") or {}
    for want in labels:
        if want in seen:
            f = seen[want]
            tapxy(f["x"] + f["width"] / 2, f["y"] + f["height"] / 2)
            print(f"      (native: pressed {want!r})")
            return want
    if seen:
        print(f"      (native buttons seen: {list(seen)})")
    return None


def native_guard():
    d = native_alert()
    if d:
        print(f"      (native element over the page: {d.get('type')} {d.get('AXLabel')!r})")
        press_native()
        time.sleep(1.2)


def text(s):
    sh(["idb", "ui", "text", "--udid", udid(), s])


def key(code):
    sh(["idb", "ui", "key", "--udid", udid(), str(code)])


def drive(cmd):
    data = json.dumps(cmd).encode()
    req = urllib.request.Request(f"{BASE}/diag/drive", data=data, method="POST",
                                 headers={"content-type": "application/json"})
    with urllib.request.urlopen(req, timeout=5) as r:
        return r.read().decode()


def js(code, timeout=6.0):
    """run a script on the page (rig only) and return its value"""
    before = readout().get("t")
    drive({"js": code})
    t0 = time.time()
    while time.time() - t0 < timeout:
        snap = readout()
        if snap.get("t") != before and "js" in snap:
            return snap.get("js")
        time.sleep(0.1)
    return None


def shot(name):
    EVIDENCE.mkdir(parents=True, exist_ok=True)
    p = EVIDENCE / f"{name}.png"
    sh(["xcrun", "simctl", "io", udid(), "screenshot", str(p)])
    return p


def wait_for(sel, timeout=8.0, absent=False):
    t0 = time.time()
    while time.time() - t0 < timeout:
        node, _ = find(sel)
        if (node is not None) != absent:
            return node
        time.sleep(0.15)
    return None


# ---- the login page, driven ------------------------------------------------

def phone_of(name):
    """the rig's own user list (SIM_RIG_HOME/.miso-auth/users.json), else the name as a phone"""
    home = os.environ.get("SIM_RIG_HOME", "")
    try:
        for u in json.loads((pathlib.Path(home) / ".miso-auth" / "users.json").read_text()):
            if u.get("name") == name:
                return u.get("phone")
    except Exception:
        pass
    return name


def login(name):
    phone = phone_of(name)
    drive({"type": "#phone", "value": phone})
    time.sleep(0.4)
    drive({"tap": "#phoneStep button"})
    pin = ""
    for _ in range(40):
        time.sleep(0.25)
        try:
            for line in pathlib.Path(LOG).read_text().splitlines():
                if f"test user {name} pin" in line:
                    pin = line.split()[-1]
        except Exception:
            pass
        if pin and wait_for("#pin", 0.1):
            break
    if not pin:
        sys.exit("simrig: no login code in the rig log (SIM_RIG_LOG)")
    drive({"type": "#pin", "value": pin})
    print(f"login {name}: code {pin} typed")


# ---- the device -------------------------------------------------------------

def relaunch():
    """a cold launch of the installed app: iOS keeps a home-screen app alive
    in the background, so tapping its icon reloads nothing — a reboot does"""
    u = udid()
    sh(["xcrun", "simctl", "shutdown", u]); time.sleep(2)
    sh(["xcrun", "simctl", "boot", u]); sh(["xcrun", "simctl", "bootstatus", u, "-b"]); time.sleep(8)
    # a Safari tab restored at boot answers the drive door instead of the app
    sh(["xcrun", "simctl", "terminate", u, "com.apple.mobilesafari"])
    sh(["idb", "ui", "button", "--udid", u, "HOME"]); time.sleep(1)
    tapxy(201, 717); time.sleep(2)             # the home screen's Search pill
    text("miso"); time.sleep(3)
    # Spotlight's Top Hit row, by label: a ghost tile or another app can sit
    # first, so the miso clip is the rightmost tile labelled "miso" there
    hit = None
    try:
        tree = json.loads(sh(["idb", "ui", "describe-all", "--udid", u]).stdout or "[]")
        # a tile is square (Safari's "miso" suggestion rows are wide); the
        # Top Hit row is preferred, and there the rightmost wins over a ghost
        tiles = [e for e in tree if (e.get("AXLabel") or "").strip() == "miso"
                 and e.get("frame", {}).get("width", 999) < 120]
        top = sorted([e for e in tiles if e["frame"]["y"] < 200], key=lambda e: e["frame"]["x"])
        rest = sorted([e for e in tiles if e["frame"]["y"] >= 200], key=lambda e: e["frame"]["y"])
        hit = top[-1] if top else (rest[0] if rest else None)
    except Exception:
        hit = None
    if hit:
        f = hit["frame"]; tapxy(int(f["x"] + f["width"] / 2), int(f["y"] + f["height"] / 2))
    else:
        tapxy(63, 149)
    time.sleep(10)
    return readout().get("url")


def home():
    """back to the launcher, by the loop (setup, not the interaction under test)"""
    drive({"send": {"type": "click", "ev": "tools_home"}}); time.sleep(0.8)


# ---- scripts ----------------------------------------------------------------

def check(a, snap):
    body = snap.get("body") or {}
    sel = a.get("find")
    node = find_all(body, parse_sel(sel))[0] if sel and find_all(body, parse_sel(sel)) else None
    if a.get("exists") is False:
        return node is None, f"{sel} absent"
    if node is None:
        return False, f"{sel} not found"
    if "text" in a and a["text"].lower() not in (node.get("text") or "").lower():
        return False, f"{sel} text {node.get('text')!r} != {a['text']!r}"
    if "face" in a and node.get("face") != a["face"]:
        return False, f"{sel} face {node.get('face')} != {a['face']}"
    if "ce" in a and bool(node.get("ce")) != a["ce"]:
        return False, f"{sel} editable {bool(node.get('ce'))} != {a['ce']}"
    return True, f"{sel} ok"


def run(path):
    steps = json.loads(pathlib.Path(path).read_text())
    name = pathlib.Path(path).stem
    fails = 0
    for i, st in enumerate(steps):
        label = f"{name}[{i}]"
        if "tap" in st:
            native_guard()
            ok = tap(st["tap"])
            print(f"  tap {st['tap']}: {'ok' if ok else 'NOT FOUND'}")
            if not ok: fails += 1; shot(f"{label}-notfound")
        elif "tapxy" in st:
            tapxy(*st["tapxy"]); print(f"  tap at {st['tapxy']}")
        elif "text" in st:
            text(st["text"]); print(f"  text {st['text']!r}")
        elif "drive" in st:
            drive(st["drive"]); print(f"  drive {st['drive']}")
        elif "js" in st:
            v = js(st["js"]); print(f"  js -> {json.dumps(v)[:120]}")
            if "expect" in st and v != st["expect"]:
                print(f"  [FAIL] {st.get('name', st['js'][:60])} — got {json.dumps(v)[:80]}"); fails += 1; shot(f"{label}-fail")
            elif "expect" in st:
                print(f"  [PASS] {st.get('name', st['js'][:60])}")
        elif "login" in st:
            login(st["login"])
        elif "home" in st:
            home(); print("  home")
        elif "relaunch" in st:
            print(f"  relaunch -> {relaunch()}")
        elif "wait" in st:
            time.sleep(st["wait"] / 1000)
        elif "wait_for" in st:
            node = wait_for(st["wait_for"], st.get("timeout", 8))
            print(f"  wait_for {st['wait_for']}: {'ok' if node else 'TIMEOUT'}")
            if not node: fails += 1; shot(f"{label}-timeout")
        elif "assert" in st:
            time.sleep(0.4)
            ok, why = check(st["assert"], readout())
            print(f"  [{'PASS' if ok else 'FAIL'}] {st.get('name', why)}" + ("" if ok else f" — {why}"))
            if not ok: fails += 1; shot(f"{label}-fail")
        elif "shot" in st:
            print(f"  shot {shot(st['shot'])}")
        elif "note" in st:
            print(f"  -- {st['note']}")
    print(f"{name}: {'all green' if not fails else f'{fails} failure(s)'}")
    return fails


def main():
    a = sys.argv[1:]
    if not a:
        sys.exit(__doc__)
    cmd, rest = a[0], a[1:]
    if cmd == "readout":
        print("\n".join(summary(readout().get("body") or {})))
    elif cmd == "find":
        node, _ = find(rest[0]); print(json.dumps(node)[:400] if node else "not found")
    elif cmd == "tap":
        print("ok" if tap(rest[0]) else "not found")
    elif cmd == "native":
        print(native_alert() or "nothing native over the page")
    elif cmd == "press":
        print(press_native([rest[0]] if rest else PREFER))
    elif cmd == "tapxy":
        tapxy(rest[0], rest[1])
    elif cmd == "text":
        text(rest[0])
    elif cmd == "js":
        print(json.dumps(js(rest[0])))
    elif cmd == "drive":
        print(drive(json.loads(rest[0])))
    elif cmd == "relaunch":
        print(relaunch())
    elif cmd == "home":
        home()
    elif cmd == "login":
        login(rest[0])
    elif cmd == "shot":
        print(shot(rest[0]))
    elif cmd == "run":
        sys.exit(1 if sum(run(p) for p in rest) else 0)
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main()
