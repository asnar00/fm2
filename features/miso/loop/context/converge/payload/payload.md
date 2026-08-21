# payload

*a migrated var goes back into the page at the key it used to live at*

> (transcripts/2026-08-21-hybrid.md#p32)
> just keep going until all rungs are built and working - test as you go

## user

For agents: a `.vars` declaration may name a page key.

```
asks: String = "[]".to_string() (user, last-write, own) js:asks
```

The value lives in the `/context` like every other var — synced, layered,
persisted, resolved — and the bridge writes its **resolved** value back into the
loop's state at `asks` before every paint, including the first one. A fragment
that does `JSON.parse(s.asks)` keeps working, unedited.

Two vars may not claim one key, and a `js:` column in a composition without this
node is a link error naming both. Nothing has a `js:` column yet: the migration
that needs them is rung 7.

## spec

Rung 7 went to look at the migration and came back with a wall: **six of the
seven migratable vars are read out of the loop's state by JavaScript**, and a
fragment cannot call `with_context`. `open_tool` alone is read by six of them.
Moving those values into the Context without this node means a blank panel, a
missing toolbar, and — for `asks` — a page that no longer shows a user the
things they asked for.

The bridge is the smallest thing that unblocks it: **the value moves, the key
stays.** A var names the key it used to live at, and the linker emits one line
per bridged var writing its resolved value back into the state before the paint.
No fragment changes, no fragment needs to know, and the fragments can be
migrated later one at a time — which is the point of doing it this way rather
than editing eleven JavaScript files in a rung whose subject is Rust.

This is deliberately the *cheap* half of #p23. The full arrival — a page reading
a context object rather than a state key — is a cleanup for after the ladder,
and it will be easier from here because by then every bridged key has exactly
one declared owner, which is a thing the current state string cannot say.

**Placement, and why the name.** It sits under `converge` beside `overlay`,
because `converge` is where values move between places and this moves them one
more hop — out of the world-object and into the payload the page renders from.
It is called `payload` rather than `bridge` for a reason that matters: node
order is provenance-then-path, so a node named `bridge` would sort BEFORE
`overlay` and its `update` link would be inner — republishing before an arriving
`CtxUpdate` had been applied, and painting the value the event was carrying
away. `payload` sorts after, so its link is outermost and it republishes last.
The name is honest and the ordering is load-bearing; both are worth saying out
loud, because the next node under `converge` inherits the same constraint.

**The paint re-freezes — the LAYER, and only the layer (corrected 2026-08-21).**
Rung 3 froze the context for the duration of a turn, which is what makes
`(context, event) → context` replayable. But the render that follows the update
is not part of the update — it is what the user sees — and it should show what
is true now, including an edit the event itself carried. So this node's `update`
link, after the chain beneath it has finished, re-opens the layer's frozen view
from the live one and republishes from that.

It used to call `context_turn_begin()` on the line above, for the user's own
world, and that call was a mistake rather than a mechanism. `/first-turn`'s
depth counter — which arrived after this line was written — makes a begin inside
a turn a no-op, so it re-froze nothing; and having no matching end it left the
depth one higher after every event, so the client's own view was taken at the
first event and never retaken. Nothing observably misbehaved, because
`edit_context` mirrors a turn's own writes into the frozen view, which is the
real reason the user's own world needs no re-freeze here: everything that
reaches it during a turn goes through that door. The call is gone, and
`context_turn_stats()` counts begins against ends so the claim is checkable.

The layer keeps its re-freeze for the one thing no mirror covers: a `CtxUpdate`
arriving from the server for the layer is written straight to the live cell by
`/overlay`. A layer edit made *locally* no longer needs it either — `edit_layer`
gained the same read-your-own-writes replay — which is what stops a node newer
than this one from painting a stale number.

Nothing about the update's determinism changes: it had already completed under
the view it opened with. What the layer's re-freeze buys is that the paint shows
a shared value that arrived during this very event rather than on the next tap.
For the user's own world the same freshness comes from the mirror instead, and
it always did — this paragraph used to claim the re-freeze delivered it, which
was true when it was written and stopped being true when the depth counter
landed.

**First paint is covered by `init`, not by the server.** The server never
materialises loop state — `boot()` runs `init()` and then `render()` inside the
wasm place, and the values a client learns from the server arrive later as
events (`VarJoin`, `CtxUpdate`), each of which is a turn that republishes on its
way out. So bridging `init` is what makes the first frame correct, and there is
no server-side seam to add. A value the client has not yet heard about resolves
through the overlay to the layer or to its declared default, which is exactly
what the page would have shown before.

**Resolved, not raw.** The republished value is `<field>_get()` — own value,
then layer, then default — so what a fragment reads is what a gate would read.
A bridged var that a user has never overridden shows the shared layer's value,
which is the whole point of the overlay reaching the page.

**The key is a promise, so an unkept one is a link error.** A `js:` column with
no bridge composed would render blank rather than fail, so it fails instead,
naming the declaration and the node. Two vars claiming one key is refused for
the same reason: the second would silently win, and which one is second is a
linearisation detail nobody should have to know.

## glossary

- **bridged var**: a var that names a legacy page key and is republished into
  the loop's state at that key before every paint.

## code description

`payload.rs`, `update()` /extension/: the paint's freshness — re-freeze the
LAYER after the update, then republish. It carries the `fm:context-bridge`
token. The user's own view is not re-frozen (see above); the unmatched
`context_turn_begin()` that used to do it is gone.

`payload.rs`, `init()` /extension/: the first paint, before anything renders.

`payload.rs`, `ctx_republish()`: parse, hand the state to the generated
`republish`, re-serialise.

`tools/fmlink.py`, `VAR_DECL_RE` (scaffolding, per the standing arrangement): a
declaration may end with `js:<key>`; the key is a plain identifier.

`tools/fmlink.py`, `emit_context_republish` (scaffolding): one assignment per
bridged var, resolved value into the state at its key.

`tools/fmlink.py`, the bridge checks (scaffolding): a `js:` column without this
node, a `js:` column without the overlay's resolved read, and two vars claiming
one key are each a link error naming both sides.

## risks

**The bridge is one-way.** A fragment that WRITES `s.<key>` is writing to a copy
that the next republish overwrites. No fragment does today — writes go through
events — but the moment one tries, its write vanishes silently. If that becomes
a real pattern the bridge needs a complaint, not a fix.

**Every paint pays for every bridged var.** One `to_value` per bridged var per
turn, plus a re-freeze of the layer (one `Context` clone, down from two). At tap rate that
is nothing; with a large bridged `String` on a chatty event it would show. The
migration should watch `asks` and `feature_ticks`, which are the two that grow.

**A disable arriving mid-event silences that event's paint, not just the next
one.** True, deliberate, and a real difference from rung 4's proof — but it
comes from `edit_context`'s mirror rather than from a re-freeze here, so it
holds whether or not this link runs. Update determinism is untouched; anything
reasoning about render as part of the turn should know.

**For rung 7:** every var it migrates that a fragment reads needs a `js:`
column, and the migration should add them in the same commit as the declaration
so no build ever exists where the key has left the page. The list, from rung 7's
enumeration: `asks`, `open_tool`, `tools_catalog`, `update_ticks`,
`update_accepted`, `update_policy` — and `feature_ticks`, which rung 8 retires
rather than migrates.
