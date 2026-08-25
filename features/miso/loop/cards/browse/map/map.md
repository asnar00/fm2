# map
*the third view: the cards that have a place, standing on a real street map*

> (transcripts/2026-08-25-accounts.md#p87)
> my hitlist before tara comes in: 1) projects; 2) posts; 3) map view.

> (accounts #p59, 2026-08-25 15:29, the promise this keeps — where the switch
> goes)
> ah, I had an idea: let's add the selector button for grid/list at the top
> left of the screen, same height as the noob button. also, later when we add
> a map view, we'll select it from there

## user

Tap the third glyph in the picker at the top left — a folded map — and the
set becomes a map. Real streets, real names, drawn dark like the rest of the
app. Every card you hold that knows where it is stands on the map as a pin
wearing its own picture; the map opens showing all of them at once. Drag it,
pinch it, and it stays where you put it. Tap a pin and that card opens,
exactly as tapping its tile does; tap the cards button to come back, and the
map is where you left it. Which view you chose is remembered on this device,
so the map is still the map next time.

## spec

The two existing views draw the set as things on a page. This one draws it as
places in the world, which needs three parts the tree did not have: a picture
of the world, a library that can pan and zoom one, and somewhere for that
library to live across a repaint.

**The picture of the world is ours.** `/tiles` — a route in `serve`, this
node's other half — proxies and caches the basemap, so the device asks miso
for map squares and nothing on this page ever talks to a third party. The
2026-08-16 map was withdrawn partly for being a stranger's imagery on our
screen; `agents.md`'s law above the laws answers that with *own it* rather
than *refuse it*, and this is what owning it costs: one route.

**The library is vendored.** Leaflet 1.9.4 (BSD-2-Clause), the whole `dist`
file, lives in this node's `assets/map/` and is served from `site/map/`. No
CDN — the shell has to work offline, and the service worker caches whatever
the page fetches from us. The marker images Leaflet ships are not vendored
because nothing here uses them: the pins are `L.divIcon`, our own markup, so
the only rules that reference those images never match.

**The map lives outside `#app`.** `feature_Loop.paint` replaces the whole of
`#app` on every event, and a map re-mounted per event would refetch its tiles,
lose the pan and flicker. So the Leaflet instance is made once, on a
`div#misoMap` appended to `document.body` at load — `/keep`'s idiom for
furniture a repaint must not destroy, at a larger size — and the render puts
only an **argument** inside `#app`: an empty `#mapData` element carrying the
located cards as JSON. The page half takes the `paint` seam, and after each
paint asks one question: is `#mapData` there? Present, show the host and sync
the pins; absent, hide it. The map therefore appears, disappears and survives
without the loop knowing it exists.

**Which cards, and which of them are placed.** The set is whatever
`browse_cards` gives this surface — everything you hold under the cards tool,
people under `👤`, a type's own set under a surface that filters — narrowed to
the cards with a sound location block. That narrowing is `/location`'s own
`card_place_of`, so what counts as a place is decided in one place and this
node inherits its coordinate test.

**A pin is a face, not a marker.** The same face the grid tile draws — the
card's first picture, or its title's initial — in a ringed circle over a grey
stem. There is no colour on it and no accuracy disc. That is the direct
correction of the withdrawn map, which arrived with a blue marker and a blue
disc, *the only colour in the entire app*, imported from every other map
anyone has seen (`notes.md`, 2026-08-16). The basemap is dark for the same
reason and by the same means: `/tiles` defaults to a dark-drawn source rather
than a bright one with a filter over it (`/taste` 9).

**Tapping a pin is the tap the tool already answers.** `browse_open:<id>` —
`/browse`'s own event, so the card opens through the same path as a tile tap
and `/keep`, `/frame` and `/undo` act on it unchanged. Leaflet stops the DOM
event on its own markers, so the tap is sent by hand through
`feature_Loop.send` rather than relying on the loop's delegated `[data-ev]`
listener. It is the same event either way.

**Fitting, once.** The map fits the bounds of the pins when the set of pins
changes, and not otherwise: a refit on every repaint would fight the hand
that had just dragged it. With no pins at all it asks the device where it is,
once, and if the device will not say it leaves the world where it is. Neither
is an error state.

**The picker's third button, and why the row is restated.** `browse_views()`
is a chain and joining it costs one link — except that `/browse` lit the grid
by the rule *"whichever view is not the list"*, which appending cannot
extend: with the map chosen, grid would light up beside it. So this node
redefines `browse_views` with all three buttons, each lit by its own name, and
redefines `browse_view_button` to add the map's glyph while passing every
other name down the chain. A fourth view does the same again. `browse.rs` is
not edited.

**The glyph is a folded map, not a pin.** The pins are the things on this
view, and a screen carrying the same shape as both its control and its
content reads as a mistake — the call `/browse` already made when it refused
four squares for the tool that holds the grid.

**No new vars.** `view` is `/browse`'s existing device var and `"map"` is one
more value it can hold; `/world-cache` therefore remembers the map view across
a reload for free, and nothing goes on the wire.

## hostile cases

- **No card has a place.** The map draws, empty, and asks the device for its
  own position once. Refused or unavailable: the world stays where it is. No
  message, no error.
- **The server is unreachable** (offline, or stopped). Leaflet comes from the
  service worker's cache, tiles 404, and the map draws its dark ground with
  the pins standing on it. Pan and zoom still work; tapping a pin still opens
  its card, because that is the wasm and not the network.
- **`/tiles` unticked.** Identical to being offline, permanently.
- **Leaflet missing** (`assets/` half-copied): `L` is undefined, the mount
  returns false, the surface is the empty dark ground. Nothing throws.
- **A card with a garbage location block.** `card_place_of` returns null for
  an unsound coordinate, so it is simply not on the map — the same answer as
  having no location at all.
- **Forty pins on one street.** They overlap; the newest drawn is on top.
  Clustering is named and parked.
- **A repaint mid-drag** (a message arrives while you are panning): the map is
  not rebuilt, only the pins, and only if the pins changed — so the drag is
  undisturbed.
- **The tool is closed with the map open.** `#mapData` is gone from the next
  paint, the host hides, and the Leaflet instance waits with its tiles.
- **A logged-out device through the tunnel** requests `map/leaflet.js` for the
  half-second before the shell redirects it to the login page, and is answered
  with the login page. Harmless — the shell has already left — but it is why
  the file is not in the service worker's cache until someone has logged in.
- **`/map` unticked.** The picker has two glyphs, `browse_set_html` is
  `/browse`'s own again, and nothing loads Leaflet. A `view` var left reading
  `"map"` on a device falls back to the grid, because `/browse` draws the grid
  for every value that is not `"list"`.

## glossary

- **pin**: one located card on the map — its face on a stem, standing where
  the card says it is.
- **basemap**: the drawn world under the pins, served by `/tiles`.

## code description

`map.rs` redefines `browse_views()` with the three-button row and
`browse_view_button(which, on)` with the map's button, passing `grid` and
`list` down the chain to `/browse`.

`map.rs` extends `update` with one click, `browse_map`: it writes the `view`
var and clears `open`, which is what `/browse`'s own two view clicks do.

`map.rs` redefines `browse_set_html(cards)` — the grid/list switch, therefore
also the map switch. In map view it returns `map_surface_html`, which walks
the surface's cards, keeps the ones `card_place_of` calls placed, and emits a
single empty `#mapData` element carrying them as escaped JSON:
id, lat, lon, the face data URL, the title's initial and the title.
`map_face_of`, `map_title_of` and `map_initial_of` pull those three strings
out of a card. `map_fold_svg()` is the drawn glyph.

`map.js` owns the map. `feature_Map.host` is a `div#misoMap` appended to
`document.body` at load and hidden; the block at the end of the file also
replaces `feature_Loop.paint` with a wrapper that calls the original and then
`sync()` — property replacement at load, not a timer, per `notes.md`'s
apply-wrapper race.

`sync()` is the whole of the page-side logic: no `#mapData` means hide, and
otherwise show, `mount()` if the map does not exist yet, `draw()` the pins and
`invalidateSize()` (the host was `display: none` until now, so Leaflet had
measured nothing).

`mount()` makes the Leaflet map with no zoom control, adds the
`tiles/{z}/{x}/{y}.png` layer, and calls `credit()`, which fetches
`tiles/attribution` once and shows whatever line the server's tile source
earned. `draw(pins)` rebuilds the markers only when the pins have actually
changed, gives each an `L.divIcon` of this node's own markup, hangs a click
handler that sends `browse_open:<id>`, and fits the bounds. `locate()` is the
no-pins case: one geolocation attempt, silent on refusal.

`map.head.html` puts the vendored Leaflet stylesheet and script in the page
head — synchronous, so `L` exists before any fragment runs.

`map.css` places the host under the furniture at `/beneath`'s depth 1,
overwrites Leaflet's light-grey container and blue links with the house
palette, quiets the attribution to the dimmest text in the app and lifts it
clear of the toolbar, and draws the pin: a 34px ringed face on a grey stem,
with no colour anywhere on it.

`assets/map/leaflet.js` and `assets/map/leaflet.css` are Leaflet 1.9.4's
`dist` files, verbatim but for the stripped source-map comment, served from
`site/map/`.

*(Same-day fix: a picture-less card with no title — a post — pins with its owner's initial rather than a blank face.)*
