# converge

*a user's worlds agree: an edit here is an op, and it reaches their other devices*

> (transcripts/2026-08-21-hybrid.md#p32)
> just keep going until all rungs are built and working - test as you go

## user

For agents: editing a context var on one of your devices now reaches your
others. Turn a feature off on your phone and your laptop's gates obey within a
moment or two — no reload, and whatever your laptop was in the middle of is
undisturbed. The server's copy of your world agrees too, so a device that was
asleep catches up when it next asks.

What travels is an **op**, not a value dump: `{path, name, op, value}`,
addressed by the same two strings the snapshot prints. Which verb an op uses is
not the caller's choice — it is the var's declared merge. A `last-write` var
speaks `set` and the op carries the new value; a `crdt-sum` var speaks `add` and
the op carries the delta, so two devices bumping the same counter at the same
moment both count.

Scope decides how far an op goes. A `user`-scoped var reaches your other
instances. A `device`-scoped var never leaves the device it was set on — no op
is produced at all. `global` and `group` are still refused at link time, and the
refusal now names what they actually await: the overlay chain, which no rung
owns yet.

`enabled` gets all of this for nothing, because it was always
`user, last-write, inherit` — which is why the sentence at the top of the ladder
("off for you only, on all your devices") is now half true. The tickbox that
would drive it is rung 8.

Two things this rung does not do. Contexts still do not survive a restart
(rung 6a). And a `crdt-sum` op that the outbox has to retry after a lost
response will be counted twice — the transport has no dedupe, and this rung
does not invent one; see the risks.

## spec

Rung 5 gave each user a world. This rung makes a user's worlds agree with each
other, and it is the rung where the `Var` family stops being a description and
starts being an API.

**The node is called `converge`,** not `sync`: `miso/loop/tap/sync` already
holds that name and names are tree-global (fm.md). Rung 5 hit the same wall with
`users` and answered it the same way. `context` now has five children, one under
the cap — the next one forces a regroup, and the honest split when it comes is
probably *holding* (alive, per-user) from *changing* (edit, enabled, converge).

**The write API lives on the marker impls.** `Var<T, S, M, I>` gains nothing
directly. Instead this node's library declares two traits and implements each
against one merge marker: `VarLastWrite::set_at` on `Var<T, S, MergeLastWrite,
I>`, and `VarCrdtSum::add_at` on `Var<u64, S, MergeCrdtSum, I>`. So `set_at`
exists on a var that declared last-write and nowhere else, and reaching for
`add_at` on it is a rustc error rather than a convention nobody enforces. This
is ash's #p26 ruling arriving at its destination: the declared discipline does
not merely describe the var, it *selects the machinery*. It is also the answer to
the absorption note's complaint that "today nothing stops a caller using `.set()`
on a counter" — from this rung on, something does.

Each write method does two things: mutate the local value, and queue the op —
but only if the declared SCOPE says the op should leave. `S::TAG != "device"` is
the whole test, and it is the same shape SyncVar has always had
(`if self.scope != Scope::Local`). A device-scoped var is fully writable and
produces no traffic.

**The linker picks the verb, because the linker read the declaration.**
`Context::edit_op(path, name, value)` is generated with one arm per var, and each
arm calls the method that var's merge earned. A caller — a tickbox, a probe, a
future chooser — names the var and the value; it cannot name the verb, and could
not get it wrong if it tried, because the arm that would be wrong does not
compile. `edit_op` answers with the resolved value.

`Context::apply_op(path, name, op, value)` is the arriving half, and it does
check the verb: an op whose word is not the one this var's merge speaks is
rejected with both words named. That check is the wire's version of the type
system's rule — the wire cannot be type-checked, so it is checked once, here,
against the same declaration. `apply_op` then assigns **directly**, without going
through `set_at`/`add_at`, which is what stops an applied op from queueing an
echo of itself.

**The transport is the one that already exists.** An op goes into
`state["_send"]`, which `comms/messaging`'s JS drains into a persistent outbox
and POSTs to `/msg`; the server's `handle_msg` chain claims the `CtxOp` type;
the reply and the relay travel back as events. Nothing new is built: no second
channel, no second endpoint, no second retry policy. This node depends on
`comms/messaging` being composed, and unlike rung 5's identity question that
dependency is honest — messaging *is* the transport, and a feature that needs to
send a message properly depends on the feature that sends messages.

One message kind, `CtxOp`, with the verb inside the record, rather than SyncVar's
two (`VarSet`/`VarAdd`). SyncVar splits by type because it has no declaration to
consult and the type tag *is* its dispatch. A context op's merge is declared, so
the verb is not a dispatch but an assertion — "the sender believes this var
sums" — and an assertion belongs inside the record it qualifies, where the
server can disagree with it.

**Whose world an op lands in is not in the op.** `handle_msg` runs inside
`route`, beneath rung 5's identity link, so `edit_context` already addresses the
sender's table entry. There is no user field on a `CtxOp` and there is nothing to
forge. The relay audience does come from `m["_from"]` — messaging's
`sender_of` — which is the four-digit tag with the collision this ladder recorded
at rung 5; so the *authority* (which world is written) is collision-free and the
*relay* inherits messaging's existing defect. Fixing it is the queued
`sender_of` migration, not this rung.

**What the relay carries is the resolved value, not the op.** `CtxUpdate` is
`{path, name, value}` after the merge, and the client applies it with rung 3's
`set_from_json` — assignment, no new generated code, and idempotent by
construction. That is what makes the reply and the relay both arriving at the
originator harmless, and it is the same discipline `scope`'s `VarUpdate` already
uses. A relay that carried the delta would double every counter on the device
that sent it.

**Arrivals honour the boundary law by construction.** The client applies a
`CtxUpdate` inside `update`, which runs inside a turn; `edit_context` writes the
LIVE context, and rung 3's freeze means the turn now running cannot see it. So
the event that carries a disable completes under the old gates and the next
event obeys the new ones — the same property rung 4 proved for a local edit, for
free, because the arrival uses the same primitive.

**Draining is the outermost link's job.** The same `update` link takes whatever
the turn queued and appends it to `_send`. It is outermost, so any op an inner
link produced is already queued by the time the drain runs. Ops made outside a
turn — by tooling, or by a probe — wait in the outbox and leave on the next turn,
which is what "the next event ships it" means.

**Enablement gates behaviour, not truth.** An op for a var belonging to a
disabled node still applies: `apply_op` is keyed by var, and knows nothing about
`<node>_on()`. This is deliberate and it is what makes re-enabling work at all —
a disabled node's world must keep receiving the edits made elsewhere, or turning
it back on would show a stale value. Enablement decides whether behaviour runs;
it does not decide whether the world is true. The one place enablement does bite
is this node's own `enabled`: turn `converge` off and this client stops both
sending and receiving, which is #p4's "nothing exempt" applied to the syncer
itself.

**The `counter` kind: a sum that can be reset (rung 7b).** `crdt-sum` cannot be
reset — that is not an oversight in this system, it is what a grow-only counter
is — and three shipped tap tools reset, double and halve a counter that `sync`
sums across devices. Under SyncVar both verbs were available and the result was
already lossy: a `.set` racing an `.add` silently dropped the add, with no way
to know it had happened. So the merge column grows a fifth kind rather than
either breaking those features or pretending.

A `counter` var holds a `Counter` — an epoch and a sum. An `add` sums within the
current epoch and ships the epoch it was minted under. A `set` opens a new epoch
and assigns; every add still in flight from before it then carries a stale epoch
and is **dropped on arrival**, loudly, on stderr.

The loss is deliberate and its direction is chosen: **reset wins.** A user who
presses reset means "from zero, now", and a tap they made a moment earlier
arriving afterwards to make the counter read 1 would be the surprising answer,
not the safe one. What changes from SyncVar is not that a race can lose an add —
it always could — but that the loss now has a rule, a direction, and a line in
the log saying which add went and why.

`counter` is the only kind that speaks two verbs, so it is the only one with
both `set_at` and `add_at` implemented against its marker, and the only one
whose local edits need two entry points: `edit_op` is the add, `edit_reset` is
the set. Everything else about it is ordinary — it rides the same op, the same
relay, the same dedupe and the same log as every other var.

Two things it does not fix. Epochs are minted locally (`self.epoch + 1`), so two
resets racing can mint the same number and the second is applied rather than
recognised as concurrent; the sums are then whichever arrived last, and the adds
of both are dropped either way. And a `counter` var's value type is `Counter`,
not `u64`, so a reader wants `.sum` — the type is the honest one and callers
say so.

## glossary

- **op**: one addressed change to a context var — `{path, name, op, value}` —
  whose verb is chosen by the var's declared merge, not by its caller.
- **resolved value**: what a var holds after an op has been applied. Relays
  carry this rather than the op, so applying a relay twice is applying it once.

## code description

`converge.lib.rs` (verbatim library): the outbox — `context_op_queue`,
`context_op_drain`, `context_op_pending` — and the two write traits.
`VarLastWrite::set_at` is implemented only for `MergeLastWrite`, `VarCrdtSum::
add_at` only for `MergeCrdtSum`, and each queues an op unless the declared scope
is `device`.

`converge.rs`, `update()` /extension/: the client's whole half. An arriving
`CtxUpdate` is applied to the live context through `set_from_json`; then the
turn's queued ops are appended to `state["_send"]`. It carries the
`fm:context-op` hook token.

`converge.rs`, `handle_msg()` /extension/: the server's half. A `CtxOp` is
applied to the sender's world by declared merge, and the resolved value is
published to that user's audience and returned as the reply.

`tools/fmlink.py`, `emit_context_ops` (scaffolding, per the standing
arrangement): under the fifth hook, `Context::edit_op` and `Context::apply_op`,
one arm per var, each reaching for the write method its declared merge earned —
and a shared `context_op_miss` so the declared-var listing is emitted once
rather than inlined into every miss arm.

`tools/fmlink.py`, `VAR_SCOPE_AWAITS` (scaffolding, retuned): the refusal text
for `global` and `group` no longer points at this rung, which has now happened.
What those scopes actually await is the overlay chain — a value living above the
user that a user's absence falls through to — and no rung owns it.

## risks

**A retried `add` op double-counts.** The JS outbox shifts its queue only after
a successful response, so a POST that the server processed but whose reply was
lost is re-sent. For a `set` that is harmless — the same value lands twice. For
an `add` it is not. The transport carries no op ids and this rung does not invent
them, because inventing a dedupe policy for one message kind while `VarAdd` has
the same hole would be building the wrong thing in the wrong place. Today the
hazard is unreachable: every var in the composition is `last-write`. It becomes
reachable the moment anything declares `crdt-sum`, and the honest fix is an op id
plus a seen-set on the transport, shared with SyncVar — which is work for rung 7
or the transport's own prompt.

**Ordering is arrival order.** Two instances setting the same var at once
converge on whichever op the server saw last, and both instances end up there
because both receive the same resolved value. That is what `last-write` means;
it is not a defect, but it does mean the loser's edit vanishes without notice.

**`converge` disabled is a client that stops hearing.** Nothing exempt (#p4)
cuts both ways: a user who disables this node stops receiving their own edits
from elsewhere. The server-side repair path (rung 3's POST, rung 5's `?user=`)
still works, and the node re-enables like anything else.

**For rung 6a (persistence):** the op record is now the shape a disk record
should reuse — `{path, name, op, value}` plus the user key rung 5 defined, in
arrival order. A log of applied ops per user replays into a `Context::fresh()`
exactly as the wire does, through `apply_op`, with no second format to keep
honest. Eviction belongs there too.

**For rung 7 (migration):** the mapping is now concrete. A `SyncVar::local` use
becomes a `device`-scoped declaration; `SyncVar::user(...).set(...)` becomes
`user, last-write` and `edit_op`; `SyncVar::user(...).add(...)` becomes
`user, crdt-sum`. The one that does not map is `SyncVar::global`, which the
linker currently refuses — so any global SyncVar caller blocks on the overlay
chain rather than on rung 7.
