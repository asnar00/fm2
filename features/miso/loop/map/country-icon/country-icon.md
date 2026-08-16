# country-icon
*the map button becomes the country you are standing in*

> (asks#1786896478784)
> NEW ASK [proposed] … :: 'the map icon should be the silhouette of the country I’m in'
> *(a field ask, filed from the phone on 2026-08-16, miso build 203)*

## user

Once miso knows where you are, the 🗺 button turns into the outline of the
country you are in — Britain at home, somewhere else abroad. It keeps that
shape between visits, so it is right the moment you open the app, and goes
back to 🗺 if it ever genuinely doesn't know.

## spec

The toolbar already renders a tool's icon as raw markup, so an icon can be
a drawing rather than a character. This node swaps the map tool's emoji
for the silhouette of the country the current fix falls inside.

**The outlines are ours.** `tools/fetch_countries.py` vendors Natural Earth
1:110m admin-0 boundaries — public domain, pinned by commit — and reduces
them to what an icon and a point-in-country test need: an ISO code, a
bounding box, and simplified rings. 175 countries in 147KB, fetched once
and served from our own origin, so no geocoding service is asked where the
user is standing. This is the vendoring rung `/map` named as future work,
arriving because an ask needed it rather than in anticipation.

**Simplification is deliberate and has a cost worth stating.** Points
closer than 0.05° are dropped — about 5km, far finer than a 24px button
can show, and well inside the error a phone's fix carries anyway. Holes
and specks are dropped too: only outer rings, and only islands at least 2%
of the mainland's area, because a hundred dots would muddy the silhouette
rather than describe it. The consequence: **within a few kilometres of a
border the answer may be the neighbour**. For an icon that is a fair
trade; anything that depended on the country being exactly right would
want the unsimplified data, and should say so.

**State carries the code, never the drawing.** The page half decides which
country and reports `CountryFound {code}` — two letters. The Rust half
stores it and puts a placeholder in the icon; the page half paints the
outline into that placeholder after each render. An outline in loop state
would be kilobytes copied into every blackbox entry, for a picture the
device can redraw for free.

**It remembers.** The country is kept in local storage and re-announced at
boot, so the icon is right before any fix is taken — you are almost always
still in the country you were in yesterday. A new fix that disagrees
replaces it silently.

**Absence degrades:** no outlines fetched, no fix, or a fix in no country
(mid-ocean, and the 110m data has gaps) all leave `/map`'s 🗺 exactly as
it was.

## glossary

- **outline**: one country's simplified boundary, enough to recognise it at
  the size of a button.

## code description

`country-icon.rs` extends two chains. `update` claims `CountryFound`,
storing the two-letter code as `map_country` (an empty code clears it).
`tools_list`, when a code is known, finds the entry whose id is `map` and
replaces its icon with `<span class="cc" data-cc="GB"></span>` — the
placeholder, not the picture.

`country-icon.js` owns the geography. `load()` fetches the vendored
outlines once, lazily. `find(lon, lat)` rejects on the bounding box first,
then ray-casts against each ring — odd crossings means inside. `svg(code)`
builds a path from the rings with a viewBox from the bounding box,
flipping latitude because north is up while SVG's y runs down; the fill is
`currentColor`, so `/tools`' existing monochrome filter turns it into a
silhouette without this node knowing anything about the toolbar's palette.
`paint()` fills any placeholder whose drawn code has changed — renders are
whole-DOM swaps, so this runs after each apply — and `watch()` looks at
`map_fix`, resolves the country, and reports it when it differs from what
state already holds. Replay-guarded, and each step no-ops when the
outlines have not arrived.

`country-icon.css` sizes the placeholder to the button's icon box.

`assets/geo/countries.json` is gitignored like the speech and embedding
assets: the recipe is the record.
