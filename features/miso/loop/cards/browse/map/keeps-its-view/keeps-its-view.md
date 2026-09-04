# keeps-its-view
*the map opens where you left it — through an update, through a relaunch, on any screen*

> (transcripts/2026-09-04-field-walk.md#p119)
> on that last update, the map on my phone popped to a max-zoomed-out view - breaking our rule of not disturbing state during updates. be good to find out why

## user

Pan and zoom the map to the street you are canvassing. The app updates itself,
or you close it and come back tomorrow, and the map is looking at that street
at that zoom. It is never the whole world, and it is never somewhere you did
not choose.

## spec

**Why, measured rather than guessed.** `/map`'s `mount` makes Leaflet at
`setView([51.2719, 0.1904], 3)` — a placeholder — and relies on the first
`draw` to fit the pins. So the map only ever gets a sensible view *because a
set was on the page*. `/always-the-ground` (build 685) made the map appear on
screens where no set is showing, and calls `mount()` there for a map that has
never been made. Nothing then fits anything, and the map sits at the world.

On the rig: user at centre 51.27190,0.19040 zoom 15, card opened, reload — back
at **zoom 0**. The same reload with the *set* showing came back at zoom 10, the
fit's own view: not the world, but still not where the user was. So the first
suspect is confirmed and the second is not the cause — the old Leaflet does die
with the page, but `/map` remounts perfectly well; it simply has no view worth
having, and never did. 685 is what made that visible, because before it the map
was hidden on exactly the screens where nothing fits it.

**The fix is a remembered view, not a guard on the mount.** The mount was doing
what it always did; what was missing is that the map has never known where it
was looking. `map_view` is a device var — where you are on the map is a fact
about this phone and this hand, like `/browse`'s `view` and `open` — holding
`"<lat>,<lon>,<zoom>"`. It is bridged, because the page half is the only half
that can answer the question, and it survives an update the way the rest of the
world does: `/patch/world-along` carries it across a hot swap, `/world-cache`
brings it back on a reload. That is `/keep/scroll`'s promise, kept for the map.

**One pair of handlers records every road.** A drag, a pinch, `/recentre`'s
`setView`, `/floating`'s pan — all of them end in `moveend` or `zoomend`, so
recording there needs no node to co-operate and `/recentre` keeps working
without knowing this node exists. The value is deduped, so a programmatic move
to the value just read queues nothing.

**The mount opens at the remembered view**, and sets `/map`'s own `fitted` flag,
because with a view of the user's there is nothing for `locate()` to ask the
device about.

**The first fit is undone once, and only once.** `draw` fits the pins when the
set changes, which on the first draw after a mount is always — and that would
take the remembered view away again a moment after the mount restored it. So
the first draw after a restoring mount is wrapped: the pins are drawn, the fit
happens, and the view is put back. After that the wrapper stands down and
`/map`'s own rule — refit only when the set of pins itself changes — is exactly
as it was. `draw` is wrapped rather than replaced, so `/reel` and the other
nodes that ride it are untouched (misses.md, *siblings at one anchor*).

**Where this node sits.** The cause is `/always-the-ground`'s mount, but the
fix belongs to `/map`: the view is the map's own state, the repair also fixes
the plain reload onto a set (zoom 15 → 10 before this node), and a map made by
anyone, ever, should open somewhere the user recognises. `/map` is at six
children now — the next one forces a regroup.

## hostile cases

- **No remembered view** (a first run, a new device): nothing is applied,
  `fitted` stays false, and `/map` fits the set's bounds or asks the device
  where it is, exactly as it chose before.
- **A remembered view outside the constituency.** Kept. It is the user's view,
  and a map that argues with where you put it is worse than one that does not.
- **The deep-link mount** (`/always-the-ground` making a map on a card page
  with no set): the remembered view is applied there too — that is the case
  that was broken.
- **A pin set that changes while you are looking.** `restoreOnce` is spent, so
  `/map` refits as it always has. Named because it is the one moment this node
  deliberately does not defend the view.
- **A view recorded, then `/map` unticked.** The var is this node's and goes
  with it; nothing else reads it.
- **A corrupt or half-written value.** Three fields are parsed and every one
  must be finite; anything else is treated as no memory at all.
- **`/patch` taking an update in place.** The world comes across with the var
  in it, so the remembered view is the one from before the swap.

## parked

- Remembering a view per tool, so posts and 👤 can look at different places.
  One map, one view, is what the tree has today.

## glossary

- **the remembered view**: the centre and zoom the map was last left at, on
  this device.

## code description

`keeps-its-view.vars` declares `map_view`, device-scoped and bridged.

`keeps-its-view.rs` extends `update` with the `MapView` event — the page half's
answer, arriving as an event rather than as a write to a bridged key — and
skips a value that has not changed.

`keeps-its-view.js` — `feature_KeepsItsView.remembered()` parses the bridged
var; `say(map)` sends the current centre and zoom, deduped; `watch(map)` binds
it to `moveend` and `zoomend`.

The block at the end wraps `feature_Map.mount` to apply the remembered view and
set `/map`'s `fitted`, and wraps `feature_Map.draw` so the one fit that follows
a restoring mount draws its pins and then gives the view back.
