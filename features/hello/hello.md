# hello
*prints a greeting*

> (transcripts/2026-08-13-fm-spec.md#p13)
> Let's build v0 linker. Any way you like, quick and dirty for the first pass is fine.

## spec

Root demo feature, from the fm.md worked example. Introduces the program `/entry point` `main`, which prints `Hello, world!`. Exists to give the v0 linker a base function for subfeatures to extend.

## user

Build the `demo` product and run it; it prints `Hello, world!` (plus whatever subfeatures add).

## glossary

- **entry point**: the function the composed program runs first (`main`).

## code description

`hello.rs` declares `feature_Hello` (line 1) and defines `main` (lines 3-5), printing the greeting. This is the base definition of `main`; subfeatures wrap it via `existing.main()`.
