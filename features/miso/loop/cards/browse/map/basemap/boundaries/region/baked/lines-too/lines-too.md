# lines-too
*the ward lines go into the squares as well, so the map is one layer and nothing is drawn over it*

> (transcripts/2026-09-04-field-walk.md#p92a)
> let's draw the boundary lines between wards into the map tiles rather than
> drawing them separately as an overlay

## user

The ward lines and the constituency's dashed edge are part of the map now, the
way the street names are. Pinch and they move with the streets, because they
*are* the streets' picture. Nothing else changes: the same black wards, the
same grey dashed ring, the same daylight region inside it.

## spec

`/baked` put the region's ground into the squares and left one thing outside
them: `/boundaries`' `L.geoJSON` paths, drawn in the overlay pane over the top.
That was already the last thing on the map that was not a tile, and ash asked
for it to go the same way. This node draws the lines into the squares and takes
the vector layer off the map.

**Every square, not just the region's.** A boundary line crosses squares the
region never touches, so the square outside the region stops being the plain
ground and becomes a baked square too — the ground with the lines on it. That
is what makes the map **one tile layer for the whole view**: `/map`'s own
ground layer is re-pointed at the baked route, unbounded, and `/region`'s
second layer and its pane go away entirely.

**The two seams this needed, and the refactor that opened them.** `/baked`
decided per square whether to composite, and a square the region did not touch
returned the cached source unchanged — which is exactly the square that now
needs a line drawn on it. So `/baked` gains `baked_must` (claim a square the
region alone would have passed through) and `baked_extra` (the last word on the
picture before it is encoded), both the identity in `/baked` itself. Its
behaviour with this node unticked is unchanged, which is what the toggle proof
on this commit shows.

The same refactor stopped `/baked` fetching the Outdoors square for a square
the region does not reach. That was free before, because such a square returned
early; now every square is composited and, without it, a square carrying
nothing but a boundary line would have pulled a metered Outdoors square it has
no use for — across the whole district.

**A stroke is a capsule, and a capsule meets a scanline once.** The mask
rasteriser `/baked` already has fills a polygon by even-odd crossings; a stroke
is the set of points within half a width of a segment, which is a capsule — two
discs and the rectangle between them — and a capsule is convex, so its
intersection with one scanline is a single interval. That interval is the least
and greatest x of whichever of the three parts the line actually crosses, and
it goes into the same row accumulator, at the same four sub-scanlines per row,
with the same exact horizontal overlap. So the ink is anti-aliased by the same
supersample as the fill, and **no 1024 × 1024 buffer is made** — the working set
stays two rows and a segment list, which is what let the bake run in five
megabytes and had to keep doing so.

Segments are bucketed by output row before any of that. At zoom 11 the whole
district is inside one square and there are two thousand segments; without the
buckets every row would test every one of them.

**Width in pixels, not in metres.** A line is 1.2 tile pixels for a ward and
2.0 for the constituency, and a tile pixel is a CSS pixel because Leaflet draws
a 256-pixel square into a 256-CSS-pixel box. So a boundary is the same width on
the screen at every zoom, exactly as the vector layer drew it, rather than
widening as you zoom in the way a road does.

**The dash belongs to the ring, not to the square.** `/light-basemap` draws the
constituency `7 5` dashed. The pattern is applied while the ring is being
walked, from a length accumulated since the ring's first point — so it depends
on the ring and the zoom and never on which square it lands in, and a dash does
not jump at a tile edge.

**The colours are a duplication, and it is a real cost.** `#000000` at 1.2 and
`#4a4a54` at 2.0 dashed are `/outlined`'s and `/light-basemap`'s values,
restated here because those live in JavaScript and this runs on the server.
There is no way round it short of the page telling the server what colour to
use, which would make a tile url a style sheet. What there is instead: the
style string rides in the cache stamp, so changing a width or a colour here
re-bakes every square rather than leaving old ink on the disk — and the two
places have to be changed together. Said plainly because it will otherwise be
found by someone changing one of them.

**Offline had to come with it.** `/stocked` pre-fetches the patch into the
phone's cache so the map still draws in a stairwell. It stocks
`tiles/{z}/{x}/{y}.png`, and the map now asks for `tiles/region/…`, so without
a word from this node the pre-load would have filled the cache with squares
nobody asks for and a canvasser with no signal would have got a blank map —
the one thing `/stocked` exists to prevent. Its `url()` is re-pointed from
here, guarded both ways.

**Below zoom 9 the lines bake too.** `/boundaries` hides its *names* there
because there is no room for a word; it has never hidden its lines, and
`/region` already ruled that the region's ground applies at every zoom. Parity
is the answer: the lines are in the squares at every zoom, and at zoom 8 the
district is a square or two, so the cost is bounded by that.

**What it does not change.** The pins, the reel, the faces and the region's own
daylight ground are all where they were. `/boundaries` still fetches and holds
the file — `/stocked` reads it for the area to pre-load, `/region` reads it for
the polygon and the pill names, and this node reads it on the server for the
segments. Only the drawing of it moves.

## what the rig measured

*WebKit at iPhone 17 Pro size, device pixel ratio 3, headless; the server
figures from a release build on this box, using curl's own `time_total`.*

**The overlay is gone and the map is one layer.** With the node ticked the map
carries **1** tile layer, `tiles/region/E05005029/{z}/{x}/{y}.png?g=3`, and
`.leaflet-overlay-pane path` holds **0** elements; `/boundaries`' line layer is
off the map, `/region`'s own layer is gone and its pane was never made, and
`/stocked` asks for `tiles/region/E05005029/13/4097/2733.png?g=3`. Unticked, the
same page has **2** tile layers, **27** overlay paths, and `/stocked` back on
`tiles/13/4097/2733.png?g=3`.

**The lines are in the squares.** Near-black pixels on the screen, with zero
overlay paths at every one: 15,844 at zoom 11, 8,444 at 13, 1,518 at 15. The
same view with the node unticked and the vector layer drawing gives 14,011,
7,397 and 1,635 — the same lines, from a different place.

**The width is the same width.** Dark runs across the boundary, in device
pixels: median 6 / 9 / 6 baked at zooms 11 / 13 / 15 against 6 / 9 / 8 vector.
The line is not thinner or fatter for having been baked.

**Through a zoom the lines cannot fall behind, because they are the tiles.**
The ground's own transform runs 0.61 → 0.74 → 0.87 → 0.99 → 1.00 across a
slowed zoom and there are no overlay paths at any frame. One thing does change
and it is worth saying: a baked line *thickens* with the square through the
animation and settles back at the end, where the vector line held its width.
That is what a line drawn into a picture does — the streets under it have
always done it — and it is a far smaller thing than the region standing still
and jumping, which is what `/baked` was for.

**The cost per square,** release, warm source caches:

| | cold | warm |
|---|---|---|
| a boundary and no region — the new common case | 3.1 ms | 0.5–0.6 ms |
| the region's own edge, lines and fill | 3.6–3.9 ms | 0.5–0.6 ms |
| neither lines nor region | 2.1 ms | 0.6 ms |
| the whole district and all 2,120 of its segments in one square, zoom 11 | 5.1 ms | 0.5 ms |

**Resident set: 5 MB**, unchanged — the row buckets and the capsule scanline
are what keep the worst square at 5 ms and the memory flat.

**What it costs the build.** Release: `client.wasm` 2,390,571 → 2,395,459
(**+4.9 KB**), the server binary 3,673,792 → 3,690,992 (**+17.2 KB**). No new
crate: `/baked` already brought `png`.

**What it costs Stadia: nothing.** A square carrying only a boundary needs the
everyday ground, which `/tiles`' cache already holds or fetches once for the
map anyway, and the refactor above stops it asking for an Outdoors square it
has no use for. Only CPU and disk.

## hostile cases

- **A square with lines and no region** — the common case, and most of the
  district. `baked_must` claims it, the fill mask is empty so no Outdoors
  square is asked for, the ground is decoded, the ink is drawn on it and the
  result is cached. No extra metered fetch, only the CPU.
- **A square with neither lines nor region.** `baked_must` is false, the mask
  is empty, `/baked`'s marker is written and the plain ground square is served
  from the cache that already holds it. The far side of the district costs
  nothing.
- **The Outdoors source unreachable** on a square the region does reach. The
  ground is served with the lines still drawn on it, and nothing is written —
  a failure must not become a week-long ghost, and the lines are not a failure.
- **`/boundaries`' file will not load.** No segments, no ink, and `/baked`'s
  own behaviour: the route 404s for a region it cannot find, and the page has
  no code to ask with. The map is the ground.
- **The vector layer re-added.** `place()` is `/boundaries`' own beat — draw,
  then every `zoomend` and `moveend` — and the removal stands there, wrapped
  around `/outlined`'s removal of the names, so both survive any path that
  re-adds either.
- **`/boundaries`' `fit()`.** It reads `this.lines.getBounds()`, and a Leaflet
  layer group keeps its children after it is taken off the map, so the map
  still opens on the patch when there is nothing else to open on.
- **`/stocked` unticked**, or no region resolved yet. The wrapper is guarded
  and `/stocked`'s own url stands; with `/stocked` gone there is no pre-load to
  point.
- **`/region`'s pane made by an earlier paint**, before this node's `ensure`
  took over. `off()` removes the layer and hides the pane on the first call.
- **This node unticked.** `/baked`'s two seams are the identity again,
  `/region` builds its own bounded layer in its own pane and `/baked` cuts it,
  and `/boundaries`' vector lines are drawn over the top as before. The cache
  stamp reverts with the style string, so the squares baked without ink are
  found again rather than being re-baked. The proof is on the commit.

## parked, and named

- **`/stocked`'s record key does not name the region.** Stocking follows the
  region that was chosen when it ran; switching region does not restock, so
  offline shows the previous region's daylight. The key is `/stocked`'s and
  changing it is `/stocked`'s node to change.
- **Retina squares.** The ink is rasterised at tile resolution and the device
  upscales it — on a phone at three device pixels per CSS pixel a baked line is
  softer than the vector line it replaces. `@2x` squares would fix it and would
  double what Stadia is asked for; the measurement of how much softer is in
  *evidence*.
- **The disk grows.** `/baked` wrote a square only where the region's edge
  crossed it — a few dozen. A boundary crosses most of the district, so most
  squares inside it are now written, and `/stocked` asks for 1,210 of them per
  ground: call it 35 MB a generation beside the 16 MB of plain ground. It is a
  cache on a Mac mini, and the sweep parked below is the answer if it stops
  being small.
- Old `<stamp>` directories are still never swept, and this node adds a
  generation of its own.

## glossary

- **ink**: the boundary lines as a coverage mask, drawn into a baked square.
- **capsule**: the shape a stroke actually is — every point within half a width
  of a segment.

## code description

`lines-too.rs` — `feature_LinesToo`. `baked_stamp` is redefined to append a
hash of `lines_style()`, so a width or a colour changing re-bakes.

`lines_segs(kind, z, x, y)` reads `/boundaries`' file and answers every segment
of every ring of every feature of that kind, projected into this square's own
pixel space and cut to those whose box can reach it — flat, as `/baked`'s rings
are, because a vector of vectors is a comma-bearing type. `lines_keep` is that
cut; `lines_dashed` splits a segment into the "on" pieces of the dash pattern
from a length accumulated along the ring.

`lines_ink(segs, half)` is the rasteriser: segments bucketed by output row, then
four sub-scanlines per row, `lines_at` for the interval each capsule covers, and
`/baked`'s own `baked_span` to add the exact horizontal overlap. `lines_at`
answers the least and greatest x of the two end discs and the rectangle between
them, for whichever of the three the scanline crosses.

`lines_paint(rgb, ink, r, g, b)` lays one colour over the picture by that
coverage.

`baked_extra` is the seam `/baked` opened: the dashed edge first and the wards
over it, which is the order the vector layer drew them in. `baked_must` is the
other: a square any segment reaches must be composited, tested against the
segments' own boxes rather than the file's, so a square in the middle of a big
ward still costs no disk.

`lines-too.js` — `feature_LinesToo`, three load-time blocks, all typeof-guarded.
`hide()` is wrapped onto `/boundaries`' `place()` and takes the vector layer off
the map. `feature_Region.ensure` is replaced with one that calls `off()` and
`point()`, which re-points `/map`'s own ground layer at the baked route.
`feature_Stocked.url` is wrapped so the pre-load stocks the squares the map will
actually ask for.
