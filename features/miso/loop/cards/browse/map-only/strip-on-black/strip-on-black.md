# strip-on-black
*the project's name and the nøøb button sit on solid black, so the map never reads through the words*

> (asks#1788537194805)
> Project name and noob button should be on opaque black lozenges
> *(filed from the field on 2026-09-04 by ash, on build 658)*

## user

The name of the project you are in and the nøøb button both sit on their own
small black lozenges, the same shape as the filter word beside them. Whatever
the map is doing underneath — a bright ward, a road, a label — the words stay
readable, because nothing shows through them.

## spec

`/map-only` put the top strip over a map. Before it, the strip floated over the
graph-paper ground, which is nearly black everywhere, so a word with no ground
of its own was legible; over the outdoors basemap it is not. Ash asked for both
to sit on opaque black lozenges (asks#1788537194805). One reading, so it
builds.

**The nøøb button was already the right shape and the wrong opacity.**
`/lozenge` gives it `#121215` on a `#3a3a3f` hairline, fully round — the house
pill — and then `/button`, an older link, sets `opacity: .55` on the whole
element. Opacity applies to the element and everything in it, ground included,
so the map reads through the lozenge *and* through the glyph. That is the ask's
"opaque": the lozenge's own colour was never the problem. Opacity goes back to
1 and the quietness moves to the ink, which is where a thing is made quiet in
this tree (`/taste` 2 — hierarchy is dimness, not translucency). `#9a9aa2` is
what .55 of the page's white over `#121215` was already reading as, so nothing
appears to change except that the map stops coming through.

**The two pulses breathe the ink now, for the same reason.** `/update`'s blue
and `/attention`'s parchment both animate `opacity` between .55 and 1, which is
the same translucency arriving on a timer. They animate `color` between a dim
and a bright form of their own hue instead: each accent still carries exactly
the one meaning it carried (`/taste` 3 — a colour is a word), the breathing is
still 1.6s ease-in-out (`/taste` 5), and the ground stays solid throughout.

**The project's name had no ground at all.** `/title` draws it as bare accent
text; it was legible over the dot ground and is not over a bright ward. It gets
the strip's own pill — which is `/since`'s slot's pill, which is the nøøb
lozenge's, which is the house pill. It **hugs its name** rather than filling
the gap it was pinned into: `right: auto` lets a fixed box shrink to fit, and a
cap stops it short of the lozenge, keeping `/title`'s own ellipsis for a long
name. Its height and top are set so its centre sits on the line the filter word
and the lozenge share, which is what makes the three read as one strip rather
than three floating things.

**Why this is a child of `/map-only`.** The two elements are on every screen,
so the ground is too, and unticking this node takes it off all of them. But it
is `/map-only` that made a bright ground the normal one — before it, the strip
floated over the graph paper — so this is the node it is a consequence of, and
the node it should leave with. Neither `/title`'s files nor `/lozenge`'s are
edited: the ground is redefined from this node's own stylesheet, the way
`/since` already positions `.proj-title` from its.

## hostile cases

- **A long project name.** `/title`'s `overflow: hidden; text-overflow:
  ellipsis` are untouched and now clip inside the pill; the cap keeps the pill
  clear of the nøøb button.
- **No project current.** `/title` draws nothing, so there is no empty pill —
  the ground is on the element, not in the strip.
- **A build waiting and a message at once.** `.update` and `.attention` both
  set `animation`; the later rule in the composition wins, as it did before,
  and both are this node's, so the behaviour is the one that was there.
- **`/title` unticked.** Nothing to give a ground to; the `#build` half stands
  alone.
- **`/attention` or `/update` unticked.** Their classes are never set, so the
  keyframes here are never reached.
- **This node unticked.** The lozenge is translucent again and the name is bare
  text — legible over the dot ground, which is the tree as it was before
  `/map-only`.
- **A card page open over the map.** The strip is above it at `/beneath`'s
  depth; the pill sits on the card the same way it sits on the map.

## parked

- The filter word, the name and the lozenge as one drawn strip rather than
  three pills that happen to line up. Three is what the ask asked for.

## glossary

- **the strip**: the top row of the screen — the filter word at the left, the
  project's name in the middle, the nøøb button at the right.

## code description

`strip-on-black.css` returns `#build` to full opacity and gives it the quiet
ink that opacity was standing in for, restates `/update`'s and `/attention`'s
pulses as colour animations of their own hues, and gives `.proj-title` the
house pill — hugging its name, capped clear of the lozenge, and centred on the
strip's line.
