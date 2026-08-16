# scope
*scoped variables: declare where data lives, use it without thinking*

> (transcripts/2026-08-14-fm-spec.md#p131)
> Local -> User -> Group -> Global is a nice hierarchy; we can always add to that as we want. I think the right thing to do is to put all that into an easy-to-use generic that we can just use without thinking too much about.

## user

For agents: `let taps = Var::<u64>::global("tap_count");` then `taps.add(&mut s, 1)` in an update extension — replication, identity-keying, broadcast and arrival are inherited. Choose the scope that names who should see it; that choice is also the privacy boundary.

## spec

The scope lattice as a Rust generic. `Var<T>` names a piece of loop state with a scope — `Var::local`, `::user`, `::group`, `::global` — and a key. `get`/`put` read and write the local replica; `set` (register, last-write-wins) and `add` (counter, op-fold) also queue a sync message. Everything after that is generic machinery owned by this node: the server keys its store by scope instance (user scope keys by the sender's cookie-proven identity, stamped into messages by `/messaging`), publishes a scoped `VarUpdate`, and every instance's generic `update` extension writes arriving updates into state — **a feature using a Var writes no sync code at all**. Implemented now: `Local` (never leaves the device), `Global`, `User` (server-filtered broadcast — one user's values cannot reach another's instances). `Group` is declared structure: local ops work, sync is an honest error awaiting the membership model. This node is also the first *verbatim library* — `scope.lib.rs` is full Rust (generics, traits) the linker emits as-is, outside the chain machinery.

## glossary

- **scoped variable**: loop state declared with a scope (device ⊂ user ⊂ group ⊂ everyone); the scope is both replication domain and access boundary.
- **register / counter**: the two write semantics — `set` (last write wins) and `add` (operations fold, so concurrent increments all count).

## code description

`scope.lib.rs` (verbatim library): the `Scope` enum and `Var<T>` — `get`/`put` via serde against the state JSON; `set`/`add` also append `VarSet`/`VarAdd` messages to the `_send` outbox (`add` is provided for counter-shaped `Var<u64>`).

`scope.rs`, server half: a `handle_msg` /extension/ claiming `VarSet`/`VarAdd` — resolves the store key per scope (`global.<key>`, `user.<from>.<key>`; group answers not-implemented), applies the write to a file-backed store, publishes a scoped `VarUpdate` to the matching audience, and returns the same update as the direct reply.

`scope.rs`, client half: an `update` /extension/ that writes any arriving `VarUpdate` into state under its key — generic arrival, so consuming features never handle it.
