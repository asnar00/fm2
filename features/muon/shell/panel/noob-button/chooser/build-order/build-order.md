# build-order
*the list orders by the number it shows: highest build first, strictly downwards*

> (transcripts/2026-08-15-fm-spec.md#p3)
> For some reason I'm seeing release numbers out of order in the list: 124, then 125, then 126. I'd prefer it if release numbers start at highest and strictly go downwards.

## spec

`/chooser` sorts by provenance timestamp but *displays* each feature's
latest-touching build — two orderings that usually agree and visibly
don't when several nodes share one prompt (day 4's ladder: one anchor,
four builds, shown as 124, 125, 126, 122). The number the eye reads is
the order the list must keep: sort by build, highest first, strictly
descending; equal builds fall back to newest provenance, then path, so
the order stays stable.

## user

The feature list counts straight down — the biggest release number at
the top, never a smaller number above a bigger one.

## code description

`build-order.index.js` wraps `feature_Chooser.load` (the one place the
flat list is built and sorted): after the original runs it re-sorts
`flat` by build descending, ties by timestamp descending, then path.
The awaiting-update section already sorts its own rows by build and is
untouched.
