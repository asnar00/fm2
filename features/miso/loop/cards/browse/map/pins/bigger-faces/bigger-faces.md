# bigger-faces
*a map pin's face is half again as big*

> (asks#1788448692127)
> let's make map Thumbnails 50% bigger
> *(filed from the field on 2026-09-03 by ash)*

## user

The faces on the map pins — a post's picture, a person's face — are half again as big, and easier to see and to tap.

## spec

`/map` draws a 34 px face on a 40 × 50 pin whose tip is the place. Ash asked for the thumbnails 50% bigger (asks#1788448692127). One reading, so it builds: the face is 51 px, the pin 60 × 75, the stem in proportion; the pin is drawn from the same Leaflet box as before, pulled up and out by margins so its tip still stands on the place and `/fan-out`'s turn about the tip is unchanged. `/fan-out` grows a stem by writing the pin's height and the stem's length from the old base; this node follows each turn and writes them from the new base, so a fanned pin's tip stays put too. Untick and the pins are their old size.

## glossary

(no new terms)

## code description

`bigger-faces.css` — the sizes, and the margins that keep the tip on the place.

`bigger-faces.js` — wraps `feature_FanOut.turn` to grow from the new base.
