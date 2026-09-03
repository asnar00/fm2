# share-glyph
*the report sheet's export button is the standard share glyph*

> (asks#1788449200573)
> share pdf button should be the standard share icon
> *(filed from the field on 2026-09-03 by ash)*

## user

The button at the top-right of an open report is the share symbol everyone knows — the box with the arrow rising out of it — rather than the words "export PDF".

## spec

`/viewer` labelled the button "export PDF" (#p15). Ash asked for the standard share icon (asks#1788449200573). One reading, so it builds: per `/glyphs`, a drawn SVG in `currentColor` — the tray with an arrow up out of it, the shape iOS uses — in a round button the toolbar's size, with the words kept for assistive readers as its label. Untick and the words return.

## glossary

(no new terms)

## code description

`share-glyph.js` — wraps `feature_Viewer.make` to put the glyph in the button.

`share-glyph.css` — the round button.
