# undo

*every toolset's control row carries an undo, and it works because the world is one object*

> (asks#1787346956331)
> All Toolsets should have an undo button
> *(a field ask, filed from the field on 2026-08-21, birthplace `taps @
> miso/loop/tap/counter`, proposal approved in the ask box — stamped building
> on the asker's phone)*

## user

Open any tool and its control row now ends with a **↶**. Press it and the
last change you made in that tool, on this device, is undone — three taps
then undo leaves you on two; ×2 then undo puts the count back where it was;
n² then undo the same. The undo travels like any other edit, so your other
signed-in devices follow it within a moment. Press undo again and you get
the change back: an undo is itself a change, so undoing it redoes it.

The button is there in every tool, including tools that do not exist yet —
nothing about it knows what a tap is. When there is nothing to undo in the
tool you are looking at, the button is shaded and does nothing. Each device
remembers its own last ten steps, and only its own: undo on your phone
reverts what you did on your phone, never what you did on your laptop —
that laptop has its own undo button for that.

What it deliberately leaves alone: the feature list's tickboxes (those are
not a toolset, and switching a feature back on silently would be answering a
question nobody asked), which tool is open, and anything a person did on
another device or a tool did on the server.

## spec

The ask arrived as **"All Toolsets should have an undo button"**, filed from
inside taps. The literal reading — a button on every toolset, not just this
one — is also the only one the system can honour cheaply, and it is the one
built. That is a judgement made here rather than handed back to the asker
(agents.md: an ask is built and shipped, and the judgement is documented in
the node).

**Undo is generic because the world is one object.** Since the contexts
ladder, every piece of a user's situation is a declared `/var` and every
local change to one is an `/op` — `{path, name, op, value}` — carrying its
own merge discipline. So "the last change" is not a per-tool concept that
each tool must implement: it is the ops one turn minted, and "put it back"
is the inverse op for that var's declared merge. A tool written next month
gets undo by editing vars, which is the only way it could change anything
anyway. There is no per-tool code in this node and no seam for a tool to
fill.

**Capture reads the outbox, not the edit.** A local edit queues its op in
`/converge`'s outbox. This node notes how long that queue was when the event
arrived and reads everything past that mark when the event is done: those are
exactly this turn's local ops, whichever feature made them. They are still in
the outbox at that moment — `/turn-end` drains it after the paint, which is
after this link. Intercepting at the point of edit would
have been the other option and was rejected — the queue function is a
verbatim library another node owns, so wrapping it is not available, and
more importantly the outbox is the one place every local edit already ends
up, which is precisely the property that makes this work for tools that do
not exist yet.

**The prior comes from the frozen view.** Before the event runs, the node
takes `Context::snapshot()` through `with_context` — the turn's frozen view,
which is what the turn is running under, so the values in it are the values
the user could see. Each captured op looks its var up there and keeps three
things: the `resolved` value before the edit, the var's declared merge, and
its declared scope. Nothing about the prior is guessed and nothing is read
after the fact, which is what makes an undo exact rather than approximate.
The snapshot also answers the read-modify-write case for free: an earlier
link's edit is mirrored into the frozen view by rung 3's read-your-own-writes,
so a second edit of the same var in the same turn is still one step whose
prior is the pre-event value.

**One turn is one step.** An entry holds every var the event changed, so
undoing reverts what the person did, not what the machinery did on the way.
A turn that touched one var twice contributes one change with the pre-event
prior.

**The inverse is a real op, chosen by the declaration.** `last-write` vars
are put back with `edit_op(prior)`; `counter` vars with
`edit_reset(prior.sum)`. Merges with no reversible write (`crdt-sum`,
`none`) are filtered out at capture time, so a recorded step never promises
something the undo cannot deliver. Which world receives the op comes from
the same snapshot: a `global`-scoped var's authority is the layer, so its
inverse goes through `edit_layer`, and everything else through
`edit_context`. Because the inverse is an ordinary op it inherits everything
ops already have — the outbox, the op id, the server's dedupe, the relay to
the user's other instances, the log. Nothing about undo is a second
mechanism.

**Undoing a counter is a reset, and a reset wins.** The tap count is the
`counter` merge, so putting it back to 2 opens a new epoch at 2. Any add
still in flight from another device — a tap made a moment ago that has not
landed — carries the old epoch and is dropped on arrival, loudly, exactly as
`/reset-taps` and `×2` and `n²` already behave. So an undo that races
another device's taps loses those taps. This is not a defect introduced
here: it is what reset-wins means, argued in converge.md, and it is the same
answer every other control in that row gives. Said plainly because it is the
one place undo is lossy.

**Redo is not a feature.** The inverse op is minted inside the same turn
that handles the undo press, and capture runs after it, so the undo is
recorded as a step like any other. Pressing undo again reverts the revert.
This costs no code and no second stack, and it means "undo" and "redo"
cannot drift apart — a redo is only ever the undo of an undo.

The consequence, stated rather than discovered later: **undo is one level
deep.** Pressing it repeatedly oscillates between two states instead of
walking back through history, because every press files a step of its own.
That is what the ask asked for, in as many words, and it is the behaviour a
single button can carry honestly — a button that walked backwards and a
button that redid would be two buttons. So the ten-deep stack is not ten
levels of history the button can reach: it is what lets several tools' last
steps coexist, and what bounds the memory. Going further back is a different
ask, and it would want a redo stack rather than this one.

**The stack is per-device machinery, not a var.** It is a thread-local `Vec`
in this node's verbatim library, deliberately, and the alternative — a
declared `(device, none, own)` var — was weighed and rejected. A var would
have bought snapshot visibility and toggling with the node; against that:
`none` has no write API, so writes would have to poke the field directly,
which is the one thing the merge column exists to prevent; the write would
land inside a turn that is already inside `edit_context`, which is not
re-entrant; and the visibility is illusory, because the stack that matters
lives on the client, where there is no `/diag/context` route to read it
with. The deciding argument is the trusted-base rule from the world-object
design: the machinery that records and syncs contexts lives *beneath* the
context and is not a slot on it. An undo stack is a record *of* edits to the
world, not a part of the world. `/converge`'s op outbox is the exact
precedent and has the exact same shape.

Thread-local also gives device-locality for free, which is what the ask
wants: the stack lives in the one wasm thread of one instance, so it cannot
see another device's edits, and another device's undo cannot reach it. On
the server each request thread has its own empty one, which is the honest
answer to "server-side tooling edits are not undoable".

**The stack is bounded at ten and drops the oldest.** The eleventh step
evicts the first, never the newest — the step most likely to be wanted is
the one just made. A full stack does not fail, refuse or clear; it forgets
its far end, which is the only failure mode a bounded history is allowed.

**The filter is the tool that was open, recorded when the edit was made.**
Every step remembers which tool was open at the moment of the edit, and the
button takes the newest step tagged with the tool now open; other tools'
steps are stepped over rather than disturbed. The alternative the design
brief proposed — a path-prefix match of the op's node against the open
tool's owning node — was tried against the ask's own example and does not
work: the taps tool is registered by `miso/loop/tap/counter`, but the count
it shows is declared at `miso/loop/tap`, and when `/sync` is on it is
declared at `miso/loop/tap/sync`. Neither var is inside the tool's subtree,
so a prefix filter would leave the button the ask was filed from permanently
dimmed. Recording the open tool is exact where the prefix is a guess, needs
no tool-to-node map at runtime, and keeps working when a tool reaches for a
var that lives anywhere in the tree. Edits made while no tool is open are
not recorded at all — there is no button to reach them with, and not
snapshotting is also what keeps the cost off the launcher's events.

**What is excluded, and why.** `enabled` is skipped: the chooser is not a
toolset, its ticks have their own clear semantics (an untick means *off for
me, everywhere*, and re-tick means *clear*, not *set true*), and an undo
button in some unrelated tool quietly flipping a feature back on would be a
worse surprise than no undo at all. `open_tool` and `tools_catalog` are
skipped because navigation is how you reached the tool, not something you
did in it — without the exclusion, leaving a tool and coming back would put
an inert "go back to this tool" step on top of the thing you actually wanted
undone. Another device's edits are excluded by construction (their ops never
enter this device's outbox), as are arriving `CtxUpdate`s, which are applied
by assignment and queue nothing.

**The late link's ops — no longer this node's problem.** For one build this
node shipped and stamped its own ops and re-froze the layer at the end of its
link, because `/converge` drained at its own link and this node is newer than
it. That worked and re-armed the trap for whatever node came next, so the fix
moved where it belongs: `loop/context/edit/turn-end` ships and stamps at the
turn's true end for every link at every depth, and `edit_layer`'s
read-your-own-writes shows a layer edit to the paint whoever made it. The
shim is gone from here; `/square-taps` stays fixed with this node unticked,
which is how you can tell the fix is real.

**Cost.** One `Context::snapshot()` per event, and only while a tool is
open. That is a serialisation of every declared var (about 130 today) to
produce a value most events do not use. It is bounded, it is off the boot
path, and it buys a capture that needs no cooperation from any other node;
the cheaper shape — a per-var prior read, once the op is known — needs a
generated `get_json(path, name)` that does not exist yet, and is the obvious
first optimisation if event latency ever becomes a question.

## glossary

- **step**: one entry on the undo stack — the tool that was open, and every
  var the event changed with the value it held before. One turn makes at
  most one.
- **prior**: a var's resolved value immediately before the edit being
  recorded, read from the turn's frozen view.

## code description

`undo.rs`, `tool_controls()` /extension/: appends the ↶ button to the open
tool's control row, unconditionally — `tool_controls` is only called while a
tool is open, so no tool-id test appears here, and that absence is the ask's
"all toolsets". The button wears `dim` when the open tool has no step.

`undo.rs`, `update()` /extension/: the feature. It reads the open tool and
the pre-event snapshot, runs the chain beneath, claims `ctx_undo` (take the
newest step for this tool and apply it), and records what the turn changed.
Shipping, stamping and re-freezing are `/turn-end`'s.

`undo.rs`, `undo_record()` / `undo_var_before()` / `undo_skips()`: capture.
The first walks `context_op_peek()` past the mark taken on entry and turns each new
`CtxOp` into a change, one per var; the second finds that var in the
pre-event snapshot and reads its merge, scope and resolved prior, answering
null for a merge with no reversible write; the third names the three var
names undo declines to cover.

`undo.rs`, `undo_apply()`: the inverse. Per change, `edit_reset` to the prior
sum for a `counter` and `edit_op` of the prior value otherwise, through
`edit_layer` for a `global`-scoped var and `edit_context` for the rest.

`undo.lib.rs` (verbatim library): the stack — a thread-local `Vec` of steps,
`undo_push` with the ten-deep oldest-first bound, `undo_take` for the newest
step of one tool, `undo_has` for the button's shading, and `undo_stack_json`
for rigs.

`undo.css`: the shaded state of a control that has nothing to do, and the
squeeze guard. Taps is the crowded case — reset, ×2, −1, n² and now undo, six
chips with the tool's own icon where five fitted. At `/bigger-buttons`' 50px
that wants 340px and a 390px phone's toolbar has 296 between its insets, so
flexbox takes about seven pixels off each button's width; letting the height
follow keeps them squares instead of rectangles. The arithmetic is stated
rather than looked at, because an agent cannot judge how it looks — that
question goes to ash, and the guard is there so the worst case is a slightly
smaller row rather than a broken one.

`converge.rs`, `ctx_ship_ops()` /refactored/: the op drain, extracted from
`/converge`'s `update` link so a later link can ship what it minted. Its
behaviour there is unchanged.

## risks

**An undo of a counter drops concurrent adds.** Reset-wins, as above. The
window is the round trip; the direction is chosen and is the same one every
other control in the row already takes.

**The stack does not survive a reload.** It is in wasm memory. A reload is
a new device-session as far as undo is concerned, and the button is dimmed
until the person changes something. Persisting it would mean choosing where
— and a device-scoped var does not persist server-side either — so it is
left explicitly unbuilt rather than half-built.

**A step can go stale.** If another device changes the same var after your
edit, undo still writes your prior, which discards their change as well as
yours. `last-write` has no other answer; this is the same losing-edit-
vanishes property converge.md already names, reached from a different
direction.

**An inverse can fail silently.** `edit_op` answers `Result`, and a step
whose var has since left the composition, or whose prior no longer
deserialises, returns an error that this node discards — the step is spent
and the world is unchanged. That is the right shape for a button (an undo
that popped an error box would be worse), but it is a silence, and if a
second such case ever appears it wants a line on stderr rather than a
comment here.

**Untick and it is gone.** No button, no capture. Nothing else changes: the
stack is thread-local, the extension points fall through, and the op shipping
this node briefly owned belongs to `/turn-end` now, so unticking it costs
nothing but undo.
