# arrow
*undo is the traditional back-curving arrow*

> (asks#1787667598214)
> the undo icon should be a more traditional horizontal back-curving arrow
> *(filed from the field on 2026-08-25 by ash)*

## user

The undo button in a tool's control row shows ↩, the arrow everyone knows.

## spec

`/undo` drew its control with ↶ (an anticlockwise semicircle). Ash asked for the traditional horizontal back-curving arrow, ↩. One reading, so it builds. This node extends `tool_controls` and swaps the glyph in the chain's output — the button, its event and its title stay `/undo`'s. Untick and ↶ returns.

## glossary

(no new terms)

## code description

`arrow.rs` — `tool_controls` calls `existing` and replaces U+21B6 with U+21A9.
