#!/usr/bin/env python3
"""The tweak digest: how asks were refined after they shipped, so the next
build can be shaped the way the asker likes by default (ash, 2026-09-03,
housekeeping #p30–#p32).

The tree is the record: every node carries the ask that made it (its
provenance quote), and a refinement is a child node born after its parent.
This walks features/, dates each node by its first commit, and prints every
parent whose children arrived within the window after it — the ask, then
each thing the asker wanted changed, in order. All of history by default.

  python3 tools/tweaks.py                  # every refinement, oldest first
  python3 tools/tweaks.py --within 3        # children born within 3 days of the parent
  python3 tools/tweaks.py --since 2026-09-01   # parents born on or after
  python3 tools/tweaks.py --user ash       # quotes filed by / mentioning a user only

Read the digest, then distil what it teaches into
features/miso/shell/taste/learned/learned.agent.md (the skillset reads it).
"""
import argparse, datetime, os, re, subprocess

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
FEATURES = os.path.join(ROOT, "features")


def nodes():
    out = []
    for dirpath, dirs, files in os.walk(FEATURES):
        dirs[:] = [d for d in dirs if not d.startswith(".") and d != "assets"]
        name = os.path.basename(dirpath)
        if name + ".md" in files:
            out.append(dirpath)
    return out


def born(path):
    # the node's first commit anywhere in git history (renames followed by
    # the spec file, which keeps its name through a regroup)
    name = os.path.basename(path)
    r = subprocess.run(["git", "log", "--follow", "--diff-filter=A", "--format=%at",
                        "--", os.path.join(path, name + ".md")],
                       cwd=ROOT, capture_output=True, text=True).stdout.split()
    return int(r[-1]) if r else 0


def quote(path):
    name = os.path.basename(path)
    lines, tail = [], ""
    for line in open(os.path.join(path, name + ".md"), encoding="utf-8"):
        if line.startswith("> *(") or line.startswith("> ("):
            tail += " " + line[2:].strip()
            continue
        if line.startswith("> "):
            lines.append(line[2:].strip())
        elif lines and not line.startswith(">"):
            break
    return (" ".join(lines)[:420] or "(no quote)"), tail.strip()


def rel(path):
    return os.path.relpath(path, FEATURES)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--within", type=float, default=14, help="days after the parent")
    ap.add_argument("--since", help="parents born on/after YYYY-MM-DD")
    ap.add_argument("--user", help="only quotes filed by or naming this user")
    a = ap.parse_args()
    all_nodes = nodes()
    t = {p: born(p) for p in all_nodes}
    since = 0
    if a.since:
        since = int(datetime.datetime.strptime(a.since, "%Y-%m-%d").timestamp())
    groups = []
    for p in sorted(all_nodes, key=lambda x: t[x]):
        if t[p] < since:
            continue
        kids = [c for c in all_nodes if os.path.dirname(c) == p
                and t[c] > t[p] and (t[c] - t[p]) <= a.within * 86400]
        if not kids:
            continue
        q, tail = quote(p)
        if a.user and a.user not in (q + tail).lower():
            kids = [c for c in kids if a.user in "".join(quote(c)).lower()]
            if not kids:
                continue
        groups.append((p, q, tail, sorted(kids, key=lambda x: t[x])))
    n = sum(len(g[3]) for g in groups)
    print("# tweaks — %d refinements of %d asks (children within %g days)\n" % (n, len(groups), a.within))
    for p, q, tail, kids in groups:
        day = datetime.datetime.fromtimestamp(t[p]).strftime("%Y-%m-%d")
        print("## %s  (%s)\n> %s\n" % (rel(p), day, q))
        for c in kids:
            cq, ctail = quote(c)
            hours = (t[c] - t[p]) / 3600
            when = ("+%.1fh" % hours) if hours < 48 else ("+%.0fd" % (hours / 24))
            print("- **%s → %s**\n  > %s\n" % (when, os.path.basename(c), cq))


if __name__ == "__main__":
    main()
