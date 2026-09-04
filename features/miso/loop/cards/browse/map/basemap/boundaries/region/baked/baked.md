# baked
*the boundary is drawn into the squares themselves, so the region zooms with the map instead of catching up at the end*

> (transcripts/2026-09-04-field-walk.md#p36)
> checking out the constituency boundary render. It's good, but has one major
> flaw - when you zoom in and out, the light region stays fixed while zooming
> and then pings into place, which looks messy - I'd like it to zoom exactly
> in sync with the background. So we need to cache multi-level tiles that
> already include the boundaries

## user

Pinch the map and the daylight region grows and shrinks with the streets under
it, edge and all — no lag, no snap at the end of the gesture. Everything else
is as it was.

## spec

`/region` cut its second ground with a `clip-path` on a pane, rebuilt on
`zoomend`. Its spec said the cut "travels and scales with the very tiles it is
cutting", and **that was wrong** — the measurement in *evidence* below says so
in three decimal places, and this node exists because it was wrong.

Leaflet does not scale one element for a zoom. It puts a class on the map pane
and lets every animated child transform *itself*: each grid layer's current
level container, and the SVG renderer's root. A pane created with
`map.createPane` is not one of those children, so its transform stays at
1.000 for the whole animation while the level containers inside it run
0.63 → 0.77 → 0.90 → 1.00. A `clip-path` written on the pane is therefore
pinned to the screen at the *target* zoom's geometry while the tiles it cuts
are still growing underneath it — which is exactly "the light region stays
fixed while zooming and then pings into place". Measured: on the first frame
of a slowed zoom the cut region is already at 87% of its final area while the
map is 63% of the way there.

There is no version of that road that does not snap. Recomputing the outline
every animation frame would mean re-projecting a few hundred points at 60 Hz
*and* guessing at the interpolation Leaflet is running. The right answer is the
one ash named: stop drawing the boundary on the client and draw it into the
squares.

**One layer is what makes it synchronous.** Every `L.GridLayer` transforms its
own level container on `zoomanim`, by the same `getZoomScale` for every layer
on the map — so two tile layers are in step by construction, and a tile layer
and a pane are not. The region is now carried by a tile layer, so it is scaled
by the same number as the ground at the same moment; the rig reads both
transforms frame by frame and they agree to three decimals. Two grounds and a
mask were three things that had to agree; a square with the boundary already in
it is one thing, and one thing cannot fall out of step with itself.

**The route.** `GET tiles/region/<code>/{z}/{x}/{y}.png` answers the everyday
ground square with the Outdoors square drawn over it inside the region's
polygon. The code is an ONS code from `/boundaries`' file, letters and digits
only and at most twenty of them — the same argument `/tiles` makes for its
three numbers, one segment further along: a name made of `[A-Za-z0-9]` cannot
leave the directory it is joined to.

**The mask is ×4 in y and exact in x.** For each output row, four sub-scanlines
are crossed against every edge of every ring, the crossings sorted, and the
even-odd spans added into a row accumulator as *exact* horizontal overlap
rather than a subpixel count — which is better than the ×4×4 the brief asked
for and costs less, because the x direction never needed sampling. The
accumulator becomes one byte of alpha per pixel. Holes and detached parts need
no special case: every ring of every part is an edge set and even-odd does the
rest, the same rule `/region`'s `clip-path` used.

**Three cases, and only one of them composites.** A square whose tile box does
not meet the polygon's own box is the ground square, returned as it stands. A
square whose mask comes out wholly zero is the same, and wholly full is the
Outdoors square. Only a square the boundary actually crosses is composited, and
only those are written to the bake cache — a straddling square is a handful per
zoom level along the edge of a region, and the interior is served from the two
caches that already exist. That is the brief's rule and it is also what keeps
the disk from holding a third copy of every square in Kent.

**The cache is keyed by what could change the picture.** The path is
`$MISO_CONTEXT_DIR/tiles-baked/<stamp>/<code>/{z}/{x}/{y}.png`, and `<stamp>`
is a 64-bit FNV-1a over three things: the bytes of `site/map/boundaries.geojson`,
the everyday ground's url **with its query removed**, and the Outdoors url with
its query removed. A new boundary file re-bakes; a new basemap style re-bakes;
rotating the Stadia key does not, because the key lives in the query and a key
is not a picture. FNV and not a hash from `sha2`: this is a cache key, not a
signature, and nothing about it has to resist anyone.

**Write-once, and last writer wins.** Two requests for the same missing square
both bake it and both write the same bytes to the same path — `/tiles`' own
argument, and true here for the same reason: the inputs are files on disk and
the composite is deterministic. A lock would buy nothing.

**The ground square is read from `/tiles`' own cache, and written back into
it.** `/region` deliberately calls nothing of `/tiles`' so that a node in the
browse tree does not tie its tick to a node in `serve`, and this node keeps
that: it reads `$MISO_CONTEXT_DIR/tiles/{z}/{x}/{y}.png` by path, and on a
miss fetches `MISO_TILE_URL` itself and writes the bytes *to that same path*.
Sharing the file rather than keeping a second copy is not tidiness — the
everyday ground is Stadia too, and a private cache would have meant fetching
every square of Sevenoaks twice from a metered account. The bytes, the naming
and the PNG check are identical to `/tiles`', so either writer produces a file
the other reads.

**The client draws one layer and stops cutting.** `/region`'s `URL` is set to
this node's template before its `ensure()` builds the layer, so the layer is
*created* pointing at the baked route and never asks for a raw Outdoors square
it would have to hide; a later change of region re-points it with `setUrl`.
`/region`'s `cut()` is replaced outright by `dress()`, which clears the
`clip-path` and shows the pane once the layer is pointing at the right place.

**The clip-path road goes; it is not kept as a fallback.** Two reasons. There
is nothing left to clip — a baked square carries its own boundary, so a cut on
top of it would only be able to make the region *smaller* than it already is.
And a fallback that snaps is the bug: the whole ask is that nothing lags the
gesture, and a clip that appears whenever a square is late would put the
snapping back exactly when the network is worst. What happens when a baked
square does not arrive is better than a clip anyway: the square is absent, the
everyday ground layer beneath shows through, and the map is the map — the rule
`/stand-in` set for a missing square, reached here for free because the baked
layer is bounded to the region and the ground layer is still underneath it.

**The two layers still line up.** The baked layer keeps `/region`'s bounding
box, so it covers the region and a little around it, and where a baked square
lies outside the polygon its pixels are the ground square's own bytes,
unchanged by the composite. It is drawn over the ground layer in `/region`'s
pane at z-index 250, so the seam at the edge of the box is between two copies
of the same picture. The boundary *line* is untouched: it stays `/boundaries`'
vector path in the overlay pane at 400, above both.

**What it costs the build.** One crate, `png 0.17` with default features off —
`flate2`/`miniz_oxide`, `fdeflate`, `crc32fast`, `bitflags`, all pure Rust and
all wasm-safe, which they have to be because fmlink merges every node's
`deps.toml` into the wasm crate as well as the native one. Not `image`: that
would have brought jpeg, gif, webp, tiff and rayon along for a job that is one
format in and the same format out. The measured cost is in *evidence*.

## what the rig measured

*On this box — the mini, an M2 with little free memory — against a release
build, which is what ships. The harness first timed `subprocess.run` around
`curl` and made every number eight times worse than it is; spawning a process
costs 8.5 ms here and the figures below are curl's own `time_total`.*

**The snap, and its cause.** A zoom from 11 to 12 with Leaflet's transition
stretched to three seconds, five frames across it, the lit area counted by
luminance and every animated element's transform read at each frame:

| frame | ground layer | region layer | the pane | lit px, baked | lit px, clip |
|-------|-------------|--------------|----------|---------------|--------------|
| 0     | 1.000       | 1.000        | 1.000    | 7,636         | 7,636        |
| 1     | 0.631       | 0.631        | 1.000    | 10,164        | 21,354       |
| 2     | 0.764       | 0.764        | 1.000    | 13,989        | 22,860       |
| 3     | 0.897       | 0.897        | 1.000    | 19,454        | 23,997       |
| 4     | 1.000       | 1.000        | 1.000    | 24,510        | 24,517       |

Both roads finish in the same place, so both draw the same region. In flight
they are nothing alike: baked grows with the ground, and the clip is at 87% of
its final area on the first frame while the map is 63% of the way there. The
pane's own transform is **1.000 at every frame of both** — that is the whole
diagnosis, and the sentence in `/region`'s spec that said otherwise.

**The edge.** Same stretch of boundary, same zoom 16, same viewport at device
pixel ratio 3, same window as `/region`'s own crispness proof: the transition
across the cut is a median of **1 device pixel** and a maximum of 3 — the same
numbers the clip-path road gave. Across the boundary the two profiles are
`0 0 0 21 143 213` (clip) against `0 0 1 44 172 213` (baked): one device pixel
more ramp, and all of it inside the black boundary stroke drawn on top.

At tile resolution the composite is exact: of 65,536 pixels of a straddling
square, every one is either the ground square's own bytes, the Outdoors
square's own bytes, or a blend of the two — **zero** are anything else — with a
blended run of 1 to 2 pixels per row, which is the anti-aliasing.

**The cost per square,** release, warm source caches:

| | median | max |
|---|---|---|
| a square the boundary crosses, first ask (decode two, rasterise, composite, encode, write) | 2.6–3.1 ms | 3.9 ms |
| the same square, later (the bake cache) | 0.6 ms | 0.8 ms |
| a square wholly inside or outside, first ask | 1.7–3.3 ms | |
| the same square, later (the marker) | 0.6 ms | |
| a plain cached square from `/tiles`, for scale | 0.5 ms | |

A screenful is about fifteen squares, so a region opened cold costs the server
something under 50 ms of work spread across fifteen threads, and nothing at all
after that. **Resident set: 5 MB for the whole server**, unchanged through
every measurement — the rasteriser holds two 256-wide rows and a crossing list,
and nothing 1024 × 1024 is ever allocated.

**What it costs the build.** Release: `client.wasm` 2,257,953 → 2,265,129
bytes (**+7 KB**, +0.3% — the png code is dead weight in the wasm and mostly
stripped); the server binary 3,236,736 → 3,463,296 (**+227 KB**, +7%). Seven
crates on the lockfile's 75: `png`, `flate2`, `miniz_oxide`, `fdeflate`,
`crc32fast`, `adler2`, `bitflags`. A full release link is 1 minute 47 either
way.

**What it costs Stadia.** Nothing, and probably less than before. A baked
square needs both grounds, but the everyday one is read from and written to
`/tiles`' own cache — the same file the map's own ground layer uses — so it is
fetched once whoever asks first. The Outdoors squares are now fetched only for
the squares the boundary crosses and the squares wholly inside it, where
`/region`'s bounded layer fetched every square in the bounding box; the corners
of the box no longer cost an Outdoors square at all.

## hostile cases

- **A square straddling a hole, or two parts of one ward.** Every ring of every
  part contributes edges to the same scanline crossing list and even-odd fills
  the alternation, so a hole is a hole and a detached part is a part. Proven
  against a planted geometry, since the shipped file has neither.
- **The Outdoors source unreachable** (no key, over quota, no signal). The
  Outdoors square is empty, so there is nothing to draw over the ground and the
  ground square is served as the baked square — which matches the ground layer
  beneath it exactly, so the region simply is not in daylight and nothing else
  moves. Not cached: a failure must not become a week-long ghost.
- **The ground square unreachable too.** No bytes at all, 404, and the tile is
  absent; the ground layer beneath draws whatever it has. `/stand-in`'s rule.
- **A code that is not in the file.** The geometry is null, no mask is made,
  404. The page never asks for one — `/region`'s `featureFor` has already
  resolved an unknown code to the constituency before the url is built — so
  this is the route being asked directly.
- **The same code in a different case.** Found on the rig: this box's
  filesystem folds case, so `e14001465` read a square baked for `E14001465`
  and answered 200, while the same url with a cold cache answered 404 because
  the geometry lookup matched exactly. One url must not be two answers. The
  lookup folds case as well now, and the cache directory is upper-cased, so a
  region has one directory and a differently-cased code is an alias rather
  than a coin toss. ONS codes are unique either way.
- **A path that is not three numbers and a code**: `tiles/region/../../etc`,
  `tiles/region/E05005029/a/b/c.png`, `tiles/region/E05005029/99/0/0.png`, a
  segment too many or too few. Every one is a 404 before any path is joined to
  any directory.
- **Zoom below 9**, where `/boundaries` hides its labels. The bake applies, as
  `/region` ruled for the mask: which ground you stand on is not clutter. The
  cost is bounded by the layer's own box — at zoom 8 the constituency is a
  square or two.
- **A square wholly inside or wholly outside.** Served from the Outdoors or the
  ground cache with no composite and nothing written. The mask is still
  rasterised to decide which — a few milliseconds per request rather than per
  square, and the box test skips even that for anything the polygon cannot
  reach. Named below as the one repeated cost.
- **`/region` unticked.** This node is its child; the linker excludes it with
  its parent. Nothing composed, no route, no crate.
- **This node unticked.** `/region`'s `URL` and `cut()` are its own again, the
  raw Outdoors layer returns under a `clip-path`, and the region snaps on zoom
  as it did — which is the behaviour this node exists to remove, so the untick
  is visible and that is the point.
- **A repaint or a gesture with everything already in place.** `dress()` sets a
  url only when it differs and a display only when it differs; `/region`'s own
  key still stops the work it was stopping.

## parked, and named

- Old `<stamp>` directories are never swept. A boundary file change leaves the
  previous bake on disk; it is a cache, and a `--prune` in `tools/` is the
  shape when it matters.
- Pre-baking a region ahead of the finger, as `/stocked` pre-fetches the
  everyday ground. A second ground was already a second plan; a third is a
  third, and it is a node.
- The wholly-inside decision is recomputed per request rather than remembered.
  A zero-byte marker beside the square would remove it; the measurement below
  says whether it is worth the extra file shape.

## glossary

- **baked square**: a map square with the region's boundary already drawn into
  its pixels — the everyday ground outside, the Outdoors ground inside.
- **stamp**: the cache generation, a hash of the boundary file and the two
  basemap styles.

## code description

`baked.rs` — `feature_Baked`. `route` claims `tiles/region/` and hands
everything else to `existing.route`.

`baked_code(path)` and `baked_coords(path)` are the parser, split in two
because the chain parser cannot read a comma-bearing return type: the first
answers the region code or the empty string, the second `[z, x, y]` or the
empty vector. Both must succeed or the route 404s.

`baked_serve(path)` is the whole flow: parse, find the geometry, take the
polygon's box, and if the tile's box does not meet it answer the ground square;
else read the bake cache; else rasterise the mask, take the wholly-out and
wholly-in exits, composite, write and serve.

`baked_geo(code)` reads `site/map/boundaries.geojson` and answers the matching
feature's geometry. `baked_rings(geometry)` flattens a `Polygon` or a
`MultiPolygon` into one `Vec<f64>` of length-prefixed rings — a flat vector
because a vector of vectors is a comma-bearing type. `baked_box(rings)` is the
polygon's lon/lat bounds, four numbers.

`baked_mask(rings, z, x, y)` is the rasteriser: four sub-scanlines per output
row, crossings against every edge, sorted, even-odd spans added as exact
horizontal overlap, one byte of alpha out per pixel. `baked_px` and `baked_py`
are Web Mercator, with the latitude clamped where the projection stops being
finite.

`baked_rgb(bytes)` decodes a PNG of any colour type into 256×256 RGB8 through
the decoder's own expand and strip-16 transformations, or answers empty.
`baked_over(ground, over, mask)` is the composite, one multiply-add per
channel. `baked_png(rgb)` encodes at the fast compression level, because the
bake is on the request's own thread.

`baked_ground(z, x, y)` reads `/tiles`' cache path and, on a miss, fetches
`MISO_TILE_URL` and writes the bytes back into that same path.
`baked_outdoors(z, x, y)` does the same through `/region`'s own `region_dir()`
and `region_fetch()`, which is this node's parent and therefore always
composed with it.

`baked_stamp()` is the FNV-1a cache generation; `baked_strip_query(url)` is
what keeps the Stadia key out of it. `baked_dir()` is the cache root.

`baked.js` — `feature_Baked`. At load it wraps `feature_Region.ensure` to set
`/region`'s `URL` to `template(code)` before the layer is built, and replaces
`feature_Region.cut` with `dress()`, which clears the clip, re-points the layer
when the region has changed, and shows the pane. Both typeof-guarded.
