# context

*the world-object: one namespace of feature-scoped slots, lifecycle carried in the type*

> (transcripts/2026-08-21-hybrid.md#p21)
> ok. so the crux of the design is how we check, at runtime, whether a particular feature is enabled or disabled. I had the idea of turning the "context" into an actual object, of which the "global composed functions" are actually methods. The context object contains all feature-scoped state (eg. enabled, logging verbosity, font size, grid spacing, server port) which means that we can keep multiple contexts alive at any point and switch between them trivially.

## user

For agents: a feature declares its own tunables and state in a `/sidecar` file beside its spec, `<name>.slots`, one `/slot` per line — `size: u32 = 40 (user, last-write, inherit)`. Name, Rust type, default, then the three columns every slot answers for itself: **scope** (`global` | `group` | `user` | `device`), **merge** (`last-write` | `crdt-sum` | `better` | `none`), **inherit** (`inherit` | `own`). `#` comments and blank lines are allowed; a malformed or duplicate line is a link error naming the file and line, never a silent skip. What you get is a field on the emitted `Context` struct, typed `Slot<T, Scope…, Merge…, Inherit…>` and defaulted in `Context::fresh()`. Unticking your node takes the field with it. A declaration that contradicts itself — a device-scoped slot asking to inherit — fails to compile.

## spec

The `/context` is the object a user's whole situation lives in: what is on, how it is tuned, and what is in flight. It is one **flat** namespace — no config/state taxonomy — of feature-scoped `/slot`s, each declaring its own attributes (#p24, ruled #p26). `enabled` is not special; it is the one slot every feature will eventually have.

This node is **rung 1** and builds only the foundation: nodes declare slots, the linker collects them, and a typed `Context` struct with defaults is emitted into every place's crate. Nothing constructs or reads a `Context` at runtime yet, so behaviour is unchanged by construction. The composed functions becoming methods on this object (#p21), the runtime `enabled` gate, per-user Contexts on the server, snapshot/replay and the overlay chain are later rungs.

Lifecycle is enforced through generic vars, ash's ruling at #p26: the attributes live in the type, so a mis-declared slot is a rustc error rather than a doctrine violation found in review. Rung 1 enforces one such rule for real — `ScopeDevice` permits only `Own`, so `device, …, inherit` does not compile — and leaves the remaining combinations open until later rungs earn them.

Declarations live in a `/sidecar` file rather than a spec stanza. This resolves the open question at #p21 (edge 2) the same way `order.md` and the verbatim-library convention already resolve it: linker-read, machine-shaped, and outside the regex chain parser's reach, so a slot's Rust type may carry commas and generics that a spec stanza would have to fight.

Field names are `<node name>_<slot name>`, not the flattened node path, even though the path is what the design conversation sketched. Node names are already tree-global and linker-enforced unique (fm.md), so they disambiguate two nodes declaring the same slot name just as well — and unlike a path they survive a regroup, which agents.md makes a law: regrouping carries grouping and selection only and can never rewire behaviour. A path-derived field name would rename fields on a pure regroup. The full path is recorded in a comment above each field, so provenance is not lost.

Open, and deliberately not answered here: context versioning across builds (an update must migrate live Contexts), the default scope for undeclared legacy state during the transition, and whether hypothetical contexts need a lifetime discipline.

## glossary

- **slot**: one named, typed, defaulted piece of a context, carrying its own scope, merge discipline and inheritance. A constant earns a slot when it earns a variable; the declaration line *is* the promotion.
- **sidecar**: a machine-shaped file beside a node's spec that the linker reads and the chain parser never does — `order.md`, `deps.toml`, `<name>.lib.rs`, and now `<name>.slots`.

## code description

`context.slots`: this node's own declarations. One real slot, `heartbeat: u32 = 0 (user, last-write, own)`, so emission is provable without touching another node.

`context.lib.rs` (verbatim library): the slot family. `SlotScope` / `SlotMerge` / `SlotInherit` are the three attribute traits, each carrying a `TAG` string; the zero-sized marker types (`ScopeUser`, `MergeCrdtSum`, `Inherit`, `Own`, …) implement them. `Slot<T, S, M, I>` holds the value and phantoms the attributes, with a `const fn new` for defaults and `attrs()` for machinery that must walk slots generically. `Permits<I>` is where lifecycle enforcement lands: a scope declares which inheritance modes it accepts, and `ScopeDevice` accepts only `Own`.

`tools/fmlink.py`, slot parsing (scaffolding, per the standing arrangement that the linker holds mechanism and the node holds design): `FeatureCode` reads each node's `*.slots` files into declarations, failing loudly with file and line on a malformed line, a duplicate name within one node, or an unknown scope/merge/inherit word. An empty file is fine. The same slot name on two different nodes is fine — the field namespace disambiguates.

`tools/fmlink.py`, `emit_context` (scaffolding): if any composed verbatim library contains the hook token `pub struct Slot<`, the linker emits `struct Context` — one field per collected slot, typed via the wrapper family, with the declaring node's path in a comment — followed by `Context::fresh()` carrying the declared defaults. Emission happens after the verbatim libraries and before the chains, into the one composed body every place's crate is cut from. Without the hook, nothing is collected and nothing is emitted, so the source is byte-identical to a build predating this node.

`tools/fmlink.py`, `contributes` (scaffolding): a `.slots` file now counts as composition material, so a node that declares slots and nothing else must still cite a provenance anchor.
