# brighter
*the dot grid reads on a phone*

> (asks#1787750117273)
> make the grid of dots brighter
> *(filed from the field on 2026-08-26 by ash, urgent)*

## user

The graph-paper dots behind everything are easier to see.

## spec

`/dots` drew 1 px dots in `#555` every 32 px — mid-grey on a laptop, near-invisible on a phone's darker, denser screen. Ash asked for brighter (`asks#1787750117273`). One reading, so it builds: the same grid in `#8c8c94`, the dot a touch wider (0.7 px) so a phone's pixels render it instead of dropping it. Untick and the dots are the quiet `#555` again.

## glossary

(no new terms)

## code description

`brighter.css` — one `background-image` on `body`, overriding `/dots`' by provenance.
