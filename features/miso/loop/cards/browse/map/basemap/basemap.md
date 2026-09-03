# basemap
*what the map is drawn on: the tiles, the ward lines, the credit*

> (transcripts/2026-09-03-housekeeping.md#p18)
> In map view, I'd like one change: in the bottom area of the map, I'd like a zone that shows posts as a most-recent first scrolling horizontal list

## user

The ground of the map: its tiles and how they are kept, the constituency and ward boundaries drawn over them, and the quiet credit line.

## spec

A grouping node, code-free. `/map` had six children and the reel asked for a seventh (#p18), so the three nodes about the ground itself — `/squares` (the tiles, stand-in and stocked), `/boundaries` (ward lines) and `/quiet-credits` (the credit behind a button) — sit here, beside `/pins`, `/live`, `/recentre` and `/reel`. A regroup rewires nothing: composition is provenance-ordered, so each keeps its place in every chain; only their path-keyed enablement flags move (misses.md, "the regroup that moved addresses"). Untick and the map is bare ground with no tiles, no lines and no credit.

## glossary

(no new terms)

## code description

(none — a grouping node)
