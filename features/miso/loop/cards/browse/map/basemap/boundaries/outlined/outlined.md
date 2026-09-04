# outlined
*the wards are black lines and nothing else: no names to read, no pile to untangle*

> (transcripts/2026-09-04-field-walk.md#p17)
> OK. next batch of work (hopefully parallel with the first): I'd like to
> display the constituency / ward boundaries slightly differently. (if
> possible): to have a selected map region (either the whole constituency, or
> just a single ward - selectable under the posts toolbar maybe- show using
> "stadia outdoors", and outside the region show in the current look. So we'd
> need to "mask" the current region where the boundary intersects a tile.
> Also, let's not print the ward labels for all wards, as they make the map
> look messy when zoomed out; instead, we'll show ward boundaries as black
> outlines.

*(The second half of the ask — the region in Stadia Outdoors — is `/region`,
this node's sibling, briefed in parallel. This one is the ink.)*

## user

The map's ward lines are black. No ward carries its name any more, at any
zoom: the district reads as a shape divided into shapes, and the only words
on the map are the street map's own. The dashed constituency ring is
unchanged.

## spec

`/boundaries` drew twenty-six named wards in grey and worked hard to keep the
names from piling up — measured, sorted by ward width, seated biggest-first,
each one standing down if it would land on a name already placed. That
machinery answered the right question, and ash's look at the result answered
a better one: on a phone the names are clutter whatever the seating does, and
what a canvasser needs from a ward line is the line.

**The names go; they are not merely hidden.** The label layer is taken off
the map and the pane it lived in is hidden. Removing the layer is what makes
this honest — twenty-six `divIcon` markers behind a `display: none` pane
would still be twenty-six markers Leaflet repositions on every pan. The pane
is hidden as well, because it is `/boundaries`' own and holds nothing else.

**The seating is replaced, not disabled.** `feature_Boundaries.place()` is
what `zoomend`, `moveend` and `draw()` all call, and it is the only function
that touches the names. This node replaces it outright with the removal — so
the removal runs at draw and again on every move, which is the re-entry that
matters: a label layer re-added by any later path is taken off again on the
next gesture rather than surviving until a reload. It is a replacement and
not a wrapper because there is nothing left of the original worth running:
seating names that are not on the map is work with no result.

**The wards are black; the constituency is not touched.** `styleOf` is
*wrapped*, not replaced: this node calls the style the chain already produced
and overrides the three properties a ward's line needs — `#000`, 1.2 px, full
opacity — leaving every other key, and every non-ward feature, exactly as the
chain made it. That is deliberate. `/light-basemap` replaced `styleOf`
wholesale to re-ink both lines for a light ground, and a second wholesale
replacement here would have silently discarded its constituency ink;
`misses.md`'s "siblings at one anchor" is the same shape one level along.
Wrapping means the dashed ring keeps whatever colour and weight the ground of
the day gives it, and a future ground change re-inks it without touching this
node.

**1.2 px, and why not 1.** A ward line at 1 px on a phone at DPR 3 is a
hairline the dark ground swallows at the zooms a canvasser walks at; at 2 px
it competes with the constituency ring, which must stay the stronger of the
two. 1.2 px is the width at which a black ward line is continuously visible
at zoom 11 and still reads as thinner than the ring — judged on the simulator
against the live Alidade Smooth Dark ground and against `/region`'s Stadia
Outdoors, where black on pale terrain is at its strongest.

**Black on a dark ground is a real cost, and it is the ask.** On Alidade
Smooth Dark's `#333333` land a black line reads as a *darker* line rather
than a brighter one — quieter than the grey it replaces, not louder. That is
what was asked for, and it is the right choice for the pair: the same ink
serves the light Outdoors ground inside `/region`'s mask, where it is
emphatic, and one boundary ink across two grounds is one thing to understand
instead of two.

## hostile cases

- **`/boundaries` unticked.** `typeof feature_Boundaries` is undefined, the
  block does nothing, and there are no boundaries to ink or names to remove.
- **`/map` unticked.** `feature_Boundaries` exists but never draws, so
  `place()` is never called and `styleOf` never asked. No-op either way.
- **This node unticked.** `place()` is `/boundaries`' own again, the seating
  returns with the names, and the wards are the grey the ground of the day
  gives them.
- **`place()` before `draw()`.** `hush()` reads `feature_Map.map` and returns
  at once if there is no map; `feature_Boundaries.labels` is `null` until
  `draw()` makes it, and removing nothing is not an error.
- **Called again on every pan and zoom.** `hasLayer` is false after the first
  removal, the pane is already hidden, and the whole call is two property
  reads. This runs on every `moveend` for the life of the page and must be
  cheap; it is.
- **A repaint that re-enters `draw()`.** `/boundaries`' `ensure()` returns
  early once `lines` exists, so `draw()` runs once per page. If it ever ran
  twice, the second `draw()` would make a second label layer and call
  `place()` immediately after adding it — which removes it. The re-entry is
  covered by the same code path as the first entry, which is why the removal
  lives in `place()` rather than in a once-only block at load.
- **The file loads after this fragment.** Everything here is installed at load
  on an object that exists at load; the fetch that follows finds the
  replacements already in place.
- **A later sibling wrapping `place()`.** It wraps this node's version and the
  names stay off. A later sibling *replacing* it would put them back — the
  same trap `misses.md` recorded; the note is here for whoever reads it next.

## glossary

(no new terms — `patch` and `label point` are `/boundaries`'.)

## code description

`outlined.js` installs two replacements on `feature_Boundaries` at load,
guarded by `typeof`, the idiom `/boundaries` itself used on `/map`'s `sync`.

`styleOf` is wrapped: the captured original is called first and its result
passed to `feature_Outlined.styleOf(f, prev)`, which returns `prev` unchanged
for anything that is not a ward, and a copy with `color`, `weight` and
`opacity` overridden for a ward.

`place` is replaced by a call to `feature_Outlined.hush()`, which removes
`feature_Boundaries.labels` from the map if it is on it and sets the label
pane's `display` to `none`. It is guarded against being called before the map
or the layer exists, and is idempotent.
