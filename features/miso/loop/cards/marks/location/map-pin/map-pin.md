# map-pin
*a map-locator glyph on "map location"*

> (asks#1787669323367)
> Use a map locator icon for "map location" (looks like a downward pointing guitar pick)
> *(filed from the field on 2026-08-25 by ash, birthplace `👤 @ miso/shell/panel/account`)*

## user

The **map location** pill on your card starts with a small map pin.

## spec

`/location` draws its pill as two words. Ash asked for a map-locator icon — the downward-pointing pin. One reading, so it builds. Per `/glyphs`, the icon is drawn: an inline SVG pin (outline plus a dot) in `currentColor`, so it dims when the pill is dimmed and inks with it. This node extends `card_page_html` and puts the glyph before the words in the pill `/location` inserted. Untick and the words stand alone.

## glossary

(no new terms)

## code description

`map-pin.rs` — `card_page_html` replaces `>map location<` in the page `existing` returns with the glyph and the words; `pin_svg` is the drawing.

`map-pin.css` — the glyph's box inside the pill.
