#!/usr/bin/env python3
"""Move pictures out of the cards list and back again — /pic-beside's retrofit.

The work is the server's (`POST pic/retrofit`, screened as `POST diag/context`
is: open on localhost, cookie-gated through the tunnel). This is the door
handle: it defaults to a dry run, prints what would move, and needs --go to
write anything.

  python3 tools/pics.py                      # dry run, every world
  python3 tools/pics.py --go                 # move the pictures out
  python3 tools/pics.py --world phone:+44…   # one world
  python3 tools/pics.py --back --go          # put every picture back inline
  python3 tools/pics.py --port 8121          # a rig instead of the live server

`--back` is the tested inverse: it reads the bytes out of the store and writes
the data URL into the block again. Neither direction touches `edited`, so
/guard's tie-to-incoming lets each land, and /remember's op log holds every
prior value either way.

Written 2026-09-03 with /pic-beside (transcripts/2026-09-03-invite-test.md#p159).
"""
import argparse, json, sys, urllib.error, urllib.request


def call(port, body, cookie):
    rq = urllib.request.Request(
        f"http://localhost:{port}/pic/retrofit",
        data=json.dumps(body).encode(),
        headers={"Content-Type": "application/json"}
        | ({"Cookie": f"miso_auth={cookie}"} if cookie else {}),
        method="POST")
    try:
        with urllib.request.urlopen(rq, timeout=120) as r:
            return json.loads(r.read().decode())
    except urllib.error.HTTPError as e:
        sys.exit(f"pics: the server said {e.code}: {e.read().decode()[:200]}")
    except Exception as e:
        sys.exit(f"pics: could not reach the server on port {port}: {e}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", type=int, default=8095, help="the server's port")
    ap.add_argument("--world", default="", help="one world key, else every world")
    ap.add_argument("--back", action="store_true",
                    help="put pictures back inline instead of moving them out")
    ap.add_argument("--go", action="store_true", help="write; without it, a dry run")
    ap.add_argument("--cookie", default="", help="miso_auth, needed through a tunnel")
    a = ap.parse_args()

    mode = "back" if a.back else "out"
    rep = call(a.port, {"mode": mode, "dry": not a.go, "world": a.world}, a.cookie)
    worlds = rep.get("worlds", [])
    moved = sum(w.get("moved", 0) for w in worlds)
    was = sum(w.get("was", 0) for w in worlds)
    now = sum(w.get("now", 0) for w in worlds)
    print(f"{'DRY RUN — ' if not a.go else ''}mode {mode}")
    for w in sorted(worlds, key=lambda w: -w.get("moved", 0)):
        if not w.get("moved") and not w.get("unreadable"):
            continue
        note = f", {w['unreadable']} unreadable" if w.get("unreadable") else ""
        print(f"  {w['world']}: {w['moved']} picture(s){note}, "
              f"list {w.get('was')} -> {w.get('now')} bytes")
    print(f"  {len(worlds)} world(s) seen, {moved} picture(s), "
          f"{was} -> {now} bytes in all")
    if not a.go and moved:
        print("  nothing was written — pass --go")


if __name__ == "__main__":
    main()
