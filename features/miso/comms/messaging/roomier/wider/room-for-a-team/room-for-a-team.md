# room-for-a-team
*a dozen canvassers can trade pictured cards in one day without the world filling up*

> (transcripts/2026-09-03-invite-test.md#p159)
> do everything now
> *(the diagnosis it says yes to is #p158, a worker's contact report: ash's
> cards list measured 176,020 chars — 94% inline picture data across 8
> cards — against a `LIST_CAP` of 160,000, so `/poster` had silently refused
> every video thumbnail for days, and one more pictured colleague card put
> the whole-list write over the 196,608-byte wire cap.)*

## user

Your cards no longer fill up after half a dozen pictures. Room for roughly
thirty pictured cards now, so a canvassing session where a dozen people
exchange cards with photographs fits, and video posts get their thumbnail
back.

## spec

`/roomier` set the wire at 64KB and the cards list at 56KB; `/wider` raised
them to 192KB and 160KB, for "about six" pictured cards. That was measured
against a two-person world. On 2026-09-03 ash's own world reached 176,020
chars across 8 cards — over the 160,000 list budget — and two things had been
failing on it, both silently by their own specs:

`/poster` gates on `feature_Cards.held('', 1) + data.length > LIST_CAP` and
returns `null` without a word, so every video recorded since the list passed
160,000 became a post with no face. Nothing was broken; the budget said no.

And the whole-list write itself was within 19,000 bytes of the wire. One more
pictured colleague card takes the op body past `msg_body_cap()`, where
`/messaging` truncates it, fails to parse it, and answers 400 — measured on a
rig at 177,704 bytes → 200, 197,903 → 400, 217,153 → 400.

This node raises the two numbers about fourfold and touches nothing else.
`LIST_CAP` becomes 640,000 — room for roughly thirty cards at ash's current
average of ~21,000 chars each. `msg_body_cap()` becomes 1,048,576, which
leaves ~62% headroom over a full 640,000-char list once the op's envelope and
JSON escaping are counted (ash's 176,020-char list measures 177,704 bytes on
the wire, about 1% over the raw list). `CAP` stays at 24,576 per picture,
`EDGE` stays at 384, and `/poster`'s own size ladder is untouched: this widens
the shelf, not the objects on it. The serve layer's own read limit is 16MB, so
nothing under this has to move.

**The cost, named.** Every text edit to any card still rewrites the entire
list as one op. At the new ceiling that is up to ~640KB on the wire, in the
device's op log, and in the server's world file, per keystroke-batch. The
per-card foundation — pictures stored beside a card rather than inside the
list every edit sends — is what retires the whole-list write; it is being
built in parallel and this node is the stopgap that keeps Saturday working
until it lands. Raising a budget is not the fix; it is the thing that buys
time for the fix.

**The cost this node makes worse, measured.** `/converge` relays each accepted
op to the sender's other instances by `publish`ing the resolved value into
`/messaging`'s broadcast slot, which keeps its last 50 entries **by count, not
by bytes**, and every parked long-poll re-reads and re-parses that whole file
five times a second. So the slot's worst case scales with `LIST_CAP`: 50 × 160KB
≈ 8MB before this node, 50 × 640KB ≈ 32MB after. Measured on a debug rig, one
parked `POST /msg/wait` — nominally 125 ticks of 200ms, 25s — took 26.1s on an
empty slot, 32.2s at 8MB and 43.1s at 32MB: 9ms, 57ms and 145ms of read-and-
parse per tick, so at the new ceiling a single parked client spends about
three-quarters of a core doing nothing but re-reading a file that has not
changed. A release build is faster than a debug one by some factor nobody here
has measured, and the shape is the same either way: it is linear in
`LIST_CAP × 50 × clients`. This is a pre-existing shape, not a new one — it is
reachable today at 8MB — but this node moves it four times further along, so it
is on the record here rather than discovered on Saturday. The honest
mitigations are a byte cap on the slot, a per-user slot file, or a modification
time check before the parse; all three are outside this node's footprint. The
per-card foundation removes the premise entirely, because a per-card op is
small.

**Untick and `/wider`'s 192KB/160KB return.** A world already over 160,000
chars would then have every whole-list write refused — dropped and reported
now that `/past-a-refusal` exists, jammed forever before it — so unticking
after a big list exists is a migration, not a toggle. That is the same hazard
`/roomier` and `/wider` each documented, one rung larger.

## glossary

(no new terms)

## code description

`room-for-a-team.rs` redefines `/messaging`'s `msg_body_cap` to 1048576.

`room-for-a-team.js` sets `feature_Cards.LIST_CAP` at load, the way `/roomier`
and `/wider` do — `cards.js` reads it at use, so the number leaves with this
node.
