# sums
*demonstrates dispatch: adding colours and vectors*

> (transcripts/2026-08-13-fm-spec.md#p29)
> let's implement full-signature dispatch in the linker

## spec

Extends `main` to add two `colour`s and two `vec2`s through the shared name `add`, proving `/multiple dispatch` routes each call to its own chain (colour addition includes /alpha/'s `/extension`), and that operator glue makes `v1 + v2` work.

## user

With this feature included, the demo prints the colour sum (with alpha channel), the vector sum, and an operator-form vector sum after the greetings.

## glossary

(no new terms)

## code description

`sums.rs` extends `main` (lines 3-13): calls the previous chain (line 4), then adds two colours via `add` (lines 5-6) — dispatched to the colour chain, alpha extension included — adds two vectors (line 8), and adds two vectors with `+` (line 10), exercising the generated `std::ops::Add` glue. Struct literals use `..Default::default()` so this feature stays agnostic to fields added by other features.
