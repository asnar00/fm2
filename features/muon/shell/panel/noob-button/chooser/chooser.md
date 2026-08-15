# chooser
*the feature tree, readable and tickable: the viewer and the tick-list become one surface*

> (transcripts/2026-08-14-fm-spec-3.md#p71)
> Before we wire in agents, let's focus in on this convergence between our feature viewer, and the tickable feature list.

## spec

The convergence #p59 named, built: from the nøøb button's panel, **features** opens the live product's own feature tree full screen — every node a row with its name, its one-line purpose, and a **tickbox keyed by node path**. Rows with children expand and collapse. Tapping a row's text opens the node's *served page* (spec, code, provenance transcript) in an inline reader — the explorer's rendering reused, not rebuilt, because deploy already publishes the exact tree that built the running app. Ticks are per-user (`feature_ticks`, a path-keyed map beside `update_ticks`; absent means on), travel across the user's devices, and are **stored, not yet enforced** — the same deliberate inertness as the queue's ticks, awaiting the same context manager. Unticking a node shades its whole subtree in the view: selection is understood to be subtree-shaped, as it is in `order.md`. The release queue (`/queue`) remains the chronological view; this is the structural one — two depths of the same tree, per the steering doctrine.

## user

Tap the nøøb button, then **features**: the whole app as a tree you can read and tick. Arrows expand a feature's refinements; tap a name to read what it is, who asked for it, and the code that does it; the tickbox is your choice about it — remembered on all your devices now, steering what runs in a coming update. Fixes and foundations you can't do without will grey out when choices become live.

## glossary

- **chooser**: the tickable tree view of the product's features — reader and consent surface in one.

## code description

`chooser.rs` claims `ftick_<path>` clicks: toggles that path in the user-scoped `feature_ticks` map (explicit values only, absent = on) — `/queue`'s storage pattern with node paths as keys.

`chooser.index.js` owns the view: a panel row (**features**) opens the full-screen surface; the tree comes from `features/tree.json` (exported at deploy by `tools/export_features.py` alongside the static pages — name, path, purpose, children, order.md order). Rows render with expand arrows (collapse state is ephemeral per instance), tickboxes carrying `data-ev="ftick_<path>"`, and name-taps that load `features/<path>/` into the inline reader (an iframe pane with a back bar). `reflect()` re-reads `feature_ticks` on every apply — a toggle on another device moves here — and computes effective state per row: a row is shaded when any ancestor on its path is unticked.

`chooser.index.css`: the full-screen surface above the panel, the indented tree, tick states (live/on/off), subtree shading, and the reader pane.
