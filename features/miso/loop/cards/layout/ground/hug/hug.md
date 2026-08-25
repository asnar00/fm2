# hug
*the card's ground is only as big as the card*

> (asks#1787666958628)
> the card background should be as small as possible to encompass the card's
> *(filed from the field on 2026-08-25 by ash, birthplace `👤 @ miso/shell/panel/account`; the sentence ends there — read as "the card's contents")*

> (asks#1787666911915, the depth ask's second sentence, added by ash)
> If the view is taller than the available space, it should scroll

## user

Your card's dark panel fits its contents — name, picture, mission, the invite rows — and stops there. A card longer than the screen scrolls inside its panel; the toolbar stays put.

## spec

`/ground` gave the card page a panel, but `/cards` had pinned the page to both the top and the bottom of the screen, so the panel stretched to the toolbar however little it held. Ash asked for the ground to be as small as its contents, and, in the same breath on the depth ask, that a view taller than the room should scroll. One reading, so it builds.

This node releases the page's bottom edge and caps its height instead: `max-height` is the viewport minus the top offset `/raised` sets (safe-area + 48px) minus the 72px the toolbar keeps. The box is sized border-box so the cap includes the ground's padding and border (measured: without it the panel ran 34px under the toolbar). `/cards`' own `overflow-y: auto` then scrolls the blocks inside the panel when they outgrow it. Untick and the panel stretches to the toolbar again.

## glossary

(no new terms)

## code description

`hug.css` — `bottom: auto` and a `max-height` on `.card-page`; composed after `/raised`, so the two agree on the top offset by construction.
