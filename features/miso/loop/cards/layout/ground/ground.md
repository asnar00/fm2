# ground
*a card sits on its own dark ground, not on the dot grid*

> (asks#1787666615776)
> card should have a background - a dark grey rounded rectangle that overwrites the dot grid
> *(filed from the field on 2026-08-25 by ash, birthplace `👤 @ miso/shell/panel/account`)*

## user

Your card is a dark grey rounded panel now, covering the dotted background, so it reads as one thing you are holding rather than pieces laid on the graph paper.

## spec

`/cards` drew the page as blocks over the app's dot-grid ground. Ash asked, from the 👤 page, for a background: a dark grey rounded rectangle that covers the grid. One reading, so it builds.

This node styles `.card-page` with the page family's ground (`#161619`, `/taste` 1), a 1px `#202026` border, a 14px radius and 16px padding. Nothing else moves: the blocks keep their own pills, the page still scrolls inside its fixed box. Untick it and the blocks float on the grid again.

## glossary

(no new terms)

## code description

`ground.css` — four declarations on `.card-page`; composed after `/cards`' own stylesheet, so they win at equal specificity.
