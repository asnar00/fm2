# source
*the app links to its own source: a view-source button in the panel*

> (transcripts/2026-08-14-fm-spec.md#p136)
> let's add a "view source" button to the update popup?

## user

Open the panel (tap the logo pill), press `view source`, and the feature tree opens in the browser — every node, its prompt, and its code.

## spec

The panel gains a `view source` button that opens the live feature tree at `/features/` — miso's own self-description, served by miso. It opens outside the PWA window (a new tab / in-app browser) so the app itself stays put. A feature-modular app's "source" is its feature tree, not a file listing: the button lands on the same orientation page any agent or human uses.

## glossary

- **view source**: the panel button opening `/features/`, the served feature tree.

## code description

`source.index.js` inserts its own `.row` with the button into the panel when the fragment evaluates (the panel's markup composes earlier in linearisation, so the div exists), placed just above the log-out row so log out stays the panel's last word; the click opens `features/` via `window.open('features/', '_blank')`. No panel code changes — the row rides in with the node, and unticking it removes the button entirely.
