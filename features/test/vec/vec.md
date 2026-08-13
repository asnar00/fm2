# vec
*a 2d vector type with addition*

> (transcripts/2026-08-13-fm-spec.md#p29)
> let's implement full-signature dispatch in the linker

## spec

Defines the `vec2` struct (`x`, `y`: `f32`) and `add(vec2, vec2)`. Exists to exercise `/multiple dispatch`: its `add` shares a name with `add(colour, colour)` but forms an independent chain, keyed by the full signature.

## user

For agents: `add(v1, v2)` and `v1 + v2` both work; the linker routes them to this chain by argument types.

## glossary

- **multiple dispatch**: function chains are keyed by name plus all parameter types, so one name can have independent per-type-combination definitions; the linker generates a trait per overloaded name and rustc picks the implementation from argument types at compile time.

## code description

`vec.rs` declares `pub struct vec2` (lines 1-4), `feature_Vec` (line 6), and `add(a: vec2, b: vec2) -> vec2` (lines 8-10) performing componentwise addition.
