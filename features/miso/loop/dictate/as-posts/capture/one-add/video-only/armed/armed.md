# armed
*the + opens a recording row — rec, stop, camera, publish level — instead of filming at once*

> (transcripts/2026-09-04-field-walk.md#p14)
> next group of work: for the "add post" button, the "+" button shouldn't start recording right away. Instead, it should open a toolbar that contains buttons "rec" [create new post and start recording], "stop" [finish and file post], "camera-flip" [change camera], "publish level" [pops up publish options]. Those settings are persistent, so you set them as often as you like, but you get a bit of time do that.

## user

Open **posts** and tap **+**. Nothing starts filming. You are one level down,
and the row reads ‹, a **rec** dot, a **stop** square, a **camera**, and a
pair of **sliders**.

Tap the camera and it becomes a face, or a face and it becomes a camera: that
is the camera your next video is filmed with. Front to begin with — it is a
selfie unless you say otherwise — and it stays as you leave it, on this phone,
for every recording after.

Tap the sliders and you are one level further down, on a short list: *same as
me* and the six ranks. Left alone, a post you make is stamped at your own rank
in the project, as it always was. Pick *volunteer* and every post you make
from now on reaches the volunteers in your project the moment you save it,
without you promoting anything. You cannot pick a rank above your own — tap
*admin* as a volunteer and your own rank stands, because a post above your
rank is one you could not see yourself. ‹ takes you back to the row.

Then tap **rec**. Filming starts, the dot goes quiet and the square lights.
Tap **stop** and the post is made where you stand, with the camera and the
level you set.

‹ at any point takes you back up a level, and if you never tapped rec there is
no post — you looked at the row and left.

Posts you have already made do not move. Promote still works, one rung at a
time, on any post of your own.

## spec

`/video-only` made the plus record: with every post a video, `/one-add` gave
the plus the video kind's own event and a tap started filming. Ash's ruling
from the field (#p14) is that a tap should not: it should open a toolbar, and
the settings on that toolbar persist, so there is a moment to set them.

**One seam changes, and it is `/one-add`'s.** `one_add_ev` answers which event
the plus carries. For the video kind it now answers `tool_record` instead of
`vid_rec`. `/tools` opens any `tool_<id>`, so nothing here navigates and
nothing here writes `open_tool` — a write from a link newer than `/payload`
paints one stale frame (misses.md, "navigation from the wrong side"). Only the
video kind is re-aimed: with `/video` unticked the mode falls back to write and
the plus is `/posts`' own new button, untouched.

**Two nested levels, and the tree gives them their ‹ for nothing.** Neither
`record` nor `level` is in `tools_list`, so `/one-level` reads both as nested,
remembers the way in, and ‹ climbs exactly one level: level → record → posts →
the toolbar. The stack is a stack, so the second level of nesting costs no
code (`/one-level` said so when it was written). `/current-only` has already
dropped the parent's own button at both levels, so each row is ‹ and this
node's controls.

**Four buttons, and only two of them ever act.** rec and stop are both always
drawn and one of them is dead: the ask named four buttons, and a row that
changes shape under the finger is the thing `/one-add`'s picker was taken away
for. The dead one carries no `data-ev`, so a tap sends nothing rather than
sending something quietly ignored, and it wears no tint and no ink — live and
dead are told apart without reading the glyph. rec and stop send `/video`'s own
`vid_rec` and `vid_stop`; the recording edges, the minute cap, the poster, the
square crop and the filing are `/video`'s and none of them is touched.

**The post is minted when the recording is saved, which is `stop`** — that was
already true and is what makes "+ then ‹" leave nothing behind. `card_new` is
reached from `/video`'s save and from nowhere else on this road.

**The camera, and the new default.** `/flip` holds the camera in `facing`,
defaulting to back, and put its control in `/one-add`'s kind picker — which
`/video-only` stopped drawing on 2026-09-03, so since build 614 there has been
no way to reach it. This node takes the setting over: it declares `camera`
(device-scoped, bridged, **default front** — "default selfie" is the word from
the prompt this button carries forward, #p5) and redefines `flip_read` and
`flip_write` onto it, so `/flip`'s own control, if a composition still draws
one, moves this value and the two can never disagree. `facing` is dead while
this node is composed.

The default is changed **here and not in `flip.vars`** on purpose: a default
written into `/flip`'s own declaration would survive this node being unticked,
which is exactly what the toggle proof forbids. Untick this node and `/flip`'s
var, its default and its control are all its own again.

**The camera reaches the lens through one function.** `/capture/video` asks for
what `constraints()` says; `/flip` redefined that to call
`feature_Flip.facing()`. This node redefines `facing()` alone — order-proof,
because `/flip` calls it at the moment the camera is asked for, so it does not
matter whether `/flip`'s 100 ms installer ran before or after this file. With
`/flip` unticked there is no `facing()` to redefine and the constraint is
written straight onto `feature_Video` instead. Both roads are typeof-guarded.

**The level a new post is stamped at.** `/audience` stamps `floor` at the
author's own grade inside `card_new`, in a line with no extension point. The
parent is refactored to open one (agents.md step 3): `audience_new_floor(grade)
-> String`, returning `grade`, called where the literal was. Behaviour is
byte-for-byte what it was — `grade` at that point is always one of the six or
the function has already returned.

This node redefines it. An empty choice answers the author's grade, which is
the rule as it stands. A chosen level answers the chosen level, **clamped never
above the author's own rank**: the floor is the lowest rank that holds a post,
so a floor above the author's rank hides the post from its own author — a
volunteer choosing *admin* would post into a room they are not in. The clamp
is the ruling the ask left open, and it fails towards the author's own grade,
which is today's behaviour. Choosing a level *below* one's rank is the whole
point of the setting: a candidate posting straight to volunteers, without three
taps of promote. Promote still lowers the floor afterwards, unchanged, and is
still the author's alone.

The level is **user**-scoped, not device: which room you are talking to is a
decision about you, and it should follow you to your other phone. `/one-add`'s
`mode` is the other kind of setting and stays on its device.

**Neither setting is on `/undo`'s stack, and the row shows no undo button.**
`/undo` files a step from the ops a turn put in the outbox, and it reads that
outbox at its own link — which is inside this node's, since this node is
newer. So a write made here lands after the mark and is not recorded, and
`/aside` finds nothing to undo and leaves the arrow out. Observed on the rig,
and left as it is: the settings are two taps to change back, and `/flip`'s
camera was never undoable either.

**The six words are held here as well.** `audience.rs` has them and
`audience.js` has them, and this node has a third copy, for `audience.js`'s own
stated reason: this half must survive `/audience` not being composed. That
keeps this node's dependency on `/audience` one-directional and compile-free —
it redefines `audience_new_floor` and calls nothing of `/audience`'s, so with
`/audience` unticked that redefinition is simply a function nobody calls. The
cost is named: the two lists must agree, and the order is a ruling (saturday
#p15) that changes by ask, not by refactor. A word this node wrote that
`/audience` did not know would rank as `team` there, which is contained, not
silent.

**What this replaces.** An earlier cut of the same morning (#p5) put an
*options* button in the posts row opening a page with camera and level on it.
Ash reshaped that before it shipped (#p14): the camera is a row button now and
the level page is what the sliders open. Nothing of that cut shipped; its two
settings, its `/audience` seam and its clamp are here.

**Parked, and named** (`/anticipation`): a *per-project* level (the setting is
one value for every project today); the chosen level shown on the row so you
can see where the next post is going without opening the page; a countdown or
a hint that says the row is waiting for rec; offering only the ranks at or
below your own rather than clamping a bad pick (it needs the author's grade in
the selected project, which is `/audience`'s to hand over and a second seam);
*demote*, which `/audience` parked already. Each is a redefinition of one
function here.

## hostile cases

- **+ then ‹, with nothing recorded.** No post. The plus opens a level and
  writes nothing; a post is minted when `/video` saves a recording, and there
  was none.
- **stop with no recording.** The stop button is drawn dead at that moment —
  no `data-ev` — so the tap sends nothing, the row keeps its shape and nothing
  is filed.
- **rec while a recording runs.** The rec button is the dead one then, and the
  stop is live. Neither can be pressed twice into the wrong state.
- **‹ while a recording runs.** The recording is not cancelled: it is
  `/video`'s, running on the page half, and it stops itself at the minute cap
  or on the stop button — which the posts level still draws, because
  `/one-add` puts the stop in the add slot while `vid_recording` stands.
- **A level above the author's own rank.** Not honoured; the author's own
  grade is stamped. Chosen once, it stays chosen in the list (it is what they
  asked for and their rank may rise), and every post is clamped as it is made.
- **A new post in no project.** `card_new` returns before the floor line, so no
  floor is stamped and this node is never asked. Nothing happens, which is
  `/audience`'s behaviour.
- **A word in the level var that is not a grade.** The pills cannot send one; a
  hand-made op that did would be refused at the tap (only a word from this
  node's own list is written) and, having got in another way, would rank last
  here and be clamped to the author's grade.
- **`/flip` unticked.** `flip_read` and `flip_write` are two functions nobody
  calls; the camera button still draws, still writes `camera`, and the page
  half puts the constraint straight on `/video`. The default is still front.
- **`/audience` unticked.** `audience_new_floor` is a function nobody calls,
  `card_new` stamps no floor at all, and the level page is a list that changes
  a var nothing reads — a setting with no effect rather than a broken page.
  (The composition itself does not stand without `/audience` today, for
  reasons older than this node: `/invited-into`, `/doors` and `/ranked` call
  its ladder. This node is not among them.)
- **`/video` unticked.** `one_add_mode` falls back to write, `one_add_ev` is
  the base's answer and the plus is `/posts`' new button. The record level is
  unreachable and nothing here draws.
- **`/undo` or `/aside` unticked.** No `ctx_undo` marker to insert in front of,
  so the controls end the row.
- **This node unticked.** The plus records again as it does today, `/flip`'s
  `facing` and its default of back answer again, `audience_new_floor` returns
  the author's grade, and every post is stamped exactly as it is now.

## glossary

- **publish level**: the rank a new post is stamped at instead of the author's
  own — one of `/audience`'s six grades, or *same as me*. It is a floor, so it
  never rises above the author's own rank.
- **the recording row**: the level the + opens — rec, stop, camera, publish
  level — where the settings are set before filming starts.

## code description

`armed.rs` — `one_add_ev` answers `tool_record` for the video kind, so the plus
opens a level instead of filming.

`armed.rs` — `tool_controls` draws the four controls at the `record` level and
the lit sliders at the `level` level, in front of undo; `armed_row` chooses
which of rec and stop is live from `vid_recording`; `armed_act_button`,
`armed_camera_button`, `armed_level_button` build them and `armed_before_undo`
is the insertion.

`armed.rs` — `render` draws the publish-level page under `open_tool == "level"`
out of `armed_level_row`, `armed_pill` and `armed_levels`; `armed_rank` is
their order.

`armed.rs` — `update` flips the camera on `armed_flip`, writes the level behind
a pill, and turns a tap on the lit sliders into `tools_home` for `/one-level`
to climb. rec and stop are `/video`'s own events and are not intercepted.

`armed.rs` — `flip_read` and `flip_write` are redefined onto this node's
`camera`, and `audience_new_floor` onto its `post_level`, clamped to the
author's own rank. `armed_camera_read`/`_write` and `armed_level_read`/`_write`
go through the live context, for `/flip`'s reason: a render after this node's
own write would read a stale bridge.

`armed.js` — `camera()` reads the bridged var; `install()` redefines
`feature_Flip.facing` when `/flip` is composed and `feature_Video.constraints`
when it is not, off the same 100 ms wait `/flip` uses, giving up after ten
seconds. It also adds the two levels and the camera button to `/tool-words`'
tables.

`armed.css` — the level page's frame and its row, drawn to match `/audience`'s
grade pills; the lit pair's black ink, the dead one's absence of it, and the
two settings' quiet ink.

`armed.vars` — `camera` (device, bridged, default front) and `post_level`
(user, bridged, default empty).
