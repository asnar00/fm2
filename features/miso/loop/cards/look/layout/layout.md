# layout
*where the card's parts sit on the screen*

> (asks#1787667662051)
> let's show the card type in the top right corner as a little coloured rounded tag, different colour per type name
> *(the ask that forced the regroup: `cards` stood at six children)*

## user

Browse the children: the card's ground (`/ground`, with `/hug`), its depth beneath the toolbar (`/beneath`), and the picture's width (`/wide`).

## spec

Grouping node, created under the 4–6 children rule: `cards` stood at six (me, keep, frame, ground, beneath, wide) and `/tag` needed a seventh. Everything about *where the card's parts sit* lives here — ground, depth, picture width — leaving `cards` with me, keep, frame, layout and room. Provenance-ordered linearisation means the grouping changes no behaviour — verified by an fmlink `--chains` diff before and after. Contributes no code.

## glossary

(no new terms)

## code description

No implementation files — a grouping node; `order.md` orders the children.
