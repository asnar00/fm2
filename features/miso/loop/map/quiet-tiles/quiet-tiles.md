# quiet-tiles
*buildings and streets, nothing shouting*

> (asks#1786897201058)
> NEW ASK [proposed] … :: 'I don’t want to see extraneous labels on the map'
> *(a field ask, filed from inside the map tool on 2026-08-16, miso build 205; its proposal read "Use a map that just draws buildings and streets, no shops or other commercial properties")*

## user

The map draws the shape of the place — buildings, roads, paths — and
nothing else. No street names, no shop pins, no brand markers. What you
are looking at is where things are, not what they are called.

## spec

The standard OpenStreetMap raster bakes its labels into the image: street
names, parking symbols, commercial pins. They cannot be filtered out
afterwards, so the fix is a different basemap rather than a different
treatment of the same one.

This node points `/map`'s tile source at **CARTO's `dark_nolabels`**
basemap, built from the same OpenStreetMap data with every label and point
of interest omitted. Buildings and streets survive — checked before
choosing, on a dense tile over Soho, because the ask asks for buildings
specifically and a style that dropped them would satisfy the words and
miss the point.

Dark rather than light because miso is a dark app and a white rectangle
would sit on the page like a hole. That basemap is drawn for dark pages,
so `/map`'s slight dimming would have buried it; this node replaces the
filter with a lift instead — enough to tell a building from the ground
without turning the map into a lightbox.

**The parent gained an extension point rather than a special case.**
`/map` now names the three things a tile style is — where the cache keeps
it (`tile_style`), where it comes from (`tile_url`), and who must be
credited (`tile_credit`) — and derives its own behaviour from them; this
node redefines all three together. The style name is part of the cache
path, so tiles of two styles can never be mistaken for each other, and
switching styles doesn't require clearing anything.

**Attribution.** CARTO's basemaps require crediting both OpenStreetMap and
CARTO; the readout does, and unticking this node restores OpenStreetMap's
credit alone, which is then the truth again.

**Named limit:** the label-free style is the whole point here, but a map
with no names at all is harder to talk about — *"meet me at the corner
of…"* has nothing to read. If names turn out to be wanted back
selectively, that is a style with labels at high zoom only, or our own
labels drawn from data we hold, and it is a different node.

## glossary

- **basemap**: the drawn background of a map — what is rendered into the
  tiles before anything of ours is put on top.

## code description

`quiet-tiles.rs` redefines `/map`'s three tile-source functions: the cache
name becomes `quiet`, the URL points at CARTO's `dark_nolabels` raster, and
the credit names both parties. Nothing else changes — the projection, the
proxy, the cache, the accuracy disc and the fix are all the parent's, and
this node cannot affect them.

`quiet-tiles.css` replaces the parent's dimming filter with a brightening
one and darkens the field behind the tiles so a tile that has not arrived
yet reads as part of the map rather than as a hole.
