# recentre
*one tap on the map view puts you in the middle of it*

> (asks#1788370642018)
> in map view, add a "center" button that centers the view on my location
> *(filed from the field on 2026-09-02 by ash, birthplace `-`)*

## user

One tap puts you in the middle of the map, close enough in to see the
street.

The crosshair joins the row of buttons at the bottom whenever the map view
is up — 👤 and the folded-map button in the picker, or the same view under
posts or projects — and leaves the row when the map does.

If the phone cannot say where you are, nothing moves and one line at the
foot says *can't find you*. Your own pin is already on the map (`/live`
draws it while the app is in front of you); this only aims the view at it.
And the map stays where you put it: a card arriving with a place on it no
longer throws the view away.

## spec

The map opens fitted to all the pins. Out canvassing that is the wrong
frame: ash asked for the one tap back to where they stand (#asks
1788370642018). One reading survives the ask — the button, the map view, the
asker's own location — so it builds.

**It is a control in the row, not a button on the map.** `/tools`' own
instruction is that the interface is a tree of tools and a tool's actions are
sub-tools in the control row beside its icon; `/quiet-credits` is the map's
own version of the same rule — nothing floats over the map. So the crosshair
is a `tool_controls` button, 50px, tinted, in front of `/undo` like every
other control, and it appears and disappears with the view rather than being
drawn over it.

**Present exactly while a map is on the screen, which takes two links.**
`browse_view_read()` says which view this device chose, and the row is
composed from it — but that var is sticky and device-wide, so someone who
picked the map inside 👤 and then opened taps is still "in map view" by the
var alone, and a crosshair in taps' row would aim a map nobody can see. What
the control actually needs is the question `/live` asks on the page half:
**is `#mapData` on the screen?** That element is `/map`'s whole contribution
to the page, emitted by the surface renderers, which run *outside* `/tools`'
toolbar — so no link on the controls chain can see it. Hence:

- `tool_controls` puts the button in whenever a tool is open and the chosen
  view is the map. Cheap, and it is the link that gets the ordering
  (in front of undo) and the long-press stamp.
- `render`, this node's outermost link, reads the finished page and takes the
  button back out when `#mapData` is not in it. That is the exact test, made
  at the only point where the answer exists.

The failure direction of the pair is *no button*, never a button that lies:
the strip runs after everything and asks the same question the map itself
answers.

**The tap is answered on the page half and costs no turn.** Nothing in the
world changes — where the map is looking is not state anybody holds, not even
on the device — so the click is swallowed in a capture-phase listener on
`document`, ahead of `/loop`'s own delegated bubble listener. The loop never
sees the event, `/undo` files no step, and no repaint happens: the map moves,
and only the map. The button still carries a `data-ev`, because that is the
row's idiom and how `/sub-tool-cards` and the tree export find it.

The position is read the way `/live` reads it — `getCurrentPosition` with
`/live`'s own options, the permission the app already holds for posts and
heartbeats — and the fix is handed to `feature_Map.map.setView([lat, lon],
16, {animate: true})`. Zoom 16 is the map's own "close enough to see it"
(`/map`'s fit caps there, `/stocked` stocks to it). Nothing is added to the
map: `/live`'s pin is already the "you" this button aims at, and drawing a
second one would be two of you.

**Aiming the map by hand marks it as aimed, and the aim wins.** Two
automatic fits would otherwise undo the tap. `feature_Map.fitted` is the flag
`/map`'s one-time fit and `/live`'s one-time fit both read before throwing
the view somewhere, so this node sets it. The second is bigger: `/map`'s
`draw` refits the whole map every time the SET of pins changes — not only on
the first draw, and `fitted` does not guard that path. The rig caught it
live: the profile picture landing added a place to the card, the pin set
changed, and the view that had just been centred was thrown to the pins'
bounds a second later. So `draw` is extended here — the redefinition-plus-
captured-original idiom `/live` and `/boundaries` already use on
`feature_Map` — to redraw the pins and then put the aim back.

The ruling that encodes: **an aim by hand outlives every automatic fit, for
the life of the page.** Before this node nothing could aim the map by hand,
so nothing is taken away from anyone; and it is exactly what "centre on me"
asks for — a view that stays where it was put.

Untick it and the row loses the crosshair; nothing else changes.

## hostile cases

- **No position.** The error callback, a device with no `navigator.geolocation`
  at all, a permission refused, and a fix that arrives without numeric
  coordinates all land in one place: `can't find you` in `#cardToast`, the
  app's own voice, and the map does not move. Nothing throws — the whole
  handler is inside a `try`, and the toast is `typeof`-guarded so `/cards`
  unticked degrades to silence, which is what `/map`'s own `locate()` has
  always done.
- **A second tap while the first fix is in flight.** `asking` holds until the
  fix lands or fails; the second tap does nothing rather than queueing a
  second sensor read. A tap after that behaves as the first did.
- **The fix arrives after the view has gone.** A fix can take seconds. The
  move checks `#mapData` again at the moment it would happen and drops the
  fix if the map view is no longer up — otherwise a person would come back to
  the map view later to find it had moved under them.
- **Leaflet missing.** `/map` degrades to a grey box with no `feature_Map.map`;
  the tap then returns before asking for a position, and says nothing. A
  toast about your location would be a lie about which thing is broken.
- **A long press on the crosshair.** `/sub-tool-cards` arms on
  `.tool-button.ctrl[data-ev]`, so the card appears; its swallow calls
  `preventDefault` on the click that follows. Two listeners on the same node
  cannot stop each other, so this node reads the mark instead: a click that is
  already `defaultPrevented` is a press that was read, and the map does not
  move. The card falls back to the button's `title` — *centre on me* — until
  the tree export's `subtools` stamp is baked, after which it carries this
  node's own words.
- **A card page opened from a pin.** `browse_set_html` is not called for a
  card page, so there is no `#mapData` and the strip takes the control out —
  the same rule `/live` uses to stop polling.
- **`/undo` unticked, or undo hidden by `/aside`.** The inserter finds no
  `ctx_undo` marker and appends instead; the crosshair is then the last
  control, which is right, because there is no undo button for it to be in
  front of.
- **`/ember` unticked.** `tool_colour` answers empty, the tint is not written,
  and the button is the row's plain white-on-grey control. That is the right
  failure and the wrong default: the shipped basemap is dark (`/map-ground`
  is `#333333`) and a `#3c3c3c` chip nearly vanishes on it. Both variants
  were built and looked at (4a); the tint is why the crosshair is tinted at
  all.
- **The colour is a hash, so it can collide.** `/ember` picks a tool's colour
  by a byte sum over its name, and all six palette entries are already some
  tool's; `recentre` lands on `#945D48`, which is also `projects`'. In 👤's
  and posts' map rows the crosshair is distinct; in **projects'** map row it
  is the same brown as that tool's own `+`, which already matches its flag.
  Two glyphs, one colour — legible, not ideal, and named here rather than
  fixed by inventing a seventh accent (`/taste` 3: a colour is a meaning).
- **The pins change under an aimed map.** A card edited, a copy arriving from
  someone you hold, a post made from where you stand: `/map` redraws and
  refits. The wrapper puts the aim back after the redraw, so the new pin is
  drawn and the view does not move. Proven in the rig by pushing a pin
  1,000 km away into `#mapData` and re-syncing.
- **A relaunch.** Nothing here is remembered — no var, no device state, and
  `aimed` dies with the page. The map opens fitted to the pins as it always
  has, and the crosshair is one tap away.
- **`/map`'s `draw` missing** (a build without it): the wrapper is installed
  only if the function is there, so the fragment does nothing rather than
  throwing at load.

## glossary

(no new terms)

## code description

`recentre.rs`, `tool_controls()` /extension/: with a tool open and this
device's `browse_view` set to `map`, inserts the crosshair chip in front of
`/undo`'s button through `recentre_before_undo` — this node's own copy of the
inserter, so it stands without `/invite` or `/posts`.

`recentre.rs`, `render()` /extension/: the outermost link on the render
chain, so it sees the toolbar and the surface together. When the finished
page carries no `#mapData` the button is cut out again by `recentre_strip`
(the opening `<div` before the marker to the first `</div>` after it —
`/aside`'s cut, valid because the glyph is an SVG and the button holds no
nested div).

`recentre_button` draws the chip and `recentre_crosshair_svg` the glyph: a
ring, four ticks and a centre dot, `currentColor`, 24-unit viewBox
(`/glyphs`). `recentre.css` makes it black on the tint without depending on
`/tinted` to say so.

`recentre.js` — `feature_Recentre.go()` is the tap: guarded on the map
existing, one `getCurrentPosition` at a time, `/live`'s options. `to()`
performs the move, re-checking that the map view is still up and marking both
`feature_Map.fitted` and this node's own `aimed`. `lost()` is the single
failure exit — one line in `#cardToast`.

The load block does two things. It replaces `feature_Map.draw` with a wrapper
that calls the captured original and, once `aimed` is set, restores the centre
and zoom afterwards — the aim surviving `/map`'s refit-on-set-change. And it
installs one capture-phase `click` listener on `document`, which stops the
event before `/loop`'s delegated listener can turn it into a turn, and reads
`defaultPrevented` so a long press that `/sub-tool-cards` already swallowed
passes through untouched.
