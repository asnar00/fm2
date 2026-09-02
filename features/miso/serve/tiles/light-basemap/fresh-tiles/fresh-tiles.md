# fresh-tiles
*the whole map is the light ground — no cached dark squares at any zoom*

> (transcripts/2026-09-01-saturday.md#p17)
> as far as the map is concerned - when I'm zoomed out it's showing dark tiles, when I zoom in it switches to lighter ones. Let's keep the same lighter map tiles style throughout.

## user

The map is the light paper style at every zoom. No dark squares left over
from before.

## spec

When `/light-basemap` changed the ground, the server's tile cache was
cleared — but every phone's service worker had cached the dark squares it
had already shown, under `tiles/{z}/{x}/{y}.png` urls, and a cache answers
by url. Zoomed-out squares (seen before the change) came back dark; freshly
fetched ones came light. The cure is a new name: this node stamps a ground
tag onto the tile url — `?g=1` — so every square is asked for under a name
no cache has heard, and the old dark entries rot unused. The server's tile
route reads coordinates from the path and ignores the query, verified live.
A future ground change bumps the tag: `g=2` on 2026-09-02, when the ground became Stadia's Alidade Smooth (`MISO_TILE_URL` in the mini's plist; ash chose it from an audition, self-check #p36).

The page half takes `feature_Map.sync` the way `/boundaries` did — property
replacement at load, one more wrapper on the chain — and once, when the map
exists, re-points every tile layer whose url does not yet carry a `g=` tag
via Leaflet's own `setUrl`, which redraws. `/map`'s mount is not edited.

## hostile cases

- **This node unticked.** Urls revert to the bare form; devices that cached
  light squares under `?g=1` refetch bare and get light from the server
  anyway — the tag is only ever a cache-buster, never a behaviour switch.
- **`/map` unticked.** The typeof guard finds nothing; no-op.
- **A repaint before the map mounts.** `sync` runs, the map is absent, the
  wrapper waits for a later paint; `done` is only set after a successful
  re-point.

## glossary

- **ground tag**: the `g=` query value naming which basemap generation a
  tile url belongs to.

## code description

`fresh-tiles.js` wraps `feature_Map.sync` at load (typeof-guarded) and, on
the first paint where the map and `L` exist, walks the map's layers and
`setUrl`s every `L.TileLayer` lacking a `g=` tag to the tagged url, then
stands down.
