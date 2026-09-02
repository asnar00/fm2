#!/usr/bin/env python3
"""toggle_proof — is a change's toggle proof implied by its shape?

The linker never reads an unticked node's files, so the composition WITHOUT
a node is a function of every file outside that node. If a change touches
only one node (its descendants included, its own order.md included) plus
additions to its parent's order.md, then the composition without the node
is untouched by the change — the untick cannot observe it, and for a new
node that composition is the last release, already built, smoke-gated and
shipped. Such a change is CONFINED and agents.md step 4's untick/relink/
re-tick is implied. Anything else (a parent refactored
to open an extension point, two nodes in one commit, a sibling unticked, a
product order.md override) keeps the full proof, and a commit of that shape
must say so in a `Toggle-proof:` trailer or the deploy gate refuses it.

Doctrine: /confined in the composed skillset (features/miso/shell/panel/
noob-button/ask/lifecycle/confined, settings #p4–#p5). Only files under
features/ and products/ count — tools/ and documents do not move a node's
toggle (a linker change is the smoke gate's business).

    toggle_proof.py                 the working tree against HEAD
    toggle_proof.py <sha>           one commit (a merge: against its first parent)
    toggle_proof.py --since <sha>   every first-parent commit after <sha>
                                    (deploy.sh: the last released commit)

Exit 0 when every change checked is confined or carries a proof record;
1 when one does not (the gate); 2 for usage errors.
"""
import argparse
import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
TREE_PREFIXES = ("features/", "products/")
TRAILER = re.compile(r"^toggle-proof:\s*(\S.*)$", re.I | re.M)


def git(*args):
    return subprocess.check_output(["git", *args], cwd=REPO, text=True)


def node_dir_of(path: str):
    """the node a tree file belongs to: the nearest ancestor directory that
    carries its own <name>.md spec — None for a file outside any node (a
    top-level order.md, a product override). order.md files are never
    attributed to a node: a parent's order.md is where a child is ticked."""
    p = Path(path)
    if p.name == "order.md":
        return None
    for d in p.parents:
        if d == Path("."):
            break
        if (REPO / d / (d.name + ".md")).exists():
            return d.as_posix()
    return None


def is_within(node: str, root: str):
    return node == root or node.startswith(root + "/")


def changed_files(rev):
    """paths a change touches. rev None: the working tree against HEAD,
    untracked files included (a new node is untracked until it is added).
    rev a sha: that commit against its first parent."""
    if rev is None:
        out = git("status", "--porcelain=v1", "-uall")
        files = []
        for line in out.splitlines():
            path = line[3:]
            if " -> " in path:
                path = path.split(" -> ", 1)[1]
            files.append(path)
        return files
    parents = git("rev-list", "--parents", "-n", "1", rev).split()[1:]
    if not parents:
        return git("show", "--format=", "--name-only", rev).split()
    return git("diff", "--name-only", parents[0], rev).split()


def additions_only(path: str, rev):
    """true when the diff of one file has no removed lines"""
    if rev is None:
        diff = git("diff", "-U0", "HEAD", "--", path)
    else:
        parents = git("rev-list", "--parents", "-n", "1", rev).split()[1:]
        if not parents:
            return True
        diff = git("diff", "-U0", parents[0], rev, "--", path)
    return not any(l.startswith("-") and not l.startswith("---")
                   for l in diff.splitlines())


def classify(rev):
    """-> (confined: bool, summary: str). confined is also True when the
    change touches no tree file at all — there is nothing to prove."""
    files = changed_files(rev)
    tree = [f for f in files if f.startswith(TREE_PREFIXES)]
    if not tree:
        return True, "no change under features/ or products/ — nothing to prove"
    nodes, loose = set(), []
    for f in tree:
        n = node_dir_of(f)
        (nodes.add(n) if n else loose.append(f))
    roots = sorted(n for n in nodes
                   if not any(o != n and is_within(n, o) for o in nodes))
    if len(roots) > 1:
        return False, "touches %d nodes: %s" % (len(roots), ", ".join(roots))
    if not roots:
        return False, "outside any node: " + ", ".join(loose)
    root = roots[0]
    parent_order = (Path(root).parent / "order.md").as_posix()
    for f in loose:
        if is_within(Path(f).parent.as_posix(), root):
            continue  # an order.md inside the node leaves with it
        if f != parent_order:
            return False, f"{f} is outside {root}"
        if not additions_only(f, rev):
            return False, f"{f} removes or reorders lines (a tick added is fine)"
    extra = " (+ %s, additions only)" % parent_order if parent_order in loose else ""
    return True, f"every tree change lies in {root}{extra}"


def check(rev, label):
    """print one verdict; return True when this change may ship"""
    confined, why = classify(rev)
    if confined:
        print(f"toggle proof {label}: implied — {why}")
        return True
    if rev is not None:
        m = TRAILER.search(git("log", "-1", "--format=%B", rev))
        if m:
            print(f"toggle proof {label}: recorded — {m.group(1).strip()} ({why})")
            return True
    print(f"toggle proof {label}: REQUIRED — {why}")
    if rev is None:
        print("  untick the node, relink, confirm its code left and nothing else "
              "changed, re-tick; then say so in the commit as a "
              "'Toggle-proof: <what was proven>' trailer")
    else:
        print("  the commit carries no 'Toggle-proof:' trailer — prove it and "
              "amend, or split the change so each commit stays inside one node")
    return False


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("rev", nargs="?", help="a commit; omitted: the working tree")
    ap.add_argument("--since", metavar="SHA",
                    help="check every first-parent commit after SHA up to HEAD")
    args = ap.parse_args(argv)
    if args.since:
        shas = git("rev-list", "--first-parent", f"{args.since}..HEAD").split()
        if not shas:
            print("toggle proof: nothing new since", args.since[:12])
            return 0
        ok = True
        for sha in reversed(shas):
            subject = git("log", "-1", "--format=%s", sha).strip()
            ok &= check(sha, f"{sha[:8]} “{subject[:60]}”")
        return 0 if ok else 1
    if args.rev:
        sha = git("rev-parse", args.rev).strip()
        return 0 if check(sha, sha[:8]) else 1
    return 0 if check(None, "(working tree)") else 1


if __name__ == "__main__":
    sys.exit(main())
