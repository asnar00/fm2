# chooser
*the feature tree, readable and tickable: the viewer and the tick-list become one surface*

> (transcripts/2026-08-14-fm-spec-3.md#p71)
> Before we wire in agents, let's focus in on this convergence between our feature viewer, and the tickable feature list.

> (transcripts/2026-08-14-fm-spec-3.md#p72, draft-phase revision)
> OK, let's try this: a list of numbered features (most recent first), displayed using one line per feature, as in the basic list. Same as now, except for each line, we show an "up to parent" button ('<') and "show me more" (+) which opens a tappable intro paragraph (from the feature.md); tapping takes you to the full node page, but right here in context; it just opens it in-place (with a 'x' button to dismiss). The (+) button would also show you the sub-node names that you can tap to drill down.

## spec

The convergence #p59 named, built — form revised at #p72: **one numbered line per feature, most recent first** (the number is provenance order: 1 is the newest thing the product became). Each line carries its **tickbox keyed by node path**, an **up-to-parent `‹`** (jumps to the parent's line), and a **show-me-more `+`**, which opens in place: the node's intro paragraph (the spec's first prose paragraph, tappable — tapping opens the *full served node page* right there, spec, code and provenance transcript, with ✕ to dismiss) and its sub-node names as chips that drill down (jump to that line, opened). Structure is thus reached from recency, not instead of it: the flat list is the timeline, `‹`/chips walk the tree through it. Ticks are per-user (`feature_ticks`, path-keyed beside `update_ticks`; absent means on), travel across devices, and are **stored, not yet enforced**, awaiting the context manager; an unticked ancestor shades its subtree's lines. The release queue (`/queue`) remains the release-grained view; this is the feature-grained one — two depths of the same tree, per the steering doctrine.

## user

Tap the nøøb button, then **features**: everything the app can do, one line each, newest first. `+` shows you a short introduction and the feature's parts; tap the introduction to read the whole story — who asked for it and the code that answers — right there (✕ comes back). `‹` takes you to the bigger feature this one belongs to. The tickbox is your choice about it — remembered on all your devices, steering what runs in a coming update.

## glossary

- **chooser**: the tickable tree view of the product's features — reader and consent surface in one.

## code description

`chooser.rs` claims `ftick_<path>` clicks: toggles that path in the user-scoped `feature_ticks` map (explicit values only, absent = on) — `/queue`'s storage pattern with node paths as keys.

`chooser.index.js` owns the view: a panel row (**features**) opens the full-screen surface; the data comes from `features/tree.json` (exported at deploy by `tools/export_features.py` — name, path, purpose, **intro** (the spec's first prose paragraph) and **ts** (fmlink's provenance rule: a node's time is its cited prompt's; grouping nodes inherit their earliest child's)). The tree is flattened, sorted newest-first, and numbered from 1. Rows carry `data-ev="ftick_<path>"` ticks; `+` toggles the in-place box (intro tappable via `data-read`, child chips via `data-goto`); `‹` and chips both `goto()` — scroll to the target line, flash it, open its box. The reader is an iframe on the served page with ✕ to dismiss. `reflect()` re-reads `feature_ticks` on every apply and shades lines whose path crosses an unticked ancestor.

`chooser.index.css`: the full-screen surface, the numbered lines (tabular numerals, single-line ellipsis), tick states, the in-place box (intro + chips), the flash-on-jump, and the ✕-dismissable reader pane.
