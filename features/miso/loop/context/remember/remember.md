# remember

*a world survives the restart, and an idle one costs nothing until it is asked for*

> (transcripts/2026-08-21-hybrid.md#p32)
> just keep going until all rungs are built and working - test as you go

## user

For agents: the server writes down every change to a user's `/context` and
replays it when it needs the world again. Set a var, disable a feature, restart
the server — everything is where you left it, per user, with nobody else's world
touched.

What is written is the **op**, in the shape rung 6 put on the wire:
`{path, name, op, value}`, one JSON object per line, in the order they were
applied, in one file per user under `~/.miso-context/` — beside the auth state,
not in `/tmp`, because `/tmp` does not survive a reboot. Set `MISO_CONTEXT_DIR`
to put it somewhere else, which is how a test server keeps its own state.

Worlds are loaded lazily: a user's log is replayed the first time that user is
touched, not at startup, so starting the server costs nothing per user who has
ever existed. A world nobody has touched for an hour is dropped from memory
again — safely, because the log holds everything it was built from — and the
next request rebuilds it. `MISO_CONTEXT_IDLE_MS` changes the hour.

`GET /diag/context/log` tells you what persistence is doing: the directory, how
many worlds are resident, how many are known, the thresholds, and — the reason
it exists — whether writing has been failing and what the last failure was.
Persistence degrading never takes the server down; it gets loud, not fatal.

Device-scoped vars are honestly absent from all of this. They never reach the
server, so the server has nothing to write down; their persistence is the
client's problem and not this rung's.

## spec

Rung 5 gave each user a world and left two things in the risk list: a restart
threw every world away, and no world was ever reclaimed. Both were the same
missing thing — a place to put a world that is not memory — and this rung is it.

**Placement, without a regroup.** `context` reaches six children here, which is
the top of fm.md's 4–6 range and not past it, so nothing is forced. The regroup
the rung-6 report proposed — *holding* (alive, per-user) against *changing*
(edit, enabled, converge) — is a real one and it should happen when the seventh
child arrives, as its own prompted event with its own byte-identical proof.
Folding it into a code rung would put two unrelated changes in one commit and
make the toggle proof answer a question nobody asked. `remember` is free as a
name; `resume`, `restore` and `drive` are taken.

**The log is the wire shape, and that is the whole design.** Rung 6 defined
`{path, name, op, value}` and a door that applies one — `Context::apply_op`,
which checks the verb against the declared merge. Recovery is that door, run over
a file instead of over a socket: `Context::fresh()`, then every record in order.
There is no second format, no schema to keep in step with the declarations, and
no code path that can persist something the wire could not have sent. A record
that names a var this composition no longer declares fails exactly as a wire op
would, with the same message.

**What each verb writes.** The log records the op **as applied**: the value for a
`set`, the delta for an `add`. It is tempting to write the resolved total for an
`add` instead, so that a duplicated record would be harmless — but a total is not
an op, `apply_op` would refuse it, and accepting it would mean a second door.
Instead the honest statement is that **replay reproduces memory exactly**,
including its mistakes: the server appends only after an op has been applied, so
the log is a faithful record of what happened rather than a corrected one. Since
a log is replayed whole or not at all, replay cannot duplicate an add by itself.
The residual hazard is entirely the wire's — rung 6 demonstrated that a retried
`add` is applied twice — and the log then records it twice, faithfully wrong.
Rung 6b's op ids fix it at the source; when they arrive the record should carry
the id, and replay should dedupe on it exactly as the wire will.

**Compaction is exact, not approximate.** A `set` assigns, so it supersedes
everything logged for its var before it; consecutive `add`s fold into one `add`
of their sum. Per var that leaves at most a `set` followed by one `add`, so a log
is bounded by the number of vars rather than by the number of edits. Ops for
different vars never interact — each arm of `apply_op` touches one field — so
grouping a log by var and preserving order within each var replays to the same
world as the original. Compaction happens on the next append after a log passes
`MISO_CONTEXT_LOG_MAX` records (512 by default), which keeps the common append
cheap and the rewrite rare.

Every write goes through a temporary file and a rename, so the log a reader sees
is always a whole one and a failed write leaves the previous version intact. The
loader is tolerant anyway — a line that is not a well-formed record is announced
and skipped, and the records around it still apply — because the thing that
corrupts a file is rarely the process that was careful about it.

**Loading is lazy and residency is explicit.** A second map beside rung 5's
table holds, per user, the time they were last touched and the cell their world
lives in. Presence in it means "this world has been replayed and is current".
`held_context()` is redefined once more: rung 5's link decides *which* cell, this
one decides whether that cell is current and rebuilds it if not. The decision
happens under one lock, so two threads meeting a cold world cannot both replay
it; the second waits and finds it warm.

The lock order is rung 5's table, then residency, then the cell, and it is never
taken the other way. That is why the residency map carries the cell rather than
looking it up again: eviction needs residency and the cell but not the table, and
a suffix of an order cannot deadlock against the order.

**Eviction, and what it honestly reclaims.** The sweep runs at the top of every
request — O(resident users) timestamp comparisons at human request rate — and
never evicts the world the current request is for. A background thread would need
a shutdown story and would evict worlds nobody was asking about.

What eviction does is empty a world, forget it is loaded, and let it go.
/Amended 2026-08-21 (#p56)./ It originally could not do the last of those: rung
5's cells were `&'static`, so a world could be reset but never freed, and
`GET /diag/context/log` reported `resident` falling while `known` stayed. The
cells are counted handles now, and a sweep drops residency's handle, rung 5's
table entry, and — through the `context_evicted` seam — the per-user dedupe
state `/overlay` keeps. Both maps hand their buckets back afterwards, because a
map that has held two hundred users otherwise keeps room for two hundred users.
Measured on 200 worlds: 147 KB in, 99.9% of it back, both counters returning to
two.

The world is emptied before the handle is dropped, so a request that is still
holding one sees a world with nothing in it rather than a stale one; its own
writes reach the log either way, and the next touch rebuilds from there. The
rule for anything else hung on `context_evicted`: it must be rebuildable from
the log, because that is all an evicted user leaves behind.

**Two write seams, because there are two ways to change a world.** Rung 6's op
path is one: `handle_msg` has already applied the op and answered, and a
`CtxUpdate` answer means it was accepted, so the record is appended after the
fact and a rejected op leaves no trace. Rung 3's tooling `POST /diag/context` is
the other, and it is the awkward one: it assigns through `set_from_json` rather
than through a merge, so the record it produces says `set`. That is exactly true
for every last-write var — all of them today — and for a `crdt-sum` var it is a
record replay will refuse and announce, because an absolute assignment is not
something `add` can express. The deeper finding is that rung 3's POST predates the
merge column and should itself become an `edit_op`; that is a prompt of its own,
queued beside the `sender_of` migration, and named here rather than smuggled in.

**The client place is untouched.** `held_context()`'s new branch is guarded by
the same empty identity that rung 5 used, and the wasm place never sets one — it
has no `route`. So no log is read, no residency is recorded, and no file is
touched on the client. Its own persistence is IndexedDB's job and out of scope.

**Counter records fold by replaying them (rung 7b).** A `counter`'s records
carry `[epoch, n]`, and the rules above would rescue exactly the records replay
is going to throw away — a stale reset, or an add minted before one. So a
counter's records are compacted by running `apply_op`'s own rule over them and
emitting the single `set` that lands where replay would land. That is exact by
construction rather than by argument, which is the same standard the other
kinds' folds are held to, reached by a shorter road.

## glossary

- **record**: one line of a user's log — the wire's op shape, as applied.
- **resident**: a world that has been replayed from its log and is current in
  memory. Eviction ends residency; the next touch restores it.

## code description

`remember.rs`, `held_context()` /extension/: the load seam. An identity that is
present makes its world current before the cell is handed back; an empty one —
the whole client, and startup — takes neither branch.

`remember.rs`, `route()` /extension/: the idle sweep, once per request, ahead of
everything; and `GET diag/context/log`.

`remember.rs`, `handle_msg()` /extension/ and `context_set()` /extension/: the
two write seams, each appending only after the change it records actually
happened.

`remember.rs`, `context_log_status()`: what persistence is doing — directory,
resident and known counts, both thresholds, and the failure count with the last
message.

`remember.lib.rs` (verbatim library): `context_dir` and `context_log_file` (every
byte outside `[A-Za-z0-9._-]` percent-encoded, so no key can name a path outside
the directory); `context_log_read` (tolerant, loud); `context_log_compact` (set
supersedes, adds fold); `context_log_append` (compact past the bound, temp file
and rename); the residency map with `context_reside` and `context_evict_idle`;
and `context_log_fail`, which makes a persistence failure loud and visible
without making it fatal.

`tools/fmlink.py`, `REMEMBER_HOOK` (scaffolding): this node emits nothing, but
declaring the hook lets a composition without the op methods fail by name rather
than as a rustc error inside a verbatim library.

## risks

**Eviction does not free the cell.** /Closed 2026-08-21 (#p56) by the `Arc`
refactor: 99.9% of 147 KB returned across 200 create-evict worlds, measured with
a counting allocator. The process's RSS does not fall with it — that is the
system allocator holding its arena, not memory the server is still using./

**Two processes on one state directory corrupt nothing and lose writes.** Every
write is a whole-file rename, so neither process can leave a torn file and a
reader always sees a complete log — but the second process's rename wins and the
first process's records are gone, silently. This is the double-start accident,
and the answer chosen here is "never corrupt, may lose", because the alternative
— a lock file — turns a stale lock after a crash into a server that will not
start, which is a worse failure for a thing whose job is to keep serving. The
right fix is upstream: one server per state directory, which is what the deploy
script should assert.

**The log is world-readable state on disk.** It sits beside the auth state and
inherits its directory permissions; a user's key is in the filename, which for a
session user is their phone number percent-encoded. That is no worse than the
auth state, and no better.

**Rung 3's POST is logged as `set`.** True for every var today, refused loudly at
replay for a `crdt-sum` one. The clean fix is migrating that route to `edit_op`.

**For rung 6b:** when ops carry ids, a record should carry the id too, and
`context_reside` should skip a record whose id it has already applied — which is
the same seen-set the wire will need, read from the log at load. That is what
finally makes a retried `add` harmless in both places at once.
