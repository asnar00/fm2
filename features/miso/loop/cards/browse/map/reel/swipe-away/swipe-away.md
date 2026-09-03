# swipe-away
*swipe a post left or right to put it away and return to the map and the reel*

> (transcripts/2026-09-03-housekeeping.md#p19)
> when a post is opened, we should be able to dismiss it by swiping left or right, which should return us to the reel+map (currently the reel disappears when we tap the background to dismiss the post)

## user

With a post open over the map, flick it sideways — either way — and it slides off; the map and the reel are there behind it.

## spec

The way back from a post is the tool's own button, which `/backdrop` sends for a tap on the ground; ash asked for a sideways swipe as well (#p19). One reading, so it builds: a pointer that goes down on the page (not in a photo window, which pans and pinches by touch, and not while a block is being written) and comes up within 600 ms having moved 60 px or more sideways and less than 40 px up or down is a swipe; the page slides out that way in 0.18 s and, when the slide ends, the tool's own button is sent — the same tap `/backdrop` sends — so `/browse` puts the set back and the map and the reel return. Only over the map view (`fm-map-behind`), where there is a map to return to. The band's own vanishing after a backdrop tap was `/reel` reading the state mirror a frame late, fixed there. Untick and the page is put away by tap alone.

## hostile cases

- A vertical scroll of the page: too much up-and-down, not a swipe.
- A pan inside the photo window: begun on the window, not read here.
- A slow drag: over 600 ms, not a swipe.
- A post opened from the grid: no map behind, no swipe.

## glossary

(no new terms)

## code description

`swipe-away.js` — capture-phase pointerdown/pointerup on the document; the slide class; the send on `animationend`.

`swipe-away.css` — the two slides.
