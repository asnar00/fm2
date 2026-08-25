# roomier
*a message may be 64KB, so a card can hold a real photograph*

> (transcripts/2026-08-25-accounts.md#p21)
> I tried to switch the user picture and it said "that picture is too big to keep" - either we should resize the picture, or increase the size limit.

## user

Your card's picture can be a proper photo now: it is shrunk to 384px on the
phone and kept at up to 24KB, and a picture straight from the camera roll
fits. "That picture is too big to keep" should not appear for ordinary
photographs any more.

## spec

`/messaging` truncates any message body over `msg_body_cap()` bytes before
parsing it — 16KB in the base — and `/cards` sized its picture budget to fit
under that: 8KB per picture, which at 256px forced JPEG quality down to 0.2
and still refused a portrait photo from ash's phone (#p21).

This node widens the wire and lifts the card budget with it. `msg_body_cap`
becomes 64KB — the serve layer's own read limit is 16MB, and the broadcast
list is capped by count (50 entries), not bytes, so nothing else has to move.
The card budget follows: `EDGE` 384px, `CAP` 24KB per picture, `LIST_CAP`
56KB for the whole list, leaving room for the op's envelope. A 384px JPEG at
quality 0.8 is ~15–20KB, so the quality ladder rarely steps at all.

Costs, named: every text edit to a card still rewrites the whole list as one
op, so a card with a 20KB picture costs ~20KB per edit on the wire and in the
op log. That is the whole-list var's price, and the var-per-card + blob rung
(cards.md) is what removes it — this node buys a usable photo until then.

Untick this node and the base numbers return: 16KB messages, 8KB pictures. A
card already holding a 20KB picture will then jam that instance's outbox
(the failure cards.md describes), so unticking after photos exist is a
migration, not a toggle.

## glossary

(no new terms)

## code description

`roomier.rs` redefines `/messaging`'s `msg_body_cap` to 65536.

`roomier.js` sets `feature_Cards.CAP`, `LIST_CAP` and `EDGE` at load —
`cards.js` reads them at use, so a later fragment owns the numbers. `EDGE`
was extracted from `shrink` for this node (behaviour unchanged at 256).
