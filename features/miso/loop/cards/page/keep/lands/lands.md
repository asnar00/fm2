# lands
*the tap that saves your words still lands*

> (transcripts/2026-08-26-session.md#p150)
> I'm seeing an issue where the "back" button doesn't work the first time you press it, then works the second time you press it. I also see that issue sometimes with the grid/list/map selector up top.

## user

Writing in a card, tap ‹ or the grid/list/map picker: it works the first time.

## spec

A tap is a pointerdown and then a click. While a block is being written the pointerdown blurs it; the blur is `/keep`'s save; the save repaints the screen; and the button under the finger is a new element by the time the click arrives — the click lands on nothing, and `/backdrop` may even read the bare ground as "close the card". Ash saw it on ‹ and on the picker (#p150); `/editing/toolbar` had met the same thing on its pencil and swallowed its own click. One reading, so it builds, and generally: the pointerdown remembers the `data-ev` under the finger; a click within the same tap (700 ms) that finds no `data-ev` sends that event itself and stops there. A click that lands is left to `/loop`; a hold that `/long-press` fired is not a tap. Untick and the second press is needed again.

## hostile cases

- A tap that lands normally: `/loop` sends it once; this node sees a button under the click and stays out.
- A long-press on a tool button: `/long-press` marks `fired`; no resend.
- Pointerdown on a button, finger dragged off, released elsewhere: the click lands off-button within 700 ms and the event is sent — the same as a repainted button; a drag cancels by taking longer.

## glossary

(no new terms)

## code description

`lands.js` — a capture-phase pointerdown that notes the event under the finger; a capture-phase click that re-sends it when the click found no button.
