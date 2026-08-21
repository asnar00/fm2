# lozenge
*the logo button reads as a button*

> (transcripts/2026-08-13-fm-spec.md#p70)
> Button looks good, let's put a little lozenge outline around it to make it clear it's a button?

## user

The corner logo visibly looks like a button now.

## spec

The bare logo glyphs didn't announce tappability. The button sits in a lozenge: thin border, dark fill, fully-rounded corners. The update highlight tints the border along with the glyphs.

## glossary

- **lozenge**: a fully-rounded pill outline around a small control.

## code description

This node owns `lozenge.index.css`: the pill outline (`border`, `border-radius: 999px`, dark fill) and the accent `border-color` when an update is waiting. Untick it and the button is a bare glyph again.
