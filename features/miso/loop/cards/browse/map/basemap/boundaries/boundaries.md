# boundaries
*the patch, drawn: the constituency's edge and every ward inside it, named*

> (transcripts/2026-09-01-saturday.md#p2)
> I demoed to Tara - she liked it, and our first actual canvassing session is
> on Saturday! So we have some work to do to get ready. In no particular
> order: 1] show constituency and ward boundaries on the map; 2] clean up the
> "make posts" interface to include video, audio, photo+type, transcription 3]
> make a quicker QR-code based invite workflow that we can use instantly
> during canvassing 4] AI interrogation / report generation.

## user

Open the map and the patch is on it. A dashed line rings the Sevenoaks
constituency; inside it, thinner lines divide the twenty-six wards, and each
ward carries its name. The lines are grey and quiet — they sit under the pins,
never over them, and tapping one does nothing, because they are the ground and
not the thing. Zoom out past the district and the names step aside rather than
pile up. With nothing of yours on the map yet, the map opens on the patch
instead of on the whole world.

## spec

A canvasser standing on a street needs two answers the pins cannot give:
*which ward is this* and *where does the patch end*. Both are lines on the
ground, so they are drawn on the ground.

**The boundaries are a file this node owns.** `assets/map/boundaries.geojson`
is served from `site/map/` beside Leaflet, and the page fetches it from us —
the same promise `/tiles` made and for the same reason: nothing on this page
talks to a third party, and the service worker caches what the page fetches
from us, so the second visit has the patch offline. It is **committed, not
fetched at build time**: a build-time download is an artifact a fresh clone
silently ships without (`misses.md`, 2026-08-28, six builds with no whisper
model).

**The file is the seam, not the code.** Every feature carries `kind`
(`constituency` or `ward`), `code` (the ONS code — `E14001465`, `E05005026`),
`name`, and for a ward a `label` point. Nothing about Sevenoaks is written in
JavaScript. A later "which ward am I in", "colour the wards by coverage" or a
second constituency is a new node reading the same features, or a new file —
not a rewrite of this one.

**A ward's name sits at its pole of inaccessibility**, not at its centroid: an
L-shaped ward — Swanley, Edenbridge — has a centroid outside itself, and a
name floating in the next ward is worse than no name. The point is computed
once, when the file is made, and shipped in the file; the page places what it
is given.

**The names are placed, not merely drawn.** Twenty-six names over a district
that fits on one phone screen is a pile, and a pile is not a readable name —
the first rig shot had eight names lying across each other over Swanley. So
every name is measured and seated: biggest ward first, and a name stands down
if it is wider than the ward it belongs to or if it would land on a name
already placed. Zooming in makes room and the missing names arrive one by one.
The rule is the same at every zoom, so what is on the screen is always exactly
what fits, and below zoom 9 — where nothing fits — the names are simply not
there.

**Nothing here is interactive.** The lines are drawn `interactive: false` and
the label pane has `pointer-events: none`, so a drag across a boundary is a
drag on the map and a tap on a pin is a tap on the pin. `/map` already owns
every gesture this view answers and this node adds none.

**The lines are under the pins.** Leaflet draws paths at pane 400 and markers
at 600; the names get a pane of this node's own at 450 — above the line it
belongs to, below every face. A boundary that could cover a pin would have
inverted the view: the pins are the content, the patch is the furniture.

**With no pins, the patch is where the map opens.** `/map` fits the bounds of
the pins when there are pins and otherwise asks the device where it is. With
neither — a fresh canvasser's first look — the map sat at zoom 3 over the
Atlantic. This node fits the constituency instead, once, and only when `/map`
has not already fitted and there are no markers; if geolocation then answers,
the device's own position still wins, which is the right precedence.

**The credit is the file's, and the file says it.** The boundaries are ONS and
Ordnance Survey data under the Open Government Licence v3.0, which requires
attribution. The short line rides in the file's own `credit` field and is added
to Leaflet's attribution control beside `/tiles`' line, so a different source
ships its own words instead of needing this file edited. The full statement is
the file's `attribution` field.

**Where the file came from.** ONS Open Geography Portal, generalised-clipped
(BGC, 20 m) boundaries, queried as GeoJSON in EPSG:4326:
`Westminster_Parliamentary_Constituencies_July_2024_Boundaries_UK_BGC` with
`PCON24NM='Sevenoaks'`, and `Wards_December_2024_Boundaries_UK_BGC` with
`LAD24NM='Sevenoaks'`. Both were then Douglas–Peucker simplified at a ~25 m
tolerance and rounded to five decimal places, which takes 120 KB of download
to **45 KB** of committed file across 27 features and 2,147 points — a tenth
of a second on a phone, and less than a single map tile.

## hostile cases

- **The file will not load** (offline before the service worker has it, or
  `assets/` half-copied). The fetch fails, no layer is made, and the map is
  exactly `/map`'s map. The next paint tries again; after four failures it
  stops asking rather than retrying forever on a broken deploy.
- **The file is malformed or empty.** `r.json()` throws into the same catch, or
  the feature list is empty and is refused. Nothing is drawn, nothing throws.
- **Leaving the map and coming back.** The layers live on the Leaflet instance,
  which `/map` never destroys, so `ensure()` finds them and returns. No refetch,
  no reflicker, no second fit.
- **`/map` unticked.** `feature_Map` is undefined, the seam is never taken, and
  no fetch happens — there is no map to draw a boundary on.
- **`/boundaries` unticked.** The fragment leaves the page and the file leaves
  `site/map/`; the map is the map it was.
- **A repaint mid-drag.** `ensure()` sees the layer and returns before touching
  the map, so the drag is undisturbed — `/map`'s own rule, kept.
- **Zoomed out to the world.** The names' pane is hidden; the lines remain, as
  a small grey shape somewhere in Kent, which is honest.
- **Two wards too small to name at this zoom.** Neither name is drawn, and
  neither is drawn wrongly. Zooming in brings them back; there is no state to
  go stale, because the seating is recomputed from scratch on every move.
- **A pin outside the patch.** It draws where it is. This node states where the
  boundaries are and never filters anything by them.

## glossary

- **patch**: the ground a canvassing team covers — here the Sevenoaks
  constituency and the wards of Sevenoaks District.
- **label point**: the point inside a ward furthest from its own edge, where
  the ward's name is drawn.

## code description

`boundaries.js` takes `/map`'s `sync()` by property replacement at load — the
same idiom `/map` used on `/loop`'s `paint`, and guarded by `typeof
feature_Map` so an unticked map is simply a no-op.

`feature_Boundaries.ensure()` is what runs on every paint: it returns at once
if the layers exist, and otherwise `load()`s the file. `load()` fetches
`map/boundaries.geojson` from our own site, keeps the parsed collection, and
counts its own failures so a permanently broken file stops being asked for.

`draw()` makes the two layers and the pane they need: an `L.geoJSON` of every
feature, styled by `styleOf()` — dashed `#8b8b95` at 2 px for the
constituency, plain `#5c5c66` at 1 px for a ward, no fill in either case — and
an `L.layerGroup` of one zero-size `L.divIcon` per ward at its `label` point,
in a pane of this node's own at z-index 450 with pointer events off.

`place()` runs on Leaflet's `zoomend` and `moveend` and once at draw: it hides
the whole pane below zoom 9, and otherwise measures each name (hidden but laid
out, so `offsetWidth` is real), works out how wide its ward is on screen from
the bounds `onEachFeature` kept, sorts by that width descending, and shows a
name only if it is on screen, no wider than about its own ward, and clear of
every name already seated. `credit()` adds the file's `credit` line to the attribution control,
once. `fit()` fits the constituency's bounds when there is nothing else to fit,
once.

`boundaries.css` styles one thing: `.ward-label`, 10 px 500-weight `#8e8e97`
centred on its point with a six-way `#101012` halo so a grey word reads over a
pale building without a box behind it.

`assets/map/boundaries.geojson` is the committed data — one `FeatureCollection`
carrying `attribution`, `credit`, the constituency and the twenty-six wards.
