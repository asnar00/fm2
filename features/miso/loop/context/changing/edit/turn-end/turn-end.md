# turn-end

*the turn's end belongs to the turn, not to whichever link happens to be newest*

> (transcripts/2026-08-21-hybrid.md#p56)
> let's fix all residuals next.
> *(the finding of record is notes.md, "the late link's ops", measured by the
> undo worker on the two-instance rig)*

## user

For agents: an op minted anywhere in a turn now leaves in that turn. It used
to leave only if the feature that minted it happened to be older than
`/converge`, which is a property nobody could see and nobody was told about;
two shipped features had already fallen through it. Nothing about how you
write a feature changes — `edit_context`, `edit_layer`, `edit_op` are what
they were. What changes is that they now work the same at every depth.

## spec

Two defects with one root, named in notes.md and measured on the rig:

`/converge` drained the op outbox at its own link of the `update` chain, and
`/overlay` stamped it at its own. Both were the outermost link when they were
written. Composition order is provenance order, so every node authored since
wraps them, and an op minted by one of those nodes is minted **after** the
drain — it waits in the outbox for the next event, or on a device where
nothing else happens, forever. `/square-taps` shipped with this: pressing n²
left the count unchanged on the device that pressed it and never reached the
user's other phone. `/undo` shipped a shim for it — its own calls to
`ctx_ship_ops`, `ctx_stamp_outbox` and `context_layer_begin` at the end of its
link — which worked, and re-armed the trap for whatever node came next.

And `/payload` called `context_turn_begin()` with no matching end. With
`/first-turn`'s depth counter that re-froze nothing and left the depth one
higher after every event, so a client's own frozen view was taken at the first
event and never retaken. Nothing observably misbehaved, because
`edit_context`'s read-your-own-writes keeps that view current — but the
boundary law was being upheld by a mirror rather than by the boundary.

**The fix is a moment, not a position.** `/edit`'s `on_event` link gains one
named call, `context_turn_close`, between the event and the drop of the
freeze. That moment is guaranteed to be after **every** link of the `update`
chain, whatever its provenance, and the guarantee is structural: `update` is
called from inside `on_event`, so every `on_event` link is outside every
`update` link by construction. It is not a matter of who is newest, and a node
authored next year cannot get in front of it.

This node is what fills that moment. It ships the turn's ops with
`/converge`'s `ctx_ship_ops` and stamps them with `/overlay`'s
`ctx_stamp_outbox` — the same two functions, unchanged, called from a place
whose position is a fact about the shape of a turn rather than about the order
of prompts. Their own links keep the work that genuinely belongs at a link
(applying an arriving record, retyping one for the layer) and lose only the
part that had to be last.

**A turn that minted nothing pays almost nothing.** The phase reads
`context_op_pending()` and returns the payload untouched when it is zero. That
is one integer read on the overwhelming majority of events; only a turn that
actually changed something parses and rewrites the payload.

**The paint's freshness is answered separately, and in the right place.** The
other half of the same trap was that a layer edit made by a late link landed
after `/payload`'s re-freeze and the frame showed the old number. Moving the
re-freeze here would not have fixed it — this moment is after the paint, not
before it. The honest fix belongs to the layer's own write path, and it is the
one `edit_context` has had since rung 7: `edit_layer` now replays the caller's
closure against the turn's frozen layer view. A turn sees what it just wrote,
at any depth, without anybody re-freezing anything; another device's edit
stays invisible until the next turn. `/payload` keeps its `context_layer_begin`
for the one thing the mirror cannot cover — a record arriving from the server
for the layer, which is written straight to the live cell — and loses the
`context_turn_begin` that was leaking.

**What this does not cover, said plainly.** The phase is the client's. The
server's turn is a request, and this moment has no analogue there because
nothing on the server mints ops: every server-side write is an `apply_op` or a
`set_from_json`, both of which assign directly and queue nothing. If that ever
stops being true, `route`'s close is where the same phase goes.

And `/payload`'s republish still runs at `/payload`'s link, because it must:
the bridged page keys are read during the paint, by Rust as well as by
JavaScript, so republishing at the turn's end would be a frame too late. That
leaves one position-dependence in the family — a node newer than `/payload`
that edits a **bridged** var would paint one stale frame. There is no such
node: all six bridged vars (`open_tool`, `tools_catalog`, `asks`,
`update_policy`, `update_accepted`, `update_ticks`) are written only by nodes
older than `/payload`. It is on the parked register with that as its revisit
trigger, and the structural answer if it ever fires is a pre-paint moment
emitted by the linker into the `render` entry, which is a bigger change than
this one earns today.

**The invariant, and how it is checked.** An op minted at any link depth ships
in its own turn; begins and ends balance; the frozen view is taken exactly
once per outermost turn. The last two stopped being arguable when
`context_turn_stats()` started counting them.

## glossary

- **turn-end phase**: the moment after every update link and after the paint,
  before the turn's freeze is dropped, where work that has to be last belongs.

## code description

`tools/fmlink.py`, `check_turn_end` and the ninth hook pair (scaffolding, per
the standing arrangement that the linker holds mechanism and the node holds
design): `fm:turn-end-phase` marks this node as the provider,
`fm:turn-end-required` marks a node that has handed work to it, and a
composition with the second and not the first fails naming both. It is the
only hook in the family that emits nothing — what it protects is not a
generated function but an ordering.

`turn-end.rs`, `context_turn_close()` /extension/: the phase. Early-exits on
an empty outbox; otherwise unwraps the event payload, ships and stamps the
turn's ops into its state, and rewraps. A payload that is not the loop's
`{state, html}` is returned exactly as it came.

`edit.rs`, `on_event()` /refactored/ and `context_turn_close()`: the seam. The
base is the identity, so `/edit` alone behaves as it did.

`edit.lib.rs`, `context_turn_stats()` and the counters: begins, ends, freezes
and the current depth, on this thread. Three increments in the existing
begin/end/freeze paths, and the instrument that makes the balance invariant
observable rather than argued.

`edit.lib.rs`, `context_mirror_set()`: the read-your-own-writes flag, raisable
by the layer's write path as well as by `edit_context`, so a replay there is
recognised by whatever queues ops.

`overlay.lib.rs`, `edit_layer()` /refactored/: gains the mirror. Its closure is
`Fn` rather than `FnOnce` now and runs twice, the same tax `edit_context`
already charges.

`converge.rs`, `update()` /refactored/: stops shipping. `ctx_ship_ops` is
untouched and is what the phase calls.

`overlay.rs`, `update()` /refactored/: stops stamping. `ctx_stamp_outbox` is
untouched and is what the phase calls.

`converge.lib.rs`, `context_op_peek()`: the outbox, read without draining.
Ops now sit there for the whole turn instead of moving into `state["_send"]`
mid-update, so a link that wants to know what this turn changed — `/undo`
does — looks here.

`payload.rs`, `update()` /refactored/: the unmatched `context_turn_begin()` is
gone; the layer re-freeze stays.

`undo.rs`, `update()` /refactored/: the shim calls are gone, and its capture
reads `context_op_peek()` instead of `state["_send"]`.

## risks

**Its tickbox is compose-time only, and should stay that way.** The gate hook
gates a chain link whose first parameter is the loop state; this one's is an
event payload, so `--coverage` reports the node as reaching nothing at runtime.
That is the right answer rather than an oversight: a runtime disable would do
exactly what the link error above exists to prevent — queue ops that nothing
drains, in a running app, for one user. The compose-time refusal is the only
switch this node should have.

**The phase is one link of a chain that nothing else extends.** If a second
node ever extends `context_turn_close`, order between them is provenance
order like everything else — which is fine for independent work and wrong for
work that must follow the shipping. Anything that must be after the ops go out
should call `ctx_ship_ops` first rather than assume.

**`edit_layer` closures now run twice.** Every caller was already written for
`edit_context`'s identical rule, and the three in the tree clone rather than
move, but a future caller that moves a value into the closure will not
compile — which is the right failure.

**Unticking this node is a link error, deliberately.** `/converge` handed its
shipping over; without the phase it would queue ops that nothing drains, and
the app would look completely normal while nothing synced. That is the worst
shape a failure can have, and it is the one shape rustc cannot see — every
other dependency in this family removes a generated function, so its absence
is a compile error. So the linker is told instead: `/converge` carries
`fm:turn-end-required`, this node carries `fm:turn-end-phase`, and a
composition with the first and not the second fails by name, the way unticking
`loop/context` has failed since rung 7. The toggle is provable in the only way
that is honest here — off, and the build stops and says what to tick.
