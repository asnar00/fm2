# raised
*the nøøb lozenge goes as high as it can, clear of the main display*

> (asks#1787666672844)
> raise the nøøb button up as high as possible, so it doesn't overlap the main display
> *(filed from the field on 2026-08-25 by ash, birthplace `👤 @ miso/shell/panel/account`)*

## user

The nøøb lozenge sits right at the top of the screen now, tucked under the clock, and your card starts beneath it — nothing sits on top of anything.

## spec

`/top-right` placed the lozenge 12px under the safe-area top, and `/cards` starts its page 16px under the same edge, so on the 👤 page the lozenge lay across the card's top-right corner (visible in the `/ground` screenshot). Ash asked for the button raised as high as possible so it stops overlapping the main display. One reading, so it builds.

Two rules. The lozenge's `top` becomes the safe-area inset plus 2px — as high as a phone allows without going under the status bar. And the card page's `top` becomes the inset plus 48px — the lozenge's 36px height, its 2px offset, and a 10px gap — so the one surface that reaches the top edge today begins below the button. Other surfaces (the tap pill, the dictaphone grid) are centred and never reached it. Untick this node and both return to their own nodes' values.

## glossary

(no new terms)

## code description

`raised.index.css` — `#build { top }` and `.card-page { top }`, both against `env(safe-area-inset-top)`; composed after `/top-right` and `/cards`, so they win at equal specificity.
