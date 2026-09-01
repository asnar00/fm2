# clear-of-toolbar
*the card page's scroll reaches its own last line on a phone*

> (transcripts/2026-09-01-saturday.md#p27)
> In expanded-profile view (and by extension other card views) the bottom of the card falls under the toolbar, but I'm unable to scroll upwards to make that part visible.

## user

Scrolling an open card — a profile, a post, any card — reaches everything on
it. Nothing hides under the toolbar.

## spec

`/cards` ends the card page at a flat `bottom: 72px`, and `/tools` seats the
toolbar at `safe-area-inset-bottom + 8px` with 50px buttons — so on a phone
with a home indicator the toolbar's top edge (~92px up) stands ABOVE the
page's own floor, and the strip between them is fenced off: a fixed
container's boundary is also its scroll limit, so no gesture can bring that
strip into view. On a desktop, where the inset is zero, the numbers happen
to clear each other — which is why every headless rig passed while the
phone did not: the safe area is the one quantity the rigs cannot feel.

The fix restates the page's floor with the inset in it:
`bottom: calc(env(safe-area-inset-bottom) + 72px)`. Desktop is unchanged
(inset 0 → 72px); the phone's floor rises above the toolbar's top with air
to spare. Composed after `/cards`, equal specificity, so it wins.

## hostile cases

- **No safe-area (desktop, older phones).** `env()` resolves to 0; the
  floor is exactly today's.
- **This node unticked.** The flat 72px returns — the phone's fenced strip
  with it, no worse.
- **Landscape.** The bottom inset is smaller there; the calc follows it.

## glossary

(no new terms)

## code description

`clear-of-toolbar.css` — one rule: `.card-page`'s `bottom` restated with
the safe-area inset added.
