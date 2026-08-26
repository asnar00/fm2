# toolbar
*edit and save are toolbar buttons, not pills on the page*

> (transcripts/2026-08-26-session.md#p136)
> the "edit" and "save" buttons should go in the toolbar - there should never be a button just floating in the main space

## user

Open one of your own cards: a pencil sits in the toolbar. Tap it and the card unlocks; the pencil becomes a tick. Tap the tick and the card is a page again. Nothing floats over the card.

## spec

`/editing` and `/manual` each hung a pill above the toolbar — `edit` on a locked own card, `save` while a block was being written. Ash's ruling (#p136): controls live in the toolbar; nothing floats in the main space. One reading, so it builds. This node hides both pills and puts one control in the toolbar's row whenever an own card is on screen — before `/delete`'s bin, before undo (undo stays last, `/glyphs`), in the open tool's colour read off its lit button. Its face is `/editing`'s state: a drawn pencil while the card is locked, a drawn tick while it is open. The tick's tap is a tap-away — the blur saves, then the card locks. The control is placed from the DOM on every paint by extending `/editing`'s own `apply`, so a repaint of the toolbar (every loop event) puts it back. Untick and the pills return.

## hostile cases

- A repaint mid-edit: `apply` runs again, finds the control gone, puts it back with the right face.
- On the phone, the pencil's tap closed the post instead (#p140): `edit()` focuses the words, the keyboard rises, the toolbar shifts, and the tap's click hit-tests the ground — `/backdrop`'s cue to close. The pointerdown arms a swallow for the one click that follows, wherever it lands — not for a time (600 ms was still too short with the keyboard rising, #p158) but until that click comes or another press lands elsewhere.
- A foreign card: `/editing` has no page for it, so no control.
- A tool without a colour: the control is untinted, like undo before `/tinted`.

## glossary

(no new terms)

## code description

`toolbar.js` — wraps `feature_Editing.apply` to place the control before delete/undo; a capture-phase pointerdown that edits or saves-then-locks (a click would miss the repainted button); the click swallowed; the two glyphs in currentColor.

`toolbar.css` — hides the pills; black glyph on the tint.
