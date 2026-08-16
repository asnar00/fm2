# features-button
*the long list folds away behind one button; the awaiting update stays out front*

> (transcripts/2026-08-15-fm-spec.md#p15)
> actually yeah let's do those now.
> *(the mod, from #p14: "…and then a 'features' button that opens the long feature list. That'll make things less busy.")*

## user

The panel shows the long feature list only when you ask: tap
**features** to browse everything, tap again to put it away. An update
waiting for review still appears right at the top, list folded or not.

## spec

The feature list is the panel's deepest surface but not its most
frequent errand, so it folds: `/chooser` still mounts everything —
rows, wiring, ticks, reader — but the rows start hidden, and a
**features** row (placed after the policy picker, `/less-busy`'s slot
for it) unfolds and refolds them. Folding is presentation only:
`/review`'s awaiting-update section lives in its own box inside the
same container and stays visible while the list is folded — the panel
leads with what needs deciding, not with everything that exists.
A drill-down from an awaiting row's child chips unfolds the list first,
so `goto` always lands somewhere visible.

## code description

`features-button.index.js` adds the `folded` class to `#changes` inside
a wrap of `feature_Chooser.mount` (after the original, so a re-mount
re-folds), inserts the **features** row after `#policySeg` (falling
back to before the update row) with a click that toggles `folded`, and
wraps `feature_Chooser.goto` to unfold before the original scrolls.

`features-button.index.css` does the folding: direct-child `.crow` and
`.cmore` of `.chooser-home.folded` are hidden — `#awaiting`'s rows are
nested inside their own box and so stay visible, the paid-for #p81
lesson (never touch another feature's show/hide) observed: the panel's
own lifecycle is untouched, only rows this node's parent owns are
folded. A folded box also surrenders `/chooser`'s min-height budget
(#p16), so the panel shows no reserved blank between the build line
and the policy row until something is actually there.
