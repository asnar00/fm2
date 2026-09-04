# map-only
*the map is the only view: the grid, the list and the switch that chose between them are gone*

> (transcripts/2026-09-04-field-walk.md#p13)
> ok, superb. next batch of work: I want to lose the grid/list views and standardise on map view for everything - the "reel" feature, coupled with smooth open/close/scroll, does everything we need. So let's remove the grid/list/map switch, and replace it instead with a time-domain filter: options are "today", "week", "month", "all". Today just shows posts made today, week shows this week (from monday as day 1 of the week), month shows all this month's, all shows all. This applies to all the other views as well (users, projects).

## user

Every tool that shows you a set of cards — posts, 👤, projects — opens on the
map. There is no grid, no list, and no pill at the top left to choose between
them: the map is what a set looks like now. What has a place stands on it;
what has none is in the band along the bottom, which holds every card in the
set whether it is on the ground or not. Tap a pin or a lozenge and the card
opens over the map, as it already did.

The slot the picker used to hold is empty, and the next thing that belongs at
the top left goes in it — the time filter does, in the same breath (`/since`).

## spec

`/browse` built two views and a picker; `/map` joined it as a third. Ash's
ruling (#p13) is that the third is enough: *the "reel" feature, coupled with
smooth open/close/scroll, does everything we need*. So this node makes the map
the only view and takes the picker off the screen.

**One function decides the view, and it now answers one thing.**
`browse_view_read()` is `/browse`'s reader for the `view` device var and every
surface asks it — `/map`'s `browse_set_html` switch, `/posts`' empty line,
`/browse`'s own grid-or-list. This node redefines it to `"map"` and does not
consult `existing`: a device that stored `"list"` a week ago gets the map, and
no migration is needed because nothing writes the var any more. The var itself
is left declared and left alone — unticking this node hands it straight back.

**The picker's slot becomes a seam rather than a hole.** `browse_picker_html()`
is redefined to `browse_slot_html()`, whose default is the empty string, so
nothing draws at the top left and no empty pill is left hanging there.
`browse_slot_html()` is the extension point the next occupant of that slot
redefines — which `/since` does within the minute, and which is why the slot is
named for the place rather than for the picker (`/anticipation`, `/learned` 13).
`browse_views()` and `browse_view_button()` are left exactly as `/browse` and
`/map` wrote them; nothing calls them, and they come back the moment this node
is unticked.

**A card with no place is not lost, because the band is the set and not the
pins.** `/map` drops a placeless card from `#mapData`'s pins — `card_place_of`
returns null and it is simply not drawn — and while the grid existed that was
harmless, because the grid held everything. It is this node that makes the
question sharp. `/reel` had already answered it for posts: `data-ids` on
`#mapData` is **the surface's whole set**, placed or not, and the posts band
lists that, so a post with no place is a lozenge that opens and pans nothing.

**On 👤 the band was the pins, and this node makes it the set.** `/everyone`
(shipped last night) reads "all visible users" as *the pins, live or placed*
and drops a card with no location block — which was right while the grid was
there to hold everyone else, and is wrong the moment this node takes the grid
away: a colleague who is offline and has never been placed would have no pin,
no lozenge and no way in. So `map-only.js` adds back, on every surface, the
cards that are in `#mapData`'s set and not in the band the chain returned.
That is the direction `/everyone`'s own ask was pointing (*show all visible
users* — a widening), continued past the premise it was written against
(misses.md, *a spec's promise outlived its premise*). A person with no place
is a lozenge that opens their card and pans nothing, exactly as a placeless
post is.

**The projects map is the one that had no band, so this node gives it one.**
`/reel` shows on the posts tool and `/on-people-map` widened it to 👤; the
projects tool was never in either gate, and a projects map with no band would
be a dark rectangle with no way to open, select or delete a project — the tool
would be unreachable, which is a break and not a simplification. The ids the
band needs are already on the page (`/reel` writes `data-ids` on every
`map_surface_html`, whatever the surface), so this costs a wrap and no new
data: `map-only.js` wraps `feature_Reel.showing` to say yes on the projects
tool, and `feature_Reel.posts` to answer with the set's own cards there — the
same reading `/people-there` makes on 👤, applied to a set that is projects.
Nothing under `/reel` is edited; the wraps live in this node's own file, which
is the idiom `/on-people-map` and `/people-there` already use from theirs.

**What a project's lozenge says.** Its picture or its initial, its title, and
its owner and time in `/reel`'s own words — `/reel` renders the rows, so a
project row is a project-shaped row of the same grammar (`/learned` 9: when one
kind gets a shape, the others get it too). A project with a location block pans
the map; one without leaves it where it is, exactly as a placeless post does.

**Why the two views stay in the tree.** They are `/browse`'s and `/map`'s own
code and they are still correct; this node makes them unreachable, it does not
delete them. Unticking it is the whole of the way back — the picker returns,
the stored `view` is read again, and a device that had chosen the list is in
the list. Deleting them would have made this node's untick a lie.

## hostile cases

- **A device whose stored `view` is `"list"`.** The map, like everyone else's:
  `browse_view_read()` never reads the var while this node is on. The stored
  value is untouched and comes back with the picker if this node goes.
- **`/map` unticked.** Then there is no map to be the only view.
  `browse_set_html` is `/browse`'s own again and draws the grid for every value
  that is not `"list"` — so the grid returns, without a picker to leave it by.
  That is the honest degradation and not a state worth writing code for: this
  node's premise is `/map`, and a composition without `/map` should untick this
  node too.
- **`/reel` unticked.** No band anywhere. Placed cards are still on the map and
  still open; placeless cards are unreachable, and the projects tool is a dark
  rectangle. Same answer as above: this node's premise is the band, and the two
  untick together.
- **A world with no placed cards.** The map draws empty and asks the device
  where it is once (`/map`'s own behaviour, unchanged), and the band holds the
  whole set — so everything is still one tap away and the map is scenery.
- **No cards at all.** No pins, no band; `/posts`' and `/projects`' empty lines
  are `browse_view_read() != "map"`-gated and so do not show, which is `/map`'s
  ruling that an empty map is still a map. A person with nothing sees the
  ground they are standing on.
- **A card open over the map.** Untouched: `browse_open` still routes to
  `card_page_html`, and `/opens-over-map` still draws it over the map.
- **`/current-project`'s chip.** It was appended *inside* the picker's pill,
  so replacing the pill takes it with it. That is invisible today: `/title`
  already hides `.proj-chip` and draws the project's name across the top of
  every screen instead, which is where the name belongs. The combination that
  loses it is `/title` unticked while this node is on — an unticked-node
  pairing, named here rather than guarded, because the honest repair is to
  untick this node too or to give the chip its own place.
- **The `browse_grid` / `browse_list` / `browse_map` events.** Nothing draws
  the buttons that send them, so nothing sends them; if one arrives anyway
  (a replayed black-box log) it writes the `view` var, which nothing reads.
  Harmless by construction rather than by a guard.

## parked

- Deleting `/browse`'s grid and list once the map has been the only view for a
  while. They cost a few hundred lines of composition and they are the untick.

## glossary

- **the slot**: the top-left place in the top strip, level with the nøøb
  lozenge, where the view picker used to be. Empty here; `/since` fills it.

## code description

`map-only.rs` redefines `browse_view_read()` to the constant `"map"` — the one
answer every surface's view question gets while this node is on.

`map-only.rs` redefines `browse_picker_html()` to `browse_slot_html()`, and
defines `browse_slot_html()` as the empty string: the seam for whatever takes
the picker's place.

`map-only.js` gives the projects map the band, and gives every band the set.
`feature_MapOnly.onProjects()` reads the toolbar for the selected projects
button (from the screen, not the state mirror — `/reel`'s own rule, #p19).
`rows()` builds `/reel`'s row shape from `#mapData`'s `data-ids` against the
held cards: the picture or nothing, the title, the owner, the card's own time,
and the place from its location block. `andTheRest(out)` appends the rows of
that set which the chain's own answer left out, newest first.

The block at the end wraps `feature_Reel.showing` — yes on the projects tool,
the chain everywhere else — and `feature_Reel.posts` — `rows()` on the projects
tool, `andTheRest(chain)` everywhere else.
