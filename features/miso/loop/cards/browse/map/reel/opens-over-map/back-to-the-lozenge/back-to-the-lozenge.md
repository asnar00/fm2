# back-to-the-lozenge
*closing a post puts it back down onto its own lozenge*

> (transcripts/2026-09-04-field-walk.md#p6)
> 4) closing the post should animate the card area down to its reel lozenge (which should have scrolled left/right if you switched which post you were looking at, and focused on the map appropriately)

## user

Close a post — the ‹, a tap on the map, a sideways flick — and the reel is already showing the post you were actually reading, with the map on that post's pin, and the card settles down onto that lozenge as it goes. Sweep through three posts and close: it is the third one's lozenge you land on, not the one you first tapped.

## spec

`/from-the-lozenge` opens the card out of the lozenge that was tapped. Closing had no answering motion, and worse: after `/carries-the-card` has walked the list, the reel was still scrolled to the post that was tapped and the map still on its pin, so the card vanished and left the band pointing at the wrong post (#p6).

**One place catches every road out.** The ‹, the tap on the map and `/swipe-away`'s flick all end in a `feature_Loop.send` of `tools_home` or a `tool_…` click, so this node redefines `send`: when a card page is standing over the map and the event is one of those, the closing runs first and the event goes after. The view picker is not one of them — it leaves the map view, so there is no lozenge to go back to.

**The reel and the map go to the post you are on now**, on every road. The open card's own id is looked up among the reel's lozenges, the reel is shown again (`/opens-over-map` hides it while a page is up; it is coming back in a moment anyway), and its scroll is set so that lozenge sits at the left edge — which is exactly what `/current` calls current and what `/on-the-pin` rings on the map. `/reel`'s own `follow()` then pans the map to that post's place and marks both. A post that has no lozenge — opened from somewhere the reel does not list — skips all of it.

**The card settles onto it.** The reverse of the opening: from the page's own rectangle to the lozenge's, uniform scale, fading to 40%, 220 ms `ease-out`, transform and opacity only so it runs off the main thread. Then the event goes and the page is put away. If the send turns out not to have closed the page after all, the card is put back by hand — an animation left filling goes on applying its last frame for the life of the element.

**Except after a sideways flick.** `/swipe-away` has already taken the card off the side, and swiping a card away sideways is the platform idiom this app committed to (`/learned` 5). A shrink on top of that would have to haul the card back into view in order to send it somewhere else, so that road keeps its own motion and gets the reel and the map only. *This is a judgement, not a reading of the ask, which named `/swipe-away` among the closings that should shrink.*

Untick and the card vanishes on close, leaving the reel and the map where the opening tap left them.

## hostile cases

- **A post reached by sweeping** (`/carries-the-card`). The id is read off the card that is open now, so the reel scrolls to that one and the map goes to its pin.
- **A sideways flick.** The reel and the map are put right; the card keeps `/swipe-away`'s own sideways exit.
- **A post with no lozenge in the reel** (a project's map, a post the set sifts out). Nothing is scrolled, nothing is panned, the page closes plainly.
- **A post with no place.** `follow()` finds no coordinates and leaves the map where it is, as it does for a scroll.
- **`prefers-reduced-motion`.** The reel and the map are still put right; there is no shrink.
- **A second close during the first.** Dropped — the first is already running.
- **A repaint mid-shrink.** The motion is carried onto the element that came back at the point the clock says it had reached; if the card does not come back, the pending event still goes, so a tap is never swallowed.
- **A close that does not close** (a tool button that opens something else). The card is un-shrunk by hand rather than left sitting on the reel.
- **`/reel` unticked.** No lozenges to find; every close is a plain close.

## glossary

(no new terms)

## code description

`back-to-the-lozenge.js` — `feature_BackToTheLozenge`.

`closes(event)` is the test for a closing event; `page()` is the card standing
over the map, read from `/opens-over-map`'s own body mark.

`aim(id)` is the whole of the "put it right" half: find the lozenge for the
post that is open now, show the reel, set its scroll so that lozenge is the one
at the left edge, and let `/reel`'s `follow()` pan the map and mark the pin. It
returns the lozenge's rectangle, or null when there is none. It runs twice —
once before the motion, so the card has somewhere to land and the map moves
under it, and once after the send, because `/reel` redraws its band when the
map's set comes back and puts the scroll at the head while it does (the first
cut aimed only before, and the band was back at the first post by the time the
card had gone).

`close(send)` runs the motion and sends afterwards; `play` is the same
wall-clock animation the opening uses, and `clear(page)` un-shrinks a card that
outlived its own close.

`shrinks(page)` and `frames(page, r)` are the two /extension points/ of the
closing: whether this road has a shrink of its own to run — `/swipe-away`'s
does not, having taken the card off sideways already — and the two keyframes
the card travels between, the page's own rectangle and the lozenge's at a
uniform scale.

The redefinition of `feature_Loop.send` is the interception, and the wrapper on
`feature_Loop.paint` carries the shrink across a repaint and releases the
pending event if the card does not come back.

`back-to-the-lozenge.css` — the moving state, with `/opens-over-map`'s arrival
animation off while it runs.
