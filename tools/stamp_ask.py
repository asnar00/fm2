#!/usr/bin/env python3
"""Stamp an ask's lifecycle status from the builder's bench, LIVE.

Post-migration (rung 7, build 207): asks live in each user's context
world, not the old var store. This finds the user whose `asks` var
holds the text, updates the matching entries, and writes back through
`POST /diag/context` — the same door as any edit — so the op applies to
the world, lands in the op log (remember), and relays to the user's
open panels within a beat (converge). Rewritten 2026-08-21 (hybrid
#p68) after the old store-writing version stamped into the void.

  stamp_ask.py --text "reset tap" --status building
  stamp_ask.py --text "reset tap" --status shipped --build 148
  stamp_ask.py --local ...          # dev server on this machine
"""

import argparse
import json
import subprocess
import sys
import urllib.parse

MINI = "microserver@microservers-Mac-mini.local"
ASKS_PATH = "miso/shell/panel/noob-button/ask"


def sh(cmd, local):
    if local:
        r = subprocess.run(["bash", "-c", cmd], capture_output=True, text=True)
    else:
        r = subprocess.run(["ssh", "-o", "BatchMode=yes", MINI, cmd],
                           capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f"stamp_ask: {cmd!r} failed: {r.stderr.strip()}")
    return r.stdout


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--text", required=True, help="substring of the ask's text")
    ap.add_argument("--status", required=True,
                    choices=["asked", "proposed", "building", "shipped"])
    ap.add_argument("--build", default=None, type=int,
                    help="build number to stamp (shipped)")
    ap.add_argument("--local", action="store_true", help="dev server on this machine")
    a = ap.parse_args()

    # whose world holds this ask? the log filenames name every known user.
    ls = sh("ls ~/.miso-context/*.log 2>/dev/null || true", a.local)
    users = [urllib.parse.unquote(f.split("/")[-1][:-4])
             for f in ls.split() if f.endswith(".log")]
    stamped = 0
    for user in users:
        if user == "_global":
            continue
        q = urllib.parse.quote(user, safe="")
        snap = sh(f"curl -s 'localhost:8095/diag/context?user={q}'", a.local)
        try:
            vars_ = json.loads(snap)
        except json.JSONDecodeError:
            sys.exit(f"stamp_ask: bad snapshot for {user}: {snap[:120]}")
        row = next((v for v in vars_
                    if v["name"] == "asks" and v["path"] == ASKS_PATH), None)
        if not row:
            continue
        asks = json.loads(row.get("value") or "[]")
        hit = False
        for entry in asks:
            if (a.text.lower() in str(entry.get("text", "")).lower()
                    and entry.get("status") != a.status):
                entry["status"] = a.status
                if a.build is not None:
                    entry["build"] = a.build
                hit = True
        if not hit:
            continue
        body = json.dumps({"path": ASKS_PATH, "name": "asks",
                           "value": json.dumps(asks)})
        body_sh = body.replace("'", "'\\''")
        out = sh(f"curl -s -X POST 'localhost:8095/diag/context?user={q}' "
                 f"-d '{body_sh}'", a.local)
        if '"ok":true' not in out:
            sys.exit(f"stamp_ask: POST refused for {user}: {out}")
        print(f"stamped {user}: -> {a.status}"
              + (f" (build {a.build})" if a.build is not None else ""))
        stamped += 1
    if not stamped:
        sys.exit(f"stamp_ask: no ask matching {a.text!r} found in any world")
    return 0


if __name__ == "__main__":
    sys.exit(main())
