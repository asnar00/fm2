# centred
*the type tag sits on the title's centreline, its right edge on the media's*

> (transcripts/2026-09-03-housekeeping.md#p14)
> on general cards, let's move the "profile"/"post" lozenge indicator down a bit (so its centerline matches the centerline of the title) and left a bit (so its right edge matches the right edge of the media area below it)

## user

The little type tag in a card's corner sits level with the middle of the title, and its right edge lines up with the right edge of the picture beneath.

## spec

`/aligned` put the tag's top edge on the title's top edge (16 px, the page's padding). Ash asked for its centreline on the title's, and its right edge on the media's (#p14). Measured on the page: the title's box is 40 px tall (22 px type at 1.2, 6 px padding each side, a 1 px border) starting 17 px in from the page's top edge (16 padding, 1 border), so its centre is 37 px down; the tag is 20 px tall, so its top is 27 px. The media block starts 17 px from the page's right edge for the same reason, so the tag's right is 17 px. Two numbers, both derived from `/cards`' own padding and type; a change there moves them. Untick and the tag rides the title's top edge again.

## glossary

(no new terms)

## code description

`centred.css` — one rule: `top` and `right` on `.card-tag`.
