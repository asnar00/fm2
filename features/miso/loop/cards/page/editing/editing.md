# editing
*a card opens read-only; edit unlocks it, save locks it*

> (transcripts/2026-08-25-accounts.md#p125)
> rather than being able to tap a post to edit it, there should be an edit button that enables that action (only visible to author).

## user

Open one of your own cards and it reads like a page. Tap **edit** to change the words or the picture; tap **save** (or tap away) and it is a page again. Other people's cards have no edit.

## spec

Every own card was editable on touch — a tap in the words placed a caret, a tap on the picture opened the chooser — which made reading a post feel like standing on a form. Ash's ruling (#p125): an edit button, author only. One reading, so it builds. This node keeps the renderer as it is and locks the page from the DOM on every paint: an own card (`.card-page` without `/exchange`'s `.foreign`) loses `contenteditable` on its blocks and gains `.locked`, under which the picture's tap and long-press are swallowed in the capture phase; an **edit** pill sits above the toolbar while a locked own card is on screen; tapping it marks the card open, restores `contenteditable`, focuses the words (so `/manual`'s save pill takes over); a tap on **save** locks again after the saving blur. A card you have just made — **+** or **new** — opens in edit, ready to write. A foreign card was already read-only and shows no pill. Untick and touch-to-edit returns.

## hostile cases

- A repaint mid-edit: the observer re-applies the open state — the blocks stay editable.
- Closing the card while editing: the page leaves the DOM; the open mark is dropped on the next apply of that card.
- Two own cards in a row: the mark is per card id.

## glossary

(no new terms)

## code description

`editing.js` — `apply` locks or unlocks the own page from the DOM; `edit`/`lock` flip the per-card mark; the pill; capture-phase guards on the locked picture; a MutationObserver on `#app`.

`editing.css` — the locked look and the pill.
