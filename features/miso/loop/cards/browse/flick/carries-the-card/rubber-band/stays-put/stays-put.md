# stays-put
*a card the finger has held does not play its arrival again*

> (transcripts/2026-09-04-field-walk.md#p151)
> At the top of the list, the bounce works correctly, but the entire card flashes (including the background - not just the media). At the bottom of the list, it feels like a new card scrolls into view, but it's the same as the card scrolling out. In both cases, I want the bounce to happen (i.e. we ping back to the card and prevent scroll) and the same card to stay onscreen, and not flash or be updated in any way.

## user

Pull past the newest post or the oldest and let go. The card springs back and that is all that happens: the same card, the same picture, the same ground, nothing fading in, nothing arriving.

## spec

**Both halves of the ask are one fault, and it is not end detection.** Measured in WebKit at each end, with the two ends' own numbers: at the top the open card is index 0 of 27 in both `/rubber-band`'s list and `/unbroken`'s; at the bottom it is index 26 of 27 in both. Neither node disagrees with the other, no strip starts, **no paint, no click, one `.card-page` throughout, the same node object and the same parent before and after**. Nothing is reparented, nothing is rebuilt, no turn goes.

What the frames show instead, at the moment the carry ends:

```
 44 ms  will-change transform,opacity  matrix(1,0,0,1,0…)  fm-carried
563 ms  will-change transform          matrix(0.96,0,0,0…) fm-settling
575 ms                                 matrix(0.965393,…)
 …                                     climbing to 1
745 ms  will-change auto               none
```

The card **scales from 0.96 back to 1** after the finger has gone. That is `/opens-over-map`'s `fm-card-grow`, whose keyframes are `from { transform: scale(0.96); opacity: 0 }` — the whole card and its ground fading in from nothing. `/carries-the-card` suppresses that animation while it holds the card (`animation: none` under `.fm-carried`) and lets the suppression go when the carry ends; **a rule going from `animation: none` back to a named animation starts it**. So the card replays its own arrival every time a carry finishes — which at the top reads as the entire card flashing, and at the bottom, on a card with no other mark on it, reads as a new card scrolling into view when it is the same one.

(The same trap was found and written down once already, in `/from-the-lozenge`, which is why a card opened from a lozenge carries `fm-loz-settled` and never showed this. A card reached by sweeping never gets that mark, because it is only added on a repaint of a card already open — and at the ends there are no repaints at all.)

**So the arrival is ended for good.** One class goes on the card when a gesture takes it and never comes off: not a toggle, because a class that comes off is a class that can restart what it was holding back, which is the whole of this bug. The element is thrown away by the next paint in any case, so nothing accumulates.

Nothing else changes: `/rubber-band` still damps at the ends, no strip is started there, `/no-flash` still holds the layer across the hand-off, and the mid-list roads are untouched.

Untick and a card that has been carried plays its arrival again when the carry ends.

## hostile cases

- **A carry that switches** (mid-list). The card that leaves is thrown away and the one that arrives is a new element with no mark — it plays its arrival exactly once, as it should.
- **A card opened from a lozenge.** Already carries `/from-the-lozenge`'s own settled mark; this adds a second rule saying the same thing, and the two agree.
- **A gesture that never engages** (a scroll, a tap). The mark goes on at the arm, so a card that was merely touched also stops replaying its arrival — which is right: it has arrived.
- **`prefers-reduced-motion`.** The parent never arms a carry, so nothing is marked; the arrival was never suppressed and is not restarted either.
- **A card page that is not over the map.** `fm-card-grow` only applies under `/opens-over-map`'s body mark; the rule is harmless everywhere else.
- **The end detection itself.** Measured sound at both ends and left alone; if a list ever did disagree with itself, `/unbroken` already refuses a strip for a card it cannot place.

## glossary

(no new terms)

## code description

`stays-put.js` — `feature_StaysPut.mark()` puts the class on the open card, and
the wrapper on `feature_CarriesTheCard.begin` calls it once the parent has
armed a gesture.

`stays-put.css` — the one rule the class carries: no arrival animation on this
card, under `/opens-over-map`'s body mark or anywhere else.
