# black-stem
*a pin's stem is the same near-black as the face's halo*

> (transcripts/2026-09-03-housekeeping.md#p13)
> can the "arrow" part of the marker be black, the same as the outline of the post? That would make it stand out better against the dark grey map.

## user

The little arrow under every map pin is black, like the dark ring around the face, so a pin stands out against the dark grey map.

## spec

`/map` drew the stem in the app's border grey (`#3a3a3f`), which sits too close to the Stadia dark ground (`/map-ground`, `#333333`) to read. Ash asked for the stem in the colour of the post's outline — the face's halo, `#101012` (#p13). One reading, so it builds: one colour on the stem, the halo's; the grey drop-shadow the stem carried under it goes, since a near-black stem needs no darker edge. Untick and the stem is grey again.

## glossary

(no new terms)

## code description

`black-stem.css` — one rule on `.map-pin-stem`.
