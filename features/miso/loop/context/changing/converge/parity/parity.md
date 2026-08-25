# parity

*a fresh instance is handed its user's world, instead of reading defaults*

> (transcripts/2026-08-21-hybrid.md#p32)
> just keep going until all rungs are built and working - test as you go

## user

For agents: log in on a device that has never seen this server and it starts
with your real values, not the declared defaults — your update policy, your
asks, whatever you have turned off, and whatever the shared layer says. It
happens on the same join that `/join` already fires: boot, reconnect, and
coming back to the foreground.

Nothing to operate, and nothing new to wait for. The values arrive as ordinary
events a moment after the first frame, exactly as `/join`'s always have, and the
frame after that is correct.

What is deliberately NOT sent: a var you have never touched (so it keeps
inheriting from the layer, which is the whole point of never having touched it),
and a device-scoped var (it never left the device it was set on, so the server
is not an authority about it).

## spec

Rung 7 migrated seven vars out of the loop's JSON state and into the
`/context`, and in doing so took them out of the one place that told a new
instance about them. `/join` answers a `Join` with a snapshot of `/scope`'s var
store; a declared `/var` is not in that store, and nothing replaced the
snapshot. The only thing carrying a migrated var to an instance that has never
run before is `/messaging`'s broadcast backlog — **fifty entries, shared by
every user** — so a var whose last write has aged out arrives as its declared
default.

That was measured, not deduced (rung-7 worker, two-instance rig, own state
directory, one test user, a browser profile that had never seen the server).
With the backlog flooded past fifty: before the update migration a fresh device
read `update_policy` as the user's real `"fixes"`, delivered by `VarJoin`; after
it, the empty string, and `policy.index.js` fell back to `auto`. **A user who
chose "ask me" would have got automatic updates on their new phone.** The
already-shipped `asks` migration failed the same test the same way — a fresh
device reading `[]` where the user has a list. This rung is that hole closed,
and `notes.md` carries the finding.

**The trigger is the one that already exists.** `/join`'s `init` queues
`{"type":"Join"}` through the state outbox at boot, and `/resume` re-queues it
on foreground return and on the browser's `online` event — so boot, reconnect
and resume are already one act, and a queued Join already survives being
offline. Inventing a second trigger would have meant a second thing to remember
to fire on each of those three moments, and a second thing for the next
absence-shaped bug to have missed. It is also the honest rhyme: `/resume` exists
because of the fifty-entry hole, and so does this node.

**The records ride the reply that already exists.** `handle_msg` answers one
message, so a second reply is not a thing that can be returned. The context
records therefore ride the join reply as a sibling of `values`: `/join` reads
`data.values`, this node reads `data.ctx`, and neither knows about the other's
*field*. If the chain ever answers a `Join` with nothing — which is what will
happen when rung 8 deletes SyncVar's handler while the `Join` message itself
survives — this node builds the envelope rather than falling silent, so the
context's join outlives the payload it was born beside. It does **not** outlive
the trigger: unticking `/join` removes the only thing that sends a `Join`, and
the context join goes quiet with it. That is a dependency on a *message*, the
same kind `converge` has on `/messaging`, and it is stated rather than hidden.

**What a record is, and why it needs no id.** It is the shape an arriving
`CtxUpdate` already has — `{path, name, value, at, present}` — carrying the
**resolved value** of a var that is present. Rung 6 chose that shape for the
relay precisely because assigning a resolved value twice is assigning it once
(converge.md); a join inherits the property whole. So there is no op id, no
seen-set, and no dedupe policy here: applying the same parcel twice is applying
it once, and a parcel that arrives during a reconnect after one already landed
is harmless. Assignment also queues nothing, so a join puts **nothing** on the
wire and cannot echo a fleet into a storm.

An id-bearing `set` op was the other candidate and is strictly weaker: it would
need the seen-set to be safe, and the seen-set is bounded, so a join replayed
after the bound would double-apply — for a `counter`, wrongly. The resolved
value has no bound to fall off.

**A counter joins at its epoch.** A `counter` var's value serialises as
`[epoch, sum]`, so a record carries both and the write path assigns both. A
joining instance therefore lands in the epoch the fleet is in, and an `add` it
mints afterwards is minted under the right one — which is what stops a fresh
device's first tap being dropped as stale, or landing on top of a reset it never
heard about.

**Four silences, each a rule.** A **device**-scoped var is not sent: it never
left the device it was set on, and the server's per-user copy of it is a
containment, not an authority. An **absent** var is not sent: absent means never
touched, and the write path sets `present` — so a joiner told about it would
stop inheriting from the layer, which is the opposite of what the presence bit
is for. A **global**-scoped var is not sent from the user's world: its authority
is the layer, and the field every user carries for it is unread ballast.

The fourth is the mirror of the third, and the rig found it: the layer is a
`Context` like any other, so it carries a field *and a present bit* for every
var — including the `own`, user-scoped ones whose resolver will never look at
it. `own` means "answers from its own field and nothing else", so those layer
entries are unreadable by construction, and the first measurement of the parcel
showed five records of nobody's value riding in every join. They are skipped, so
the layer half sends what can actually be read from the layer: `global`-scoped
vars, and the fallbacks an `inherit` var resolves through.

**This also bounds the parcel**, and the bound is the answer to "what about a
user with two hundred vars": it is one record per var the user has actually
*touched*, plus one per readable layer entry — not one per var declared.
Measured on the rig, in a composition of 122 declared vars: a user with a
policy, an acceptance, a tick map, an ask list and a shared counter joins on
**6 records, 1730 bytes**. Switching 40 nodes off took it to **46 records, 5384
bytes** — one more record per newly-touched var, exactly and only. The growth
is in what a person has done, which is the right thing for it to be in, and it
is the same order the reply already carries for SyncVar's whole store, in the
same message, over the same POST.

**Where the client link sits is load-bearing.** `payload` re-freezes both worlds
and republishes the bridged keys *after* the chain beneath it, so a record
applied inside that link is in the state key a fragment reads before this very
paint. Applied outside it, the values would be true and invisible until the next
event. Node order is provenance-then-path and every node here cites the same
anchor, so the name decides: `parity` sorts after `overlay` and before
`payload`, which is exactly the position this needs — the same constraint
`payload` itself was named for, and the second time it has bitten.

**What the user sees in the gap.** The join reply cannot beat the first frame:
the message is queued during `init` and answered over HTTP, so `boot()` renders
before it lands. Every painted frame was recorded on a cold load of a fresh
profile: the first, at **73ms**, showed the declared default; at **118ms** the
reply landed and the frame showed the user's real policy. So the honest sentence
is *defaults briefly, then truth*, with the brief measured at 45 milliseconds —
and it is precisely what `/join` has always done for SyncVar values, which is
why `/veil` exists to decide whether a short wait beats a wrong frame. This rung
changes what is *in* the reply, not when it arrives; the gap is the one the
product already has, and no new one is introduced.

**The fifty-entry backlog stops being load-bearing for context vars.** It
remains the transport for live relays — an edit made on another device now
arrives that way and always did — but it is no longer the only thing that can
tell an instance what is true. Correctness now rests on a reply the joiner asked
for, and the backlog is an optimisation on top of it.

`/scope`'s own join path is untouched: `snapshot_vars`, the var store and
`data.values` behave exactly as before. Rung 8 deletes them wholesale, and when
it does it must keep the envelope this node rides — or rename both halves
together (see the risks).

## glossary

- **parity**: the state of a joining instance that has been told everything its
  user's world says — the point at which it can be trusted to show truth rather
  than defaults.
- **parcel**: the list of records a join reply carries; one per touched var,
  plus the present layer entries.

## code description

`parity.rs`, `handle_msg()` /extension/: the server half. It claims `Join`, lets
the chain answer, and hangs `data.ctx` on the reply — making the envelope itself
if the chain had nothing to say.

`parity.rs`, `ctx_join_records()`: the parcel. Both worlds are read through the
generated `snapshot()` — the same walker, the same two addressing strings and
the same serialisation `GET /diag/context` prints, so there is no second format
and adding a var to the composition needs no line here. Both reads are frozen
for the request, so the parcel is one consistent moment rather than a mix of
two. It carries the `fm:context-snapshot` token, which is the ask for that
walker.

`parity.rs`, `ctx_join_skip()` and `ctx_join_record()`: the four silences, and
the `CtxUpdate`-shaped record.

`parity.rs`, `update()` /extension/: the client half, claiming `VarJoin` and
applying each record — sitting between `overlay`'s link and `payload`'s for the
reason argued above.

`parity.rs`, `ctx_join_apply()`: one record through the door that exists. A
layer record goes to `overlay`'s `ctx_apply_update`, which is the one place that
writes the layer; a user record is rung 3's `set_from_json`, which is the same
assignment an arriving `CtxUpdate` already uses and which sets `present`.

## risks

**This node depends on `/overlay` being composed**, for `ctx_apply_update` and
the layer accessors. That is honest rather than incidental — a join that could
not carry the layer would be a join that quietly loses every global var — but it
is a link-time dependency, and unticking `overlay` alone stops the build rather
than degrading. That is the same bargain `context.md` names for the whole
subtree from rung 7 on.

**Rung 8 must keep the `Join` message and the `VarJoin` name.** This node
supplies its own envelope if the chain answers nothing, so deleting `/join`'s
*handler* is safe — but the client link claims an event typed `VarJoin`, and the
trigger is the `Join` that `/join`'s `init` and `/resume` queue. Delete either
name and this node goes quiet **silently**, because an absent join looks exactly
like a join with nothing to say. Whatever rung 8 does to SyncVar, the join
message, its reply type and `/resume`'s two re-fires have to survive it or be
renamed in the same commit. Named here so the summit does not have to
rediscover it; measured too — with `/parity` unticked the reply's `data` has
only `values`, and the rung-7 failure comes straight back.

**A user who has disabled this node does not get joined**, and therefore does
not get told that they disabled it. Self-consistent, and the same property
`converge` names for itself (#p4, nothing exempt): the server-side repair paths
— rung 3's POST, rung 5's `?user=` — still work, and the node re-enables like
anything else.

**A join racing a concurrent edit resolves last-write, per var.** The parcel is
frozen at the moment the server answers; an edit that lands afterwards arrives
as its own relay and wins, because both are assignments of a resolved value. The
reverse order — parcel applied after a newer relay — would put the older value
back until the next write. Both worlds are frozen together so the parcel is
never torn *within itself*; what it is not is a transaction against the future.
The window is one HTTP round trip, and the loser is a value the user set on
another device in that window.

**Nothing here bounds the reply's size.** One record per touched var is a real
bound and today it is small, but a var holding a large `String` (`asks` grows
without limit, and `payload.md` already watches it) is carried whole on every
join, and a join happens on every foreground return. If that starts to show, the
fix is a version or hash per var so a joiner can say what it already has — which
is a different rung, and this one should not pretend to it.
