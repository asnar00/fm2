#!/usr/bin/env python3
"""Switch a feature's logging on or off, on a live device, from here.

Writes the user-scoped `feature_log` var in the mini's var store and
publishes it, so an open instance picks it up on its next long-poll — about
half a second. See features/miso/diag/logging.

  tools/set_log.py --on miso/loop/dictate      # hear the recorder and everything under it
  tools/set_log.py --off miso/loop/dictate
  tools/set_log.py --list                      # what is switched on
  tools/set_log.py --clear                     # silence everything

Then watch it arrive:
  ssh <mini> tail -f /tmp/miso-blackbox.log
"""

import argparse
import json
import subprocess
import sys

MINI = "microserver@microservers-Mac-mini.local"

REMOTE = r'''
import glob, json, sys
mode, path = sys.argv[1], sys.argv[2]
files = glob.glob("/tmp/miso-vars/user.*.feature_ticks.json")
users = sorted({f.split("/")[-1][5:-len(".feature_ticks.json")] for f in files})
if not users:
    print("no users in the var store"); sys.exit(1)
for tag in users:
    vf = "/tmp/miso-vars/user.%s.feature_log.json" % tag
    try:
        cur = json.loads(json.load(open(vf))["v"])
    except Exception:
        cur = {}
    if mode == "list":
        on = sorted(k for k, v in cur.items() if v)
        print("%s: %s" % (tag, ", ".join(on) if on else "(silent)"))
        continue
    if mode == "clear":
        cur = {}
    elif mode == "on":
        cur[path] = True
    else:
        cur.pop(path, None)
    value = json.dumps(cur)
    open(vf, "w").write(json.dumps({"v": value}))
    bf = "/tmp/miso-broadcast.json"
    try:
        b = json.load(open(bf))
    except Exception:
        b = {"v": 0, "entries": []}
    v = int(b.get("v", 0)) + 1
    b["v"] = v
    b.setdefault("entries", []).append({
        "v": v, "aud": "user." + tag,
        "msg": {"type": "VarUpdate",
                "data": {"scope": "user", "key": "feature_log", "value": value}}})
    b["entries"] = b["entries"][-50:]
    open(bf, "w").write(json.dumps(b))
    print("%s: %s (broadcast v%d)" % (tag, value, v))
'''


def main():
    ap = argparse.ArgumentParser()
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--on", metavar="PATH", help="switch this node's subtree on")
    g.add_argument("--off", metavar="PATH", help="switch this node's subtree off")
    g.add_argument("--list", action="store_true", help="show what is switched on")
    g.add_argument("--clear", action="store_true", help="silence everything")
    ap.add_argument("--host", default=MINI)
    a = ap.parse_args()

    if a.list:
        mode, path = "list", ""
    elif a.clear:
        mode, path = "clear", ""
    elif a.on:
        mode, path = "on", a.on
    else:
        mode, path = "off", a.off

    r = subprocess.run(
        ["ssh", "-o", "BatchMode=yes", a.host,
         "python3 - %s %s" % (mode, path or "-")],
        input=REMOTE, capture_output=True, text=True)
    sys.stdout.write(r.stdout)
    sys.stderr.write(r.stderr)
    return r.returncode


if __name__ == "__main__":
    sys.exit(main())
