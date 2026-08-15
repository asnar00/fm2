#!/usr/bin/env python3
"""Stamp an ask's lifecycle status from the builder's bench, LIVE.

Updates matching entries in the miso var store (user.*.asks.json) and
appends a per-user VarUpdate to the server's broadcast file — the same
file `publish` writes and every client long-polls — so open panels see
the status change within a beat, no relaunch (see
ask/lifecycle/being-built).

  stamp_ask.py --text "reset tap" --status building
  stamp_ask.py --text "reset tap" --status shipped --build 148
  stamp_ask.py --local ...          # dev store on this machine

Named risk, accepted: this and the server both write the broadcast
file; single-writer in practice."""

import argparse
import json
import subprocess
import sys

MINI = "microserver@microservers-Mac-mini.local"

REMOTE = r'''
import glob, json, sys
text, status, build = sys.argv[1], sys.argv[2], sys.argv[3]
for f in glob.glob("/tmp/miso-vars/user.*.asks.json"):
    d = json.load(open(f))
    asks = json.loads(d.get("v") or "[]")
    hit = False
    for a in asks:
        if text.lower() in str(a.get("text", "")).lower() and a.get("status") != status:
            a["status"] = status
            if build != "-":
                a["build"] = int(build)
            hit = True
    if not hit:
        continue
    value = json.dumps(asks)
    open(f, "w").write(json.dumps({"v": value}))
    tag = f.split("/")[-1][5:-10]
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
                "data": {"scope": "user", "key": "asks", "value": value}}})
    b["entries"] = b["entries"][-50:]
    open(bf, "w").write(json.dumps(b))
    print("stamped %s -> %s (broadcast v%d, aud user.%s)" % (f, status, v, tag))
'''


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--text", required=True, help="substring of the ask's text")
    ap.add_argument("--status", required=True,
                    choices=["asked", "proposed", "building", "shipped"])
    ap.add_argument("--build", default="-", help="build number to stamp (shipped)")
    ap.add_argument("--local", action="store_true", help="dev store on this machine")
    a = ap.parse_args()
    args = [a.text, a.status, str(a.build)]
    if a.local:
        r = subprocess.run([sys.executable, "-c", REMOTE] + args,
                           capture_output=True, text=True)
    else:
        r = subprocess.run(
            ["ssh", "-o", "BatchMode=yes", MINI,
             "python3 - " + " ".join("'" + x.replace("'", "'\\''") + "'" for x in args)],
            input=REMOTE, capture_output=True, text=True)
    print(r.stdout, end="")
    if r.returncode != 0:
        print(r.stderr, file=sys.stderr)
    return r.returncode


if __name__ == "__main__":
    sys.exit(main())
