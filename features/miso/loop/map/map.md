# map
*a map of where you are, with you on it*

> (asks#1786895599674)
> NEW ASK [proposed] … :: 'new tool: map, showing me my current surroundings'
> *(a field ask, filed from the phone on 2026-08-16, miso build 201)*

## user

Tap 🗺 and miso finds you and draws the streets around you, with you at the
centre. The faint disc is how sure your phone is about the fix — a wide
disc means "somewhere around here". It keeps up as you move. Tap ⟳ to
start the fix again.

## spec

A slippy map of the user's surroundings: OpenStreetMap raster tiles, the
standard Web Mercator projection, the fix centred, the accuracy drawn to
the map's own scale, and the zoom chosen from the accuracy so the picture
never implies more precision than the phone has (a 15m fix opens at z18, a
300m fix at z15).

**Tiles come through miso, not from the device.** The device asks its own
server for `tiles/<z>/<x>/<y>.png`; the server serves it from disk, or
fetches it from OpenStreetMap once and keeps it. Two things fall out of
that: the tile service never sees a user's coordinates, only the mini's;
and a place you have looked at is already on your own server the next
time, so the same ground redraws with no upstream request at all.

**The correction this node was rebuilt around.** The first cut shipped no
map. It reasoned from doctrine — imagery means a third-party dependency,
miso doesn't take those — and delivered a position readout with distance
rings instead. Ash: *"No, the user actually asked for a map - that's what
we should deliver… The doctrine is never as important as what the user
requested"*, and *"Doctrine compliance is eventual, not mandatory."* The
principle is now in `agents.md` because it outranks this node. What is
worth keeping from the wrong version is that the doctrine-shaped concern
was real — and answering it took a proxy route and a cache, roughly an
hour, not a refusal. **Ship the ask; converge on the doctrine.**

**Honest limits, named rather than hidden:** the view does not pan or zoom
by hand (it follows the fix); tiles are not wrapped at the antimeridian;
and a place never visited will not draw with the mini unreachable, since
only the tiles already fetched are ours. Offline-first for *revisited*
ground, not yet for new ground — vendoring an area at build time, the
`fetch_stt.py` pattern, is the rung that would close it.

**Privacy.** The fix stays in loop state on the device: no message, no
var, no route carries it. The mini learns which tiles were asked for,
which is the cost of not asking OpenStreetMap directly.

**Attribution.** OpenStreetMap is credited in the readout, as its licence
requires, and the fetch identifies itself honestly in its User-Agent.

## glossary

- **fix**: one position reading — latitude, longitude, and the radius the
  device believes it accurate to.
- **tile**: one 256×256 square of map at one zoom level, addressed by
  `z/x/y` in the standard slippy-map scheme.

## code description

`map.rs` server half: `route` claims `tiles/<z>/<x>/<y>.png`, cookie-gated
like other data routes. `z`, `x` and `y` are parsed as integers and
range-checked against the zoom's tile count, and the cache path is rebuilt
from the parsed integers — never from request text, so there is nothing to
traverse with. `tile_response` serves the cached file if present,
otherwise shells out to `curl` (the `/vonage` idiom: TLS is curl's
problem), stores what comes back, and serves it with a week's cache
header; anything under 100 bytes is treated as a failure rather than
cached as a broken tile.

`map.rs` client half: `tools_list` registers `{map, 🗺}`. `update` claims
`Located` (store the fix, clear any error), `LocateFailed` (store the
reason) and the `map_again` click (drop both). `render` appends `map_view`
when map is the open tool; `tool_controls` adds ⟳.

`map_view` renders one of three honest states — the error and what it
means, "finding you…" before a first fix, or the map. For the map it
computes the fractional tile coordinates (`map_tile_x`/`map_tile_y`, the
standard projection), lays a 5×5 grid of tiles positioned so the fix falls
exactly at the centre of the field, and sizes the accuracy disc from
`map_mpp` — metres per pixel at this latitude and zoom — so the disc and
the streets share one scale.

`map.js` is the hardware half, following `/dictate`'s pattern: state edges
drive effects, results return as events. It watches `open_tool`, starting
`watchPosition` on the rising edge and clearing it on the falling one, and
reports readings as `Located` and failures as `LocateFailed` —
distinguishing a refusal from an unavailable sensor, since the two ask
different things of the reader. Replay-guarded.

`map.css` fills the display surface with the tile field, dims the tiles
slightly so they sit on the dark shell without reading as a negative, and
draws the accuracy disc and the centre dot above them.
