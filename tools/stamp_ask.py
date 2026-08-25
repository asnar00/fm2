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

A did-you-mean asks instead of stamping (ask/lifecycle/did-you-mean):
the entry gets `status: "question"` and a question object, and the
asker answers it with one tap in their panel.

  stamp_ask.py --text "square" --question "did you mean the button's \\
      shape, or squaring the count?" --option shape="the button's shape" \\
      --option count="square the count" --likely shape \\
      --note "built the button shape for now — tap if you meant the count"

`--note` also rides along with a plain `--status` stamp, which is how a
hedge reaches the asker beside the build they did not quite ask for.
"""

import argparse
import json
import subprocess
import sys
import urllib.parse

import os
MINI = os.environ.get("MISO_HOST") or "microserver@microservers-Mac-mini.local"
ASKS_PATH = "miso/shell/panel/noob-button/ask"
CTX_DIR = "${MISO_CONTEXT_DIR:-$HOME/.miso-context}"


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
    ap.add_argument("--status",
                    choices=["asked", "proposed", "building", "shipped"])
    ap.add_argument("--build", default=None, type=int,
                    help="build number to stamp (shipped)")
    ap.add_argument("--question", help="ask the asker instead: the question text")
    ap.add_argument("--option", action="append", default=[], metavar="KEY=LABEL",
                    help="one reading, repeatable, order preserved")
    ap.add_argument("--likely", help="the option key silence would get built")
    ap.add_argument("--note", help="the builder's hedge, shown under the ask")
    ap.add_argument("--local", action="store_true", help="dev server on this machine")
    a = ap.parse_args()

    if bool(a.status) == bool(a.question):
        ap.error("give exactly one of --status and --question")
    question = None
    if a.question:
        if not a.option:
            ap.error("--question needs at least one --option KEY=LABEL")
        options = []
        for o in a.option:
            if "=" not in o:
                ap.error(f"--option {o!r} is not KEY=LABEL")
            key, label = o.split("=", 1)
            options.append({"key": key, "label": label})
        keys = [o["key"] for o in options]
        if len(set(keys)) != len(keys):
            ap.error("two options share a key")
        if a.likely and a.likely not in keys:
            ap.error(f"--likely {a.likely!r} is not one of the options")
        question = {"text": a.question, "options": options}
        if a.likely:
            question["likely"] = a.likely
    status = a.status or "question"

    # whose world holds this ask? the log filenames name every known user.
    ls = sh(f"ls {CTX_DIR}/*.log 2>/dev/null || true", a.local)
    users = [urllib.parse.unquote(f.split("/")[-1][:-4])
             for f in ls.split() if f.endswith(".log")]
    stamped = 0
    for user in users:
        if user == "_global":
            continue
        # the server does no percent-decoding (one parser to keep honest):
        # by-key takes the raw key, whose chars are all query- and shell-safe
        q = user
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
            if a.text.lower() not in str(entry.get("text", "")).lower():
                continue
            # a re-run that would change nothing stays quiet, as it always has
            same = (entry.get("status") == status
                    and (question is None or entry.get("question") == question)
                    and (a.note is None or entry.get("note") == a.note))
            if same:
                continue
            entry["status"] = status
            if a.build is not None:
                entry["build"] = a.build
            if question is not None:
                entry["question"] = question
            if a.note is not None:
                entry["note"] = a.note
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
        print(f"stamped {user}: -> {status}"
              + (f" (build {a.build})" if a.build is not None else "")
              + (f" ({len(question['options'])} readings)" if question else ""))
        stamped += 1
    if not stamped:
        sys.exit(f"stamp_ask: no ask matching {a.text!r} found in any world")
    return 0


if __name__ == "__main__":
    sys.exit(main())
