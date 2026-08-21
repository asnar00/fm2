# chooser
*the feature tree, readable and tickable: the viewer and the tick-list become one surface*

> (transcripts/2026-08-14-fm-spec-3.md#p71)
> Before we wire in agents, let's focus in on this convergence between our feature viewer, and the tickable feature list.

> (transcripts/2026-08-14-fm-spec-3.md#p72, draft-phase revision)
> OK, let's try this: a list of numbered features (most recent first), displayed using one line per feature, as in the basic list. Same as now, except for each line, we show an "up to parent" button ('<') and "show me more" (+) which opens a tappable intro paragraph (from the feature.md); tapping takes you to the full node page, but right here in context; it just opens it in-place (with a 'x' button to dismiss). The (+) button would also show you the sub-node names that you can tap to drill down.

> (transcripts/2026-08-14-fm-spec-3.md#p73, draft-phase revision)
> That's great. On reflection, the text we show at the top level should be the "user" paragraph.

> (transcripts/2026-08-14-fm-spec-3.md#p75, draft-phase revision)
> Instead of the '<' and '+' buttons on every line, let's just allow the user to tap on the line itself - that should expand to the user paragraph, and maybe there's a "<" to the left of that text that you can tap to go up a level; and tap-text to go to full page as now. This is very close.

> (transcripts/2026-08-14-fm-spec-3.md#p76, draft-phase revision)
> then, move the tick button the RHS of the line; and print the version number to the left in bold, as in the main list.

> (transcripts/2026-08-14-fm-spec-3.md#p77, draft-phase revision)
> the number on the left should be the build number that introduced that feature; i.e. we're unifying this list with the main versions list in the update popup.

> (transcripts/2026-08-14-fm-spec-3.md#p78, draft-phase revision)
> that's perfect. So now we can replace the version list underneath the "logged in as..." line with this new feature list view, but enough lines that we can stretch the view up to the top of the screen if need be. We can get rid of the "features" button (since we're already looking at the view) and the "view source" button (since we get to feature source per feature) so there's just the update policy and then "logout". Later on we'll make it context sensitive and so on. But for now, this works nicely.

> (transcripts/2026-08-14-fm-spec-3.md#p82, draft-phase revision)
> that's great, that's exactly right. One small tweak: use the most-recent release number in the feature list (i.e. 112 instead of 103 for chooser)

> (transcripts/2026-08-14-fm-spec-3.md#p81, draft-phase revision — the first cut leaked the panel at boot and broke scrolling)
> What should happen is: on the main screen, I don't see "updates:" or "logout", just the toolbar; when I hit the noob button, I want a panel starting with "logged in as", then a scrollable feature-list view whose size is restricted; and below that, "updates:" with the update policy, then "log out".

## user

Tap the nøøb button: under your name, everything the app can do — one line each, newest first, numbered by the build that introduced it. Tap a line for a short introduction and the feature's parts; tap the introduction to read the whole story — who asked for it and the code that answers — right there (✕ comes back). The `‹` beside the introduction takes you to the bigger feature this one belongs to. The tickbox is your choice about it — remembered on all your devices, steering what runs in a coming update.

## spec

The convergence #p59 named, built — and at #p78, moved home: the list IS the panel's centrepiece. Tap the nøøb button and under "logged in as…" sits the feature list itself (the six-line version teaser it replaces is the seam's fallback — untick this node and the teaser returns); the list itself has a restricted height and scrolls within the sheet (#p81 — the panel's own show/hide lifecycle is untouched: the first cut restyled `#panel` and thereby showed the sheet's buttons at boot and broke the sheet's sizing; the lesson is recorded in the CSS comment). Panel order: who-line, the list, update policy, log out. The panel slims around it: no "features" button (you are already looking at it), no "view source" (every feature's page carries its source) — who-line, features, update policy, logout. Context-sensitivity is named future work. The full-screen `/queue` view keeps its node but its teaser-tap entry stands down while this occupies the area — its numbers now live on every line here. Form, from #p72: **one line per feature, most recent first**, each numbered with **the most-recent build that touched it** (#p77's unification, sharpened at #p82: the number moves forward as the feature evolves) — the chooser and the release queue speak the same numbers, one list keyed by what shipped, the other by what it is. Each line reads bold build number left (the release-list styling, #p76), name and purpose, **tickbox at the right** (keyed by node path); **tapping the line itself** (#p75 — no per-line buttons) opens in place: a **`‹`** at the left (up to the parent's line) beside the node's intro paragraph (its `## user` paragraph — written for exactly this reader (#p73); tapping the text opens the *full served node page* right there, spec, code and provenance transcript, with ✕ to dismiss), and its sub-node names as chips that drill down (jump to that line, opened). Tapping the line again closes it. Structure is thus reached from recency, not instead of it: the flat list is the timeline, `‹`/chips walk the tree through it. Ticks are per-user (`feature_ticks`, path-keyed beside `update_ticks`; absent means on), travel across devices, and are **stored, not yet enforced**, awaiting the context manager; an unticked ancestor shades its subtree's lines. The release queue (`/queue`) remains the release-grained view; this is the feature-grained one — two depths of the same tree, per the steering doctrine.

## glossary

- **chooser**: the tickable tree view of the product's features — reader and consent surface in one.

*(Revised by [enforced](#miso/shell/panel/noob-button/chooser/enforced), rung 8:
the sentence above that ticks are "**stored, not yet enforced**, awaiting the
context manager" was true for four builds and is not any more. The context
manager arrived as the absorption ladder, and unticking a line now stops that
feature running — for that user, on all their devices, with their state intact
for the re-tick. `chooser.rs` is gone with it: the stored map was a `SyncVar`,
and the map this file's page half reads is derived from the context now. Nothing
in `chooser.index.js` changed.)*

## code description

`chooser.rs` /retired/: it claimed `ftick_<path>` clicks and toggled that path
in a stored `feature_ticks` `SyncVar`. Rung 8 deleted both — the click is
`/enforced`'s now, and the map is derived rather than stored.

`chooser.index.js` owns the view: `mount()` — called through `/panel`'s fill seam on every open — renders the list into the panel's changes area (claiming it from `/queue`'s tap handler, which is guarded and stands down); the data comes from `features/tree.json` (exported at deploy by `tools/export_features.py` — name, path, purpose, **intro** (the `## user` paragraph, spec-paragraph fallback) **ts** (fmlink's provenance rule: a node's time is its cited prompt's; grouping nodes inherit their earliest child's) and **build** (deploy's convention build = commit count, read back: the count at the last commit touching the node's own files — spec, code, assets, children excluded, they carry their own numbers — computed by `latest_build()`). The tree is flattened and sorted newest-first. Rows carry `data-ev="ftick_<path>"` ticks (tick taps are excluded from row handling); tapping the row toggles the in-place box (`‹` via `data-up`, intro tappable via `data-read`, child chips via `data-goto`); `‹` and chips both `goto()` — scroll to the target line, flash it, open its box. The reader is an iframe on the served page with ✕ to dismiss. `reflect()` re-reads `feature_ticks` on every apply and shades lines whose path crosses an unticked ancestor.

`chooser.index.css`: the full-screen surface, the numbered lines (tabular numerals, single-line ellipsis), tick states, the in-place box (intro + chips), the flash-on-jump, and the ✕-dismissable reader pane.
