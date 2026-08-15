# top-tied
*the nøøb panel hangs from the top of the screen, clear of the toolbar below*

> (transcripts/2026-08-15-fm-spec.md#p18)
> another small detail: let's tie the noob popup to the top of the screen, so it doesn't overlap the toolbar at the bottom.

## spec

The panel anchored to the bottom of the screen sat on top of the
toolbar — the surface that steers muon covering the surface that uses
it. It ties to the top instead: **just below the nøøb button's row**
(#p23 — the button that opened the panel stays visible above it),
centred as before, growing downward, so the toolbar keeps its ground
at the bottom of the screen. Everything inside the panel is untouched;
only the anchor changes — untick this node and the panel drops back to
the bottom.

## user

The system panel now hangs from the top of the screen, so your tools
stay visible and tappable underneath while it's open.

Tied to the top, a tall panel could still grow down into the toolbar,
so the tie comes with a bound: the panel's height stops 160px short of
the viewport (worst-case top inset plus the toolbar's ground) and
scrolls inside when content exceeds it.

## code description

`top-tied.index.css` overrides `/panel`'s anchoring later in the
cascade: `top: calc(env(safe-area-inset-top, 0px) + 55px)` (the
lozenge row's 12+33 plus a 10px gap), `bottom: auto`, `max-height:
calc(100dvh - 200px)` with `overflow-y: auto`. The max-height uses a
plain constant rather than env() inside calc — the #p81-era
height-calc trap. Nothing else.
