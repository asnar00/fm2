# alpha
*adds an alpha channel to colour*

> (transcripts/2026-08-13-fm-spec.md#p13)
> Let's build v0 linker. Any way you like, quick and dirty for the first pass is fine.

## spec

Subfeature of `/colour`, from the fm.md worked example. Adds an `a` channel (`f32`) to the `colour` struct via `/flat struct merge`, and extends the `add(colour, colour)` chain so addition sums the alpha channel too.

## user

For agents: with this feature included, `colour` values carry `col.a` alongside `col.r`, `col.g`, `col.b`.

## glossary

(no new terms)

## code description

`alpha.rs` re-declares `pub struct colour` (lines 1-3) containing only the new field `a`; the linker merges it into the composed `colour` struct after the base channels. `feature_Alpha` (line 5) redefines `add` (lines 7-12) as an `/extension`: it computes the alpha sum before the moves, calls the previous chain via `existing.add(a, b)` (line 9), then writes the summed alpha into the result.
