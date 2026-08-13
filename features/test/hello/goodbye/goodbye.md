# goodbye
*prints a farewell after the greeting*

> (transcripts/2026-08-13-fm-spec.md#p13)
> Let's build v0 linker. Any way you like, quick and dirty for the first pass is fine.

## spec

Subfeature of `/hello`, from the fm.md worked example. Extends `main` to print `Goodbye...` after the greeting. Exists to exercise the linker's `/extension` chain: the redefined `main` calls the previous definition via `existing.main()`.

## user

With this feature included, the demo prints `Goodbye...` as its last line. Untick it in `hello/order.md` to remove the farewell.

## glossary

- **extension**: a redefinition of a function that may call the previous definition via `existing`.

## code description

`goodbye.rs` declares `feature_Goodbye` (line 1) and redefines `main` (lines 3-6): it first calls the previous definition through `existing.main()` (line 4) — which the linker rewrites to `feature_Hello::main()` — then prints the farewell (line 5).
