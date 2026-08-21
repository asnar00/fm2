# bigger-buttons
*the toolbar's buttons, a quarter larger*

> (transcripts/2026-08-15-fm-spec-2.md#p9)
> NEW ASK [proposed] … :: 'increase the size of the tool buttons by 25%'
> *(a field ask, filed from the launcher on 2026-08-15, miso build 169)*

## user

The tool buttons are a quarter bigger — easier to hit, easier to read.

## spec

Every button in the toolbar grows by 25%: the square tool buttons from
40px to 50px with their icons from 19px to 24px, and the narrow back
chevron in proportion (24px wide to 30px, its glyph 22px to 27px). The
sub-tool controls share the `tool-button` base, so reset, ×2, −1 and
record grow with the rest — one toolbar, one size. Nothing else about
the buttons changes: colour discipline, selection, the slide, the
long-press cards all ride along.

## glossary

(uses `/tool` and `/toolbar` from `/tools`)

## code description

`bigger-buttons.css` overrides the base sizes set by `tools.css`: the
`.tool-button` square, its icon `font-size`, and the `.back` variant's
width and glyph size. Same selectors, later in the cascade (fragments
compose in provenance order, newest last), so unticking the node
returns the toolbar to 40px exactly.
