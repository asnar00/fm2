# touches
*the finger is in the black box*

> (transcripts/2026-08-26-session.md#p159)
> I feel like we're just guessing, rather than actually testing. Let's think about how to properly replicate this issue - we're talking about a simple button press, after all

## user

Nothing to see. When a tap does the wrong thing on your phone, the builder can read exactly what your finger did — touch, press, click, focus, and how the screen had shifted under it — from the same flight recorder that already keeps your taps.

## spec

The black box records loop events — what the app decided — but not what the finger did, so a tap that closed a card instead of editing it could only be theorised about from a laptop (#p159). This node records the finger: every `touchstart/end/cancel`, `pointerdown/up/cancel`, `click`, `focusin/out` on the document, in the capture phase and passively, as a `{type:'ui'}` entry with the event's target, the element under the point, the point, the visual viewport's offset and height, the scroll, and what had focus; plus every visual-viewport resize and scroll (the keyboard's rise). They ship with the rest of the box. `/replay` re-sends entries to the loop; a `ui` entry is a type no update link knows, so it passes through. Untick and the box is back to loop events only.

## hostile cases

- Volume: a tap is ~7 entries; the ring's count cap (500) and age cap (5 min) bound it — a busy minute shortens the window, and that is the trade.
- A touch with no point (focus events): recorded with 0,0 and no `under`.

## glossary

- **ui entry**: a black-box record of an input event rather than a loop event.

## code description

`touches.js` — the listeners, and `feature_Blackbox.record` for each.
