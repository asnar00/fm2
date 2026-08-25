# tinted
*the undo button is black on a palette colour, like every tool*

> (asks#1787667967261)
> undo icon style should match house style: black on colour from our palette
> *(filed from the field on 2026-08-25 by ash)*

## user

The undo button looks like the other tool buttons now: a black arrow on one of the toolbar's colours.

## spec

`/ember` tints every tool button — a palette colour behind a black glyph — but `/undo`'s control was drawn plain, white on dark grey, because it is a control rather than a tool. Ash asked for it to match the house style. One reading, so it builds.

This node extends `tool_controls` and adds `/ember`'s `tinted` class and `--tool-colour` to the undo button in the chain's output, with the colour from `tool_colour("undo")` — `/ember`'s stable pick for a name it never assigned, so it is one of the six and identical on every device. The glyph is plain text, not an emoji `.icon` span, so `/ember`'s grayscale filter does not reach it; one rule sets it black. With `/ember` unticked `tool_colour` is empty and this node leaves the button as `/undo` drew it. Untick this node and the plain control returns.

## glossary

(no new terms)

## code description

`tinted.rs` — `tool_controls` calls `existing` and rewrites the undo button's opening tag to carry `tinted` and the colour variable.

`tinted.css` — black glyph on the tint; the dimmed state keeps `/undo`'s opacity.
