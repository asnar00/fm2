# by-activity
*you first, then whoever was busiest most recently — and the list holds still while you read it*

> (transcripts/2026-09-04-field-walk.md#p162)
> yeah, let's always show all users on the project, but let's sort them as follows: a) self first b) sort by most recently active first [with a mod to stop things pinging around constantly while looking at the list]

*(The "always show all users" half is `/not-people`, a child of `/since`. This
node is the sort and the mod.)*

## user

Open 👤 and you are first, then the people who have done something most
recently — posted, edited their card, or been out there with the app open. The
order is worked out the moment you open the list, and then it stays put: a
colleague's pin moving, a post arriving, a card syncing — none of it makes the
list jump under your finger. Go back and open it again and it is sorted afresh.

## spec

`/people` sorted by how near you are in the invite tree, and named its own
`people_order` as "the chain the next proximity cue joins at". This is that
node, and it replaces the order rather than mixing into it. The proximity word
stays on the row: `existing` is called first, so its `near` decoration is
intact and `/people`'s row still says how near someone is — it is just no
longer what decides the order.

**Active means the latest of three things.** A person's last post's own moment
(`when`, else `created` — `/post-time`'s rule read off the card), their card's
last edit, and the last time their phone said where it was. The first two are
in the world. The third is the server's, reaching the page through `/live`'s
poll, so it is handed in as `PeopleActive` under this node's own key —
`/people` does exactly this with the invite distances, and for the same reason:
a fact the server holds is not world state and must not pretend to be.

**The hold is the mod ash asked for.** The order is worked out when the surface
is **opened** and not again while it is on screen, however many syncs arrive.
Without it a live tick every few seconds would reorder the list under the eye,
which is the thing he did not want; with it, nothing moves under a reader,
which is `/keep`'s stance for the caret and the scroll.

The frozen order lives on the turn's state — `/in-place`'s idiom for exactly
this shape: no op on the wire, nothing stored, and a relaunch simply works it
out again. It is set on the turn that opens the tool, and on the first turn
after a relaunch that landed there; it is cleared the moment the tool is left,
so the next open re-sorts.

**A newcomer joins at the end.** Someone who arrives while the list is on
screen is not in the freeze, so they sort after everyone who is — until the
next open, when they take their real place. That is the same rule as the hold,
seen from the other side.

**The order is total, so it never wobbles.** The key is (you, then where the
freeze put them, then the name), so a person with no activity at all, and two
people with the same activity, both have a stable place; and the name is the
last word in the freeze's own sort too.

**The band follows.** `/everyone` lists the map's set by `#mapData`'s ids, which
is `browse_cards`' output in this node's order, so the people band along the
bottom is in the same order as the list without knowing this node exists.

## hostile cases

- **No activity known for a person.** They sort after everyone who has some, by
  name — both in the freeze and in the key.
- **Two people equally active.** By name, in both places, so the answer is the
  same every time.
- **Your own card absent** (before the first run has made it). `people_own_id`
  is empty, nobody takes the first slot, and the rest sort normally.
- **A live tick while the list is open.** `PeopleActive` is written, the freeze
  is not, and nothing moves. This is the mod.
- **A person arriving while the list is open.** Last, until the next open.
- **‹ and open again.** The freeze was cleared on the way out, so the order is
  worked out afresh — including that newcomer.
- **`/live` unticked.** No `PeopleActive` ever arrives; activity is the post
  and the card edit, which is most of it.
- **A live row for someone whose card you do not hold.** Skipped: there is no
  row of theirs in the list to order.
- **The project filter** (`/current-project`): it narrows `browse_cards` before
  this sees it, so the freeze names only people who are in the set. Someone
  sifted out is simply not there to order.

## parked

- Saying *why* someone is at the top — "posted 4m ago" where the proximity word
  is. The word that is there is `/people`'s and the ask did not name it.

## glossary

- **active**: the latest of a person's last post, their last card edit, and the
  last time their phone said where it was.
- **the freeze**: the order worked out when the list was opened, held until it
  is left.

## code description

`by-activity.rs` redefines `people_order(cards, state)`: it calls `existing`
first, so `/people`'s `near` decoration survives, then re-sorts by
`by_activity_key` — you, the freeze's position, the name.

`by_activity_fresh(state)` is the order worked out from scratch — every profile
you hold, by `by_activity_at` descending, name breaking ties.
`by_activity_at(card, state)` is the latest of the person's last post, their
card's `edited`, and their last live sighting. `by_activity_frozen(state)`
reads the held list back.

`by-activity.rs` extends `update` with two things: `PeopleActive`, the page
half's map of card id to last-seen, kept on the state under `active`; and the
freeze itself — written on the turn that opens the tool or the first turn of a
relaunch that landed there, cleared on the turn that leaves it.

`by-activity.js` wraps `feature_Live.draw` to send `PeopleActive` from the rows
`/live` has just polled, deduped and **deferred** — `/live-only` repaints the
band from inside that same call, so sending straight from there would put an
event inside a paint, which is the fault that took build 690 down.
