# urgency
*an ask says whether it is urgent or whenever*

> (transcripts/2026-08-25-accounts.md#p108d)
> hmmm, an idea: what if there were two "send to builder" buttons: one "urgent" and one "whenever"

## user

When nothing found does what you asked, you can send it to the builder as **urgent** or **whenever**. Your requests list shows which, and the builder takes urgent ones first.

## spec

The single *send to the builder* left triage to guess how much an ask mattered — and with `/ship-as-built` the queue is ordered by arrival alone. Ash's idea (#p108d): two buttons. One reading, so it builds. `/ask`'s `file()` gains an `urgency` argument and its results footer is redrawn with **urgent** and **whenever** in place of the one button (the no-match path files as whenever). The filed entry gains `urgency` — this node's `update` link patches the entry `/ask` just wrote, by its `t`. The ask monitor prints `URGENT` on an urgent ask; triage doctrine: urgent asks jump the queue like bugs, whenever-asks ship as built but never pre-empt. Urgency is weight, not colour — a new colour would be a new word. Untick and the single button returns and entries carry no urgency.

## glossary

- **urgency**: `urgent` or `whenever`, the asker's own word on how soon.

## code description

`urgency.rs` — `update` patches the just-filed entry with `urgency` from the `Ask` event.

`urgency.index.js` — replaces `feature_Ask.file` (adds the argument) and wraps `feature_Ask.go` to swap the footer's one button for two.

`urgency.index.css` — the row.
