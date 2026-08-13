# tidy
*support folders are not features*

> (transcripts/2026-08-13-fm-spec.md#p79)
> I'm seeing weird "assets" nodes in the tree which have nothing in them - can we get rid of those?

## spec

The tree walker treated any subfolder as a subfeature and appended unlisted ones, so `assets/` folders surfaced as empty nodes. The explorer now skips support folders (`assets`, `build`, `target`, dotfolders) — matching the linker's own skip list. Noted for the housekeeping pile: the two tools each carry a copy of the tree-walking rules, and copies drift; they deserve one shared home.

## user

The tree shows features only.

## glossary

(no new terms)

## code description

`NOT_FEATURES` in explorer.py's `load_children`, filtering `iterdir()` before order.md reconciliation.
