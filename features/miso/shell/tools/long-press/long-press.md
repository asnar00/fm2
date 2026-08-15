# long-press
*hold a tool's button and it introduces itself*

> (transcripts/2026-08-15-fm-spec.md#p46a)
> PROPOSAL: A long press on a tool button should pop up a tooltip with user documentation
> *(a field ask, filed 2026-08-15 on muon build 155)*

## spec

Press and hold any tool button in the toolbar (half a second) and a
small popover appears above it: the tool's name and its user
documentation — the `## user` paragraph of the node that registered
it, straight from the served tree (`tree.json`'s intro, resolved
through the stamped `tool` ids). Releasing after a long press does
NOT open the tool — reading about it and going there stay separate
acts. A short tap behaves exactly as before. The popover dismisses on
the next touch anywhere, and a drift of more than a few pixels while
holding cancels the hold (a scroll is not a question). Without the
chooser's catalog in the composition, the popover degrades to the
button's title.

## user

Hold your finger on any tool button and a little card tells you what
that tool does, in plain words. Tap anywhere to put it away; a normal
tap still just opens the tool.

## glossary

- **tool card**: the long-press popover — a tool's name and its user
  paragraph, shown where you're already looking.

## code description

`long-press.js` arms on `pointerdown` over `[data-ev^="tool_"]`: a
500ms timer shows the card (`#toolCard`, positioned above the button,
clamped to the viewport); movement past 12px, `pointerup`, or
`pointercancel` before the timer disarms it. When the card fired, a
capture-phase click listener swallows the button's next click once, so
the tool doesn't open under the reader. Content resolves the tool id
against `feature_Chooser`'s flat catalog (`load()` kicked lazily,
typeof-guarded; title-attribute fallback) — name bold, intro beneath.
Any later `pointerdown` outside the card dismisses it.

`long-press.css` styles the card: small, dark, rounded, above the
toolbar's plane.
