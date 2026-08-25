# page
*the card as a page you open and edit*

> (asks#1787669564115)
> add grid and list views for multiple cards
> *(the ask whose node needed a seventh child of `cards`, forcing this regroup)*

## user

Browse the children: your own card behind 👤 (`/me`, with `/stay` and `/patient`), editing that keeps your words (`/keep`, with `/newline`), and choosing the bit of a photo you want (`/frame`).

## spec

Grouping node, created under the 4–6 children rule: `cards` stood at six (me, keep, frame, layout, marks, guard) when `/browse` arrived. The three nodes about *the card as a page* — reaching it, editing on it, choosing its picture — live here; `layout` keeps placement, `marks` the card's own data, `guard` the store's safety. Provenance-ordered linearisation means the grouping changes no behaviour — verified by an fmlink `--chains` diff before and after. Contributes no code.

## glossary

(no new terms)

## code description

No implementation files — a grouping node; `order.md` orders the children.
