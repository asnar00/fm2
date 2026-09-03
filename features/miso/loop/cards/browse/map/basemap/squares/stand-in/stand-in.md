# stand-in
*a missing map square stands in with the one above it, scaled — a blurry map beats a dark hole when the signal drops*

> (transcripts/2026-09-02-self-check.md#p49)
> ok do the fallback first, then the pre-load

> (transcripts/2026-09-02-self-check.md#p47b, the question this answers)
> a note about caching: if we lose connection to the server while wandering about, does the app use locally cached tiles? Is there some way of pre-loading tiles to the local cache so in case of loss of signal, we still get at least a low-res version?

## user

Your phone loses signal on the people map. Wherever you look at a square the
phone has never fetched, the map shows the square above it, stretched to fit —
blurry streets, not a dark hole — at any zoom the phone has ever seen the area
at. Pins stand on it as before. With signal, nothing changes.

## spec

The service worker keeps every square the phone ever fetched (`/fresh` is
network-first and every success refreshes the offline copy), so a phone
offline shows the squares it has looked at and `/map-ground`'s dark grey for
the rest. Leaflet's tile layer has no fallback of its own: a square that
errors is left blank. This node gives it one.

**When a square errors, ask for its parent.** Every square at zoom `z` lies
inside exactly one square at `z-1`, and that one inside one at `z-2`. When
the image for a square fails to load — a 404 from `/tiles`, the service
worker answering a cache miss offline, anything the `img` reports as an error
— the layer re-points the same `img` at the square one level up, and if that
fails too, one more, up to a **reach** of three levels (`REACH`). Three is the
number that turns "I looked at the district zoomed out" into "the district is
drawn at street zoom": a phone that has seen zoom 13 draws zoom 14, 15 and 16
from it; at 17 the hole returns. Beyond three the stand-in is sixteen squares
wide and reads as a colour, not a map.

**Cropped and scaled to fit the quadrant, inside a frame.** A tile of this
layer is a frame Leaflet positions — a 256 px `div` with its overflow
hidden — and the picture is an `img` inside it. A stand-in from `n` levels
up is that `img` drawn at `2^n` times the tile size and shifted so the
missing square's own quadrant lands in the frame; the frame crops the rest,
so a stand-in never covers a neighbour that did load. The frame is not
decoration: Leaflet's own tile *is* the `img`, and a scaled `img` carrying
the tile's `transform` with any clip on it (`clip-path`, the legacy `clip`,
or a `background-image` in its place) is drawn coarse and cut at its edges
— a visible line along every tile boundary, measured on Chrome at DPR 2 and
3 (`scratchpad/standin-rig/seam.py`); the same picture inside a frame that
carries the transform is smooth and seamless (boundary difference 0.3, the
same as anywhere else). The frame answers `src` and `complete` for its
`img`, so Leaflet's abort-on-zoom and prune paths, written for an `img`,
keep working. A stand-in is a `.leaflet-tile-loaded` tile like any other:
no border, no dimming, no mark that it is a stand-in (the taste note:
absence). What it shows is the parent zoom's cartography — at zoom 13
Stadia draws a zoo as a block — which is what a stand-in is.

**The ground tag rides on every stand-in.** The parent's url is built from
the layer's own template, `this._url` — the string `/fresh-tiles` stamped
`?g=N` onto with `setUrl` — so a stand-in is asked for under the current
ground's name and a stand-in from the old ground can never come back from the
cache. `getTileUrl` is overridden for stand-in coordinates only, because
Leaflet's own reads the zoom from the map rather than the coordinates.

**Swapped in at load, not edited in.** `/map`'s `mount()` calls
`L.tileLayer(url, options)`; this node wraps `feature_Map.mount` — property
replacement at load, `/quiet-credits`' idiom on the same function — and, for
the duration of that one call, makes `L.tileLayer` return this node's
`L.TileLayer` subclass, restoring the factory in a `finally`. `map.js` is not
edited; `keepBuffer: 1` and `updateWhenIdle` pass through unchanged.
`/fresh-tiles`' `instanceof L.TileLayer` walk still finds the layer, because
a subclass is one.

**Written rather than vendored.** The MIT `leaflet.tilelayer.fallback`
plugin does this job, and the choice was between vendoring it under
`assets/` and writing the ~60 lines here. Written, for three reasons: the
url of a stand-in must come from the layer's live template so the ground tag
is carried, and that path has to be read and trusted either way; a stand-in
must be cropped to its own square rather than left to overlap its
neighbours; and the re-entry cases — a tile pruned mid-stand-in, a zoom
change aborting the load — needed guards this node could name and test.
Sixty lines we wrote are cheaper to trust than a hundred we read.

**Parked, named.** Pre-loading the area (`/stocked`, briefed in parallel):
it fills the cache this node reads from and extends nothing here. A low-res
"always" mode. "Show me which squares I have" is engineer-level and belongs
behind `/engineer`'s gear. A different reach per zoom extends `REACH`.

## hostile cases

- **All zooms served** (signal, or everything cached): no square errors, no
  stand-in is asked for, the map is exactly as before. Proven: zero parent
  requests with the proxy serving every zoom.
- **Offline entirely, nothing cached at any zoom**: every level up errors
  too; after the reach the original error path runs, the square stays the
  ground colour, Leaflet fires `tileerror` as it always did. Nothing throws;
  the pins still open cards (that is the wasm, not the network).
- **Four levels up** (zoom 17 over a phone that has only seen 13): the hole.
  The reach is a constant, stated above.
- **A tile pruned while its stand-in is loading** (the map was dragged
  on): Leaflet blanks the `img` to its empty image and removes it before
  the error can arrive; the handler sees the blank src, or no parent node,
  and hands the error to Leaflet's own path, which finds no tile under that
  key and does nothing. No request is made for a square nobody will see.
- **A zoom change mid-stand-in**: `_abortLoading` blanks and removes the
  tiles of the old zoom the same way; same guard.
- **A stand-in at zoom 13 when the map is at 15, then `/fresh-tiles` bumps
  the tag**: `setUrl` redraws every tile from scratch under the new tag;
  the stand-ins are asked for again with it.
- **`/fresh-tiles` unticked**: the template has no tag; stand-ins have none
  either — the tag is only ever a cache-buster.
- **`/map` unticked**: the typeof guard finds nothing; no-op.
- **This node unticked**: `L.tileLayer` is never swapped; a missing square
  is the ground, as today.
- **Leaflet missing** (`L` undefined): the subclass is never made; the
  wrapper calls through to `/map`'s mount, which returns false as before.

## glossary

- **stand-in**: a square drawn from its parent square (one to three
  levels up), cropped and scaled to fit the place of the square that would
  not load.
- **reach**: how many levels up a stand-in may be taken from — three.

## code description

`stand-in.js` wraps `feature_Map.mount` at load (typeof-guarded): for the
duration of the original call it replaces `L.tileLayer` with a factory
returning `feature_StandIn.layer()`, restoring it in a `finally`.

`feature_StandIn.layer()` makes the `L.TileLayer` subclass once. It
overrides `createTile` to return a frame — an overflow-hidden `div` holding
the `img`, with `src` and `complete` accessors forwarding to it, the load
and error listeners bound as Leaflet binds them, and the square's
coordinates remembered on the frame; `getTileUrl` to build a parent's url
from `this._url` when the coordinates are marked `standIn` (Leaflet's own
would take the zoom from the map); `_tileOnError` to climb: while the frame
is still on the map, its `img` still holds a real src, and it is fewer than
`REACH` levels up, it computes the parent coordinates, dresses the `img` and
re-points its src, otherwise it hands the error to Leaflet's own
`_tileOnError`; and `_removeTile` to blank the `img` before Leaflet removes
the frame, so a request in flight for a pruned square is dropped as it
would be for a bare `img`.

`dress(img, size, coords, up)` sets the `img` to `2^up` tile sizes and the
negative `left`/`top` that bring the square's quadrant into the frame.
`count` totals the stand-in requests made, for the rig's eyes.

The rig (`scratchpad/standin-rig/`, 2026-09-02) proved it against a tile
stub serving the mini's live cache up to a chosen zoom: at zoom 14, 15 and
16 every square drawn from zoom 13 under `g=3`, at 17 the ground, with
every zoom served no stand-in asked for, offline the ground and the pin
still opening its card, and a fast zoom-and-pan mid-load leaving no orphan.
A pixel reconstruction from the source PNGs matched the screen to within
0.2 levels per tile, neighbours from different parents included. The
tree's step file is `tests/sim/stand-in.json`, which fails the squares above
zoom 13 in the page so it runs against any server.
