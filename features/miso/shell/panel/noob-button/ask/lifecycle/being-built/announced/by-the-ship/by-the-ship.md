# by-the-ship
*the deploy stamps: what shipped says so itself, and what nothing can close is named at every release*

> (transcripts/2026-09-04-field-walk.md#p143)
> can we figure out the fix to the build reporting? that shouldn't require intervention - something went wrong, let's make sure it doesn't happen again?

## user

The **building** list on your sheet keeps itself honest. When a release goes
out, everything it carried moves to shipped by itself — your own requests and
the ones the builder announced in conversation — with the build number that
carried it. Nothing waits for somebody to remember.

## spec

Three things went wrong on 2026-09-04 and they are one fault. An announcement
(`--announce "<words>" --status building`) was matched to its shipping by
**the same words typed again by hand**, so when the options-button
announcement was superseded by a node built under other words, its line sat
`building` all day until ash saw it and said so. Field asks were stamped
shipped by hand after each deploy, so one deploy's stamps went out against a
build that did not carry the work (misses.md, "the fast-forward that never
happened"). And all of it depended on the builder remembering. Ash: *"that
shouldn't require intervention — something went wrong, let's make sure it
doesn't happen again."*

**The rule: the deploy stamps.** A release already knows what it carried —
deploy.sh prints the feature nodes it touched before it ships, and every
commit subject that answers a field ask cites `asks#<t>`. Nothing else has to
be typed, remembered, or matched.

**An announcement names its node.** `stamp_ask.py --announce "<words>" --node
<path>` records `node` on the entry beside its words. A brief always knows the
placement — it is the first thing triage decides — so the node is knowable at
announcing time in a way the eventual commit subject is not. The path is
written as a brief writes it (`browse/map-only/since`), and matches a touched
node when some prefix of the touched path ends with it, so a tail is enough
and a node's descendants count. A bare `--announce` still works and still
warns: without a node nothing can close it.

**The deploy calls one thing.** `tools/stamp_ship.py --build <N> --since <the
sha the last release stood at>` reads the range, stamps `shipped (build N)` on
every `builds` entry whose `node` the release touched and on every ask whose
`t` a subject cites, prints each one, and then prints **the reminder**: every
announcement still `building` for more than a day that no deploy can ever
close — no `node`, or a `node` that has left the tree. A superseded
announcement now surfaces at every release instead of living in the builder's
memory. That is the retrofit for the one still-building entry today: it has no
node, so tomorrow's first deploy names it.

**Only on the way out, and never fatal.** The call sits after `released.sha` is
written — after the binary, the handover and the site have all landed — because
that is the first line at which "shipped" is a true sentence. Every gate above
it (the toggle proof, the wasm import test, the smoke gate) exits before it, so
a deploy that fails stamps nothing; `PROOF=skip` and `SMOKE=skip` still stamp,
because the ship still happened. Its own failure is a printed note, never an
exit: a stamp that did not go out must not turn a live release into a failed
one.

**The words are still the key to an entry**, so announcing and re-announcing
work exactly as before; `node` is added beside them, not instead. `ask_ack.py`
is untouched — the arrival stamp is a different act from the ship stamp.

## hostile cases

- **A subject cites an ask no world holds** (a rig's citation, a removed
  world): skipped, with a line saying so.
- **One release touches a node two announcements name**: both are shipped.
  The entries are independent and the build number is the same.
- **A node deleted and re-placed**: its announcement can no longer be matched,
  which is exactly what the reminder is for — it is listed as "not in the tree
  any more" at the next release.
- **A release that touches no feature node at all** (a docs or tooling
  release): nothing is stamped, and the reminder still runs.
- **Run twice for the same build**: an entry already shipped at that build is
  left alone; the tool is idempotent and safe to run by hand after a deploy.
- **No `released.sha`** (a first deploy on a fresh checkout): the range is the
  head commit alone, and the reminder still runs.
- **This node unticked**: the instruction leaves the skillset and announcing
  goes back to two calls matched by their words. `--node` and the deploy's
  call are scaffolding and stay — they cost nothing when no entry carries a
  node.

## glossary

- **ship stamp**: the `shipped (build N)` a release writes for itself, on the
  announcements and asks that release carried.

## code description

`by-the-ship.agent.md` is the whole of this node's composed material: the
build flow, restated — announce with `--node`, and never stamp a ship by hand.
It supersedes the second call in `/announced`'s instruction, and being newer
it composes after it in the skillset.

`tools/stamp_ship.py` (scaffolding) is the entry point: `touched_nodes` reads
the release's node directories the way deploy.sh already prints them, `covers`
is the tail match between an announced path and a touched one, the ask ids come
from `ASK_ID` over the subjects, and the reminder uses `tree_nodes`/`in_tree`.
Writes go through `POST /diag/context` — the same door as every other stamp.

`tools/stamp_ask.py` (scaffolding) gains `--node` on an announcement, and its
`builds` read and write are now `builds_read`/`builds_write` so the two tools
name the address once.

`tools/deploy.sh` (scaffolding) reads `released.sha` into `PREV` before it
overwrites it, and calls `stamp_ship.py` after the new sha is written, with
`--local` when the deploy is running on the box.

## risks

**The node is written by hand and could be wrong.** A misspelt `--node` closes
nothing and is listed by the reminder a day later; it cannot close the *wrong*
announcement unless the path also matches another node's tail, which
tree-global names make unlikely but not impossible.

**A release that touches a node for an unrelated reason closes its
announcement.** A one-line fix inside `browse/map-only/since` would stamp an
announcement naming that node as shipped. The alternative — matching the
commit that added the node's spec — would miss every announcement whose work
lands as an edit, which is most of them. Named, not solved.

**deploy.sh is `tools/`, outside the tree**, so this node's own toggle proof
covers the node and its instruction; the deploy step and the two tools are
scaffolding and are not composed. The step's placement after `released.sha` is
what makes "only on success" true, and it is held by reading, not by a test the
linker can run.
