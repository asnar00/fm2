# from-the-lozenge
*the post grows out of the lozenge you tapped*

> (transcripts/2026-09-04-field-walk.md#p6)
> 1) when you tap a reel lozenge, I'd like the post to animate open from the lozenge to the full card, so it's clear that it's the same thing

## user

Tap a lozenge in the reel and the post opens out of it: the card starts exactly where the lozenge was, at the lozenge's size, and grows into place over the map in about a quarter of a second. The lozenge and the card are plainly the same thing. Once it is open it stays still — nothing pulses at it while it is being read.

## spec

`/opens-over-map` brings the page up with a short grow from 96% of its own centre — a nice arrival, but not one that says *this* post came from *that* lozenge (#p6). This node gives the opening the lozenge's own rectangle to start from.

**The rectangle is read at the tap.** `/opens-over-map` hides the reel the moment the page is up, and a hidden element has no rectangle left to read — so a capture-phase listener notes the tapped lozenge's rectangle and the post's id as the tap goes through, ahead of `/loop`'s own listener. A tap anywhere else forgets it, so a pin tapped after a lozenge does not open from the lozenge's old place, and the opening runs only when the id the page arrives with is the id that was tapped.

**The motion is a translate, a uniform scale and a fade.** Two keyframes on the page: the lozenge's rectangle at 55% opacity, then the page's own at full. The scale is uniform — a page squashed into the lozenge's shape is not the same thing seen smaller. 240 ms, `ease-out`, nothing bouncy: longer than the house 0.18 s because this one crosses the whole screen, and at 0.18 s it reads as a snap rather than a move. Transform and opacity are the whole of it because they are the only two properties a browser runs off the main thread, and the main thread is at its busiest in the moment a post opens: a first cut carried a `clip-path` leg as well — a band widening into a page, which reads better still — and on the rig it froze with the main thread for 226 ms in the middle of the opening. Under `prefers-reduced-motion` nothing here runs and `/opens-over-map`'s own arrival stands.

**It survives the repaints, and it ends the pulsing.** Every paint is an `innerHTML` swap that throws the card element away, so an opening in flight is measured by the wall clock, taken off the old element and put on the one that comes back at the point it had reached. (The animation's own `currentTime` is not that number: it reads 0 until the browser has resolved a start time, and carrying *it* across a burst of paints held the card still for a quarter of a second before it moved — measured, then fixed.) The same swap is why the card pulsed while it was being read: `/opens-over-map`'s grow-from-96% is a CSS animation on a fresh element, so it ran again on every single paint. From the second paint of a post onwards it is switched off, and the switch is handed straight from the growing class to the settled one — a rule going from `animation: none` back to a named animation *starts* it, so merely dropping the class made the card pulse the moment the growth finished.

Untick and the page arrives with `/opens-over-map`'s grow from its own centre, once per paint.

## hostile cases

- **A post opened from a map pin, or from the grid or the list.** No lozenge was tapped, or the tapped id does not match; the page arrives as `/opens-over-map` draws it.
- **`prefers-reduced-motion`.** No opening motion at all; the parent's arrival stands.
- **A repaint mid-motion.** The motion is carried onto the new element at the point the clock says it had reached. Proven with three repaints forced into the first 180 ms.
- **A repaint that switches to a different card mid-motion** (the flick). The id no longer matches; the carried motion is dropped and the new card arrives plainly.
- **A repaint of the same card once it is open.** No motion at all — this is the pulsing that is now off.
- **The reel not yet drawn, or a lozenge with no size.** No rectangle, no animation.
- **The card page a different width than when the keyframes were built** (the clip arrived and the layout moved). The start is a few pixels off the lozenge — 7 px on the rig — and the end is the card's own size, always.

## glossary

(no new terms)

## code description

`from-the-lozenge.js` — `feature_FromTheLozenge`.

`mark(el)` notes the tapped lozenge's rectangle and post id. The document
click listener that calls it is capture-phase, so it runs ahead of `/loop`'s
own, and passing it anything that is not a lozenge forgets the last one.

`over()` is the page standing over the map right now, read from
`/opens-over-map`'s own body mark.

`frames(page, from)` builds the two keyframes — the lozenge's rectangle as a
translate and a uniform scale at 55% opacity, then the page's own — and
`play(page, frames, began)` runs them with the Web Animations API from
wherever the wall clock says the opening has got to. `settle(page)` hands the
suppression of the parent's grow to the settled class, takes the growing one
off, and *cancels* the animation rather than leaving it finished: an animation
left filling goes on applying its last frame for the life of the element, and
that beats any inline transform a later gesture puts there. Its last frame is
the page's own place, so cancelling changes nothing on screen.

The wrapper on `feature_Loop.paint` is the whole of the timing: before the
swap it takes the id, keyframes and start time of an opening in flight and
cancels it; after the swap — which is after `/map`'s sync and
`/opens-over-map`'s mark, both of which run inside this one — it resumes that
opening on the element that came back, or marks a repaint of a card already
open, or starts an opening for a page that has newly arrived carrying the id
that was tapped.

`from-the-lozenge.css` — `/opens-over-map`'s arrival animation is off while
the growth runs, and off for good once the card is open.
