# on-the-pin
*the current post's pin on the map wears the light outline; the lozenge keeps a plain outline, no arrow*

> (transcripts/2026-09-03-housekeeping.md#p21)
> I wanted the current post marked on the map, not the reel lozenge - and we don't need the arrow on the lozenge. I guess we can still highlight the lozenge with an outline to show the connection; but the focused post pin on the map should get the light grey outline. […] also, the highlighted lozenge in the reel should always be the leftmost fully displayed one

## user

The pin on the map for the post the reel is on wears a light grey ring; the lozenge keeps a plain light outline so the two read as one, and no arrow. The current post is always the leftmost lozenge that is fully in view.

## spec

`/current` marked the lozenge nearest the left edge with an outline and an arrow, and nothing on the map. Ash's ruling (#p21): the mark belongs on the map — the focused post's pin gets the light grey outline — the lozenge keeps an outline for the connection and loses the arrow, and the current lozenge is the leftmost one fully displayed. One reading, so it builds: `current` becomes the first lozenge whose left edge is at or past the band's scroll position (a few pixels of grace for snapping), the arrow is hidden, and `mark` also rings the face of the current post's own pin — `/map`'s pins carry no id, so this node writes the card's id onto each pin as it is drawn (`/square-posts`' own move), and the match is exact even when two posts share a place — in the light grey, in place of the dark halo; every other pin's ring is taken off. A post with no place marks no pin. Untick and the arrow returns and the mark stays on the lozenge alone.

## glossary

(no new terms)

## code description

`on-the-pin.js` — redefines `feature_Reel.current` (leftmost fully displayed), wraps `feature_Map.pinHtml` (the id on the pin) and `feature_Reel.mark` (the pin's ring).

`on-the-pin.css` — the arrow off, the pin's ring.
