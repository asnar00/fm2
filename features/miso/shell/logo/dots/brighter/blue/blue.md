# blue
*the dot grid is brighter still, and blue*

> (asks#1787761149510)
> make the dot grid brighter and blue
> *(filed from the field on 2026-08-26 by ash, urgent)*

## user

The graph-paper dots behind everything are a clear, light blue — easy to see on a phone, and no longer grey.

## spec

`/brighter` lifted the dots from `#555` to `#8c8c94`; ash asked for brighter again, and blue (`asks#1787761149510`). One reading — a light blue that reads on a phone's screen — so it builds: the same 32 px grid in `#8ab4ff` (a sky blue, well above the grey's luminance on the black ground), the dot a full 1 px so the colour has enough pixel to show — the third step up, so it need not come back. Untick and the dots are `/brighter`'s light grey again.

## glossary

(no new terms)

## code description

`blue.css` — one `background-image` on `body`, overriding `/brighter`'s by provenance; same grid, new colour and a slightly wider dot.
