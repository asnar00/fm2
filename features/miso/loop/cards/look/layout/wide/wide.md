# wide
*the picture spans the card at its own aspect ratio*

> (asks#1787667280766)
> draw card picture so it takes up the full width of the card, with appropriate margins (keep correct aspect ratio!)
> *(filed from the field on 2026-08-25 by ash, birthplace `👤 @ miso/shell/panel/account`)*

## user

Your picture fills the width of your card, inside the card's margins, and keeps its true proportions — a square stays square, a landscape photo stays landscape.

## spec

`/cards` drew the picture as a 148px square, cropping whatever was stored to fit. Ash asked for the picture at the card's full width, with appropriate margins, and its aspect ratio kept. One reading, so it builds.

The block stretches to the card's width — the ground's 16px padding is the margin — and drops its fixed height; the image is `width: 100%; height: auto` at `object-fit: contain`, so its stored proportions decide the height. A framed picture (`/frame`) is square and so is drawn square; a picture kept before `/frame` is the whole photo and is drawn at the photo's proportions. The empty "add a picture" box keeps a 148px height so it reads as an invitation rather than a full-width void. Untick and the 148px square returns.

## glossary

(no new terms)

## code description

`wide.css` — `.card-pic` stretches and releases its size; `.card-pic img` keeps its aspect; `.card-pic.empty` keeps a height. Composed after `/cards`' stylesheet, so it wins at equal specificity.
