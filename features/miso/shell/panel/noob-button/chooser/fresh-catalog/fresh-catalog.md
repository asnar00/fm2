# fresh-catalog
*a quiet apply also empties the catalog's memory*

> (transcripts/2026-08-15-fm-spec-2.md#p5)
> after upgrade, the app is still showing the old tooltip
> *(the tooltip fix had shipped as a data-only delta; the device applied it quietly and kept answering from the catalog it had already read)*

## spec

The chooser reads `features/tree.json` once per page and holds the flat
catalog in memory — every later reader (the features list, a long-press
tool card) answers from that copy. A quiet apply updates the device
without a reload — the data-only delta, and the wasm patch that borrows
its ending — so the held catalog would outlive the update: cards and
the list would keep speaking the previous build's documentation, even
though the fresh file is already on the device. When a quiet apply
lands, this feature makes the chooser forget its held catalog before
anything re-renders; the next reader fetches the build it is actually
running. Applies that reload were never affected — a reload empties
memory by itself.

## user

When an update arrives silently, the little documentation cards and the
features list speak the new build's words right away — not the ones the
app happened to have read earlier.

## glossary

- **held catalog**: the chooser's in-memory copy of the feature tree,
  read once per page and shared by every reader.

## code description

`fresh-catalog.js` wraps `feature_Delta.quiet` (redefinition + the
saved original): it nulls `feature_Chooser.flat` and `byPath`, then
calls down. Being the newest wrap it runs outermost, so the forget
precedes live-panel's re-render inside the same quiet apply — a panel
already open repaints from the fresh fetch. Both sides typeof-guarded;
with delta or the chooser unticked the fragment does nothing.
