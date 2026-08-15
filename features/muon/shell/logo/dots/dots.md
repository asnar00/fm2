# dots
*the display background is a quiet dot grid, not the big grey logo*

> (transcripts/2026-08-14-fm-spec-3.md#p56a)
> Instead of the big gray logo on the main screen background, can we have a simple grid of dots? A single pixel spaced (say) 32 pixels apart, mid-grey.

## spec

The big grey `ᕦ(ツ)ᕤ` leaves the display surface; in its place, the background carries a grid of single-pixel mid-grey dots, 32px apart — a quiet graph-paper ground for whatever the open tool draws. The logo glyph survives where it still earns its place: the corner lozenge (`/panel/button`) and the login page are untouched. Untick to bring the big glyph back.

## user

The main screen's background is now a subtle grid of dots instead of the large grey logo. Everything sits on graph paper.

## glossary

(no new terms)

## code description

`dots.css`, two cascade rules (this node is newest, so they win): `.logo { display: none }` retires the big glyph without touching `/logo`'s code, and `body` gains the grid — a 1px `radial-gradient` dot tiled every 32px (`background-size: 32px 32px`), `#555` on the black ground, matching the grey the glyph wore. The grid lives on `body`, which whole-DOM swaps never replace.
