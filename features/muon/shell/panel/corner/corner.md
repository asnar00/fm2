# corner
*chrome respects the rounded display corners*

> (transcripts/2026-08-13-fm-spec.md#p58)
> I think the number in the corner is being swallowed by the round display corner

## spec

The panel handle was positioned from the physical screen edge and vanished into the iPhone's corner radius. Rule established: fixed-position chrome is placed relative to the safe area (`env(safe-area-inset-*)`), never the physical edge — the app claims the full display (`viewport-fit=cover`), so respecting the insets is our job.

## user

The corner button sits fully visible above the home-indicator zone on any display shape.

## glossary

- **safe area**: the region iOS guarantees free of notches, corners and the home indicator, exposed to CSS as `env(safe-area-inset-*)`.

## code description

`#build`'s position in `/shell`'s loader CSS: `bottom`/`right` computed as `calc(env(safe-area-inset-…) + margin)`. The panel sheet uses the same inset for its bottom offset.
