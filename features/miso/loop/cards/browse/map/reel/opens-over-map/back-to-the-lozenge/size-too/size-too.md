# size-too
*the card closes onto the lozenge's whole rectangle, on every road out*

> (transcripts/2026-09-04-field-walk.md#p34)
> the "animate-closed" should animate the size of the rectangle as well, so that it animates to the final lozenge position and size, making it clear that it's the same thing

## user

Close a post and the card becomes the lozenge: it travels to the lozenge's place and shrinks to the lozenge's size — its height as well as its width — so the two are plainly one thing. Every way out does it: the ‹, a tap on the map, and a sideways flick.

## spec

`/back-to-the-lozenge` scaled the card uniformly to the lozenge's width. That is a third of the card's height at the end, not the lozenge's: the bottom two thirds went off the foot of the screen while the top slid down, so the card read as moving away rather than as becoming the lozenge. Ash saw it move and not shrink (#p34).

**Both axes.** The scale is taken on the width *and* the height, so the card ends exactly the size and the place of the lozenge. A card squeezed to a lozenge's height is a squashed card for the last of those milliseconds, so the fade is deeper and front-loaded — full, then 55% at the halfway mark, then 20% — and by the time the squash is legible the card is nearly gone. Transform and opacity only, as before: the phone's main thread is at its busiest in exactly this moment.

**Every road.** `/back-to-the-lozenge` left the sideways flick alone, on the reasoning that `/swipe-away`'s exit is the platform idiom and a shrink on top of it would haul the card back into view. Ash's word is that the closing animates size on every road, so this node owns the whole of the closing instead of laying a second motion over the first: `/swipe-away`'s sideways animation is off while this node is ticked (a rule in this node's own stylesheet, not an edit to its file), and the shrink starts on the gesture rather than waiting for the send. `/swipe-away` still sends — its send has a timer behind the animation as well as the animation's own end — and by then the card is already on its lozenge, so the send goes straight through.

Untick and the card closes to the lozenge's width at its own proportions, and the sideways flick takes it off the side again.

## hostile cases

- **The ‹ and the tap on the map.** The parent's road, with this node's shape: one shrink onto the whole lozenge.
- **A sideways flick.** The shrink starts on the finger's release; `/swipe-away`'s own send arrives a quarter of a second later and passes through, because the parent's interception sees a closing already running.
- **A flick with no lozenge to land on** (a post the reel does not list). No rectangle, no early shrink; `/swipe-away`'s send closes the page as it would have, and with its own motion off it simply goes.
- **`prefers-reduced-motion`.** No shrink on any road; the reel and the map are still put right.
- **A card page that is not over the map.** `/swipe-away` never runs there and the parent never intercepts; nothing here applies.
- **Two flicks in a row.** The second finds a closing already running and is dropped.
- **A lozenge of zero height** (the reel not laid out). No frames; the parent falls back to sending without a motion.

## glossary

(no new terms)

## code description

`size-too.js` — `feature_SizeToo`.

`frames(page, r)` is the parent's keyframe /extension point/, redefined: a
scale on both axes onto the lozenge's rectangle, with the fade taken to 55% at
the halfway mark and 20% at the end.

`shrinks(page)` is redefined to true, so the sideways flick is no longer the
one road without a shrink.

`catchFlick()` starts that road's shrink on the release rather than on the send
that follows it, and leaves the parent's `going` flag set so the send passes
straight through when it arrives. The `pointerup` listener that calls it is
registered after `/swipe-away`'s, so the card is already marked when it runs.

The wrapper on `feature_Loop.send` lets that flag go once the send has gone,
and aims the reel again afterwards for the reason the parent does: `/reel`
redraws its band when the map's set comes back.

`size-too.css` — `/swipe-away`'s sideways exit off while this node is ticked.
