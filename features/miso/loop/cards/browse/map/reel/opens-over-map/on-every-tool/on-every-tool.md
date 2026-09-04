# on-every-tool
*the map stays behind an open card, whichever tool you opened it from*

> (asks#1788537420957)
> opening a post should keep the map background behind it (same for users etc)

## user

Open a post and the map is still there behind the card, where you left it. Open a person, or a project, and the same. Tap the map to put the card away, as before.

## spec

`/opens-over-map` promised exactly this and stopped keeping it. Its test asked the view picker whether the map was the view — and `/map-only` took the picker away, so the question has had no answer since. Measured on the rig, on every tool: `picker: false`, therefore `behind: false`, therefore `mapShown: "none"` the moment a card opens, and the dotted ground under the card instead of the map. Ash saw it and filed it. The second half of the ask — "same for users etc" — was never true even before that: the test also insisted on the posts tool, so a person's card and a project's card never had the map behind them at all.

**The question, asked without the picker.** Rust draws the page *instead of* the set, so no `#mapData` with a card page on the screen is what "a card is open" looks like from here — that part is the parent's and is unchanged. What the picker used to say is answered instead by memory of the screen: on every sync where the set IS on the page, the selected tool is noted, and a card counts as standing over the map when the tool that is still selected is the one the map was last drawn for. So it holds for posts, for people and for projects without naming any of them, a tool that never draws a map is never noted and never claims one, and a tool added later needs nothing from this node.

The markers are not redrawn: `/map`'s `draw` only runs when the set is on the page, so the pins the map already holds are exactly the pins that were under the card — measured, eight markers still on the map with the card open. Showing the host again is the whole of it, which is what the parent already does once its question is answered yes; the reel is hidden behind the card by the parent too, on every tool now.

Untick and the map is hidden behind a card again, on every tool.

## hostile cases

- **A card opened from a surface that is not a map** (a project card reached from a person's page, a report). The tool selected is not the tool the map was last drawn for, or none is; no map behind, the ground as before.
- **The reports tool.** It never draws a map, so it is never noted.
- **The first card of a visit, before any map has been drawn.** Nothing noted yet; the map does not appear behind it. The map is drawn on the way in to every one of these surfaces, so this is the case where the card was opened without ever seeing the surface — a deep link, a restored tool.
- **Switching tools while a card is open.** The selected tool stops matching; the map goes, which is right — that surface is not underneath any more.
- **A tool whose map has no pins** (a project set with no places). The map is still the ground behind the card; an empty map is what that surface shows.
- **`/map-only` unticked.** The picker comes back and this node's test still holds — it never asked for the picker in the first place.

## glossary

(no new terms)

## code description

`on-every-tool.js` — `feature_OnEveryTool`.

`tool()` is the selected tool button's event; `note()` records it on every
sync where `#mapData` is on the page; `behind()` is the parent's
/extension point/ redefined — no set, a card page, and the selected tool is
the one the map was last drawn for.

The wrapper on `feature_Map.sync` runs after `/opens-over-map`'s own, so the
note is taken on the syncs where the set is present, which are the syncs
before a card opens.
