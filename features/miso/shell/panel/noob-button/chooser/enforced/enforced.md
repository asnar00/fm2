# enforced

*the tickbox means it: unticking a feature stops it running, for you, everywhere*

> (transcripts/2026-08-21-hybrid.md#p32)
> just keep going until all rungs are built and working - test as you go

## user

Untick a feature in your feature list and it stops. Not at the next release, not
after a reload — on the very next thing you do, on this device and on every other
device you are signed in on, within a moment. Nobody else is affected: the choice
is yours, and another person's list is untouched.

Re-tick it and the feature comes back exactly where it was. Your taps are still
counted, your recordings are still there, your asks are still listed. A feature
that is off is not a feature that was deleted; it is one that declined to look
at its own state for a while.

Unticking something takes its whole subtree with it — the children are silenced
by their ancestor, and their own tickboxes stay as you left them, so re-ticking
the parent restores exactly the shape you had.

Nothing is exempt, including this list. If you untick the feature list itself
you have genuinely lost the tickboxes; the way back is the server:
`curl -X POST .../diag/context -d '{"path":"miso/shell/panel/noob-button/chooser","name":"enabled","value":true}'`,
or `?user=<name>` on localhost.

## spec

This is the summit of the absorption ladder (notes.md, rung 8). Every rung
beneath it built a piece of one sentence, and this node is where the sentence
becomes true: **untick a feature in the chooser, it is off for you only, on all
your devices, and re-tick finds your state intact.**

Everything it needs already exists. Rung 4 gave every node an implicit
`enabled: bool (user, last-write, inherit)` and a gate at every state-carrying
chain link. Rung 6 made an edit into an op that reaches the user's other
instances; 6a persists it; 6b gave it an id and a shared layer; 7a bridges a
value back into the page; 7c hands it to a device that has never run before. So
this node contains no mechanism of its own. It is a **translation**: a click on
one side, an `enabled` edit on the other, and a map that says what the answer
currently is.

**The page half does not change, and that is the point.** `chooser.index.js`
still reads `s.feature_ticks` and still parses the same JSON object of explicit
`false` entries with absent-means-on. What changed is where that object comes
from: it used to be a stored map of the user's choices, and it is now
**derived** from the context, one entry per node whose own enablement resolves
off. Same format, new truth — and because it is derived rather than stored, it
cannot disagree with the gates. The map and the gates are two readings of one
field.

Ancestor shading stays the page's own prefix walk, deliberately. `reflect()`
already computes effective enablement by testing every path prefix, and the map
therefore carries a node's OWN answer rather than its resolved one. Publishing
resolved-off for descendants would have shaded them the same way but would also
have unticked their boxes, and a user who unticks a parent has not unticked its
children — they must come back as they were. The linker's `<node>_on()`
conjunction and the fragment's prefix walk agree because they are computed from
the same per-node values.

**Two verbs, and the second one is the argument.** Unticking writes an explicit
`false` — this user's own answer, which is what a person means when they switch
something off. Re-ticking your own untick writes **`clear`**, not `true`.

That is the exact analogue of what the old map did. It stored only explicit
choices, so re-ticking removed the key and put the user back under whatever the
build decided. `enabled` is declared `inherit`, and `clear` is precisely that:
the var becomes absent again and resolves through the shared layer to its
default. Writing `true` would look identical today and would be a quiet trap —
it would detach that user from the layer permanently, so the next thing switched
off for everyone would leave them the only person still running it, with nothing
on their screen to say why. A tickbox should return you to the default, not pin
you to today's value of it.

The tick therefore asks one more question than the map can answer: is this
world's own `enabled` **present**? A node switched off on the shared layer is off
here too, but not *by this user*, and the two cases want opposite verbs — `clear`
would be a no-op on a var that was never set, leaving the box apparently stuck.
So a click whose node has no own answer writes an explicit value that overrides
the layer, and a click whose node has one either clears it or flips it. Presence
comes from the generated snapshot, once per click.

**Where the link sits is load-bearing, again.** This node's provenance ties with
the whole `/context` subtree and its depth is greater, so it linearises last and
its `update` link is the outermost of the chain. Three things happen in it, in an
order that two separate constraints agree on:

1. **the edit, before the chain beneath runs.** `converge`'s link drains the
   turn's queued ops into `_send` and is deep inside this one, so an op queued
   after it would sit in the outbox until the next event. And `payload`
   re-freezes both worlds on its way out, so an edit made first is the one the
   paint — and the gates that run during `render` — see.
2. **the chain.**
3. **the map, published from the re-frozen view**, so it carries this turn's own
   edit *and* any `CtxUpdate` that arrived from another device during it. That
   is what makes the other instance's list shade without a reload: the arrival
   is an ordinary event, and every event republishes.

**Nothing exempt (#p4) survives contact.** The chooser gates like everything
else, so it can untick itself, and this node can untick itself. The way back is
the one rung 4 argued structurally: `POST /diag/context` and `?user=` are not of
the gated shape and cannot be, so a context can always be repaired whatever the
user has switched off. That is not a promise this node makes; it is a property of
what the machinery *is*.

**What this rung retires.** The old enforcement half — `feature_ticks` as a
stored `SyncVar` in `chooser.rs` — is gone, and with it the last caller of
`SyncVar` anywhere in the tree. `scope`'s library, its var store, its relay and
`/join`'s value snapshot go in the same commit (see scope.md). The `Join`
message and the `VarJoin` reply type survive it, because `/parity` rides them
and `/veil` waits on them; that was named as a risk in parity.md before it was
a problem.

## glossary

- **derived tick map**: the `feature_ticks` object the page reads, computed from
  the context on every paint rather than stored — explicit `false` per node whose
  own enablement resolves off, absent meaning on.
- **clear**: the op that returns an `inherit` var to absent, which is what
  re-ticking your own untick means.

## code description

`enforced.rs`, `update()` /extension/: the outermost link. Tick first, chain,
then publish — the order argued above.

`enforced.rs`, `init()` /extension/: the first paint, so the map is in the state
before the chooser's page half has ever looked for it.

`enforced.rs`, `ftick()`: one click, and the choice between `clear`, explicit
`false` and explicit `true`. The local half and the wire half are separate
because `apply_op` is the arriving door and deliberately queues nothing, so the
`clear` op is queued beside it; `edit_op` is the local door and queues its own.
Both give the optimistic update the loop has always had — the tick moves before
the server has heard.

`enforced.rs`, `ftick_own()`: this world's own `enabled` for one node, or Null
when it is absent. It carries the `fm:context-snapshot` token, which is the ask
for the generated walker.

`enforced.rs`, `publish_ticks()`: the derived map into the state at
`feature_ticks`.

`tools/fmlink.py`, `Context::enabled_off_map()` (scaffolding, per the standing
arrangement): one line per composed node, emitted beside the `<node>_on()`
predicates under the gate hook rather than behind a hook of its own — it *is*
the gate machinery's view of itself, the same fields in the same order. Emitted
there also because the alternative, walking the snapshot on every event, would
serialise every var's value to answer a question about bools. The consequence is
worth stating: unticking this node removes its links and its reader, and leaves
the generated method standing beside the predicates it belongs to — the map's
*publication* is this node's, the map's *existence* is the gate machinery's.

## risks

**The map is recomputed on every event.** It is one bool test per composed node
— 121 today — and a small JSON object built from the ones that fail. That is
cheap beside the `Context` clones the same turn already pays for, but it is
per-event work that grows with the tree, and if the tree grows an order of
magnitude the honest fix is to publish only when the map changes.

**A gate whose function does not take `state: String` does not exist**, so a
node whose only behaviour lives in such a function has a tickbox that changes
nothing and says nothing. That is rung 4's named risk, inherited here and now
user-visible rather than theoretical: this is the node whose surface would show
it. The fix rung 4 proposed — a linker report of what each node gates — belongs
here, and is not built.

**Unticking a node does not stop its page fragments.** Gates are Rust; a
JavaScript fragment keeps running, and a feature whose visible half is a
fragment will look half-off. Every fragment is typeof-guarded against its
siblings' absence, so nothing breaks, but the surface can disagree with the
tickbox. Making the composed page obey `enabled` is the natural next rung and
this one does not pretend to it.
