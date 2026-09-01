# light-basemap
*the basemap is fieldnote's: light paper, keyless, clean*

> (transcripts/2026-09-01-saturday.md#p6)
> if you look around you should find the "fieldnote" project (it was a standalone project served from the mac mini) - use the same map settings it used

## user

The map's ground is a light street map — the one fieldnote drew — instead of
the dark one. Streets, names and buildings read the way a paper map does; the
ward lines and names sit on it in darker ink; the pins are unchanged. No
watermark anywhere.

## spec

Fieldnote's map (via the muon `/map` component it borrowed,
`devnoob/core/fm/muon/ui/map/imp/map.js`) drew CARTO's voyager raster tiles,
keyless: `https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png`.
Between fieldnote's day and this one CARTO began stamping **every** keyless
tile "API KEY REQUIRED" — the dark default and voyager alike, verified by
fetching both on 2026-09-01. So the literal setting is broken at the source,
and this node ships the nearest working equivalent: **plain OSM**
(`https://tile.openstreetmap.org/{z}/{x}/{y}.png`), the same light OSM
cartography voyager restyles, keyless and clean. `MISO_TILE_URL` still
overrides everything, so a CARTO key restores voyager exactly —
`https://a.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}@2x.png?api_key=…`
— without touching the tree.

**The server half is one redefinition.** `tiles_default_url` now answers the
OSM url. `/tiles`' attribution helper already answers plain
"© OpenStreetMap contributors" for a non-CARTO source, and its cache, agent
string and coordinate checks are untouched.

**The page half re-inks the furniture for a light ground.** The dark basemap
had grey lines and a dark halo; on paper they vanish. This node, composed
after `/boundaries` (its prompt is newer), replaces
`feature_Boundaries.styleOf` by property replacement at load — the same idiom
`/boundaries` used on `/map`'s sync — with darker strokes, and its CSS
restates the ward label (ink on paper, light halo) and the map host's
loading ground. The pins already carry their own dark ring and stand fine on
either ground.

**The deploy note.** `/tiles` caches squares on disk under
`MISO_CONTEXT_DIR/tiles`; a basemap change makes every cached square stale.
The cache is a cache — delete `~/.miso-context/tiles` after the deploy that
ships this and the next fetch refills it light.

## hostile cases

- **This node unticked.** `tiles_default_url` is `/tiles`' own again (dark
  CARTO), the style replacement never loads, the dark inks return. Cached
  light tiles would then be stale the other way — same cure, clear the cache.
- **`/boundaries` unticked.** The typeof guard finds no `feature_Boundaries`
  and the block does nothing; the basemap is still light.
- **OSM unreachable.** Identical to the dark source failing: cached squares
  serve, missing ones 404, the map draws its ground colour.

## glossary

(no new terms)

## code description

`light-basemap.rs` redefines `tiles_default_url()` with the plain-OSM url.

`light-basemap.js` replaces `feature_Boundaries.styleOf` at load, guarded by
`typeof`, keeping the two-tone rule (dashed constituency a step darker than
the wards) in light-ground inks.

`light-basemap.css` restates the ward label for paper — ink text, light halo
— and gives `#misoMap` a paper loading ground in place of the dark one.
