# alive

*the Context gets its first caller: each place builds one at startup, and the server shows you it*

> (transcripts/2026-08-21-hybrid.md#p30)
> ok, that looks fine. I'd rename "Slot" to "Var" but otherwise good. Let's go to the next step.

## user

For agents: a running server now has a real `/context` object, and you can look at it. `curl localhost:8095/diag/context` returns a JSON array with one entry per declared `/var` — `{path, name, value, scope, merge, inherit}` — where `path` is the declaring node's tree address and the last three are the attributes that node declared in its `.vars` line. Use it the way `curl localhost:8095/diag/readout` is used for the screen: to assert on state rather than infer it. Through the tunnel the route needs a valid session cookie; on localhost it is open, because that is the tooling case.

What this asks of your declarations: a var's Rust type must implement `serde::Serialize`. It is the snapshot walker that demands it — `serde_json::to_value` on each value — so declaring a type without it is a rustc error, and the linker's line map points that error back at the offending line of your `.vars` file rather than at generated code. Primitives, `String`, `Vec`, and anything deriving `Serialize` are fine.

## spec

Rung 1 built the `/context` and left it uninhabited: the struct and `Context::fresh()` were emitted into every place's crate and nothing ever called them. This rung gives `fresh()` its first caller and makes the result observable.

Each place constructs one `/context` at startup and holds it for the life of the process — the server before its accept loop opens, the client on boot. That is deliberately **one Context per process**, not one per user: the per-user table, the overlay chain, and switching between contexts are later rungs, and building them now would mean designing session plumbing before there is anything to put in it. The client holds its Context and exposes nothing yet — no UI, no readout extension.

The server carries the observation point: `GET diag/context` serves the snapshot. It follows the `/diag/readout` precedent exactly, including its screening rule — a Context holds user-scoped state, so tunnel traffic needs a valid session cookie while localhost stays open for tooling — and its route-matching rule, that paths arrive slash-stripped from `clean_path`, so the match is on `diag/context`.

The walker that produces the snapshot is generated, not hand-written, for the same reason the struct is: only the linker knows which vars exist in a given composition. `Context::snapshot()` is emitted beside `fresh()`, one entry per collected var, reading the three attribute strings back out of the marker types through `Var::attrs()` — so a snapshot cannot disagree with a declaration, because both come from the same type.

Snapshot emission is gated on its own hook rather than riding along with the struct, because it is the one thing in this rung that constrains what may be declared: `serde_json::to_value` imposes `serde::Serialize` on every var type. A composition that does not ask for a snapshot should not pay that tax, so the linker emits the walker only when a composed node's source carries the token `fm:context-snapshot`. Unticking this node removes the only asker, and with it the walker and the constraint; unticking everything leaves the emitted source byte-identical to a build predating the mechanism, as rung 1 requires.

Unticking `context` itself is quiet, and correctly so: unticking a node excludes its whole subtree, so this node goes with its parent and composes to nothing. That is the tree's containment rule doing its job, and the emitted source returns byte-for-byte to the pre-rung-1 baseline.

The case that *is* loud is an asker the containment rule cannot reach — a node outside `context`'s subtree carrying the hook while the var family is absent. That is a link error naming both halves, deliberately, rather than a rustc complaint about an undefined `Context` type from generated code that no one wrote.

## glossary

- **snapshot**: a context rendered as JSON — every declared var with its value and its three attributes. The context's answer to what `/readout` is for the screen.

## code description

`alive.rs`, `held_context()`: the place's one `/context`, built on first ask and held forever. The `static` lives inside the function body because the composition machinery carries functions, not free items, and it is a `std::sync::OnceLock` rather than a `thread_local!` because one composed body is compiled for both places — `OnceLock` is correct in the native server and costs nothing in the single-threaded wasm client.

`alive.rs`, `serve()` and `boot()` /extensions/: each forces the construction before handing on, so a running place always holds a Context whether or not anything has asked for one. Neither changes what the chain beneath it does.

`alive.rs`, `route()` /extension/ and `context_get()`: `GET diag/context` returns `snapshot()` as JSON, refusing tunnel traffic without a session cookie exactly as `readout` does. `context_get` also carries the `fm:context-snapshot` token that switches the walker's emission on.

`tools/fmlink.py`, `emit_context` (scaffolding, per the standing arrangement): `FeatureCode` now keeps each node's Rust source text so the linker can see the second hook. When a composed node asks, `Context::snapshot()` is emitted after `fresh()` — a `Vec<serde_json::Value>` built one var at a time, with the value line mapped back to its `.vars` declaration so the `Serialize` error arrives with the right address. When a node asks and the var family is absent, the linker fails naming the orphaned dependency.

## rung ladder

Named here so they are not mistaken for omissions: per-user Contexts on the server, the overlay chain that makes `inherit` mean something, snapshot/replay, the runtime `enabled` gate, and the composed functions becoming methods on the object (#p21).
