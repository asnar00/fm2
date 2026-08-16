# test
*demo features exercising the linker*

> (transcripts/2026-08-13-fm-spec.md#p30)
> one thing: can we move all these features down into a feature called "test" so they don't mess with the main feature space we're going to create later

## user

Browse the subfeatures to see each linker mechanism demonstrated. Products compose from this subtree (`demo` imports all of it). Untick entries in `test/order.md` to exclude examples.

## spec

Container feature holding the worked examples from fm.md (`/hello`, `/colour`, `/vec`, `/sums`) that exercise linker mechanisms: `/extension` chains, `/flat struct merge`, `/multiple dispatch`, operator glue, and product subsetting. Keeps the feature-space root clear for real features. Contributes no code of its own.

## glossary

(no new terms)

## code description

No implementation files — this node exists for tree structure and ordering (`order.md` lists the demo features in composition order).
