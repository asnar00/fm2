# pins
*how the map's pins look and lie: a post's square face, and pins that share a point fanning out around it*

> (transcripts/2026-09-03-housekeeping.md#p5b)
> When there's more than one post / user at the same location, the markers overlap so you can't distinguish them.

## user

The pins on the map: what each kind looks like, and how several at one place are arranged so each can be seen and tapped.

## spec

A grouping node, code-free. `/map` had six children and the fan-out asked for a seventh (#p5b), so the two nodes about the pins themselves — `/square-posts` (a post's face is a rounded square) and `/fan-out` (pins at one point fan out around it) — sit here. A regroup rewires nothing: composition is provenance-ordered, so `/square-posts` keeps its place in every chain; only its path-keyed enablement flag moves (misses.md, "the regroup that moved addresses"). Untick and both the square faces and the fan leave the map.

## glossary

(no new terms)

## code description

(none — a grouping node)
