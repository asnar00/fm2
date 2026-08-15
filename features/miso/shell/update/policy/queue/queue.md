# queue
*the full updates queue: every change, full screen, with a per-user tickbox on each*

> (transcripts/2026-08-14-fm-spec-3.md#p50)
> in the updates queue view: let's expand that to cover the full screen, make the list of feature updates scrollable, and put a user-fillable tickbox next to each one [or gray out the tickbox if it's a static feature]. For now, we don't need to do anything with the tick values other than store them associated with that user; later, we can plug them into the context manager. Does that make sense?

*(Revised by [chooser](#miso/shell/panel/noob-button/chooser), #p78: the feature list now occupies the panel area whose tap opened this view, and carries the same build numbers per feature — this node stays composed (tick storage and the view remain) but its entry point stands down while the chooser is ticked.)*

## spec

The panel's six-entry changes teaser gains a full view: tap it and the whole queue opens full screen — every changes.json entry, newest first, scrollable, with a tickbox per entry. `feature` entries are tickable and default ticked; `fix` entries are grayed and permanently ticked — a fix is not a choice, the same reading `/policy` gives them. (True static/dynamic markers per node arrive with the context manager; release kind is the honest v0 proxy.) Ticks are stored per user and follow them across devices — `update_ticks`, a user-scoped var like `update_policy` beside it — and are deliberately inert for now: this node stores choice, it does not yet act on it. The first half of `/policy`'s named refinement ("fine-grained per-feature consent"); the context manager plugs in later.

## user

Tap the changes list in the system panel and it expands to fill the screen — every update that ever shipped, scrollable. Each new-behaviour update has a tickbox that's yours: untick the ones you'd rather not have. (For now miso just remembers your choices — they'll start steering what runs in a coming update. Fixes are always on.) Your ticks follow you to all your devices. ✕ closes the view.

## glossary

- **tick**: a user's stored yes/no against one shipped change; the raw material of per-feature consent, not yet an enforcement.

## code description

`queue.rs` claims `qtick_<build>` clicks: it toggles that build's entry inside `update_ticks` (a user-scoped var holding a JSON object of explicit choices, absent keys meaning the default: ticked). `/scope` ships it, `/join` restores it — storage was free.

`queue.index.js` owns the view: the panel's `#changes` div becomes a tap target; opening fetches `changes.json` (no-store), renders every entry — build, kind chip, text, tickbox (`data-ev="qtick_<build>"` for features; a grayed inert box for fixes) — into a full-screen overlay, and re-reflects tick states from loop state on every apply (so a toggle on another device moves the box here). ✕ closes. Rows honestly label their kind; unknown kinds (pre-classification builds) count as fixes, matching `/policy`'s caution.

`queue.index.css`: the overlay (fixed, inset 0, above the panel's z-index, safe-area padded), the scrollable list, row layout, and the two tickbox states — live (tappable, filled when ticked) and static (dimmed, non-interactive).
