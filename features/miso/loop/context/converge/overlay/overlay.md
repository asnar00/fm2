# overlay

*a value can live above the user, and an op can only land once*

> (transcripts/2026-08-21-hybrid.md#p32)
> just keep going until all rungs are built and working - test as you go

## user

For agents: a var declared `(global, ...)` is now one value for everyone. It
lives in a shared layer rather than in anybody's world, an op on it reaches
every connected instance, and the linker no longer refuses the declaration.

Vars declared `inherit` now genuinely inherit. A var you have never written is
**absent** — it falls through to the shared layer, and only to its declared
default if the layer has nothing either. The moment you write it, it is yours
and stops following. A new op verb, `clear`, gives it back:

```
curl -X POST localhost:8095/msg -d '{"type":"CtxOp","data":{
  "path":"miso/loop/tap","name":"enabled","op":"clear"}}'
```

So `enabled` — `user, last-write, inherit` since rung 4 — behaves the way the
ladder's last line always meant: set it once on the shared layer and every user
who never overrode it is affected, while anyone who did keeps their own answer.
`GET /diag/context` now reports `present` (is this world's own value set?) and
`resolved` (what a reader actually gets) beside the raw `value`.

Ops now carry an **id**, and an id is only ever applied once. The retried `add`
that rung 6 demonstrated double-counting is counted once now, across a restart
as well, because the log carries the id too. An op with no id — from a client
too old to stamp one — is applied unguarded, exactly as before.

`group` is still refused, and the message says why: the overlay chain resolves
straight through the group layer because nothing in this system can yet say who
is in a group.

## spec

This is the rung the shared tap counter's migration blocks on, and it is two
things at once: values that live above a user, and ops that land exactly once.
They are one rung because the first needs a wire and the second is what makes
that wire trustworthy.

**Placement: under `converge`, and NOT after a regroup.** The brief asked for the
holding/changing regroup first. I built it, measured it, and did not ship it,
because it is no longer behaviour-neutral and the reason is worth recording.

The chains survive it — I checked, and the relative order of every context node
is unchanged, because `change/*` and `keep/*` happen to sort in the same order
as the bare names did. What does not survive is **rung 4**: every node gets an
implicit `enabled`, grouping nodes included, and a grouping node's `_on()` joins
the conjunction chain. Regrouping therefore adds two vars to every user's world
and reroutes six predicates through them: `alive_on()` becomes
`alive_enabled && keep_on() && context_on()`. The generated diff is 74 non-path
lines, which is exactly the number agents.md's law says should be zero
("regrouping carries grouping and selection only and can never rewire
behaviour").

That is a real doctrine fork and it belongs to ash, not to a code rung. Either
the law loses its absoluteness — and a group becomes a legitimate thing to
switch off, which is arguably a feature — or rung 4 stops giving code-free
grouping nodes an implicit `enabled`, which would restore byte-identity at the
cost of that switch. So: no regroup here, and this node lands under `converge`,
which had no children and is the honest parent anyway. Rung 6 made a user's
worlds agree with each other; this rung makes them agree with a layer above them
and makes the agreement reliable. `context` stays at six.

**Presence, and where it lives.** Inheritance needs "never touched" to be a
thing a var can be. So the linker emits a `Present` record — one bool per var,
mirroring the Context's own fields — and a `present: Present` field beside them.
A var declared `own` starts present, because it has nothing beneath it to fall
to and is always its own answer; a var declared `inherit` starts **absent**,
holding its declared default, which is what makes the question askable.

The tidy home for that bit would be a field on `Var`, and the brief said so. It
is not there because `Var` is rung 1's verbatim library, and a bit added there
would sit in every composition whether or not it wanted overlays — a permanent
seam, the third in three rungs. A parallel record generated under this node's own
hook keeps the property exactly as toggleable as the feature that needs it, costs
one bool per var, and reads as named fields rather than as an index. The
trade is encapsulation for the toggle law, and the toggle law is doctrine.

**The resolved read is the migration.** Every read of a var moves from
`.value` to a generated `<field>_get()`, which falls through the overlay chain:
this world's own value if present, then the group layer, then the `_global`
layer, then the declared default. The group step is a comment rather than a line
because nothing can say who is in a group — when membership exists it lands
between two existing lines and changes nothing else.

Rung 4's gates are the read that matters. `<node>_on()` now reads
`<node>_enabled_get()`, so a value set once on the layer reaches every user who
never overrode it. That is the whole point of the rung: it is what turns
"disable a feature" from a per-user act into something an operator can do for
everybody without touching anyone's own choice.

A `global`-scoped var never consults the user's own field at all — its authority
is the layer. Every user still carries a field for it, unread. That is
deliberate: `Context` stays one shape, which every generated walker (snapshot,
`Clone`, `set_from_json`, `edit_op`, `apply_op`) depends on, and the cost is one
unread field per user per global var. A second struct shape would have saved a
bool and paid for it in every one of those walkers.

**The layer is a table entry, and it is not a user.** It lives in rung 5's table
under the key `_global`, which no cookie and no `?user=` can spell — real keys
are `phone:`- or `local:`-prefixed. Being an ordinary entry is what makes it
persist and evict for free: rung 6a's residency and log machinery treat it like
anybody's world, so the layer survives a restart and reclaims like the rest.

**The turn freezes twice.** A resolved read consults two worlds, so both must be
frozen at the same boundary or a value could fall through halfway through an
event. This node's `route` and `on_event` links are outermost, so the layer
freezes *before* rung 3 freezes the user's own world and thaws after it. Inside a
turn a resolved read takes no lock at all, on either world. The one rule left is
the one rung 3 already stated in its own terms: do not call a resolver from
inside `edit_context` on the layer, because that path holds the write lock. Every
read path on both places runs inside a turn, where nothing is locked.

**Routing, not a second door.** An op for a global-scoped var is applied to the
layer by the simplest available means: this node's `handle_msg` link sets the
thread's identity to `_global` for the duration of the call, and rung 6 applies
the op and rung 6a logs it, both addressing the layer because that is who the
thread now is. No interception, no duplicated apply, no second log format. Only
the relay is new — a layer op is published to the `global` audience that
`/messaging`'s `wait_filter` already delivers to everyone, rather than to one
user's.

`clear` is the exception, and it is handled here rather than passed down,
because rung 6's merge column knows only `set` and `add` and would reject the
verb by name. A clear on an `own` var is refused for the same reason a clear is
meaningful at all: there is nothing beneath it.

**A CtxUpdate now says which layer it belongs to and whether the var is still
present** — `at` and `present`, defaulted to `user` and `true` so a record from a
build that predates them reads exactly as it did. The client applies a layer
record to its own copy of the layer, and a `present: false` record by clearing.
Rung 6's link, running inside this one, will also have written a layer record
into the user's own field; for a global var that field is never read, so the
write is dead ballast rather than a bug — and saying so is cheaper than
intercepting it.

**Op identity is stamped at the outbox.** An id is `<instance nonce>.<counter>`,
and it is stamped by this node's `update` link onto every unstamped `CtxOp` in
`state["_send"]` — after rung 6's link has drained the turn's ops into it. One
place knows about identity, nothing else changes, and a message the transport
has to re-send carries the id it was stamped with the first time, which is
exactly the property dedupe needs.

The nonce has to be minted at runtime, and a wasm place has no clock and no
entropy source this composition can reach — `getrandom` is compiled with
`custom` and nothing registers a source. So the client **asks**: `init()` queues
a `CtxHello` through the same outbox as everything else, the server answers with
a `CtxNonce` minted from its pid, the clock and a counter, and the reply becomes
an event exactly as `Join`'s does. Until the answer arrives the client stamps
nothing and its ops are unguarded — a window of one round trip at boot, before
any tickbox can have been touched.

**The seen-set is bounded and log-primed.** The server remembers `(user, id)`
pairs in a FIFO of 4096 (`MISO_CONTEXT_SEEN_MAX`), and a repeat is
*acknowledged and skipped*: the sender is told the op is in, so its outbox stops
retrying, and nothing is applied twice. The first time this process meets a user
it primes the set from their log, through rung 6a's own reader — so an op
applied before a restart is not applied again after one. That closes the demo
rung 6 shipped as a known hole.

What it does not close: an id evicted from the FIFO would be applied again if it
somehow arrived a fifth thousand ops later. At human edit rate that is a
retry-after-days, and the honest bound is the one written down rather than the
one pretended to.

## glossary

- **layer**: the `_global` world, whose values sit above every user's and are
  what an absent inherit var falls through to.
- **present**: whether this world has its own value for a var. Absent is what
  `inherit` means; `clear` is how a var becomes absent again.
- **resolved read**: `<field>_get()` — own value, then layer, then default.

## code description

`overlay.lib.rs` (verbatim library): `context_layer_cell` and the layer's frozen
view (`context_layer_begin`/`_end`/`context_layer`, the same shape rung 3 gave
the user's world); op identity (`context_instance`, `context_op_next_id`,
`context_mint_nonce`); and the bounded seen-set with `context_seen_mark` and
`context_seen_prime`. It carries the `fm:context-overlay` hook token.

`overlay.rs`, `route()` and `on_event()` /extensions/: the layer's turn
boundary on each place, outside rung 3's.

`overlay.rs`, `init()` /extension/: the `CtxHello` that asks for an instance
nonce.

`overlay.rs`, `update()` /extension/: adopts a `CtxNonce`, applies the layer half
of a `CtxUpdate`, and stamps the outbox.

`overlay.rs`, `handle_msg()` /extension/: mints a nonce for a hello, suppresses a
duplicate id, applies `clear`, and routes a global var's op to the layer.

`overlay.rs`, `ctx_relay()`: one `CtxUpdate`, published to the `global` audience
when it belongs to the layer.

`tools/fmlink.py`, `emit_context_presence` and `emit_context_resolve`
(scaffolding, per the standing arrangement): the `Present` record, the
`<field>_get()` resolvers, `scope_of` and `inherits`. `emit_gate_predicates`
takes the resolved read when this node is composed; `set_from_json`, `edit_op`
and `apply_op` mark presence and grow a `clear` arm; the snapshot gains
`present` and `resolved` additively.

`tools/fmlink.py`, `VAR_SCOPE_AWAITS` (scaffolding, retuned): `global` is no
longer refused when this node is composed, and is refused *because of this node*
when it is not. `group` stays refused, now naming membership rather than the
overlay.

`remember.rs`, `handle_msg()` /amended/: a record carries the op's id when the op
had one. A composition whose ops have no ids writes exactly the records it wrote
before, so rung 6a's logs are unchanged when this node is unticked.

## risks

**The regroup is blocked, and the block is doctrine.** Named in full above:
since rung 4, no regroup in this tree is behaviour-neutral, because grouping
nodes get an implicit `enabled` and a link in the conjunction chain. Ash needs
to choose: amend agents.md's law, or exempt code-free grouping nodes from rung
4's implicit var. Until then `context` cannot grow a seventh child.

**An id evicted from the seen-set is unguarded.** 4096 ops per process, FIFO. A
retry arriving after that many other ops would apply twice.

**An id-less op is unguarded by policy.** That is the stale-client case and the
client's own boot window, and it is the same exposure the system had before this
rung — but it is now the only one, so a client that never gets its nonce is
silently in the old world.

**The nonce round trip is a dependency on the transport being up.** A client
that is offline at boot stamps nothing until its `CtxHello` gets through. Its
ops queue in the outbox unstamped and are stamped only if they are still there
when the nonce arrives — which they will be, since the hello is first in the
same FIFO.

**Two publishes for a global op.** Rung 6's link publishes to the sender's own
audience and this one publishes to everybody; the sender's instances get both.
Both are absolute assignments, so it is idempotent — but it is one message more
than the wire needs, and a chattier fix is not obviously better than the
duplicate.

**For rung 7:** the tap counter can now be declared. `SyncVar::<u64>::global("tap_count")`
becomes `tap_count: u64 = 0 (global, crdt-sum, own)` on the `sync` node, and its
`add_op` becomes `edit_op`. `own` is right at global scope: the layer is the top
of the chain, so it has nothing to inherit from, and `fresh()` therefore gives it
its declared default present. That is the last thing the migration was waiting
for.
