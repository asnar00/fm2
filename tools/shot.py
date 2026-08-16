#!/usr/bin/env python3
"""Screenshot a live miso page: shot.py <port> <out.png> [css-selector]

Without a selector, captures the viewport. With one, captures that element's
box. Exists because "the tiles loaded" is not "the map looks right", and
until now there was no way to answer the second question.
"""
import base64, json, sys, urllib.request
import websocket

port, out = sys.argv[1], sys.argv[2]
sel = sys.argv[3] if len(sys.argv) > 3 else None

tabs = json.loads(urllib.request.urlopen(f"http://127.0.0.1:{port}/json/list").read())
page = next(t for t in tabs if t["type"] == "page")
ws = websocket.create_connection(page["webSocketDebuggerUrl"], timeout=60)
mid = 0


def call(method, params=None):
    global mid
    mid += 1
    ws.send(json.dumps({"id": mid, "method": method, "params": params or {}}))
    while True:
        m = json.loads(ws.recv())
        if m.get("id") == mid:
            if "error" in m:
                sys.exit(f"{method}: {m['error']}")
            return m.get("result", {})


call("Page.enable")
params = {"format": "png"}
if sel:
    r = call("Runtime.evaluate", {"expression": f"""(() => {{
        const e = document.querySelector({json.dumps(sel)});
        if (!e) return null;
        const b = e.getBoundingClientRect();
        return {{x: b.x, y: b.y, width: b.width, height: b.height}};
    }})()""", "returnByValue": True})
    box = r.get("result", {}).get("value")
    if not box:
        sys.exit(f"no element matching {sel}")
    params["clip"] = {**box, "scale": 2}

png = call("Page.captureScreenshot", params)["data"]
open(out, "wb").write(base64.b64decode(png))
print(f"wrote {out}")
ws.close()
