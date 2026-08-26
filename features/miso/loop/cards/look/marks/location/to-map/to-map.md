# to-map
*"map location" goes to the map*

> (asks#1787703005356)
> pressing the "map lo location" button should just go to the map view of the post
> *(filed from the field on 2026-08-26 by ash, birthplace `projects @ miso/loop/cards/kinds/projects`)*

## user

Tap **map location** on a card and you are on the map, with that card's pin among the others.

## spec

`/location` opened a placeholder sheet with the coordinates — the ask's own "for now". Now that `/map` exists, ash asked for the pill to go there (`asks#1787703005356`). One reading, so it builds: this node replaces `feature_Location.show` at load — the pill's tap leaves the open card (the tool's own button is the way back) and then switches the surface's view to the map. The sheet stays in the tree, unreached; untick and the sheet is back. Centring on the one card rather than fitting all pins is the anticipated next ask (a focus id the map can read).

## hostile cases

- A card with no location: the pill is dimmed and `/location` asks the phone first; this node runs only when `show` is called with a place.
- `/map` unticked: `browse_map` is not a view; the tap goes back to the set and nothing else.

## glossary

(no new terms)

## code description

`to-map.js` — replaces `feature_Location.show`: sends `tool_<open>` (back) then `browse_map`.
