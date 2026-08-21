# less-busy
*the panel breathes: ask first, build status under it, updates, policy — and who-you-are shares the last row with log out*

> (transcripts/2026-08-15-fm-spec.md#p14)
> let's combine the "logged-in-as-asnaroo" and log out button into one row, right at the end; so "ask muon" will come first. Then below that let's have the features-available-for-update list and the update button, and then below that the update policy button, and then a "features" button that opens the long feature list. That'll make things less busy.
> *(#p15: "incidentally 'build 132: up-to-date' could be just below 'ask muon'")*

## user

The panel stops shouting: ask miso at the top, a one-line build status
under it, updates and their policy next, and your name with the log out
button in one quiet row at the bottom.

## spec

The panel reads top to bottom in the order a user actually needs it:
the **ask box** first; a quiet **build line** under it ("build 132 — up
to date", or "build 132 → 133 available", or the can't-reach-server
honesty); then the awaiting-update section and the standing update
button; then the **updates policy** picker; then whatever occasional
rows are visiting (Face ID / notifications enrolment); and finally one
calm last row where "logged in as asnaroo" and **log out** live
together. The who-line loses its build freight — that moved to the
build line — and `/features-button`'s row slots in after the policy
picker (each node places itself; this node only arranges what exists).

## code description

`less-busy.index.js` runs once at load: it creates `#buildLine` and
`#whoRow` (a flex row receiving the existing `#who` and `#logoutBtn` —
moved, not recreated, so every standing handler and writer still
finds them), then arranges the panel's known children in the spec's
order, each typeof/existence-guarded so absent siblings simply don't
appear.

It wraps `feature_Panel.open`: after the original runs (which fills
`#who` with the combined who+build text), the wrap splits the freight —
`#who` gets just the logged-in-as line, `#buildLine` gets the build
status, recomputed from `feature_Update`'s running/server and
`/honest`'s wording, all guarded.

`less-busy.index.css` styles the build line (small, dim) and the last
row (name left and dim, log out button its natural width).
