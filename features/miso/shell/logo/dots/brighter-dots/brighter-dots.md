# brighter-dots
*the graph paper reads more clearly*

> (asks#1786895298624)
> NEW ASK [proposed] … :: 'Can we make the dots brighter'
> *(a field ask, filed from the phone on 2026-08-16, miso build 199)*

## user

The background dots are easier to see — still quiet, but no longer nearly
invisible on a phone screen in daylight.

## spec

`/dots` chose `#555` to match the grey the big logo glyph had worn. In the
field that reads as almost nothing: a single sub-pixel dot at a third of
white, on black, on a bright screen outdoors.

The dots go to `#8a8a8a` — a little over half white. Judgement, since the
ask names a direction rather than a value: one clear step, chosen to stay
inside `/dots`' stated intent of a **quiet** ground rather than becoming a
foreground pattern. If it is still too dim it wants another step, not a
different mechanism, and the next ask can say so.

Only the colour changes. The dot's radius and its soft edge (the
`0.5px`/`0.6px` gradient stops that keep it from aliasing into a square)
stay exactly as `/dots` drew them, as does where the grid is anchored.

## glossary

(no new terms)

## code description

`brighter-dots.css` restates `body`'s `background-image` with the same
`radial-gradient(circle, … 0.5px, transparent 0.6px)` shape and a brighter
stop colour. Composing after `/dots`, it wins on the cascade; unticking it
returns `#555`.

The whole declaration is repeated rather than the colour alone because a
gradient's colour is not separately addressable in CSS — the smallest
honest unit of override here is the image.
