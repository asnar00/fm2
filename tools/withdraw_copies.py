#!/usr/bin/env python3
"""Withdraw every post copy whose floor now stands above its holder.

The retrofit behind /audience/withdrawn (transcripts/2026-09-04-field-walk.md
#p113). Promote lowered a post's floor and handed a copy to everyone it then
reached; undo raised the floor again and `exchange_give` simply did not send
the raised card, so those copies stayed. Ash saw one of his admin-only posts
on Tara's phone, and she is a candidate.

/withdrawn fixes it from now on — a raise hands the holder a tombstone. This
is the door handle for the copies already out there. For every world, every
copy of a post whose `floor` outranks the holder's grade in that post's
project becomes a tombstone (/delete's shape: `deleted` and `edited` stamped,
one empty title, no links) written through `POST /diag/context`, the same door
every other repair uses. /guard merges by id and takes the newer `edited`, so
a phone that resends its older copy loses to the tombstone.

  python3 tools/withdraw_copies.py                 # dry run: list what would go
  python3 tools/withdraw_copies.py --go            # write the tombstones
  python3 tools/withdraw_copies.py --world phone:+44…
  python3 tools/withdraw_copies.py --port 8143     # a rig instead of the live server

Only a COPY is ever touched (a card carrying `from`); an owner's own post is
never withdrawn from them. Two other refusals the live gate also makes are
counted but NOT written unless --all-refused is given, because either can be
a copy of the project card that has not arrived rather than a real loss of
standing: a holder who does not hold the post's project card at all, and one
who holds it with no role link naming them.

Run on the machine that holds the state; from elsewhere it re-runs itself over
ssh like reset_user.py and prune_posts.py.
"""
import argparse, json, os, subprocess, sys, time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import reset_user as r  # the op door, the world keys, the guest list

# /audience's ladder, in its order: admin 0 … public 5. Higher rank is the
# smaller number, and "may hold" is rank(grade) <= rank(floor).
GRADES = ["admin", "candidate", "team", "volunteer", "supporter", "public"]
DEFAULT_GRADE = "team"


def rank(grade):
    return GRADES.index(grade) if grade in GRADES else GRADES.index(DEFAULT_GRADE)


def floor_of(card):
    f = card.get("floor")
    return f if f in GRADES else DEFAULT_GRADE


def in_of(card):
    if card.get("type") != "post":
        return ""
    for l in card.get("links") or []:
        if isinstance(l, dict) and l.get("kind") == "in" and l.get("to"):
            return l["to"]
    return ""


def link_name(l):
    to = l.get("to") or ""
    return to[:to.rfind(".")] if "." in to else (l.get("name") or "")


def grade_in(proj, name):
    """where one person stands in one project, or '' for not in it —
    /audience's audience_grade_in, rule for rule"""
    if not name:
        return ""
    if proj.get("owner") == name:
        return "admin"
    for l in proj.get("links") or []:
        if not isinstance(l, dict) or l.get("kind") != "role" or not l.get("to"):
            continue
        if link_name(l) != name and l.get("name") != name:
            continue
        g = l.get("grade")
        return g if g in GRADES else DEFAULT_GRADE
    return ""


def project_in(cards, pid):
    for c in cards:
        if c.get("id") == pid and c.get("type") == "project" and not c.get("deleted"):
            return c
    return None


def name_of(key, guests):
    """the guest list maps a world key (a phone) to the name a role link uses,
    as the server's exchange_name_of does"""
    phone = key.split(":", 1)[-1]
    for u in guests:
        if str(u.get("phone", "")) == phone:
            return str(u.get("name") or "")
    return ""


def title_of(card):
    for b in card.get("blocks") or []:
        if isinstance(b, dict) and b.get("kind") == "title":
            return (b.get("text") or "")[:40]
    return ""


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--go", action="store_true", help="write; the default is a dry run")
    ap.add_argument("--world", help="one world key, e.g. phone:+447…")
    ap.add_argument("--all-refused", action="store_true",
                    help="also withdraw copies whose project card the holder "
                         "does not hold, or holds with no role for them")
    ap.add_argument("--port", default=r.PORT)
    a = ap.parse_args()
    r.PORT = a.port

    if not r.local():
        argv = " ".join("'" + s.replace("'", "'\\''") + "'" for s in sys.argv[1:])
        sys.exit(subprocess.run(["ssh", "-o", "BatchMode=yes", r.MINI,
                                 f"cd ~/fm2 && MISO_RESET_LOCAL=1 "
                                 f"python3 tools/withdraw_copies.py {argv}"]).returncode)

    try:
        guests = r.users()
    except Exception as e:
        sys.exit(f"withdraw_copies: no guest list ({e}) — a world's holder cannot be named")

    # pass one: the floor of record. A copy carries the floor it was handed at,
    # which for exactly the cards this repair is looking for is the OLD, lower
    # one — that stale floor is the fingerprint of the bug, not the truth. The
    # owner's own card is the truth, and it is in the owner's world on this
    # machine. The live gate has it fresh because the arriving card is the
    # owner's; a repair reading the copies has to go and fetch it.
    keys = sorted(r.world_keys())
    floors = {}
    for key in keys:
        snap = r.door_get(key)
        row = next((v for v in snap if v["name"] == "cards" and v["path"] == r.CARDS_PATH), None)
        if not row:
            continue
        for c in json.loads(row.get("value") or "[]"):
            if c.get("type") == "post" and not c.get("from") and not c.get("deleted"):
                floors[c.get("id")] = floor_of(c)

    now = int(time.time() * 1000)
    total = no_project = no_role = unknown = stale = 0
    for key in keys:
        if a.world and key != a.world:
            continue
        name = name_of(key, guests)
        if not name:
            unknown += 1
            print(f"{key}: not on the guest list — skipped")
            continue
        snap = r.door_get(key)
        row = next((v for v in snap if v["name"] == "cards" and v["path"] == r.CARDS_PATH), None)
        if not row:
            continue
        cards = json.loads(row.get("value") or "[]")
        take = []
        for c in cards:
            if c.get("deleted") or not c.get("from"):
                continue            # their own card is never withdrawn from them
            pid = in_of(c)
            if not pid:
                continue            # no project, no floor to stand above
            proj = project_in(cards, pid)
            soft = True             # a refusal that may be a missing copy, not a demotion
            floor = floors.get(c.get("id"), floor_of(c))
            if floor != floor_of(c):
                stale += 1
            if proj is None:
                no_project += 1
                why = "does not hold the project"
            else:
                grade = grade_in(proj, name)
                if not grade:
                    no_role += 1
                    why = "holds the project with no role"
                elif rank(grade) <= rank(floor):
                    continue        # they may hold it: nothing to do
                else:
                    soft = False
                    why = (f"{grade} is below floor {floor}"
                           + (f" (their copy still says {floor_of(c)})"
                              if floor != floor_of(c) else ""))
            if soft and not a.all_refused:
                print(f"{key} ({name}): LEFT  {c.get('id')} — {why} "
                      f"(add --all-refused to withdraw it)")
                continue
            take.append((c, why))
        if not take:
            continue
        print(f"{key} ({name}): {len(take)} of {len(cards)} cards")
        for c, why in take:
            print("   ", ("withdraw " if a.go else "would withdraw "),
                  c.get("id"), f'"{title_of(c)}"', "—", why)
        total += len(take)
        if a.go:
            ids = {c.get("id") for c, _ in take}
            # one past the newest stamp this world holds, so the tombstone
            # cannot lose to a copy the phone resends (/guard, /revert)
            top = max([c.get("edited") or 0 for c in cards] + [now])
            out = [dict(c, blocks=[{"kind": "title", "text": ""}], links=[],
                        deleted=top + 1, edited=top + 1)
                   if c.get("id") in ids else c for c in cards]
            r.door_set(key, r.CARDS_PATH, "cards", json.dumps(out))
    print(f"{'withdrew' if a.go else 'would withdraw'} {total} copies"
          + (f"; {stale} copy floor(s) disagreed with the owner's card" if stale else "")
          + (f"; left {no_project} without the project card and {no_role} without a role"
             if not a.all_refused and (no_project or no_role) else "")
          + (f"; {unknown} world(s) not on the guest list" if unknown else "")
          + ("" if a.go else " — add --go to write"))


if __name__ == "__main__":
    main()
