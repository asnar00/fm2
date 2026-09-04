# carries-the-card
*the sweep moves the card with your finger, and the next one slides in*

> (transcripts/2026-09-04-field-walk.md#p6)
> 3) swiping up and down should scroll the card physically upwards/downwards and the next/prev card should scroll properly in

## user

At the bottom of a post, drag up and the card comes with your finger. Let go far enough and it carries on off the top while the next post slides up into its place. Let go short and it settles back where it was. Drag down at the top of a post and the previous one comes down into place the same way. A card that still has more to read scrolls, as it always did.

## spec

`/flick` and `/on-touch` already read the sweep and switch the card — but nothing moved until the switch had happened, so the gesture had no body: you swept, nothing, then a different card. Ash asked for the card to move with the finger and the next one to come in properly (#p6).

**The rule is `/flick`'s, unchanged.** Sixty pixels, under forty sideways, within 600 ms, and only at the end the card is already at. This node adds no threshold of its own: it reads `feature_Flick.down` — which `/flick` has already set, because its listeners were registered first — to know where the sweep began and which end it began at, moves the card while the finger is down, and then does whatever `/flick` decided. If `release` called `go`, the switch is under way; if it did not, the card springs back.

**The carry.** Once the finger has moved 8 px, more vertically than sideways, in the direction `/flick` would act on, the card takes a `translateY` equal to the finger's travel, one pixel to one pixel. The page is looked up again on every move, so a repaint under the finger does not drop the gesture. On the phone `touchmove` is the road and the move is `preventDefault`ed once the carry has been claimed — otherwise the browser turns it into a rubber-band on the page behind. The pointer road is the desktop's: a pointer event of type touch is left alone, because iOS cancels those the moment it takes a gesture for itself (`/on-touch`'s own lesson).

**The switch.** `go` is `/flick`'s send-once, and this node redefines it: the card slides on from where the finger let go to a full screen height away in 140 ms, `/flick`'s event goes then, and the card that arrives comes in from the far side over 190 ms with a short fade. `last` is zeroed just before the deferred send so `/flick`'s own 400 ms rule reads it as this gesture's rather than a second one — and only on that road, so the immediate road keeps the parent's dedupe intact. A sweep arriving while a switch is running is dropped, which is what stops the pointer road and the touch road starting two switches for one gesture.

**Nothing is left behind.** Every paint throws the card element away, so an animation in flight is measured by the wall clock and put on the element that comes back; when the card does not come back at all — the post was closed, the tool was left — the gesture is let go rather than held for ever, and a switch that somehow never finishes releases after a second and a half.

Untick and the sweep still switches the card, with nothing moving in between.

## hostile cases

- **A card with more to read.** `/flick` armed away from an end; the carry never engages and the browser scrolls as before.
- **A sweep short of the threshold.** `/flick` does not call `go`; the card springs back over 190 ms from wherever the finger left it.
- **A sweep at the end of the list.** `go` sends, the Rust changes nothing, and the same card comes back the way it went — 140 ms out, 190 ms back, which is the answer "there is nothing that way".
- **A sideways sweep** (`/swipe-away`). More sideways than vertical; the carry never engages and `/flick` refuses it too.
- **Two fingers.** The gesture is dropped.
- **A device that fires both roads.** The touch road holds the gesture; the pointer road is ignored for touch pointers, and a second `go` inside the switch is dropped.
- **A repaint mid-carry or mid-slide.** The finger keeps the new element; an animation is resumed on it at the point the clock says it had reached.
- **The card page gone mid-slide** (closed, or the tool left). The pending send still goes — the Rust does nothing without an open card — and the gesture is released.
- **`prefers-reduced-motion`.** Nothing is carried and nothing slides; `/flick` switches the card as it did before.
- **A slow drag** (over 600 ms). `/flick`'s own rule refuses it, so the card springs back however far it was dragged.

## glossary

(no new terms)

## code description

`carries-the-card.js` — `feature_CarriesTheCard`.

`begin()` takes the sweep `/flick` has just armed; `move(x, y)` decides once
whether this is a carry and then keeps the card under the finger, returning
true so the caller can `preventDefault`; `end()` springs the card back unless a
switch is already running.

`out(dir, send)` is the switch: the card away, then `send`, then the card that
arrived in from the far side — or the same card back, when the list had no
neighbour to give.

`play(page, frames, dur, cls, began, done)` runs one animation with the Web
Animations API from wherever the wall clock says it has got to, and `clear`
takes the class and the inline transform off at the end.

The redefinition of `feature_Flick.go` is the seam: the send is deferred behind
the card leaving, and a second one during the switch is dropped.

The listeners are `touchstart`/`touchmove`/`touchend`/`touchcancel` for the
phone and `pointerdown`/`pointermove`/`pointerup`/`pointercancel` for
everything else, all registered after `/flick`'s and `/on-touch`'s so the
parent has always armed and released first.

The wrapper on `feature_Loop.paint` carries an animation in flight across the
DOM swap, and releases the gesture when the card does not come back.

`carries-the-card.css` — the two moving states, and `/opens-over-map`'s own
arrival animation off while either is running.
