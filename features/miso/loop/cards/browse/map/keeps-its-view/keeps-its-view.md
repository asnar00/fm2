# keeps-its-view
*the map opens where you left it — through an update, through a relaunch, on any screen*

> (transcripts/2026-09-04-field-walk.md#p119)
> on that last update, the map on my phone popped to a max-zoomed-out view - breaking our rule of not disturbing state during updates. be good to find out why

> (#p127, on the first cut of this node shipping as build 690 — the reason
> this spec has a second half. Quoted as a revision; the anchor stays #p119.)
> my instace started "syncing..." but it's stuck on that message

*(That build was reverted on main. This is the node re-landed with the fault
found and fixed; what broke it is written up below, because the shape of it —
a wrapper that is not idempotent, feeding a handler that re-enters the loop —
is the kind of thing that will be built again.)*

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

**What broke build 690, found on the rig and not guessed.** Two faults, one
feeding the other.

`/map`'s `mount` self-guards (`if (this.map) return true`) and `/map`'s `sync`
calls it on **every sync**. The first cut applied the remembered view on every
one of those calls, not only when a Leaflet was actually made — so a second
after every drag the map snapped back out from under the hand. And each snap
fired `moveend`, whose handler sent an event **synchronously**: Leaflet fires
`moveend` inside `setView`, `setView` was inside `sync`, `sync` is inside
`paint`, and `paint` is inside `apply`. So the send re-entered the loop from
inside its own paint, producing another sync, another snap, another send.
Measured on the rig: three re-entrant sends at boot, one nested to depth 2.

`/veil` lifts the "syncing…" cover in the line *after* the inner apply returns
in its own wrapper on `apply`, so anything that throws or never returns down
there leaves the cover up for good — which is what ash saw, and the growth of
that loop is the crash that followed.

**So a move the app makes is not a move the user made, and is not recorded.**
Every app move is inside `/map`'s `sync` — `mount`'s `setView`, `draw`'s
`fitBounds`, and `invalidateSize`'s own `moveend`, which was recording the
world at zoom 0. Every hand move is outside it: a drag, a pinch, and
`/recentre`, which moves the map from its own click listener. So `sync` is
hushed as a whole, which is the rule rather than a list of the moves that exist
today, and `/recentre` keeps working without knowing this node exists.

**And a view worth recording is sent after the paint, never during it** — one
deferred timer, latest value wins — which is `/keep`'s own idiom for this exact
hazard ("after the paint has finished rather than re-entering it").

**The restore happens on the transition and nowhere else**: `mount` is wrapped
to notice whether there was no map before the inner call, and only then applies
the remembered view.

**A zoom at or below `/map`'s own placeholder is treated as no memory.** That
is the globe, no hand on this app has ever chosen it, and it is exactly what the
broken build recorded — so every phone that ran 690 has one stored. Ignoring it
lets the fit take over, which is the right answer for a map that has never been
placed, and heals the field rather than only stopping the next one.

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
- **A view stored by the broken build 690.** Its zoom is at or below the
  placeholder, so it is ignored and the map fits as it would on a first run.
- **A sync that moves the map for its own reasons** (a resize, a refit): hushed,
  so it never becomes "where the user was looking".
- **`mount()` called on a map that already exists** — which is every sync: the
  wrapper notices and does nothing. This is the fault that broke 690.
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
var and refuses a zoom at or below `FLOOR`, `/map`'s own placeholder.
`hush(fn)` is the depth counter that marks a move as the app's; `where(map)` is
the current centre and zoom as a string, empty for a Leaflet with no view set.
`say(map)` schedules one deferred send — after the paint, never inside it —
and drops it if the view is already the remembered one. `watch(map)` binds
`moveend` and `zoomend`.

The block at the end wraps three of `/map`'s functions. `sync` is hushed whole,
which is the line between a move the app made and a move the hand made.
`mount` applies the remembered view — but only when there was no map before the
inner call — and sets `/map`'s `fitted`. `draw` is hushed, and when it follows a
restoring mount it draws its pins and then gives the view back.
