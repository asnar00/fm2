#!/usr/bin/env python3
"""The deploy stamps: close what a release actually shipped, from the release.

ask/lifecycle/being-built/announced/by-the-ship (transcripts/2026-09-04-
field-walk.md#p143). Until now an announcement was matched to its shipping by
the exact words typed a second time by hand, and a field ask was stamped
shipped by hand after each deploy — so a build whose words changed sat
"building" all day, and one deploy's stamps went out against a release that
did not carry the work. Both are now read from the release itself.

Called by tools/deploy.sh at the end of a successful ship, and safe to run by
hand afterwards (it is idempotent — an entry already shipped at this build is
left alone and said so).

  python3 tools/stamp_ship.py --build 700              # since released.sha
  python3 tools/stamp_ship.py --build 700 --since <sha> [--head <sha>]
  python3 tools/stamp_ship.py --build 700 --dry        # say, write nothing
  python3 tools/stamp_ship.py --build 700 --local      # the box, not over ssh

What it stamps, from the commits in (since, head]:

  * every `builds` entry whose `node` is a node the release touched, or an
    ancestor of one — the announcement's own words are never read;
  * every ask whose `t` matches an `asks#<t>` id in a commit subject.

Then it prints the reminder: every announcement still `building`, older than a
day, that no deploy can ever close — no `node`, or a `node` that is not in the
tree any more. A superseded announcement surfaces at every deploy instead of
living in the builder's memory.
"""
import argparse, json, os, re, subprocess, sys, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import stamp_ask as s

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ASK_ID = re.compile(r"asks#(\d{13})")
DAY_MS = 24 * 60 * 60 * 1000


def git(*args):
    return subprocess.run(["git", "-C", REPO] + list(args),
                          capture_output=True, text=True).stdout


def released_sha():
    p = os.path.join(REPO, "products", "miso", "build", "released.sha")
    return open(p).read().strip() if os.path.exists(p) else ""


def subjects(since, head):
    rng = f"{since}..{head}" if since else head
    out = git("log", "--format=%s", rng).splitlines()
    return [l for l in out if l.strip()]


def touched_nodes(since, head):
    """the feature node directories this release touched, features/-relative —
    the same reading deploy.sh already prints before it ships"""
    if since:
        files = git("diff", "--name-only", f"{since}..{head}", "--", "features/").splitlines()
    else:
        files = git("show", "--name-only", "--format=", head, "--", "features/").splitlines()
    out = set()
    for f in files:
        m = re.match(r"features/((?:[^/]+/)*[^/]+)/[^/]+$", f.strip())
        if m:
            out.add(m.group(1))
    return sorted(out)


def tree_nodes():
    """every node directory in the tree, features/-relative"""
    out = set()
    root = os.path.join(REPO, "features")
    for dirpath, dirnames, _ in os.walk(root):
        dirnames[:] = [d for d in dirnames if d != "assets"]
        rel = os.path.relpath(dirpath, root)
        if rel != ".":
            out.add(rel)
    return out


def names(path):
    return [p for p in str(path or "").strip().strip("/").split("/") if p]


def covers(announced, touched):
    """does an announced node name this touched node, or an ancestor of it?

    An announcement writes the path a brief carries — usually a short tail like
    `browse/map-only/since` rather than the whole `miso/loop/cards/...`. So the
    match is: some prefix of the touched path ends with the announced path. That
    makes `capture/video/flip` cover a commit in `.../capture/video/flip/while-
    recording`, and a full path cover itself, with no special cases.
    """
    a, t = names(announced), names(touched)
    if not a or len(a) > len(t):
        return False
    for k in range(len(a), len(t) + 1):
        if t[k - len(a):k] == a:
            return True
    return False


def in_tree(node, known):
    """is there still a node by this name? An announcement names a tail, so the
    test is a suffix match against every node path in the tree."""
    a = names(node)
    return any(names(t)[-len(a):] == a for t in known if len(names(t)) >= len(a))


def stage(a, nodes, since):
    """the way out, one word at a time (announced/recent). The set is the same
    one `--ship` would close — the announcements naming a node this release
    touches — and nothing here ever writes a build number or touches an entry
    that has already shipped."""
    builds = s.builds_read(a.local)
    moving = ["building", "testing", "deploying"]
    hit = []
    for b in builds:
        node = b.get("node")
        if not node or b.get("status") not in moving:
            continue
        if not any(covers(node, t) for t in nodes):
            continue
        if a.stage == "building" and b.get("status") == "building":
            continue      # nothing of this deploy's to put back
        if b.get("status") == a.stage and not a.why:
            continue
        hit.append(b)
    for b in hit:
        was = b.get("status")
        print(f"  {was} -> {a.stage}: {b.get('text')!r} [node {b['node']}]"
              + (f" ({a.why})" if a.why else ""))
        if a.dry:
            continue
        b["status"] = a.stage
        if a.why:
            b["why"] = a.why
        else:
            b.pop("why", None)
    if hit and not a.dry:
        s.builds_write(builds, a.local)
    if not hit:
        print(f"  nothing to move to {a.stage} "
              f"(since {since[:8] or 'the head commit'}, {len(nodes)} node(s) touched)")
    return 0


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--build", type=int, help="the build number that shipped")
    ap.add_argument("--stage", choices=["testing", "deploying", "building"],
                    help="move the announcements this release will close to a stage "
                         "of the way out, without shipping anything: `testing` when the "
                         "gate starts, `deploying` when it has passed, `building` to put "
                         "them back if the deploy stops (announced/recent)")
    ap.add_argument("--why", help="with --stage building: what stopped the deploy")
    ap.add_argument("--since", help="the sha the last release was at (default: released.sha)")
    ap.add_argument("--head", default="HEAD", help="the sha that just shipped")
    ap.add_argument("--dry", action="store_true", help="say what would be stamped, write nothing")
    ap.add_argument("--local", action="store_true", help="the op door on this machine")
    ap.add_argument("--port", default=s.PORT)
    a = ap.parse_args()
    s.PORT = a.port
    if not a.stage and a.build is None:
        ap.error("--build is required to stamp a ship (or give --stage)")
    if a.why and a.stage != "building":
        ap.error("--why goes with --stage building")

    since = a.since if a.since is not None else released_sha()
    head = git("rev-parse", a.head).strip() or a.head
    subs = subjects(since, head)
    nodes = touched_nodes(since, head)
    ids = sorted({int(m) for line in subs for m in ASK_ID.findall(line)})
    if a.stage:
        return stage(a, nodes, since)
    print(f"ship {a.build}: {len(subs)} commit(s) since {since[:8] or '(no released.sha)'}, "
          f"{len(nodes)} node(s) touched, {len(ids)} ask id(s) cited")

    # ---- the announcements ---------------------------------------------------
    builds = s.builds_read(a.local)
    hit = []
    for b in builds:
        if b.get("status") == "shipped":
            # closed is closed: a later release touching the same node must not
            # re-stamp an announcement at its own build number, and re-running
            # this for one build must change nothing
            continue
        node = b.get("node")
        if not node:
            continue
        if any(covers(node, t) for t in nodes):
            hit.append(b)
    for b in hit:
        print(f"  announcement -> shipped (build {a.build}): {b.get('text')!r} [node {b['node']}]")
        if not a.dry:
            b["status"] = "shipped"
            b["build"] = a.build
            b.pop("why", None)   # a stopped deploy's reason, retired by the ship
    if hit and not a.dry:
        s.builds_write(builds, a.local)
    if not hit:
        print("  no announcement names a node this release touched")

    # ---- the asks ------------------------------------------------------------
    stamped = set()
    if ids:
        ls = s.sh(f"ls {s.CTX_DIR}/*.log 2>/dev/null || true", a.local)
        import urllib.parse
        worlds = [urllib.parse.unquote(f.split("/")[-1][:-4])
                  for f in ls.split() if f.endswith(".log")]
        for user in worlds:
            if user == "_global":
                continue
            snap = s.sh(f"curl -s 'localhost:{s.PORT}/diag/context?user={user}'", a.local)
            try:
                vars_ = json.loads(snap)
            except json.JSONDecodeError:
                print(f"  NOTE: no snapshot for {user} — skipped")
                continue
            row = next((v for v in vars_
                        if v["name"] == "asks" and v["path"] == s.ASKS_PATH), None)
            if not row:
                continue
            asks = json.loads(row.get("value") or "[]")
            changed = False
            for entry in asks:
                if entry.get("t") not in ids:
                    continue
                stamped.add(entry["t"])
                if entry.get("status") == "shipped" and entry.get("build") == a.build:
                    continue
                print(f"  ask {entry['t']} -> shipped (build {a.build}): "
                      f"{str(entry.get('text'))[:60]!r} [{user}]")
                if not a.dry:
                    entry["status"] = "shipped"
                    entry["build"] = a.build
                    changed = True
            if changed and not a.dry:
                body = json.dumps({"path": s.ASKS_PATH, "name": "asks",
                                   "value": json.dumps(asks)}).replace("'", "'\\''")
                out = s.sh(f"curl -s -X POST 'localhost:{s.PORT}/diag/context?user={user}' "
                           f"-d '{body}'", a.local)
                if '"ok":true' not in out:
                    print(f"  NOTE: the door refused {user}: {out[:120]}")
    for i in ids:
        if i not in stamped:
            # a subject can cite an ask this machine's worlds do not hold — a
            # rig's citation, or a world since removed. Said, never fatal.
            print(f"  NOTE: asks#{i} is cited by a commit but no world holds it — skipped")

    # ---- the reminder --------------------------------------------------------
    known = tree_nodes()
    now = int(time.time() * 1000)
    stale = []
    for b in (builds if a.dry else s.builds_read(a.local)):
        if b.get("status") not in ("building", "testing", "deploying"):
            continue
        if now - int(b.get("t") or 0) < DAY_MS:
            continue
        node = b.get("node")
        if not node:
            stale.append((b, "no node — no deploy can close it"))
        elif not in_tree(node, known):
            stale.append((b, f"node {node} is not in the tree any more"))
    if stale:
        print("  still building, and nothing will close it by itself:")
        for b, why in stale:
            age = (now - int(b.get("t") or 0)) // DAY_MS
            print(f"    {b.get('text')!r} — {why} ({age}d)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
