# finer-grid
*two squares of graph paper per tool button, not one*

> (asks#1786895338523)
> NEW ASK [proposed] … :: 'Halve the dot spacing compared to the icons'
> *(a field ask, filed from the phone on 2026-08-16, miso build 199)*

## user

The background grid is twice as fine — two squares across a tool button
instead of one. It still follows the buttons, so if they change size the
paper still halves against them.

## spec

`/aligned-grid` had just made one grid cell equal one tool button. At 50px
that reads as a coarse chequer rather than graph paper; the ask halves it.

The relationship is what matters, not the number: the spacing stays
**derived** from `--tool-size` rather than replaced by a literal, so the
grid keeps following the buttons through any future change — halved
against them, whatever they become. Unticking `/bigger-buttons` still
drags the paper down with the toolbar, now to 20px instead of 40px.

Centring is untouched and still comes from `/aligned-grid`: this node
overrides the cell size and nothing else. Halving the cell puts a dot at
the screen's centre either way, since halving preserves the alignment of
every other cell boundary with the original grid.

## glossary

(no new terms)

## code description

`finer-grid.css` restates `body`'s `background-size` as
`calc(var(--tool-size) / 2)` on both axes. It composes after
`/aligned-grid` and wins on the cascade; unticking it returns the
one-cell-per-button grid, and unticking its parent too returns `/dots`'
original 32px.
