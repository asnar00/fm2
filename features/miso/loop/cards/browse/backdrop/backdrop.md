# backdrop
*a tap on the ground closes an open card*

> (asks#1787703391848)
> clicking off the background of an expanded card should close it and return to list/grid/map view
> *(filed from the field on 2026-08-26 by ash, birthplace `posts @ miso/loop/cards/kinds/posts`)*

## user

With a card open, tap the dotted ground beside or below it and it closes, back to the grid, list or map you came from.

## spec

The way back from an open card is the tool's own button (`/browse`); ash asked for the ground to work too (`asks#1787703391848`). One reading, so it builds: this node listens for a click that lands on nothing anybody owns — not the card page, the toolbar, the view picker, the lozenge, the panel, a sheet, a control — while a card page is on screen, and sends the same tap the tool's button would. The view you came from is `/browse`'s device var, so it is what you return to. Untick and only the button takes you back.

## hostile cases

- A tap on the map (no card open): nothing — there is no card page.
- A tap in a sheet over a card (framing, add a person): the sheet is owned; nothing.
- Tapping a card's own empty margin: inside `.card-page`, owned; nothing.

## glossary

(no new terms)

## code description

`backdrop.js` — one delegated click listener; the owned-selector list is the whole rule.
