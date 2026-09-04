#!/usr/bin/env python3
"""Delete every post that is not a video — the retrofit behind /video-only.

Ash's ruling from the field (asks#1788503662808, 2026-09-04): "remove and
delete the still image and audio posts". /video-only stopped the making of
them the evening before; this is the door handle for the ones already made.

A post is deleted the way /delete deletes one: it becomes a tombstone —
`deleted` and `edited` stamped now, the body one empty title, no links — in
every world that holds it, the owner's and every copy. /guard merges by id and
takes the newer `edited`, so a phone that resends its older copy loses to the
tombstone; a tombstone is the only shape of write that removes anything.
Nothing else is touched: the recordings in ~/.miso-blobs stay where they are,
and /remember's op log holds the prior value of every list written here.

  python3 tools/prune_posts.py                 # dry run: list what would go
  python3 tools/prune_posts.py --go            # tombstone them
  python3 tools/prune_posts.py --world phone:+44…   # one world only
  python3 tools/prune_posts.py --port 8121     # a rig instead of the live server

A post is "not a video" when its `type` is `post` and no block is a `video`
block: the audio posts (an `audio` block), the still posts (a picture and
nothing recorded) and any written post. Run on the machine that holds the
state; from elsewhere it re-runs itself over ssh like reset_user.py.
"""
import argparse, json, os, subprocess, sys, time, urllib.parse

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import reset_user as r  # the op door, the world keys, the tombstone shape


def not_video(card):
    if card.get("type") != "post" or card.get("deleted"):
        return False
    return not any(isinstance(b, dict) and b.get("kind") == "video" for b in card.get("blocks", []))


def describe(card):
    blocks = [b.get("kind") for b in card.get("blocks", []) if isinstance(b, dict)]
    title = next((b.get("text", "") for b in card.get("blocks", []) if b.get("kind") == "title"), "")
    kind = "audio" if "audio" in blocks else "still"
    return f"{card.get('id')}  {kind:5}  {card.get('owner')}  \"{title[:40]}\""


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--go", action="store_true", help="write; the default is a dry run")
    ap.add_argument("--world", help="one world key, e.g. phone:+447…")
    ap.add_argument("--port", default=r.PORT)
    a = ap.parse_args()
    r.PORT = a.port

    if not r.local():
        argv = " ".join("'" + s.replace("'", "'\\''") + "'" for s in sys.argv[1:])
        sys.exit(subprocess.run(["ssh", "-o", "BatchMode=yes", r.MINI,
                                 f"cd ~/fm2 && MISO_RESET_LOCAL=1 python3 tools/prune_posts.py {argv}"]).returncode)

    now = int(time.time() * 1000)
    total = 0
    for key in sorted(r.world_keys()):
        if a.world and key != a.world:
            continue
        snap = r.door_get(key)
        row = next((v for v in snap if v["name"] == "cards" and v["path"] == r.CARDS_PATH), None)
        if not row:
            continue
        cards = json.loads(row.get("value") or "[]")
        gone = [c for c in cards if not_video(c)]
        if not gone:
            continue
        print(f"{key}: {len(gone)} of {len(cards)} cards")
        for c in gone:
            print("   ", ("tombstone " if a.go else "would tombstone ") + describe(c))
        total += len(gone)
        if a.go:
            kept = [dict(c, blocks=[{"kind": "title", "text": ""}], links=[], deleted=now, edited=now)
                    if not_video(c) else c for c in cards]
            r.door_set(key, r.CARDS_PATH, "cards", json.dumps(kept))
    print(f"{'tombstoned' if a.go else 'would tombstone'} {total} post copies" + ("" if a.go else " — add --go to write"))


if __name__ == "__main__":
    main()
