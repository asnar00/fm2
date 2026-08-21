# edit

*the context becomes writable, and an edit waits for the next turn*

> (transcripts/2026-08-21-hybrid.md#p32)
> just keep going until all rungs are built and working - test as you go

## user

For agents: a running place's `/context` can now be written, not just read.
`curl -X POST localhost:8095/diag/context -d '{"path":"miso/loop/context","name":"heartbeat","value":7}'`
returns `{"ok":true}` and the following `GET localhost:8095/diag/context`
reports `"value":7`. The `path` and `name` are exactly the two strings the
snapshot already prints for that var, so a GET tells you how to address
anything you want to set. Edits are **in memory only** this rung: restart the
server and every var is back at its declared default.

Errors are specific, and none of them leave a var half-written. An unknown
`(path, name)` comes back with what it got and the list of what exists; a
value serde cannot deserialise into the var's declared type comes back 400
with serde's own message and the var unchanged; a body that is not JSON, or
one missing `path`/`name`/`value`, comes back 400. Through the tunnel the
route needs a valid session cookie, exactly as the GET does; on localhost it
is open, because that is the tooling case.

What this asks of your declarations: a var's Rust type must now implement
`serde::Deserialize` and `Clone` as well as `Serialize`. The write path
demands the first, the `/turn` freeze the second. Primitives, `String`, `Vec`,
and anything deriving both are fine, and the linker's line map points a
missing-impl error back at the offending line of your `.vars` file.

In code, two primitives are what later rungs are meant to build on:
`with_context(|ctx| ...)` reads, `edit_context(|ctx| ...)` writes. Reach for
those rather than the held storage — they are where the `/boundary law` is
enforced.

## spec

Rung 2 made the `/context` observable. This rung makes it writable, and it is
the primitive the `enabled` gate and every future tunable will call: rung 4's
tick-wiring changes a var, it does not invent a way to change one.

The held storage becomes mutable. Rung 2 held a `OnceLock<Context>`, which is
construct-once-read-forever by design; this rung refactors `held_context()` to
hold a `OnceLock<RwLock<Context>>`. The cell is still initialised exactly once
and lives for the process; only its contents may now move. `RwLock` rather
than a cheaper single-threaded cell because the same composed body is compiled
for both places and the server is genuinely concurrent — one thread per
connection, since `serve/threads` — while the wasm client is single-threaded
and never contends, so the lock costs it an uncontended atomic and nothing
else. Refactoring rung 2's accessor rather than standing a second store beside
it is fm.md's refactoring rule doing its job: behaviour intact, extension point
extracted, one storage for one context.

Reads and writes go through two accessors, `with_context` and `edit_context`,
both taking a closure and both generic over its return type. They live in this
node's verbatim library because that genericity is exactly what the chain
parser cannot express — the same reason the `Var` family lives in one. They
are deliberately shaped as the tree-global primitives later rungs call: a
composed function that wants to know whether it is enabled will call
`with_context`, and a tickbox that turns it off will call `edit_context`.

**The `/boundary law`** is the constraint this rung exists to honour, paid for
by the old design's lesson 3: an edit made while an event is being processed
must become visible only to *subsequent* events. A gate that re-read the
context halfway through a turn could run the first half of an update enabled
and the second half disabled, and a replay of the same event would then not
reproduce the same result — which costs the blackbox its truthfulness.

The mechanism is a `/turn`: a bounded stretch of processing that freezes the
context it opened under. `context_turn_begin()` clones the live context into a
thread-local view; every `with_context` inside the turn reads that view;
`context_turn_end()` drops it. An edit writes the live context immediately —
so it is durable the moment the POST returns — but no turn already open can
see it, and the next turn's freeze is where it becomes visible. The freeze is
thread-local because a turn belongs to one thread: one request thread on the
server, the one wasm thread on the client.

That mechanism is strictly stronger than the request-atomicity argument the
server alone could have made. Request-atomicity says "a request is short, and
the edit applies at its end" — which holds for the editing thread and says
nothing about the other seven threads that were mid-request when the write
landed. The freeze covers those too: each of them is reading its own clone.
The price is one `Context` clone per request, which is why `Clone` joins the
demands on a var type.

On the server the turn is one request, opened and closed inside this node's
`route` link. On the client the turn is one event, opened and closed around
`on_event` — the Elm update the loop already runs, so the boundary lands
exactly where the Elm law says it should. `boot()` is deliberately not a turn:
it is construction, and no edit can exist before it.

The law holds in both places, observed rather than argued. On the server a
POST landing on another thread during an in-flight turn left that turn's reads
unchanged and became visible to the next one. On the client — a single thread,
so the only mid-turn edit possible is a re-entrant one, a composed function
editing the context it is running under — an `edit_context` inside a turn was
invisible to every later read of that same turn and visible to the next, in
the wasm binary itself.

The write route is `POST diag/context`, matched slash-stripped as `clean_path`
delivers it and screened by the same rule as the GET — a context holds
user-scoped state, so the tunnel needs a cookie and localhost stays open. The
body names the var the way the snapshot names it, `{path, name, value}`, and
the handler does no per-var knowledge of its own: it hands the three to the
generated `Context::set_from_json()`, which is the only thing that knows what
was declared.

Generated, because only the linker knows which vars a given composition has —
the same argument that made the struct and the snapshot walker generated. The
write path rides its own hook (`fm:context-set`) rather than broadening the
snapshot's, because the two impose different taxes on a var's type: a
composition that only wants to look at its context should pay `Serialize` and
not `Deserialize` and `Clone`. Three hooks for three separable costs keeps
rung 1's rule intact — nothing asking in the composition means nothing emitted
and byte-identical source.

Edits are in memory and per-process. A restart returns every var to its
declared default, and the server's context is still one object rather than one
per user. Persistence and the per-user table are rungs 5 and 6; putting either
here would mean designing the store before the thing it stores can be changed
at all.

Two things this rung deliberately does not do, so they are not mistaken for
oversights. It does not gate anything: nothing yet reads a var to decide
whether to run. And **the write lock is not re-entrant** — an `edit_context`
closure that called back into `with_context` outside a turn, or into
`edit_context` again, would deadlock the thread. The read path is built so it
can never be the deadlocking half (it copies rather than holding the guard
across the caller's closure), which leaves one rule to remember rather than
two; rung 4's gates read, so they are on the safe side of it by construction.

## read-your-own-writes (found by rung 7's migration)

`edit_context` writes the live context AND replays the same closure against this
thread's frozen view. That is not a hole in the `/boundary law` — it is what the
law always meant, made precise by a case rung 7 found.

Migrating `/ask` put two chain links in one turn on the same var: `ask` appends
the wish, `birthplace` stamps it. Both read the frozen `"[]"`, both wrote, and
the second overwrote the first — a read-modify-write across links, which the
JSON state used to make safe by passing the value down the chain. The context
did not, because a turn's own edits were invisible to it.

The fix replays the turn's OWN closure against its OWN view rather than
re-cloning from the live world, which is the distinction that matters: another
device's edit is still invisible until the next turn, and this turn can see what
it just did. The replay changes the value and nothing else — `in_context_mirror`
is true while it runs, and whatever queues ops consults it, because the same
change to a second copy is not a second change and must not reach the wire
twice.

The cost is that a closure handed to `edit_context` is now `Fn` rather than
`FnOnce` and runs twice, so it must be a function of its arguments and not of
anything it consumes. Callers that moved a `serde_json::Value` in now clone it.

## glossary

- **turn**: one bounded stretch of processing that runs under a single frozen
  context — a request on the server, an event on the client. Reads inside a
  turn see the context the turn opened under.
- **boundary law**: an edit to a context becomes visible only to turns that
  begin after it. The turn already in flight completes under the context it
  arrived under, which is what keeps `(context, event) → context` replayable.

## code description

`edit.rs`, `route()` /extension/: the server's `/turn` boundary and the write
route. It opens the turn, dispatches — `POST diag/context` to `context_set`,
everything else down the existing chain — and closes the turn after the
response is built. It is the outermost link because this node is the newest in
the composition, which is what makes one request one turn.

`edit.rs`, `on_event()` /extension/: the client's turn boundary, around the
Elm update. It changes nothing about what the turn computes.

`edit.rs`, `context_snapshot_json()` /extension/: inside a turn the snapshot
reports the frozen view, so a `GET diag/context` describes what the request is
actually running under; outside one the chain beneath answers unchanged.

`edit.rs`, `context_set()`: the POST handler. It carries the
`fm:context-set` token that switches the generated write path on; screens
tunnel traffic for a cookie; parses the body; and rejects a missing `path`,
`name` or `value` before touching the context. The var-specific work is all
`set_from_json`'s.

`edit.lib.rs` (verbatim library): the accessors and the turn. `with_context`
reads — the frozen view inside a turn, a copy of the live value outside one,
never holding a lock while the caller's closure runs. `edit_context` writes
under the write lock, so concurrent edits serialise and the last one stands.
`context_turn_begin` / `context_turn_end` / `in_context_turn` maintain the
thread-local frozen view.

`alive.rs`, `held_context()` /refactored/: the held cell is now
`OnceLock<RwLock<Context>>` — same one-per-process construction, mutable
contents. `context_snapshot_json()` is extracted from `context_get` so the
read path is a chain this node can extend; rung 2's behaviour is unchanged
when this node is unticked.

`tools/fmlink.py`, `emit_context_set` (scaffolding, per the standing
arrangement): under a third hook, `impl Clone for Context` field by field and
`Context::set_from_json()` — a match keyed by `(node path, var name)` that
deserialises into the declared type, with every generated line mapped back to
its `.vars` declaration so a missing `Deserialize` or `Clone` arrives with the
right address. The miss arm names what it got and lists what exists. As with
the snapshot hook, an asker without the var family is a link error naming both
halves.

Unticking this node takes the route, the write path, the `Clone`, the turn and
both accessors with it; rung 2's GET and snapshot are untouched, and the only
composed difference from before this rung is the `alive` seam itself. One
detail worth knowing rather than fixing: with this node off, `POST
diag/context` falls through to rung 2's `route` link, which matches on path
alone, so it answers with the snapshot instead of 404. Nothing is written —
the machinery that could write is gone.

## risks

The server turn is one request only because this node's `route` link is the
outermost one. A future node with a newer anchor that wraps `route` would run
outside the boundary; its reads would fall to the copy-the-live-value path,
which is safe but not frozen. If that happens, the turn wiring moves to
whichever link is outermost — or `route` gains an explicit turn-owning
wrapper below the extension point.
