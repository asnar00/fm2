# glide
*going up a level, the toolbar's buttons slide smoothly to where they now belong*

> (asks#1788371214988)
> when we go up a level on the toolbar, animate tool position smoothly

## user

Tap ‹ and the row does not snap. A button that belongs to both levels slides
from where it was to where it now belongs; a button that is leaving fades out
where it stood; a button that is arriving fades in where it lands. Going down
a level does the same. Nothing moves while you stay on one level — tapping
away inside a tool leaves the row exactly as still as it was, and undo, when
it appears at the far right, fades in rather than pops. If your phone is set
to reduce motion, none of this happens.

## spec

A render is a whole-DOM swap (`/loop`'s `paint`), so the toolbar is thrown
away and rebuilt on every state change. `/steady` made that bearable by
stilling the re-mounted buttons when the level had not changed; what it left
was a **snap** when the level *does* change — the whole row jumps to its new
arrangement in one frame. The plus that was second becomes third with no
travel between.

This node gives the change of level its motion, by the FLIP idiom: measure
where the buttons are **before** the swap, let the swap happen, measure again,
put each surviving button back where it was with a transform, and let it
travel to zero over 220 ms with the app's ease-out. The paint is never
delayed, and the DOM the row ends in is exactly the DOM it would have ended in
without this node — only the frames in between differ.

**A level is `open_tool`, which is /steady's own test.** The launcher is the
empty string; 👤's level is `account`; the invite level is `invite`. `apply`
sets `feature_Loop.state` before it calls `paint`, so the value read inside
the paint wrapper is already the level about to be drawn, and the value
remembered from the previous paint is the level being left. Up a level and
down a level are the same event to this node — both are a change of
`open_tool` — so a tap into a tool glides exactly as ‹ out of one does. The
ask names going up; going down gets the same courtesy because refusing it
would need a direction test the DOM does not carry, and a row that glides one
way and snaps the other would read as a bug.

**Identity is `data-ev`, falling back to `data-ctl`.** That is what the
toolbar's buttons carry — `tool_account`, `tool_invite`, `tools_home`,
`invite_qr`, `ctx_undo`. A button with neither attribute is left out of the
glide altogether: not moved, not faded, not ghosted. Guessing which unnamed
button in the new row is which unnamed button in the old one is how a glide
starts moving the wrong thing.

**Three fates, one pass.** A button in both rows *moves* (or, if its rectangle
did not change — ‹ is at the left edge on every level that has it — is left
exactly where it is). A button only in the new row *arrives*: it fades in at
its landing place. A button only in the old row *departs*: a clone of it,
positioned where it stood, fades out and is then removed. Position and
opacity ride one transition, so a button that both moves and arrives is not
half-configured.

**One tap is three paints, so a glide has to be resumable.** The rig found
this: a tap on ‹ produces three `apply`s about 3 ms apart (the click's turn,
then the context write's, then its echo). The first is the level change; the
next two stay on the new level and rebuild the row again, destroying the
elements the first was animating. A glide that only ever *started* was
therefore invisible on exactly the climbs the ask was about. So the node keeps
two windows — `moveUntil` for the level glide, `fadeUntil` for the fades —
and a paint that lands inside one *continues* the motion instead of starting
a new one: it measures the old row's **visual** rectangles and opacities (a
`getBoundingClientRect` of a transforming element gives where it has got to),
and re-establishes the same journey over the time that remains. Three paints
3 ms apart are one 220 ms journey. Only a change of level opens `moveUntil`,
so a same-level paint outside a climb still cannot move anything.

**The mount slide has to be cancelled by hand, before anything is measured.**
`.toolbar .tool-button { animation: bar-slide 0.18s ease-out }` translates
every freshly mounted button 14px left on its first frame, so a rectangle read
through it is 14px wrong — the first cut's glides all began 14px past where
the button had been. And a CSS animation outranks an inline `transform` in the
cascade, so a FLIP transform under a running bar-slide is simply not painted.
Both are answered the same way: `run` sets `style.animation = 'none'` on every
identified button in the fresh row *first*, then measures. That is `/steady`'s
own idiom applied per button rather than per row. Nothing in `/steady`,
`/tools`, `tools.css`, `/one-level` or `/current-only` is edited; the two nodes
cooperate by the shape of their rules rather than by knowing about each other.
`/steady` wraps `apply` and this node wraps `paint`, so `/steady`'s stilling
runs *after* this node's work, and it only ever sets `animation`, which this
node has already set to the same value on the buttons it cares about.

**A ghost's `position` is set inline, not by the stylesheet.** `/dictate`
styles `.tool-button.ctrl { position: relative }`, a two-class rule that
outranks a one-class rule of ours; the first ghosts inherited it and appeared
hundreds of pixels from where their buttons had been. Inline beats every
author rule, so `leave()` writes `position: fixed` on the clone and the
stylesheet keeps only the layer and the inertness (at two classes, for the
same reason).

**Within a level, nothing moves.** `/steady`'s rule is kept exactly: on a
paint that does not change `open_tool`, and outside a climb's 220 ms, no
surviving button is given a transform and no rectangle changes. What such a
paint *does* get is the membership courtesy — `/undo/aside`'s arrow appears at
the far right the moment there is a step and vanishes when there is not, and
it fades in and out rather than popping. An appearance is not motion, and the
ask `/steady` answered was about a button that would not sit still.

**`prefers-reduced-motion: reduce` turns the whole node off** at the paint,
per paint (the setting can change while the app is open): no rectangles are
read, no transform, no fade, no ghost, and any ghost still on screen is taken
away. What is left is today's behaviour, snap and all.

**Named limit: undo's arrival still shifts its neighbours.** `/aside` places
undo with `margin-left: auto`, which takes the automatic right margin off the
last control, so the controls between the tool button and undo re-centre when
undo appears or goes. That is a same-level paint, so this node leaves those
buttons alone and they jump. Gliding them would mean moving buttons on a paint
that did not change the level, which is the rule `/steady` exists to hold; the
right answer is a rule about *membership* changes rather than level changes,
and it is not this ask.

## hostile cases

- **A second level change while a glide is running.** Proven in the rig: a
  second climb 90 ms into the first. The gliding buttons are destroyed by the
  next `innerHTML`, which cancels their transitions; their settle timers then
  fire on detached nodes and set inline styles nobody can see. The ghosts
  would survive, so a *climb* (never a continuation) begins by removing every
  ghost still fading: a fresh level owns the screen. The new glide measures the
  old row as it actually was mid-flight, so a button caught half-way glides on
  from where it had got to. Four ghosts mid-flight, one after the second
  climb, none once settled, and the row lands exactly where a plain paint
  would have put it.
- **Nothing to glide from.** At boot, and on the first paint after
  `/world-cache` has held the seam shut, the old row is empty. This node then
  does nothing at all and the stylesheet's mount slide plays, which is the
  right animation for a row arriving out of nothing. Only a *departure* would
  be wrong there, and with no old row there is nothing to ghost.
- **No row in the new DOM.** `/profile-first`'s gate and `/veil` can leave a
  paint with no `.toolbar`. The buttons did not go anywhere — the row was
  withheld — so nothing is ghosted, both windows are closed and the level is
  recorded.
- **A ghost taking a tap.** It cannot: the stylesheet gives it
  `pointer-events: none`, and its `data-ev` and `data-ctl` are stripped before
  it is attached, so `/loop`'s one delegated listener has nothing to match.
  Proven with `elementFromPoint` at a ghost's centre, which answers with the
  toolbar beneath it.
- **Ghosts accumulating.** Each ghost holds its own removal timer *and* is
  removed by the next climb; the list is spliced in both paths, so a ghost is
  never removed twice and never left behind. The list length is bounded by one
  row, and the rig reads zero after every case.
- **A key appearing twice in one row.** The first occurrence claims the old
  rectangle; a second button on the same key is treated as an arrival and
  fades in. It never steals another button's rectangle.
- **A throw inside the glide.** The paint has already happened by then. The
  whole of the work is inside a `try` whose `catch` does nothing, so a glide
  that fails costs an animation, never a frame.
- **Node unticked.** The fragments leave with it; `paint` is the loop's
  unwrapped seam again, `/steady` still stills, and the row snaps as it does
  today — measured: 90 ms into a climb the unticked build already has every
  button at its destination, where the ticked build has 👤 still travelling.

## code description

`glide.index.js` takes `/loop`'s `paint` seam by replacing the property at
load — `/keep`'s and `/map`'s idiom, never a timer-installed wrapper. Being
the newest link on that seam, its measurement before the swap is ahead of
every other wrapper's work and its measurement after is behind all of it, so
the rectangles it compares are the ones a finger saw. The wrapper reads
`prefers-reduced-motion` first and, when it is set, does nothing but record
the level, close both windows and clear any ghosts.

`snap()` records, for every identified button in the row, the element, its
viewport rectangle and — only while a glide or a fade is actually running —
its computed opacity. The toolbar is `position: fixed`, so those coordinates
are also where a fixed ghost belongs.

`run(was)` decides the level, whether it climbed, whether either window is
still open, and what has arrived or left; a paint with none of those is
returned from untouched, which is `/steady`'s rule. Otherwise it cancels the
mount slide across the row, opens or continues the windows, and gives each
button its journey.

`travel(b, dx, dy, o, ms)` is the FLIP and the fade in one: transition off,
transform to the old offset and opacity to the old value, one forced reflow so
that state is a painted frame, then the transition on and both to their
resting values. It leaves `fm-glide-moved` on a button that moved — the trace
a test without a frame sampler can read — and `fm-glide-move` / `fm-glide-in`
only while the motion lasts. `leave()` is the same shape on a stripped, inert
clone appended to `document.body`.

`settle()` clears the inline transform or opacity on `transitionend`, with a
timer behind it for the transition that never fires. It deliberately does not
restore `animation`: putting the stylesheet's rule back on an element still in
the row would start the mount slide over.

`MS` (220 ms) and `EASE` are constants at the top of the object — one place to
retune, and the hook a per-button duration would extend.

`glide.index.css` carries one rule: the ghost's layer (4 — `/reorder`'s, above
the map and below the panel's shade), `pointer-events: none` and no margin, at
two classes so it is not outranked by the two-class `.tool-button.ctrl` rules
other nodes write.
