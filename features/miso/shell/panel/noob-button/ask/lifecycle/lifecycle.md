# lifecycle
*your requests ride the feature list: asked, proposed, shipped — one chronology of becoming*

> (transcripts/2026-08-15-fm-spec.md#p31a)
> and this is where a list of in-progress feature requests (matching exactly the waiting-for-update and feature list formats) would be supercool.

## user

Your asks live in the system panel with the updates: each one listed
with where it's got to — waiting for your description, sent to the
builder — until it ships and becomes a real entry in the feature list.
Ask on your phone, see it listed on your laptop.

## spec

The feature list becomes the request lifecycle's display (#p85's
doctrine): a **requests** section, formatted like the awaiting-update
section — header, chooser-style rows — showing the user's own asks
that are still becoming: `asked` (filed, no proposal yet), `proposed`
(paragraph approved, waiting for the builder), and later the fuller
ladder (in progress, "!"/"?"). Each row is the ask's title with its
status where the build number would sit, and the description one tap
away (#p39: no headers, no inline prose — the feature-list grammar,
exactly).
Shipped requests leave the section: by then they are simply features,
in the list proper, with the approved paragraph as their intro.

The section lives in the same box as the awaiting update, just below
it, visible while the long list is folded (its rows are nested in its
own block, the standing pattern), and re-renders on every state
change — an ask filed on another device walks in over sync.

A named limit, honestly: status changes the builder makes land in the
server's copy of the asks store and reach a device on its next
join/launch — live mid-session status pushes await a builder→user
channel.

## glossary

- **requests section**: the feature list's block for asks still
  becoming features — the lifecycle's display half.

## code description

`lifecycle.index.js` renders `#requests` into `#changes`: one
`.crow`-grammar row per ask with status `asked` or `proposed` (newest
first, status in the number slot, title bold); tapping a row toggles
its `.cmore` expansion holding the proposal, expansion state kept in
`open` across the re-renders. `mount()` wraps `feature_Chooser.mount` to place the
section after `#awaiting` (or at the top), and a `feature_Loop.apply`
wrap re-renders it on state change when the panel is showing — sync
arrivals included. `lifecycle.index.css` styles the status stamp.
