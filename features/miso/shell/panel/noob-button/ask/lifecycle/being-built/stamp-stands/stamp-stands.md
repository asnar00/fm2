# stamp-stands
*the builder's stamp is not written over by the asker's own phone*

> (transcripts/2026-09-04-field-walk.md#p57)
> Let's fix that issue as well? It felt janky.

## user

When the builder marks your request **being built**, it stays marked. Filing
another ask a moment later, or opening the panel on a second device, no longer
puts an old status back on your sheet — and the words you typed cannot be
written over by the builder's bench either. Each request is kept whole, by
itself: your side of it is yours, the builder's side is theirs, and nothing
either of you does goes missing because the other spoke last.

## spec

Four asks arrived from the field inside two minutes on 2026-09-04.
`ask_ack.py` stamped each of them `building` through the op door and said so;
two of them showed `asked` on ash's sheet a minute later. The phone had sent
its own `asks` list again — carrying the status it had before the stamp — and
`asks` is `(user, last-write, own)`: one whole-list `set`, applied whole. The
stamps were put back by hand and the case went into the ledger (misses.md,
"the stamp the phone wrote over"). Ash: *"It felt janky."*

The list has **two authors**. The asker's device writes the ask itself — the
words, the urgency, the birthplace, the paragraph `/propose` approves, the
answer `/did-you-mean` collects. The builder's bench writes where the ask has
got to — `status`, `build`, `note`, `question`. Both send the whole list, and
neither reads the other's mind, so under last-write the second arrival wins
outright. The failure is not a race between two people; it is a race between
two *sets of fields* that happen to travel in one string.

So this node is the server's last word on an `asks` set, the shape `/guard`
gave a `cards` set for the same reason: a link on `handle_msg`, outside
`/converge`'s, that merges the arriving list into the world's before the op
is applied. The merge is **per ask, keyed by `t`** — the ask's filing
timestamp, which is already its identity everywhere else (agents.md cites it
as `asks#<t>`).

**Who owns which field decides every conflict.** For an ask both sides carry,
each field is taken from the side that can write it, and the other side may
only *fill* it — never change it, never clear it. The asker's fields are
`text`, `urgency`, `tool`, `at`, `proposal`, `answer`; everything else is the
bench's, including any field a later node adds without filing it on the
asker's list (an unclaimed field is treated as a stamp, because a stamp is
what the phone keeps writing over). So a stamp arriving while the phone has
just filed something, in either order, leaves both.

**Which side is which is read from the sender, not from the payload.**
`/messaging` stamps `_from` with the cookie-proven identity and it "cannot be
lied to by the payload"; the builder's bench reaches `/diag/context` over
localhost with no cookie, so `/same-door` mints its op with no sender at all.
An op with no sender is the bench; an op with one is the asker's device.

**`status` is the one field both sides write, so it has its own rule.** The
bench is the last word — that is what a stamp is. A device may move a status
only when it carries the thing that earns the move: a new `answer` (which
settles a did-you-mean and walks the ask back to `asked` — the one backwards
step on the ladder) or a new `proposal` (which raises it to `proposed`). Any
other status from a device is a copy that has not caught up; the held status
stands and a line says so on the log.

A ladder rule — *status only ever moves forward, asked → proposed → building
→ shipped* — was the shape triage proposed, and it is not the one built,
for two reasons said plainly. It would swallow every did-you-mean answer,
since answering moves the ask backwards to `asked` on purpose. And the ladder
is not a line: `question` is stamped over `building` when the acker got there
first, and `building` is stamped over `question` when silence gets the likely
reading built (agents.md), so no ranking of those two is right in both
directions. Ownership settles the case the ladder was reached for, and
settles the others too.

**The sender is told what the world settled on.** `/converge` answers a
`CtxOp` with the resolved value and relays the same, so the device that sent
a list the merge corrected is handed the corrected one in the reply and
applies it — the list stops being stale on the write that would have lost the
stamp, rather than on the next join.

**Nothing is deleted.** An ask the world holds and the arriving list lacks
survives, exactly as a card does under `/guard`: nothing in the app deletes an
ask, so an absence is a sender that has not caught up. The day deleting is a
feature it will be its own op with its own intent.

**The scaffolding half.** `stamp_ask.py` gains `--only-if <status>`, and
`ask_ack.py` passes `--only-if asked`, so the automatic acknowledgement — the
one writer on the bench that fires without a person watching — can only ever
stamp an ask nobody has stamped yet. That closes the out-of-order case (a
late `building` landing on a `shipped`) where it belongs, at the writer, and
leaves the bench free to correct a stamp it regrets, which happened for real
the same day.

## hostile cases

- **The case.** The bench stamps `building`; the phone sends its list a moment
  later still saying `asked`. The device may not move the status: `building`
  stands, and the relay hands the phone the merged list back.
- **Both at once.** The bench stamps `shipped` while the phone files a new ask
  in the same breath. The bench's list has no new ask; the phone's has no
  stamp. Either order: the new ask is appended, the stamp is kept.
- **The did-you-mean answer.** The world holds `question`; the phone answers.
  The answer is new, so its `asked` is honoured and the answer is stored. A
  *second* copy of the same answer from a stale page is not new, so if the
  bench has since stamped `building`, `building` stands.
- **A second question.** The bench replaces `question` with a new one; the
  phone's copy still carries the old. The question is the bench's field: the
  new one stands.
- **A stamp for an ask the phone has since dropped.** The ask is kept — as the
  builder's, because nothing deletes an ask and an absence is staleness. The
  bench's own tool refuses a stamp for an ask no world holds, as it always did.
- **Out of order.** A late `building` after a `shipped` from the acker cannot
  happen: the acker only stamps an ask still `asked`. A person stamping
  backwards on purpose is doing it on purpose, and it lands.
- **Duplicate `t`s.** Two entries under one timestamp are folded into one
  before the merge, later fields winning, with a line on the log.
- **Not a list, a layered write, or another var.** Passed through untouched
  for `/converge` to judge.

## glossary

- **the bench**: the builder's side of the ask store — `stamp_ask.py` and
  `ask_ack.py` writing through `/diag/context` on the box, with no cookie and
  therefore no sender on the op.

## code description

`stamp-stands.rs`, `handle_msg()` /extension/: claims a `CtxOp` `set` on
`miso/shell/panel/noob-button/ask`/`asks` whose value parses as a list and
which addresses no other layer, reads the world's list through `/ask`'s
`asks_read()`, rewrites `data.value` with the merge, and hands the op on.
Everything else passes straight through.

`asks_stand_merge` is the union by `t`: held asks survive, an ask on both
sides goes through `asks_merge_entry`, an ask only the sender has is
appended. `asks_fold_dupes` collapses two entries sharing a `t` first, so a
duplicate cannot shadow the merged one.

`asks_merge_entry` applies the ownership rule field by field —
`asks_asker_fields` lists the asker's, `asks_bench_fields` names every other
key either side carries, `asks_pick` takes the owner's value and falls back
to the other side's so a field is only ever filled in, `asks_put` keeps an
absent field absent. `asks_merge_status` settles `status`, with
`asks_field_earned` asking whether the device is carrying something new
rather than something it was given.

`tools/stamp_ask.py` (scaffolding), `--only-if`: a stamp that skips entries
whose status is not the one named, and says so rather than failing.
`tools/ask_ack.py` passes `--only-if asked` on the automatic ack.

## risks

**A stamp made through the tunnel is a device write.** `/diag/context` over
the tunnel requires a cookie, so such an op carries a sender and cannot move
a status. The bench runs on the box — `stamp_ask.py` ssh's to the mini and
curls localhost, and `--local` is local — so every real stamp is unsigned.
A repair typed through the tunnel wants the box instead, and the log line
says why it did nothing.

**An unsigned device is read as the bench.** `/msg` is free on localhost
(tooling), so a page open at `localhost` with no cookie would be trusted with
statuses. It is writing the empty world's own list, where there are no stamps
to lose, and every real device reaches the server through the tunnel with a
cookie.

**A later node's new asker-side field is treated as a stamp** until it is
named in `asks_asker_fields` — the device would fill it once and never change
it. The list is in this file next to the sentence that says so.

**A device can fill a builder's field the world does not hold.** Filling is
how nothing is ever lost, and the cost is that a hand-written op from a
signed-in device could plant a `build` number or a `note` on one of its own
asks — never change one the world holds, never move a status, and never on
anyone else's sheet, since a world is one person's. The app itself writes
none of those fields.

**`t` is assumed unique per ask.** Two asks minted in the same millisecond in
one world are folded into one. `t` is `Date.now()` at the tap and is already
the ask's identity in every citation; the fold says so on the log.
