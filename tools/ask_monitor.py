#!/usr/bin/env python3
"""Watch for field asks arriving, one printed line per event.

Post-migration (rung 7, build 207) an ask is not a file in a var store: it
is an op in its asker's world. Every write to the `asks` var lands as a
`set` line in `~/.miso-context/<user key>.log`, so the intake is a tail of
those logs rather than a poll of anything. This tails them on the mini over
one ssh connection (or locally with --local) and prints one line per ask
that is waiting for the builder — `asked` or `proposed` — plus a BACKLOG
line at startup for the ones already standing.

  python3 tools/ask_monitor.py                 # the mini
  python3 tools/ask_monitor.py --local         # this machine's state dir
  python3 tools/ask_monitor.py --all-statuses  # building/shipped too

Output lines (stdout, flushed per line — the Monitor event stream):

  ASK proposed user=phone:+44… t=1787346956331 tool=taps at=miso/loop/tap/counter
      text: All Toolsets should have an undo button
      proposal: All Toolsets should have an undo button

Rearm this at the start of every session (handover.md, tooling state); it
is the intake half of the flywheel — stamping is tools/stamp_ask.py.
"""

import argparse
import subprocess
import sys

import os
MINI = os.environ.get("MISO_HOST") or "microserver@microservers-Mac-mini.local"

# Runs on whichever machine holds the state dir; stdout is the event stream.
REMOTE = r'''
import glob, json, os, sys, time, urllib.parse

DIR = os.environ.get("MISO_CONTEXT_DIR") or os.path.expanduser("~/.miso-context")
WANT = set(sys.argv[1].split(","))

def say(*lines):
    for l in lines:
        sys.stdout.write(l + "\n")
    sys.stdout.flush()

def user_of(path):
    return urllib.parse.unquote(os.path.basename(path)[:-4])

def asks_in(line):
    """Every ask entry carried by one op line, or []."""
    if '"asks"' not in line:
        return []
    try:
        op = json.loads(line)
    except Exception:
        return []
    if op.get("name") != "asks":
        return []
    v = op.get("value")
    try:
        return json.loads(v) if isinstance(v, str) else (v or [])
    except Exception:
        return []

def report(user, a, tag):
    t = a.get("t")
    say("%s %s user=%s t=%s tool=%s at=%s" % (
            tag + (" URGENT" if a.get("urgency") == "urgent" else ""), a.get("status", "?"), user, t,
            a.get("tool") or "-", a.get("at") or "-"),
        "    text: %s" % (a.get("text") or "").replace("\n", " "),
        "    proposal: %s" % (a.get("proposal") or "-").replace("\n", " "))

seen = {}       # (user, t) -> the status last reported for that ask
stamps = {}     # path -> the (inode, size, mtime) last read

def sweep(prime):
    """Read every log that has moved, and report the asks that are new to us.

    A log is re-read whole rather than tailed, because `remember` writes one
    by temp file and rename: an open handle would keep reading the replaced
    inode and go silent after the first write, and compaction rewrites the
    file from the top anyway. A log is bounded (512 records, compacted), so
    the honest read is the cheap one; `seen` is what makes a re-read quiet.
    """
    for path in sorted(glob.glob(os.path.join(DIR, "*.log"))):
        if path.endswith("/_global.log"):   # the shared layer files no asks
            continue
        try:
            st = os.stat(path)
        except OSError:
            continue
        stamp = (st.st_ino, st.st_size, st.st_mtime)
        if stamps.get(path) == stamp:
            continue
        stamps[path] = stamp
        try:
            with open(path, "r") as f:
                lines = f.readlines()
        except OSError:
            continue
        user, standing = user_of(path), {}
        for line in lines:
            for a in asks_in(line):
                standing[a.get("t")] = a
        for t, a in sorted(standing.items(), key=lambda kv: kv[0] or 0):
            status = a.get("status")
            if seen.get((user, t)) == status:
                continue
            seen[(user, t)] = status
            if status in WANT:
                report(user, a, "BACKLOG" if prime else "ASK")

sweep(prime=True)
say("watching %s (%d worlds) for %s" % (DIR, len(stamps), ",".join(sorted(WANT))))

while True:
    time.sleep(0.5)
    sweep(prime=False)
'''


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--local", action="store_true",
                    help="watch this machine's state dir, not the mini's")
    ap.add_argument("--all-statuses", action="store_true",
                    help="also report building/shipped stamps")
    a = ap.parse_args()

    want = "asked,proposed,building,shipped" if a.all_statuses else "asked,proposed"
    if a.local:
        cmd = ["python3", "-u", "-c", REMOTE, want]
    else:
        cmd = ["ssh", "-o", "BatchMode=yes", "-o", "ServerAliveInterval=30",
               "-o", "ServerAliveCountMax=3", MINI,
               "python3 -u -c %s %s" % (shell_quote(REMOTE), want)]
    try:
        sys.exit(subprocess.call(cmd))
    except KeyboardInterrupt:
        sys.exit(0)


def shell_quote(s):
    return "'" + s.replace("'", "'\\''") + "'"


if __name__ == "__main__":
    main()
