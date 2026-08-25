# beneath
*the toolbar and the lozenge always sit above the main viewing area*

> (asks#1787666911915)
> toolbar should always depth sort above the main viewing area
> *(filed from the field on 2026-08-25 by ash, birthplace `👤 @ miso/shell/panel/account`)*

## user

Whatever fills the screen — your card today, other pages later — the toolbar along the bottom and the nøøb lozenge at the top stay on top of it, tappable.

## spec

Neither `/tools`' toolbar nor the lozenge (`#build`) declared a stacking level, and `/cards`' page is a fixed element that comes later in the document, so where they overlapped — the card page's bottom edge over the toolbar on a phone with a home indicator; the lozenge before `/raised` — the page painted over the controls and took their taps. Ash filed the rule from the field. One reading, so it builds.

This node states the depth order once: the card page at 1, the toolbar at 5, the lozenge at 6; `/panel`'s shade (10) and sheet (11) stay above all three, and `/cards`' toast (40) above everything. It lives under `/cards` because the card page is the one surface that reaches the edges today; `/tools` is at the six-child cap, and when a second full surface arrives the rule should move up to a node both can share.

## glossary

(no new terms)

## code description

`beneath.css` — three `z-index` declarations. Composed after `/tools`, `/logo` and `/cards`, so they win at equal specificity.
