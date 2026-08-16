# map
*where you are, drawn on miso's own graph paper*

> (asks#1786895599674)
> NEW ASK [proposed] … :: 'new tool: map, showing me my current surroundings'
> *(a field ask, filed from the phone on 2026-08-16, miso build 201)*

## user

Tap 🗺 and miso finds you. You sit at the centre, with rings marking real
distances around you and north at the top; the faint disc is how sure your
phone is about the fix, so a wide disc means "somewhere around here". It
keeps up as you move, works with no signal once the fix is taken, and your
position never leaves the device.

## spec

**The judgement this node had to make, recorded because the ask deserves
it:** a map showing surroundings normally means map *imagery*, and imagery
means somebody else's tile server — a runtime dependency, a network round
trip, and your coordinates handed to a third party on every pan. Miso
doesn't do that; the day this ask arrived we deleted onnxruntime for less.
There is a legitimate route to real cartography that keeps the doctrine —
**vendor** the data as `tools/fetch_stt.py` and `fetch_find.py` already
vendor model weights: pinned, fetched at build time, served from our own
origin, gitignored — and that is the named next rung. It is a real piece
of work (a tile pyramid, projection maths, panning and zooming) and it
presumes an area to cover.

So this rung ships the half that needs nobody: **where you are, honestly
drawn.** You at the centre, distance rings labelled in metres, north up,
and the accuracy disc drawn to the same scale so the picture never claims
more precision than the phone has. The scale chooses itself from the
accuracy — a 5m fix draws 25m of ground, a 200m fix draws 500m — so the
view is always usefully filled rather than a dot in an empty field.

**Live, not a snapshot.** Opening the tool starts a position watch and
closing it stops it, so the view keeps up while you walk and the phone
isn't listening to satellites when you're looking at something else.

**Absence degrades, as everywhere.** No geolocation, or permission
refused, and the tool says so plainly rather than showing an empty grid: a
map that quietly lies about where you are is worse than no map.

**Privacy.** The fix lives in loop state on the device and is never
uploaded — this node adds no message, no var, no route. When geotagged
posts arrive and surroundings start meaning *what miso knows around you*
rather than only *where you are*, that will be a scope decision made
deliberately, not inherited from here.

## glossary

- **fix**: one position reading — latitude, longitude, and the radius the
  device believes it to be accurate to.
- **span**: the ground distance from the centre of the view to its outer
  ring, chosen from the accuracy.

## code description

`map.rs` registers `{map, 🗺}` in `tools_list`. `update` claims `Located`
(store the fix, clear any error), `LocateFailed` (store the reason), and
the `map_again` click (drop both, so the next reading redraws from
scratch). `render`, when map is the open tool, appends `map_view`;
`tool_controls` adds the ⟳ button beside the tool.

`map_view` renders one of three honest states: the error and what it
means, "finding you…" before the first fix, or the view itself. `map_span`
picks the first of 25 / 50 / 100 / 250 / 500 / 1000 metres that comfortably
contains twice the accuracy, and the ring labels and the accuracy disc's
size are both derived from it — one number, so the picture is internally
consistent by construction.

`map.js` is the hardware half, following `/dictate`'s pattern exactly:
state edges drive effects and results return as events. It watches
`open_tool` and starts `watchPosition` on the rising edge, clears it on the
falling one, reports each reading as `Located` and each failure as
`LocateFailed` (distinguishing a refusal from an unavailable sensor,
because the two want different responses from the user). Replay-guarded —
re-enactment must touch no hardware.

`map.css` draws it: a square view centred in the display surface, rings as
bordered circles at a third, two thirds and the whole of the radius, the
accuracy disc as a faint fill sized inline, the centre dot, the north
mark, and the readout beneath — monochrome, in the same discipline as the
toolbar.
