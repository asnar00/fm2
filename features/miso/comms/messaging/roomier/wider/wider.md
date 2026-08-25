# wider
*room for several pictured cards*

> (transcripts/2026-08-25-accounts.md#p50)
> always fix residuals before calling a job done
> *(the residual: "a second card with a picture will not fit" — cards.md, roomier.md)*

## user

Your world can hold several cards with pictures — about six — before the app says there is no room.

## spec

`/roomier` set the message cap at 64KB and the cards list at 56KB, which fits one pictured profile and not a second pictured card. That was recorded as a residual for the var-per-card rung; ash ruled residuals get fixed in the run (#p50). This node widens the wire again — 192KB per message, 160KB for the list — so a profile, a project and a few pictured posts fit today. The honest cost is unchanged in kind: every edit still resends the whole list as one op, so a full world costs ~160KB per edit on the wire and in the op log. That is the price until cards travel one per op, which is the named rung, now with room to breathe rather than a wall.

Untick and `/roomier`'s 64KB/56KB return; a world already over 56KB would then jam its outbox (the migration hazard `/roomier` documents).

## glossary

(no new terms)

## code description

`wider.rs` redefines `msg_body_cap` to 196608. `wider.js` sets `feature_Cards.LIST_CAP` at load.
