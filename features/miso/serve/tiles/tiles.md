# tiles
*the map's ground, fetched once by us and served off our own disk*

> (transcripts/2026-08-25-accounts.md#p87)
> my hitlist before tara comes in: 1) projects; 2) posts; 3) map view.

*(The server half of the map ask. `/map` draws the view; this draws what it
stands on. Two nodes because they are two capabilities in two places — a
route in `serve` and a view in `browse` — and either is useful with the other
unticked: `/tiles` off leaves a map of pins on grey, `/map` off leaves a tile
proxy nobody asks.)*

## user

Nothing, on purpose. You never see this: it is why the map has streets on it.

## spec

A map needs pictures of the world, and nobody in this tree draws them. The
2026-08-16 map shipped a third party's tiles straight onto the page and was
withdrawn; the lesson recorded in `agents.md` was not *avoid the dependency*
but **own it** — proxy it, cache it, vendor it. This node is the proxying and
the caching.

**One route, one shape.** `GET tiles/{z}/{x}/{y}.png`. The path is parsed to
three numbers and nothing else is accepted: every character must be a digit,
the zoom must be 0–19, and `x` and `y` must exist at that zoom. A path that
fails any of those is a 404 before anything touches the disk, which is also
what makes traversal impossible — a name made only of digits cannot leave the
directory it is joined to.

**Disk first, upstream once.** The tile is read from
`$MISO_CONTEXT_DIR/tiles/{z}/{x}/{y}.png` (default `~/.miso-context/tiles`),
beside the op logs and outside the synced tree, so a deploy never touches it.
On a miss it is fetched with `curl` — TLS is curl's problem, `/vonage`'s
precedent, and the reason this node adds no crate — written to that path, and
served. Every later request for the same tile is a disk read. The log says
which happened, so the second request is visibly not the first.

**What we say we are.** OSM's tile usage policy requires a User-Agent naming
the application and a way to reach its operator; `MISO_TILE_AGENT` sets it and
the default names miso and ash. Nothing prefetches: the device asks for the
tiles it is about to draw and Leaflet asks for one zoom's worth at a time, so
there is no bulk download to police.

**The source is dark because dark is what miso looks like.** The default is
CARTO's `dark_all` basemap — OpenStreetMap data, rendered near-black with grey
streets and dim labels. This is the direct answer to the third of the
withdrawn map's lessons (`notes.md`, 2026-08-16): *choose a source that gives
you what you want; don't hack a filter over one that doesn't.* The standard
OSM raster is bright, pink-roofed and yellow-roaded, and the only way to put
it on this shell is the `brightness()` that got the last map deleted.
`MISO_TILE_URL` points the proxy anywhere — the plain OSM value is
`https://tile.openstreetmap.org/{z}/{x}/{y}.png` — and the credit follows the
source rather than being hardcoded beside it: `GET tiles/attribution` answers
one line of text, which is what the map shows in its corner.
`MISO_TILE_ATTRIBUTION` overrides it for a source this node has not heard of.

**A missing tile is not an error.** Unreachable upstream, a timeout, an HTTP
error, or an answer that is not a PNG (a captive portal's login page, most
often) all produce the same thing: a 404 and nothing written. The map draws
its own dark ground in that square and stays draggable. The PNG magic bytes
are checked before the write for exactly one reason — a cached error page
would be a week-long ghost.

**Closed, not open.** The route is not added to `/public`, so through the
tunnel it is behind the gate like every other data route. Anyone can fetch
tiles from CARTO; nobody should be able to fetch them through us.

## hostile cases

- **`tiles/99/0/0.png`**, `tiles/1/9/9.png`, `tiles/a/b/c.png`,
  `tiles/1/2.png`, `tiles/../../etc/passwd`: 404, no disk touched.
- **Upstream down / no network**: 404 per tile, map renders on its ground.
- **Upstream answers HTML**: not a PNG, so 404 and nothing cached.
- **A zero-length file in the cache** (a write cut short by a kill): read as a
  miss and re-fetched.
- **Two requests for the same missing tile at once**: both fetch, both write
  the same bytes to the same path; last writer wins and the content is
  identical. A lock would buy nothing.
- **The cache directory cannot be created** (read-only disk): the fetch still
  serves the tile; only the caching is lost.
- **`/tiles` unticked**: `tiles/*` falls through to the static route and 404s.
  `/map` mounts, shows its pins on the dark ground, and says so to nobody.

## glossary

- **tile**: one 256px square picture of the world at one zoom, named by
  `{z}/{x}/{y}` in the Web Mercator scheme every raster map shares.
- **basemap**: the drawn world under the pins — streets, names, buildings.

## code description

`tiles.rs` extends the `serve` route chain: `tiles/attribution` answers the
credit line, anything else under `tiles/` goes to `tiles_serve`, and every
other path falls through to `existing.route`.

`tiles_coords(path)` is the parser and the whole of the security argument: it
returns `[z, x, y]` or the empty vector, and only digits survive it. A `Vec`
rather than a tuple because the chain parser cannot read a comma-bearing
return type.

`tiles_serve(path)` is disk-then-upstream: read the cache file, else
`tiles_fetch`, check `tiles_is_png`, create the directory, write, serve.
`tiles_response(bytes)` sets `image/png` and a week's `max-age` — a tile at
`z/x/y` is immutable, so the device and the service worker may both keep it.

`tiles_dir()` reads `MISO_CONTEXT_DIR` itself rather than calling
`/remember`'s `context_dir()`, so this route depends on nothing in the loop.

`tiles_source()`, `tiles_agent()` and `tiles_attribution()` are the three
knobs, each an env var over a default; `tiles_default_attribution()` picks the
CARTO credit or the plain OSM one by looking at the source it is crediting.
