# one-pin
*one marker per person on the users map: the live position when there is one, else the profile's; tappable either way*

> (transcripts/2026-09-02-self-check.md#p45)
> ok nice. while that's brewing, the users page should only show one user marker on the map, not two: if there's no live position, it should show the profile's position; otherwise only the live position. The marker should be clickable anyway

## user

Each person is one pin on the map. Someone with the app open stands where
their phone is, wearing the live ring; the pin their profile card placed is
not drawn while that one stands. Someone not in the app stands where their
card says, as before. Tap either pin and their card opens.

## spec

`/live` draws a live pin *above* the placed pins, so a person whose profile
card carries a place showed twice while they were in the app — two pins,
one face. Ash wants one (#p45). This node is the rule that settles it: the
live position wins while it exists, the card's place otherwise, and the
tap works on whichever is standing. Nothing is added to the screen; a pin
steps aside and steps back.

**Which placed pin is whose.** `/map` keeps its markers as a flat array
with no card on them — the pin row is only closed over in each marker's
click handler — and the marker's `title` is the person's name, which is
not a key (two people called Bob stay two people; `/live`'s own review).
So this node reads the rows `/map` drew from, `#mapData`'s `data-pins`,
each of which carries the card's `id`. `/map`'s `draw` walks those rows in
order and skips any without a numeric position, pushing one marker per
kept row, so the array and the kept rows align index for index; after
every draw this node tags each marker with the id of the row at its index.
If the two counts ever disagree — a later sibling that draws differently —
nothing is tagged and every pin stands as `/map` drew it: the failure is
today's behaviour, never a wrongly hidden person.

**Standing aside, and standing back.** After every `/live` draw the rows'
card ids are the set of live people. A placed marker whose id is in the
set is taken off the map with Leaflet's `remove()`; one whose id has left
the set is put back with `addTo(map)`. The marker object stays in `/map`'s
array and keeps its click handler through both — Leaflet holds listeners
on the layer, not the map — so `/map`'s own teardown on its next draw
(`remove()` on a marker that is already off the map is a no-op in
Leaflet) and the placed pin's tap are both intact. `setOpacity(0)` was the
other candidate and was not chosen: an invisible marker still catches the
tap, and a hidden pin that opens a card is a ghost. `/live`'s `clear()`
(leaving the map view, or the page going hidden) puts every placed pin
back, since there are no live rows to stand for.

**Surviving `/map`'s re-sync.** `/map` rebuilds its array on every paint
whose pins changed, so a state change mid-live would put a fresh placed
marker under the live one. This node wraps `feature_Map.draw` — property
replacement at load, `/boundaries`' idiom — and, after the original, tags
the new markers and settles them against the last live set, in the same
synchronous turn, so no second pin is ever painted. When `/map`'s draw
returns early (the pins' signature unchanged) the markers and their tags
persist and settling is idempotent.

**The fit.** `/map` fits its bounds to its own pins before this node hides
one, so the map's first fit may include a place whose pin is not shown.
That is the fit `/map` chose and it is left alone.

**Parked, named.** A "last seen" pin between the two states — where they
were when they left, for a while — is refused by `/live`'s design: the
server forgets a position sixty seconds after the last heartbeat and
writes nothing anywhere, so there is nothing to draw it from. A live pin
that looks different from the placed one extends `/live`'s pin markup, not
this node. "Hide me from the map" is a second clause in `/live`'s `may()`.

## hostile cases

- **A person with a live pin and no profile place**: no placed marker
  carries their id; nothing is hidden, one pin, as today.
- **A person with a place and never live**: their id is never in the live
  set; their placed pin stands untouched, as today.
- **`/map` redraws mid-live** (a state change repaints the page): the
  fresh markers are tagged and settled inside the same `draw` call — one
  pin throughout.
- **The live pin goes** (hidden, killed, expired): `/live`'s next draw
  drops the row, the id leaves the set, and the placed pin returns on
  that same draw — within `/live`'s poll, five seconds, plus the server's
  own forgetting.
- **The array and the rows disagree in count**: nothing is tagged; two
  pins, today's behaviour, and never the wrong person hidden.
- **A live row with no card id** (yourself before your first card): it
  stands for nobody's placed pin and hides nothing.
- **The tap on a returned placed pin**: a real pointer click on it opens
  the card, and the card stays open (rig, 2026-09-02) — `/map`'s handler
  is on the marker object and survives `remove()`/`addTo()`.
- **The tap on a live pin — a pre-existing defect, outside this node,
  found while proving it.** A real click on a live pin sends
  `browse_open:<id>` and the card opens — and the same click then closes
  it. The card page repaints synchronously inside `/live`'s click handler;
  the card page has no `#mapData`, so `feature_Live.sync` runs `clear()`
  and removes the very marker under the finger; when the native click
  reaches `/backdrop`'s document listener its target is detached
  (`isConnected` false), no longer inside `#misoMap` or any owned
  selector, a card page is showing, and `/backdrop` sends `tool_account`.
  Proven with the node unticked too (build 471 behaves the same). A
  placed pin survives because `/map` only hides its host. The fix belongs
  to `/backdrop` (a detached target was somebody's) or to `/live` (send
  the open after the click has bubbled), not here.
- **`/live` unticked**: this node is its child and goes with it.
- **This node unticked**: `/live` draws above `/map` and a person with a
  place shows twice while in the app — the behaviour this node replaces.

## glossary

- **aside**: a placed marker taken off the map while the same person's
  live pin stands; it returns when the live pin goes.

## code description

`one-pin.js` is `feature_OnePin`, a page fragment that owns no markup.

`tag()` reads `#mapData`'s `data-pins`, keeps the rows with a numeric
`lat` and `lon`, and, when their count equals `feature_Map.markers.length`,
sets `fm_card` on each marker to the id of the row at its index. Any
other count tags nothing.

`apply(rows)` records the card ids of `/live`'s rows as the live set and
calls `settle()`.

`settle()` walks `feature_Map.markers`: a tagged marker whose id is live
and is not yet aside is `remove()`d and flagged `fm_aside`; one flagged
aside whose id is no longer live is `addTo(feature_Map.map)` and unflagged.

At load, typeof-guarded on both parents: `feature_Map.draw` is wrapped to
run the original and then `tag()` and `settle()`; `feature_Live.draw` is
wrapped to run the original and then `apply(rows)`; `feature_Live.clear`
to run the original and then `apply([])`.
