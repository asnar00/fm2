# tucked-updates
*the updates picker retires into the features list, behind the button*

> (transcripts/2026-08-15-fm-spec.md#p44a)
> PROPOSAL: Let's move the "updates:" radio into the "features" list behind the features button
> *(a field ask, filed 2026-08-15 on muon build 154)*

## user

The updates setting now lives inside the features list: tap
**features**, and the automatic / fixes auto / ask me choice is right
at the top. The panel keeps only what changes day to day.

## spec

The updates policy picker is a set-and-forget control, so it leaves
the panel's standing rows and tucks into the territory the **features**
button reveals: unfold the list and the picker sits at its top, above
the catalog rows; fold, and it goes away with them. The awaiting,
building and requests sections stay out front as before — only the
picker retires. Without `/features-button` in the composition the list
is always open, and the picker simply lives at its top.

The #p81 law is kept the honest way: the picker's row is moved inside
a container **this node owns** (`#tucked`), and it is that container's
display this node manages from JS — `/policy`'s own row is never
show/hidden by a foreign stylesheet.

## glossary

- **tucked**: moved behind the features button — revealed by the same
  tap that shows the catalog.

## code description

`tucked-updates.index.js` wraps `feature_Chooser.mount` (composing
after `/lifecycle` and friends, so the sections are already in place):
after the original, it creates `#tucked` inside `#changes` before the
first catalog row, moves `#policySeg` into it (`/policy`'s reflect and
click wiring ride along untouched), and sets the container's display
from the box's `folded` state. A wrap on
`feature_FeaturesButton.toggle` (typeof-guarded) updates that display
on every fold and unfold.
