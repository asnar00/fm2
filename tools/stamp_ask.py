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

import argparse, time
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


PORT = os.environ.get("MISO_PORT", "8095")
BUILDS_PATH = "miso/shell/panel/noob-button/ask/lifecycle/being-built/announced"


def builds_read(local):
    """the global `builds` list. The door writes the shared layer but reads it
    as a fresh world, so the current list is the last `set` in the layer's own
    log (rig-found, 2026-08-26). Shared with tools/stamp_ship.py, which stamps
    the same list from the deploy (ask/lifecycle/being-built/announced/
    by-the-ship)."""
    last = sh(f"grep '\"name\":\"builds\"' {CTX_DIR}/_global.log 2>/dev/null | tail -1 || true", local)
    if not last.strip():
        return []
    try:
        op = json.loads(last.strip())
    except json.JSONDecodeError:
        sys.exit(f"stamp_ask: bad op in _global.log: {last[:120]}")
    if op.get("path") != BUILDS_PATH:
        return []
    return json.loads(op.get("value") or "[]")


def builds_write(builds, local):
    builds = sorted(builds, key=lambda b: b.get("t", 0))[-40:]
    body = json.dumps({"path": BUILDS_PATH, "name": "builds", "value": json.dumps(builds)})
    body_sh = body.replace("'", "'\\''")
    out = sh(f"curl -s -X POST 'localhost:{PORT}/diag/context?user=_global' -d '{body_sh}'", local)
    if '"ok":true' not in out:
        sys.exit(f"stamp_ask: POST refused for _global: {out}")


def announce(a):
    """one entry per announced build, keyed by its words — and, since
    /by-the-ship, carrying the node it will ship as, so the deploy can close it
    without the words being typed a second time."""
    builds = builds_read(a.local)
    key = a.announce.strip().lower()
    entry = next((b for b in builds if str(b.get("text", "")).strip().lower() == key), None)
    if entry is None:
        entry = {"t": int(time.time() * 1000), "text": a.announce.strip()}
        builds.append(entry)
    same = (entry.get("status") == a.status
            and (a.build is None or entry.get("build") == a.build)
            and (a.node is None or entry.get("node") == a.node))
    if same:
        print(f"announced already: {a.status}")
        return
    entry["status"] = a.status
    if a.node is not None:
        entry["node"] = a.node
    if a.build is not None:
        entry["build"] = a.build
    elif a.status == "building":
        entry.pop("build", None)
    builds_write(builds, a.local)
    print(f"announced: {entry['text']!r} -> {a.status}"
          + (f" (build {a.build})" if a.build is not None else "")
          + (f" [node {entry['node']}]" if entry.get("node") else ""))
    if a.status == "building" and not entry.get("node"):
        # /by-the-ship: without a node the deploy cannot close this entry, and
        # it will sit on everyone's sheet until somebody remembers it — which
        # is the thing that went wrong on 2026-09-04 (field-walk #p143)
        print("  WARNING: no --node given, so this announcement will need a hand "
              "at ship time. Every deploy will list it until it has one.")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--text", help="substring of the ask's text")
    ap.add_argument("--announce", metavar="TEXT",
                    help="a build asked for in conversation: put it on everyone's sheet "
                         "(global `builds` var); the words match a later shipping call")
    ap.add_argument("--node", metavar="PATH",
                    help="the feature node this announcement will ship as, e.g. "
                         "browse/map-only/since — the deploy stamps it shipped when a "
                         "release touches that node or one under it, so the words never "
                         "have to be typed twice (ask/lifecycle/being-built/announced/"
                         "by-the-ship)")
    ap.add_argument("--status",
                    choices=["asked", "proposed", "building", "shipped"])
    ap.add_argument("--build", default=None, type=int,
                    help="build number to stamp (shipped)")
    ap.add_argument("--question", help="ask the asker instead: the question text")
    ap.add_argument("--option", action="append", default=[], metavar="KEY=LABEL",
                    help="one reading, repeatable, order preserved")
    ap.add_argument("--likely", help="the option key silence would get built")
    ap.add_argument("--note", help="the builder's hedge, shown under the ask")
    ap.add_argument("--only-if", dest="only_if", metavar="STATUS",
                    choices=["asked", "proposed", "building", "shipped", "question"],
                    help="stamp only an ask still at this status; one already "
                         "past it is left alone and said so. The automatic ack "
                         "passes --only-if asked so it can never write over a "
                         "stamp a person made later.")
    ap.add_argument("--local", action="store_true", help="dev server on this machine")
    a = ap.parse_args()

    if bool(a.status) == bool(a.question):
        ap.error("give exactly one of --status and --question")
    if bool(a.text) == bool(a.announce):
        ap.error("give exactly one of --text and --announce")
    if a.node and not a.announce:
        ap.error("--node belongs to an announcement (--announce)")
    if a.announce:
        if a.only_if:
            ap.error("--only-if is for an ask, not an announcement")
        if a.status not in ("building", "shipped"):
            ap.error("--announce takes --status building or shipped")
        return announce(a)
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
        snap = sh(f"curl -s 'localhost:{PORT}/diag/context?user={q}'", a.local)
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
        left = 0
        for entry in asks:
            if a.text.lower() not in str(entry.get("text", "")).lower():
                continue
            # an automatic stamp says which status it expects to find, so it
            # can never write over a stamp a person made later
            # (ask/lifecycle/being-built/stamp-stands)
            if a.only_if and str(entry.get("status") or "asked") != a.only_if:
                left += 1
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
            if left:
                print(f"left alone {user}: {left} already past {a.only_if}")
                stamped += 1
            continue
        body = json.dumps({"path": ASKS_PATH, "name": "asks",
                           "value": json.dumps(asks)})
        body_sh = body.replace("'", "'\\''")
        out = sh(f"curl -s -X POST 'localhost:{PORT}/diag/context?user={q}' "
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
