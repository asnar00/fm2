# quiet-credits
*nothing floats over the map: the credits move to the bottom of the features popup*

> (transcripts/2026-09-01-saturday.md#p17c)
> I'd like to do a bit of work on the look and feel of the map tiles. First, the openstreetmap credits are in an annying place, just above the toolbar buttons. Do we really have to show them on screen at all times? I'd prefer to add them to a "credits" section maybe at the bottom of the features popup.

## user

The map has nothing written over it any more. The credits — who drew the
ground, who drew the boundaries — sit quietly at the bottom of the features
popup.

## spec

The always-on line was Leaflet's attribution control, a small dark pill
lifted clear of the toolbar and therefore sitting exactly where the eye goes
before it presses a button. It leaves the map. The words themselves do not
leave the app: they arrive at the bottom of the panel, under everything
else, as a `credits` section — a label and one line per source, in the
dimmest text the app has, with no box, no border and no ceremony.

**Placement.** The node is a child of `/map`. It was born as map look-and-
feel ("the look and feel of the map tiles"), it is `/map`'s behaviour it
changes, and unticking `/map` should take the whole arrangement with it —
map bar and popup section together, since with no map there is nothing to
credit. It reaches the panel the way every node reaches a surface it does
not own: a fragment of its own, no edit to `panel.index.html` or to the
chooser. `/chooser` is at its six-child cap, so a child there would have
forced a regroup for what is not chooser behaviour anyway.

**Why the credit may move at all.** OpenStreetMap's attribution guideline
allows an application on a small screen to put the attribution one
interaction away rather than always on screen; the ONS/OS boundary credit is
OGL v3 and asks to be stated somewhere reasonable. A credits section one tap
inside the panel satisfies both. What it must not do is drift from the
truth, so nothing here is written prose: the map's line is fetched from the
server's `tiles/attribution` route — the same source `/map`'s own `credit()`
reads, so a change of tile source changes this line too — and the boundary
line is read from the boundaries file's own `credit` field.

**How the bar goes.** `attributionControl: false` is a Leaflet *mount*
option, and reaching it would mean editing `map.js`. So the control is made
as before and then removed — `map.attributionControl.remove()` — from a
wrapper around `feature_Map.mount`, at the first mount. Removal, not a CSS
`display: none`: a hidden control still measures, still holds Leaflet's
bottom-right corner, and would still be found by anything walking the DOM;
and one mechanism is honest where two would be belt-and-braces. `/map`'s
`credit()` and `/boundaries`'s `credit()` go on calling `addAttribution` on
the removed control, which is exactly a no-op — Leaflet's `_update` returns
early once the control has no map — so neither file needs touching and
neither breaks.

**Reading the boundary credit without the map.** If the map view has been
opened, `/boundaries` already holds its parsed file and the credit is read
straight off it. If it has not, the file is fetched once for its credit
alone: it is our own asset, already in the app's cache, and a credit that
only appears after you have visited a particular view is not a credit.

**When a line is missing.** Each source is independent. `/tiles` unticked
means the route 404s and there is no map line; `/boundaries` unticked means
no boundary line; neither present means no section at all — an empty label
is worse than silence.

## hostile cases

- **This node unticked.** Nothing wraps `mount`, so the bar comes back
  exactly as it shipped, and no `#credits` element is made.
- **`/map` unticked.** This node goes with it (a child of an unticked
  parent), and so does the section — correctly: there is no map to credit.
- **`/tiles` unticked.** The fetch 404s. A 404 answered with the shell's
  html is not a credit, so a body containing `<`, or a long one, is
  discarded rather than printed.
- **`/panel` unticked.** No `#panel` to append to; the map bar still goes
  (the ask's first half stands on its own).
- **`/boundaries` unticked.** `typeof feature_Boundaries` is undefined; one
  line, not two.
- **Offline.** Both reads are same-origin and cached by the service worker.
  Proven with the wire cut on a warmed cache: the map draws its cached tiles
  and boundaries with no bar, and the section still prints both lines. A
  failure leaves whatever lines it did get, and the next open asks again —
  an empty answer is never cached.
- **The panel opened twice quickly.** The gather is memoised as a promise,
  so the two opens share one pair of reads.

## glossary

- **credits section**: the dim block at the bottom of the panel naming every
  source the app must credit.

## code description

`quiet-credits.index.js` wraps two things at load, both by property
replacement (notes.md, "the apply-wrapper race"), both typeof-guarded.
`feature_Map.mount` gains a wrapper that calls the original and, on a
successful mount, calls `strip()` — which removes the map's attribution
control. `feature_Panel.open` gains a wrapper that starts `show()` and *then* calls
the original — that order and not the reverse: the credits wait on nothing
the panel's open does, while the open itself awaits the feature list, which
`/arrives` allows up to 2.5 seconds. Behind it, the section arrived late
enough for the rig to photograph a sheet without it.

`gather()` is the extensible function: it returns the lines the credits
section prints, and a later node with a source of its own wraps it the way
this file wraps `open` and pushes its line onto the result. Today it calls
`tileLine()` (the `tiles/attribution` route, rejected unless it looks like
one plain line) and `boundaryLine()` (`/boundaries`'s parsed file if it has
one, else one fetch of `feature_Boundaries.FILE` for its `credit` alone).

`show()` memoises the gather as a promise, prints a `credits` label and one
`.credit-line` per source into `#credits`, and hides the whole element when
there is nothing to say; an empty result clears the memo so the next open
tries again.

The load block also makes `#credits` and appends it to `#panel` — this
node's fragment is composed last, so appending puts it below `/less-busy`'s
arrangement, at the bottom of the sheet.

`quiet-credits.index.css` is three rules: 11px, `#77777e`, a hair of margin,
and the lines a touch dimmer than their label. No border, no background.
