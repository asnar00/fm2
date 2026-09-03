# floating
*the reel floats over the map; the map shows between the lozenges*

> (transcripts/2026-09-03-housekeeping.md#p19)
> the reel should float over the map but not have a dark background (you should be able to see the map between the post previews)

## user

The band along the bottom has no ground of its own: the lozenges float over the map, and the map shows between and around them.

## spec

`/reel` gave the band a dark ground and made the map shorter to sit above it. Ash asked for the band to float over the map with no ground (#p19). One reading, so it builds: the band's ground and border go and the map runs to the toolbar again; the lozenges keep their own dark faces so they read over any tile. The place the map glides to is then under the band, so the pan aims a half-band higher: the post's point is projected, moved down by half the band's height, and the map is panned to that — the place lands in the clear above the lozenges. Untick and the band has its ground again and the map stops above it.

## glossary

(no new terms)

## code description

`floating.css` — the band's ground and border off, the map's inset off.

`floating.js` — redefines `feature_Reel.pan` (the seam `/reel` opened for it, so `/current`'s wrap of `follow` survives whatever order the two load in — the first cut replaced `follow` and lost the mark, #p21) to aim a half-band higher.
