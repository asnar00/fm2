# on-people-map
*the reel shows on the people map too, and it is posts there as well*

> (asks#1788448540972)
> users view should also show reel just posts
> *(filed from the field on 2026-09-03 by ash)*

## user

Open 👤 and its map, and the band of posts is there along the bottom, just as on the posts map: posts, not people. Flick it and the map follows.

## spec

`/reel` shows only on the posts tool's map, and lists the tool's set — on the people map that set is people. Ash asked for the band on the people map too, showing posts (asks#1788448540972). One reading, so it builds: the band shows on the people tool's map as well; `#mapData` carries the posts set's ids beside the set it drew (`posts_set`, the sifted set the posts tool shows), and on the people map the band keeps to those. A post there has no pin, so its place is read from its own location block rather than from a pin — which also makes the band's pan independent of pins everywhere. No pin is ringed on the people map; the lozenge's outline still marks the current post. Untick and the band is the posts map's alone.

## hostile cases

- The people map with no posts in the set: no band.
- A post without a place: listed, no pan.
- The project's map (if any): not the people tool, no band beyond what `/reel` gives.

## glossary

(no new terms)

## code description

`on-people-map.rs` — redefines `map_surface_html` to add `data-post-ids` (the posts set) to `#mapData`.

`on-people-map.js` — wraps `feature_Reel.showing` (the people tool too) and `feature_Reel.posts` (the posts set on the people map; a place from the card's own block).
