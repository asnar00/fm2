# region
*the patch you are working, in daylight: one region drawn on the outdoors ground, the rest of the map as it was*

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

*(The black outlines and the vanished names are `/outlined`, this node's
sibling. This one is the ground.)*

## user

Open the posts on the map and there is a new button in the row: a patch of
ground with one part marked off. Tap it and the map gives way to a field of
names — the whole constituency on its own line, then every ward, alphabetical.
Tap one, tap ‹, and you are back on the map with that ward drawn in daylight:
paths, contours, greens and woodland in the colours a walking map uses, cut
exactly along the ward's black outline, with the rest of the district still in
the app's own dark. Which region you chose is yours and travels with you.

## spec

Two grounds on one map, and a line between them that has to fall where the
boundary falls rather than where a tile edge does.

**The second ground is ours, like the first.** `GET tiles/outdoors/{z}/{x}/{y}.png`
proxies Stadia's Outdoors style and caches it on disk under
`$MISO_CONTEXT_DIR/tiles-outdoors/`, a directory of its own so the two grounds
can never answer for each other. It is `/tiles`' route rewritten rather than
`/tiles`' route reused: this link claims a four-segment path that `/tiles`'
three-number parser would refuse, and it calls no helper of `/tiles`' at all,
so a node in the browse tree does not tie its tick to a node in `serve`. That
is the same argument `/tiles` itself makes for reading `MISO_CONTEXT_DIR`
rather than borrowing `/remember`'s `context_dir()`.

**The key is never in the repo, and there is no new secret to set.** The
source is looked for in three places: `MISO_OUTDOORS_URL` (a whole template,
the door `MISO_TILE_URL` is for the everyday ground); then `STADIA_KEY`, which
is the shape the reference plist documents; and failing both, the everyday
ground's own `MISO_TILE_URL` with its style segment swapped for `outdoors` and
its query — the key — carried over untouched. The third is what the live
server actually has: ash's key rides inside `MISO_TILE_URL` as `?api_key=…`
and is a variable of nobody's. So this node works on the live box, and on any
rig started with the live env, with nothing added anywhere. With none of the
three the source is the empty string, no fetch is made, every square 404s, and
the map shows the everyday ground inside the region as well as outside it —
`/stand-in`'s rule, that a missing square is never an error on the page.

**The mask is a clip on a pane, in the pane's own space.** The outdoors layer
lives in a Leaflet pane of this node's own at z-index 250 — above the ordinary
tiles at 200, below the boundary paths at 400, so the black line that marks
the region is drawn *on* the region and never under it. The pane carries a
`clip-path` whose outline is the region's polygon written in **layer-point**
coordinates, which is precisely the pane's own coordinate space.

That choice of space is what makes the whole thing hold together. Leaflet
carries a pan as a `translate3d` on the map pane and a zoom animation as a
translate plus a scale, and a `clip-path` is applied in the element's own
space *before* an ancestor's transform reaches it — so the cut travels with
the tiles during a drag and scales with them through a zoom animation, glued
to the ground it is cutting. It only has to be rewritten when the pixel origin
itself moves, which is a zoom or a `setView`, so the outline is rebuilt on
`zoomend`, `moveend` and `viewreset` behind a key of zoom, origin and region
code: a drag recomputes nothing at all.

**`clip-path: path()` rather than an SVG `clipPath`, measured not assumed.**
Both roads are written here and the `CLIP` constant chooses. Measured on
WebKit at device pixel ratio 3, on a stretch of boundary running through open
ground at zoom 16, with every square let arrive before the shutter: the two
are **byte-identical** — zero differing pixels of 3,162,132, against a
same-mode control taken the same way and the same interval apart, which is
also zero. The transition across the cut is a median of **one device pixel**
(a maximum of three, which is the black boundary stroke's own antialiasing
falling in the same band), so neither road softens the edge, halos it, or lets
a tile boundary show through it.

Byte-identical means the choice is not about quality, so it is made on cost:
the CSS form is shipped because it needs no second element in the DOM, no
document-wide id, and no third thing to keep in step with the pane. The SVG
road stays in the file, unused, because the day WebKit changes its mind about
one of them the other is a one-word edit rather than a rewrite.

**Even-odd, so a hole is a hole.** A ward's geometry may be a `Polygon` with
rings — the first its outside, the rest its holes — or a `MultiPolygon` of
several such parts; the file may carry both and a ward with a detached piece
is not a special case. Every ring of every part becomes one closed subpath of
one outline, and the `evenodd` rule cuts the holes out without this code
having to know which ring is which, or which way round it was wound.

**The map's own beats build the ground, not only the page's repaint.** The
first cut of this node hung its whole life on `/loop`'s `paint`, and made the
tile layer only when a paint found the geojson already parsed. On the rig that
was a map you could open, pan and zoom with no second ground on it at all
until some unrelated tap repainted the page — because the file arrives after
the first paints, and a map that is merely being looked at produces no more.
So the pane's `zoomend`, `moveend` and `viewreset` call `ensure()` and not
merely the re-cut, and the fetch calls `ensure()` when it lands: a gesture, or
the file's own arrival, is enough to build the ground. The paint seam remains,
because it is what notices the region *changing*.

**The layer is bounded to the region.** Leaflet knows nothing about the clip
and would ask for every square on the screen, including the ones the mask
throws away entirely. The tile layer carries the region's own bounding box as
its `bounds`, so a mask the size of one ward costs one ward's worth of a
metered budget rather than a screenful. The layer is also *made* only once a
region is known, and *removed* — not hidden — when it is not: a hidden Leaflet
layer still asks for its squares.

**The mask applies at every zoom.** `/boundaries` hides its label pane below
zoom 9; this node does not follow it, and the difference is deliberate.
Hiding names below 9 is decluttering — there is no room for a word. The mask
is not clutter, it is which ground you are standing on, and a canvasser zoomed
out to see the whole district is exactly the person asking "which part is
mine". The cost is bounded by the same `bounds`: at zoom 8 the constituency is
a square or two.

**The button is a patch, in a colour of its own.** The glyph is an irregular
patch of ground with one part of it filled — an area, and the part of it that
is yours. The first cut was a folded map, and on the rig it read as the view
picker's own map glyph three fingers away: the same shape, twice, for two
different things. The colour is `/ember`'s deterministic pick for the name
`region` rather than the posts pink, because the posts row already holds the
posts bubble and `/posts`' pink plus, and a third pink beside them was a wall
of one colour; `/recentre` — the other map act that lives in whatever row is
open — made the same call with its own name.

**The choice is a sub-tool, not a page button.** `region` is a control in the
posts tool's row (`/tools`' tree-of-tools rule: an action is a button beside
its tool's icon), shown while the posts tool is open on the **map** view with
no card page up. That last gate is `/recentre`'s, for `/recentre`'s reason: a
control that changes what the ground looks like has nothing to say over a grid
of tiles. Tapping it opens a level of its own, whose row shows ‹ and the same
button lit; ‹ climbs back to posts through `/one-level`, which needs nothing
from this node because `region` is deliberately kept out of `tools_list` —
being absent from the registry is the whole definition of a nested tool.

**The page borrows the card's ground, and opts out of the card's pencil.**
The pills stand on a `.card-page`, which is where the dark rounded ground, the
safe-area insets, the scroll and the depth order all already live — restating
those here would be five nodes' worth of numbers waiting to drift. The one
thing that comes with the class and does not belong is `/editing/toolbar`'s
edit control, which is drawn for whatever `feature_Editing.page()` answers.
So this node wraps that function to answer nothing for a `.region-page`, and
the toolbar's own `apply()` takes the button away on the next paint. That is
`/doors`' idiom for the invite page, one node along, and it was found the same
way: a pencil in the row on the rig, wearing this tool's colour, with nothing
under it to edit.

**The pills are drawn on the page, and that is not a workaround.** `render` is
compiled to wasm as well as to the server, so it cannot read a file; the names
and codes live in `/boundaries`' geojson, which is a file on the page. So the
Rust half emits an empty container and the page half fills it from the parsed
collection `/boundaries` already holds. The result is that Sevenoaks is named
in neither half — `/boundaries`' rule that the file is the seam, kept one
level out. A reload straight into the region page has no map behind it and so
no parsed file; this node then fetches `/boundaries`' own `FILE` into a slot of
its own, never into a sibling's.

**The choice travels with the person.** `region` is a user-scoped var holding
an ONS code, empty for the whole constituency — which ward you are working is
a fact about you, as `/current-project`'s project is, and it should follow you
from the phone to the laptop. It carries **no `js:` column**: a bridged key is
republished at `/payload`'s older link, so a page reading one after a write
made from this node — newer than `/payload` — would paint a frame of the
previous region (`misses.md`, "navigation from the wrong side"). The page
learns the code from a `#misoRegion` marker this node's own `render` writes,
which is by construction never a turn behind.

**Nothing new to credit.** Stadia Outdoors is the same house, the same
OpenMapTiles and the same OpenStreetMap as the ground already on the map, and
`MISO_TILE_ATTRIBUTION` already names all three; `/quiet-credits` shows that
line. A `MISO_OUTDOORS_URL` pointed at some other renderer would need a credit
of its own, and that is named below as parked rather than pretended.

## hostile cases

- **A region code that is not in the file** (a ward renamed out of the data, a
  var restored from an older world). `featureFor` finds no match and answers
  the constituency, so the map has a ground and the pill field shows the whole
  patch as chosen. No blank map, no error.
- **The outdoors squares are unreachable** — no key anywhere, Stadia over its
  quota, no signal. The route answers 404 per square and caches nothing (the
  PNG magic bytes are checked first, so a quota message can never be kept for
  a week as a piece of map); Leaflet leaves the tile transparent and the
  everyday ground below shows through. The region is simply not in daylight;
  the lines, the pins and every gesture are untouched.
- **`/boundaries` unticked.** This node is its child, so the linker excludes it
  with its parent: no route, no button, no var, no page. There is nothing to
  guard because there is nothing composed.
- **`/map` unticked.** `feature_Map` is undefined, `ensure()` returns at its
  first line, and the control's gate (`browse_view_read() == "map"`) can never
  be true because there is no map view to be in.
- **The file has not loaded when the first paint runs.** No feature, so `off()`
  — the layer is not made and no metered square is asked for — and `load()` is
  started. The next paint, or the next map gesture, finds the file and cuts.
- **A ward with a hole, or with two separate parts.** Every ring becomes a
  subpath and the even-odd rule does the rest. A `MultiPolygon` is walked part
  by part; a geometry that is neither shape yields no rings, no bounding box
  and no layer at all — checked *before* the layer is made, because Leaflet
  reads a falsy `bounds` as "every tile is valid" and a layer made first and
  removed a moment later would have asked Stadia for a screenful of the world
  on the way past. Today's file
  carries **neither** shape — every one of its twenty-seven features is a
  single-ring `Polygon`, checked on the rig — so this road is proven against a
  planted feature instead: a two-part `MultiPolygon` whose first part has a
  hole gives three rings, three closed subpaths, and a bounding box spanning
  both parts. The check is worth keeping because the file is the seam: a
  regenerated or a less-simplified extract may carry both.
- **Zoom below 9.** The mask applies, by the decision stated above. The
  outline is one or two hundred points inside a square or two of tiles.
- **A zoom animation in flight.** The clip scales with the pane it is on, so
  the cut stays on the same ground it was cut from; `zoomend` rewrites it for
  the new zoom. There is no frame in which the region is drawn in the wrong
  place, only frames in which it is drawn at the wrong scale — the same frames
  in which the tiles themselves are.
- **A drag.** The key is unchanged, so `cut()` returns before it builds
  anything. This runs on every `moveend` for the life of the page.
- **Choosing the region that is already chosen.** The write is the same value,
  the key is unchanged, and the pill field's html is identical, so the repaint
  replaces nothing.
- **The region page reloaded into with no map behind it.** The map is not
  drawn (posts renders nothing while another tool is open), `#regionPills`
  exists, the file is fetched by this node, and the pills arrive. ‹ returns to
  posts and the map mounts as it always does.
- **A path traversal in the tile route** — `tiles/outdoors/../../etc/passwd`,
  `tiles/outdoors/a/b/c.png`, `tiles/outdoors/99/0/0.png`, a fourth segment:
  every character of every segment must be a digit and the tile must exist at
  its zoom, so the parser answers the empty vector and the route answers 404
  before any path is joined to any directory.
- **`/editing` unticked.** The `feature_Editing.page` wrapper is
  typeof-guarded; there is no pencil to opt out of.
- **This node unticked.** No route, no pane, no second ground, no button, no
  wrapper on `feature_Editing.page`; the map is one ground again and
  `/outlined`'s black lines are still black.

## parked, and named

- A credit of its own for a `MISO_OUTDOORS_URL` pointing outside the Stadia
  house. Today the two grounds share one attribution line because they share
  one source; a `tiles/outdoors/attribution` route is the shape if that stops
  being true.
- Stocking the outdoors squares for offline, as `/stocked` stocks the everyday
  ground. `/stocked` walks one plan for one ground; a second ground is a second
  plan and a second key, and it is a node, not an edit.
- A region per project rather than per person — `/stocked` names the same next
  reading for the area it pre-fetches, and both land in the same place: a
  project card that carries a boundary.
- The outdoors squares are never pruned, exactly as the everyday ones are not.

## glossary

- **region**: the part of the patch you are working — the whole constituency,
  or one ward — drawn on the outdoors ground.
- **outdoors ground**: Stadia's Outdoors style, a walking map's cartography,
  proxied and cached by us under `tiles/outdoors/`.
- **mask**: the cut that makes the second ground stop exactly at the region's
  boundary rather than at a tile edge.

## code description

`region.rs` — `feature_Region`. `route` claims `tiles/outdoors/` and hands
everything else to `existing.route`. `region_coords(path)` is the parser and
the whole of the security argument: digits only, three of them, the tile
existing at its zoom, or the empty vector. `region_serve(path)` is
disk-then-upstream — read the cache file, else `region_fetch`, check
`region_is_png`, create the directory, write, serve — and `region_response`
sets `image/png` and a week's `max-age`. `region_dir()` reads
`MISO_CONTEXT_DIR` itself. `region_source()` is the three-place search
described above, with `region_from_ground()` doing the style swap and
`region_stadia_url()` holding the template.

`region_read()` and `region_write(code)` are the var's two doors, through
`with_context` and `edit_context`; the write closure clones because it runs
twice.

`tool_controls` adds `region_button(lit)` before `ctx_undo` — its own copy of
the inserter, so the node stands whichever siblings are ticked — when the
posts tool is open on the map with no card page, and again, lit, while the
region level itself is open. `update` answers `region_pick:<code>`. `render`
appends the `#misoRegion` marker on every paint and, while the region level is
open, an empty `#regionPills` container on a `.card-page`. `region_svg()` is
the drawn glyph.

`region.js` — `feature_Region`, installed on `/loop`'s `paint` at load, after
`/map`'s own wrapper so the map is mounted by the time it runs. `paint()` is
`fill()` then `ensure()`. Two more load-time blocks: one wraps
`feature_Editing.page` to answer nothing for a `.region-page`, and one puts
this tool's long-press line into `/tool-words`' `TOOLS` table — both
typeof-guarded, both arriving and leaving with the node.

`ensure()` makes the pane once (z-index 250, no pointer events, hidden until
it is cut), then makes the bounded tile layer once a region is known and calls
`cut()`. `off()` takes the layer off the map and hides the pane.

`cut(f)` is the mask: it returns at once when zoom, pixel origin and region
are unchanged, else builds the outline and sets it. `outline(f)` writes every
ring of `rings(f)` as one closed subpath in `latLngToLayerPoint` space.
`rings(f)` flattens a `Polygon` or a `MultiPolygon` to a list of rings.
`boundsOf(f)` is the box the tile layer is limited to. `svgCut(pane, d)` is
the second road: one hidden `<svg>` holding one `clipPath` in user space,
made once and repointed.

`chosen()` reads the `#misoRegion` marker. `file()` prefers `/boundaries`'
parsed collection and falls back to this node's own; `load()` fills that
fallback. `fill()` draws the pills — the constituency first on its own line,
then the wards alphabetically — and returns without touching the DOM when the
html is unchanged.

`region.css` styles the field and the pill: full-round on the pill ground,
`#9db7d8` for chosen, the whole patch on a line of its own.

`region.vars` declares `region: String` (user, last-write, own), empty for the
whole constituency, with no `js:` column and the reason why.
