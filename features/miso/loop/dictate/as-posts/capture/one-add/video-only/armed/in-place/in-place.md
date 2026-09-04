# in-place
*publish level pops the list up in the row you are in, instead of opening a level below it*

> (transcripts/2026-09-04-field-walk.md#p31)
> for the publish level, let's make the list of options vertical, and explain each publish-level in a short sentence. also, hitting the options button should just pop up the options in the current toolbar without creating a new tool level

## user

Tap **publish level** in the recording row and the list rises out of the row,
over whatever is behind it. The row does not change: rec, stop, the camera and
the sliders are all still there, and the sliders are lit while the list is up.

Pick a level and the list puts itself away. Tap the sliders again and it puts
itself away. Tap anywhere else — a button, or the bare ground — and it puts
itself away, and whatever you tapped happens as well. ‹ closes it too, and
leaves you in the recording row; a second ‹ takes you back to the posts.

## spec

`/armed` made the publish level a level of the tree: `tool_level` descended,
the row became ‹ and the lit sliders alone, and the list took the screen. Ash
asked for it to pop up in the toolbar you are already in (#p31). One reading,
so it builds.

**The tree of tools still holds.** The picker is the sub-tool's own popover,
not buttons on a page: nothing is drawn for you to choose between except the
levels themselves, and the row that owns them stays under it. What changes is
that a *setting* no longer costs a level — which is the distinction the ruling
was about, a page doing the toolbar's job.

**`/tools` needed no seam.** A popover is not a level, so nothing here writes
`open_tool` and nothing descends. `/armed` opened the two /extensible
functions/ this needs on its own button — `armed_level_ev`, which was the
literal `tool_level`, and `armed_level_lit`, which was the literal `false` —
and this node answers them with `armed_pick` and the flag. Both returned
exactly what the literals returned, so `/armed` alone behaves as it did.

**Open is a flag on the turn's state, not a var.** `/one-add`'s own idiom for
exactly this shape — a strip you come back to, closed by the next tap — and
its consequences are the ones that idiom has: no op on the wire, nothing on
`/undo`'s stack, and a relaunch starts with it shut.

**Everything that is not the sliders closes it**, which is `/one-add`'s rule
and is why there is no list of exceptions here: rec closes it and records, the
camera closes it and flips, a level closes it and is chosen. The flag is read
off the state coming in and written onto the state going out, so a tap does
both things rather than one.

**‹ is the one event caught before the chain.** `/one-level` would climb out of
the recording row, and the ask is that the row stays where it was — so while
the list is up, ‹ closes it and the event is never handed down. The second ‹
climbs exactly as it always did, because the flag is off by then.

**A tap on bare ground is the page half's**, because `/loop` sends nothing for
an element with no `data-ev`. `/backdrop` has this job for a card page and
cannot do it here — it returns early unless a `.card-page` is open, and this is
not one. So `in-place.js` sends `armed_close`, which the Rust half treats as
any other tap. It sends rather than hiding the element, because the popover is
drawn by `render` and the next repaint would bring back anything the page half
took away.

**Where it sits.** On the row, not centred on the button: `/long-press`' card
measures its button's rectangle and centres itself, which is right for two
lines of prose and wrong for seven rows that are wider than any 50px control.
So the popover is anchored to the row's own edges — the page's safe width,
directly above the toolbar — which is what "in the current toolbar" means. Its
ground is `/long-press`' card's, a step lighter than a page's, because it is
over the page rather than instead of it (learned 2: the context stays visible
under whatever opens).

**The level that is no longer reached.** `/armed`'s `render` for
`open_tool == "level"` and its lit-sliders row for the same are left exactly
where they are and are simply never entered. Untick this node and the level
comes back, whole.

**Parked, and named** (`/anticipation`): the camera button's own popover, if a
third camera or a resolution ever joins it (the same two seams would do it);
the chosen level shown on the sliders button so the row says where the next
post is going without opening anything; and a swipe down to close, which
`/reel/swipe-away` already knows how to do.

## hostile cases

- **A second tap on the sliders.** Closes it. The flag is toggled, not set, so
  the control is two-faced like every other one here.
- **‹ with the list up.** Closes it and stays in the recording row; the event
  is not handed to `/one-level`. A second ‹ climbs to the posts.
- **A tap on bare ground.** `in-place.js` sends `armed_close` and the generic
  rule closes it. A tap inside the popover, on the toolbar, on the picker or on
  any element that carries its own event is somebody's and is left alone.
- **rec with the list up.** The list closes and the recording starts: one tap,
  both things, because the flag is written onto the state the chain returned.
- **The level page.** Unreachable: nothing sends `tool_level` any more. Its
  code is `/armed`'s and untouched.
- **A device that was standing ON the level page when the update landed.**
  `open_tool` is a device var and `/restore` brings it back, so that phone
  opens on the old page once more — `/armed`'s render still draws it. One ‹
  climbs out (`/one-level` needs nothing from this node to do it) and there is
  no way back in. Seen on the rig, 2026-09-04, and left as it is: a migration
  for one frame on one device is a worse answer than a tap that already works.
- **A frame painted at another level with the flag still set.** The popover is
  drawn only while the recording row is the open one, so a stale flag cannot
  put a picker over the posts list.
- **A relaunch with the list up.** The flag is turn state, not a var: the app
  starts with it shut.
- **`/explained` unticked.** The popover holds `/armed`'s wrapping pills
  instead of the column; it is the same list either way.
- **This node unticked.** The sliders open the `level` tool again, the page
  draws, and ‹ climbs out of it — `/armed` exactly as it shipped.

## glossary

- **popover**: a box that opens on the row that owns it, over the page rather
  than instead of it, and closes on the next tap. Not a level of the tree.

## code description

`in-place.rs` — `armed_level_ev` answers `armed_pick` and `armed_level_lit`
reads the flag, so `/armed`'s own button opens a popover instead of a level;
`in_place_open` is the flag's reader.

`in-place.rs` — `update` catches ‹ before the chain while the list is up and
closes it without climbing; otherwise it lets the chain run and then toggles
the flag on `armed_pick` or clears it on anything else.

`in-place.rs` — `render` appends the popover, holding `/armed`'s own
`armed_level_row`, while the flag is up and the recording row is the open
level.

`in-place.js` — a click listener that sends `armed_close` when the tap landed
on nobody's ground, which is the one case `/loop` sends nothing for.

`in-place.css` — the popover's ground, its place above the toolbar, and the
0.18s rise it opens with.
