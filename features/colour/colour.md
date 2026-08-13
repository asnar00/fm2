# colour
*an rgb colour type*

> (transcripts/2026-08-13-fm-spec.md#p13)
> Let's build v0 linker. Any way you like, quick and dirty for the first pass is fine.

## spec

From the fm.md worked example: defines the `colour` struct with `r`, `g`, `b` channels (`f32`). Exists to exercise the linker's `/flat struct merge`; subfeatures add fields to `colour`.

## user

For agents: declare values as `colour` and access channels directly (`col.r`). Fields added by subfeatures are equally direct (`col.a`).

## glossary

- **flat struct merge**: the linker combines all fields declared for a struct name across features into one flat struct; duplicate field names are a link error.

## code description

`colour.rs` declares `pub struct colour` (lines 1-5) with the three base channels. No feature struct or functions — this node only contributes a type.
