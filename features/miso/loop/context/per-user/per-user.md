# per-user

*the server's one world becomes a table: each person's context is their own*

> (transcripts/2026-08-21-hybrid.md#p32)
> just keep going until all rungs are built and working - test as you go

## user

For agents: the server no longer holds one `/context`. It holds one per person,
made fresh the first time that person touches it, and a request only ever reads
and writes the context of whoever sent it. Disable a feature as one user and
nobody else's world moves.

Who you are comes from your session cookie, exactly as it does everywhere else
on this server — there is no way to name another user by parameter, and a
request that carries both a cookie and a `?user=` gets the cookie's world.

For tooling on localhost, where there is no session, `GET` and `POST
/diag/context` take `?user=<name>`:

```
curl -X POST 'localhost:8095/diag/context?user=alice' \
     -d '{"path":"miso/loop/context","name":"heartbeat","value":7}'
curl 'localhost:8095/diag/context?user=alice'   # heartbeat 7
curl 'localhost:8095/diag/context?user=bob'     # heartbeat 0, a fresh world
curl  'localhost:8095/diag/context'             # _local, untouched by either
```

The name is 1–64 characters of letters, digits, `.`, `_` or `-`; anything else
is a 400 saying so. These names live in their own namespace, so `?user=` can
never spell a real person's key.

The client is unchanged, and needed no change: a place that is one person's
device holds one context, which is that person's.

What this rung still does not do: **contexts do not survive a restart.** Every
world, including yours, is back at its declared defaults when the server comes
back up.

And one rule tightens: a `.vars` declaration may no longer say `global` or
`group`. Those scopes need the sync rung; the linker now refuses them by name
rather than storing them per user and pretending.

## spec

Rung 4 made a context worth having; this rung makes it *yours*. The mechanism
is small because rungs 2 and 3 left the right seams: the storage accessor is a
chain function, so it can be redefined rather than edited, and identity is
already a solved problem on this server.

**The table holds counted handles** — `HashMap<String, Arc<RwLock<Context>>>`,
and `held_context()` answers with one by value. /Amended 2026-08-21 (#p56)./
This rung originally leaked each entry (`Box::leak`) to keep the signature
`-> &'static RwLock<Context>` that thirty-six callers depended on, on the
argument that nothing would ever drop a world anyway; `/remember` then added
eviction and could reset a world but never free it. The handle is that fix: one
atomic increment per touch, and `context_forget` — called by eviction — lets the
last handle go. A request already holding one finishes against the world it
started with, which is exactly what a counted handle is for.

**Identity is thread-local, and set outside the turn.** This node's `route` link
is the outermost one — its provenance is the newest — so it runs before rung 3's
`route` opens the turn. It resolves the requester, stores that on the thread,
calls down the chain, and clears it on the way out. Because it is outside the
turn, the freeze rung 3 takes is a freeze of *the requester's* context, which is
the whole point: two users' requests can be in flight on two threads, each
frozen on its own world.

Thread-local rather than threaded through every signature, for the same reason
rung 3's frozen view is thread-local: a request belongs to one thread, and
`serve/threads` gives each connection its own. The clear on the way out is not
decoration — it is what keeps this correct if that ever becomes a pool.

**An empty identity means the process's own world**, and that is what makes the
client provably unchanged. `context_user_now()` starts empty and only a `route`
link ever sets it. The wasm place has no `route`; it has `boot` and `on_event`.
So on the client the identity is empty on every path, `held_context()` returns
`existing.held_context()` — rung 2's cell, the same object as before — and the
table is never even constructed. The client's one context *is* its user's, by
construction rather than by arrangement. Server startup takes the same branch,
which is why `serve()`'s warm-up still works with nobody logged in.

**Identity comes from the session, not from a new idea.** The cookie is asked
first, always, and it wins: `cookie_token` → `token_valid` → `token_phone`, the
same three steps every authenticated route on this server already takes. The key
is the full phone from the token rather than the four-digit `tag` that
`comms/messaging`'s `sender_of` derives, because a four-digit tag collides — two
guests whose numbers end the same would share a world, which for a blob store is
a latent bug and for a context would be a privacy failure. The key never leaves
the process: it is a HashMap key, never logged, never in a snapshot, never in an
error message.

That derivation is taken from `miso/users`, the node that owns session identity,
rather than by calling `comms/messaging`'s `sender_of`. Calling `sender_of`
would have been one line, but it would make per-user contexts depend on a
messaging feature being ticked — a foundational thing depending on a sibling
that merely happened to need the same string first. Two nodes now derive the
same identity from the same primitives, which is precisely the rule-of-two
signal that `sender_of` belongs in `miso/users`; that migration is a prompt of
its own, not a thing to do inside this one.

**Why `?user=` is localhost-only.** A parameter that could name a user would let
any logged-in person read and rewrite anyone else's world — the exact failure
this rung exists to prevent. So the parameter is only ever consulted when there
is no valid session at all, and never when the request came through the tunnel
(`r.tunnel`, the same signal rungs 2 and 3 screen on). Localhost with no session
is the agent's own instrument, and it is already fully privileged there: it can
set any var of any of its own users. To keep even that from reaching a real
person's world, tooling names are namespaced — `local:<name>` against
`phone:<number>` — so no string a parameter can carry will ever collide with a
session-derived key. The default, with no parameter, is `local:_local`: an
ordinary table entry like any other, not a special case in the code.

**Scope honesty.** Every var declared so far is `user`-scoped, and until this
rung that was aspiration — one process-wide object held them all. Now it is
true. The corollary is that `global` and `group` are not: a global var stored in
a per-user table behaves exactly like a user var, silently, and the point of
putting scope in the type was that a mis-scoped var should be caught rather than
discovered. So the linker refuses those two words by name, saying which rung
earns them back. The refusal is unconditional rather than tied to this node
being ticked, because it emits nothing: it rejects declarations that no
composition — with this node or without it — can currently honour, so it costs
no byte of output and no toggle.

`device` is accepted. The distinction that matters at this rung is leakage
*between people*, and device scope cannot leak: on the client, the place that
actually is a device, a device-scoped var is exactly right; on the server it
resolves to that user's entry, which is a containment of the value, not a
widening of it. Its imprecision — the server has no notion of which of a user's
devices is asking — is real and named in the risks.

**The one seam outside this node.** `?user=` needs a query string, and
`miso/serve` was throwing it away: `clean_path` stripped everything after the
`?` and `request` had nowhere to put it. So `request` gains a `query` field and
`parse_request` fills it from a new `query_of` — the other half of `clean_path`,
in the feature whose job is parsing the request line. This is agents.md's
"refactor the parent to create an extension point" rather than a feature edit:
nothing reads `query` unless a node asks for it, no route's behaviour changes,
and every future parameterised route gets it for free. It is also the honest
version of a trick that was available and rejected — this node could have
redefined `clean_path` to stash the query in a thread-local on its way past,
which would have needed no edit at all and would have given a pure string
function a hidden side effect that the next caller of `clean_path` would have
silently broken.

Because that seam lives in `serve` and not here, unticking this node leaves it
behind: the composed source returns to rung 4's byte for byte *except* those
three lines, exactly as rung 3 left the `alive` seam behind. The seam is inert —
a field nobody reads and a function nobody calls.

## glossary

- **context table**: the server's map from user key to that user's context, an
  entry materialised on first touch and held for the life of the process.
- **user key**: how the table names a person — `phone:<number>` from a session
  cookie, or `local:<name>` for localhost tooling. Two namespaces that cannot
  collide.

## code description

`per-user.rs`, `held_context()` /extension/: the storage seam. With no identity
on this thread it hands back the previous link's answer — rung 2's single cell,
which is the whole client and all of startup; with one, the table's entry for
that user, created fresh on first touch.

`per-user.rs`, `route()` /extension/: identity in, chain, identity out. The
outermost route link, so the turn rung 3 opens beneath it freezes the
requester's context. It also answers the one error this rung can give: a
malformed `?user=` on the context route is a 400 naming the rule.

`per-user.rs`, `context_user_of()`: the resolution order — valid cookie, else
(localhost only) the parameter, else the default. Returns empty for an
unauthenticated tunnel caller and for a malformed name, which are the two cases
that get no world of their own.

`per-user.rs`, `context_user_name_ok()` and `query_param()`: a bounded plain
name, and one parameter out of a raw query string.

`per-user.lib.rs` (verbatim library): the thread-local identity with its three
accessors, and the table — `context_table()` holding it, `context_of()` doing
get-or-create with the read guard explicitly scoped so the create path cannot
deadlock against it, and `context_user_count()` for tooling.

`serve.rs`, `request.query` and `query_of()` /refactored/: the request line's
query string, kept rather than discarded. Behaviour unchanged; nothing reads it
unless a node asks.

`tools/fmlink.py`, `VAR_SCOPE_AWAITS` (scaffolding, per the standing
arrangement): `global` and `group` declarations are refused with the file, the
line and the rung that earns them back.

## risks

**A user's world is never reclaimed.** /Closed 2026-08-21 (#p56): the entries
are counted handles and `/remember`'s eviction drops both this table's and
residency's, measured at 99.9% of 147 KB returned across 200 worlds. The rest of
this paragraph is the original risk, kept for the record./ Entries are leaked on
creation and the table only grows, so a server that meets many distinct users
accumulates one context each, for the life of the process. Each is small — a
hundred-odd bools and a couple of numbers — and cookie identity is bounded by
the guest list, so this is not urgent; but it is the same question as
persistence, and the rung that gives contexts a store should own eviction at the
same time. Naming it:
neither rung 5 nor rung 6 has it, and it should not be smuggled into rung 7's
migration.

**Restart resets every world.** Rung 3 deferred persistence and this rung
deliberately does not pick it up, because a per-user store is a different and
larger design than a per-process one. The ladder as written has no rung that
owns it; the honest place is a new rung between 6 and 7, after sync has decided
what a var op looks like on the wire — the same shape is what a disk record
wants.

**`device` scope is per-user on the server.** Accepted here because it cannot
leak between people, but the server genuinely cannot tell one of a user's
devices from another, so a device-scoped var declared today would be shared
across that user's phones. Nothing declares one; if something wants to, it needs
device identity first.

**The tag collision this rung stepped around still exists elsewhere.**
`comms/messaging`'s `sender_of` — and therefore `dictate/mirror`'s blob
storage — keys users by the last four digits of a phone number. Two guests
ending in the same four digits share a blob namespace today. Out of scope here,
but found here.
