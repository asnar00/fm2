#!/usr/bin/env python3
"""Drive a live miso page: single pokes, or scripted demos with assertions.

Usage:
  drive.py tap '#build'                     click any element
  drive.py send '{"type":"click","ev":"tap"}'   event through the Rust loop
  drive.py type '#phone' '+44...'           fill an input
  drive.py readout                          print the current screen as JSON
  drive.py run demos/<name>.json            perform a scripted demo

Scripts are JSON step lists:
  {"send": {...}} | {"tap": "sel"} | {"type": "sel", "value": "v"}
  {"wait": ms}
  {"assert": {"find": {"ev": "tap"}, "text": "taps: 3"}}
     find matches on any of tag/id/cls/ev; checks: text, text_starts,
     hidden (true/false), exists (false = must be absent)
The page must be open with ?drive=1&readout=1 (replay/demo URLs include them).
"""

import json
import os
import sys
import time
import urllib.request

# --base https://miso.xn--nb-lkaa.org and --cookie "miso_auth=..." (or env
# MISO_BASE / MISO_COOKIE) drive an instance through the tunnel, which gates
# the drive/readout endpoints
BASE = os.environ.get("MISO_BASE", "http://localhost:8095")
COOKIE = os.environ.get("MISO_COOKIE", "")


def request(path, body=None):
    req = urllib.request.Request(BASE + path,
        data=json.dumps(body).encode() if body is not None else None,
        method="POST" if body is not None else "GET")
    if COOKIE:
        req.add_header("Cookie", COOKIE)
    return json.loads(urllib.request.urlopen(req).read())


def post(path, body):
    return request(path, body)


def get(path):
    return request(path)


def find(node, want):
    if all(node.get(k) == v for k, v in want.items()):
        yield node
    for kid in node.get("kids", []):
        yield from find(kid, want)


def check(spec):
    snap = get("/diag/readout")
    node = next(find(snap.get("body", {}), spec["find"]), None)
    if spec.get("exists") is False:
        return (node is None,
                f"expected absent: {spec['find']}" if node else "absent as expected")
    if node is None:
        return False, f"not found: {spec['find']}"
    if "text" in spec and node.get("text") != spec["text"]:
        return False, f"text is {node.get('text')!r}, wanted {spec['text']!r}"
    if "text_starts" in spec and not (node.get("text") or "").startswith(spec["text_starts"]):
        return False, f"text is {node.get('text')!r}, wanted prefix {spec['text_starts']!r}"
    if "hidden" in spec and bool(node.get("hidden")) != spec["hidden"]:
        return False, f"hidden is {bool(node.get('hidden'))}, wanted {spec['hidden']}"
    return True, json.dumps(node)


def run_script(path):
    steps = json.load(open(path))
    failures = 0
    for i, step in enumerate(steps):
        if "wait" in step:
            time.sleep(step["wait"] / 1000)
        elif "assert" in step:
            ok, detail = check(step["assert"])
            print(f"  {'PASS' if ok else 'FAIL'}  step {i}: {detail}")
            failures += 0 if ok else 1
        else:
            post("/diag/drive", step)
            time.sleep(0.6)   # poll interval + readout debounce
    print(f"{path}: {'ALL PASSED' if not failures else f'{failures} FAILURE(S)'}")
    sys.exit(1 if failures else 0)


def main():
    global BASE, COOKIE
    args = sys.argv[1:]
    while args and args[0].startswith("--"):
        if args[0] == "--base":
            BASE = args[1]; args = args[2:]
        elif args[0] == "--cookie":
            COOKIE = args[1]; args = args[2:]
        else:
            break
    if not args:
        print(__doc__)
        return
    cmd = args[0]
    if cmd == "tap":
        post("/diag/drive", {"tap": args[1]})
    elif cmd == "send":
        post("/diag/drive", {"send": json.loads(args[1])})
    elif cmd == "type":
        post("/diag/drive", {"type": args[1], "value": args[2]})
    elif cmd == "readout":
        print(json.dumps(get("/diag/readout"), indent=1))
    elif cmd == "run":
        run_script(args[1])
    else:
        print(__doc__)


if __name__ == "__main__":
    main()
