# arrow
*undo is a drawn back-curving arrow*

> (asks#1787667598214)
> the undo icon should be a more traditional horizontal back-curving arrow
> *(filed from the field on 2026-08-25 by ash)*

> (transcripts/2026-08-25-accounts.md#p44, revision)
> the trouble is that you used what looks like a standard emoji character for undo - the intent was to change the *shape* of the icon, not use a coloured emoji at low brightness. Build a proper icon that matches the aesthetic of the other tool icons, rather than using a colour bitmap.

## user

The undo button shows a back-curving arrow drawn in the toolbar's own ink — black on its colour like every other tool button.

## spec

`/undo` drew its control with ↶. Ash asked for the traditional horizontal back-curving arrow; the first build swapped in ↩ (U+21A9), which on iOS has an **emoji presentation** and arrived as a colour bitmap at low brightness — a shape change that no colour rule could then touch (#p44). The revision draws the glyph: an inline SVG, two strokes in `currentColor`, sized to sit in the button box as an emoji would. Black on a tint (`/tinted`), white on a plain control, dimmed with the button. The rule this taught is an agent-instruction node of its own, `/glyphs`.

## glossary

(no new terms)

## code description

`arrow.rs` — `tool_controls` calls `existing` and replaces U+21B6 with `undo_arrow_svg()`, a 24-unit viewBox with the arrowhead and the curve as two rounded strokes.

`arrow.css` — the `.icon-svg` box inside a tool button.
