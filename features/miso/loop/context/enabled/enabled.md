# enabled

*every feature has an on/off var, and turning it off stops the feature on the very next event*

> (transcripts/2026-08-21-hybrid.md#p32)
> just keep going until all rungs are built and working - test as you go

## user

For agents: every node in the composition now has an `enabled` var, whether or
not it declared one — `enabled: bool = true (user, last-write, inherit)`, listed
in `GET /diag/context` beside the vars you wrote by hand, addressed by the same
`(path, name)` pair. Set it false and that node's behaviour stops at the next
event; set it true and the feature picks up exactly where it left off, because
nothing about its state was thrown away. There is no rebuild and no reload.

Unticking a node in `order.md` and setting its `enabled` to false are the same
switch at two speeds: the first removes the code from the build, the second
stops it running. A node you disable takes its whole subtree with it — the
children are not touched individually, they are silenced by their ancestor.

You may not declare `enabled` in your own `.vars` file. The linker gives you
one, and a second is a link error naming your node.

Nothing is exempt, including the chooser that shows you the tickboxes and the
panel it lives in. If you disable your own chooser, the way back is the server:
`curl -X POST .../diag/context -d '{"path":"...","name":"enabled","value":true}'`.

## spec

This is the rung the ladder was built for. Rungs 1–3 gave a place a `/context`
of typed `/var`s that can be read and written while it runs; this rung spends
that on the one var #p24 said every feature would eventually have, and wires it
to behaviour.

**`enabled` is implicit, not declared.** The alternative was 110 hand-written
`.vars` lines saying the same thing, which would make `enabled` a thing a node
could forget, mistype, or scope wrongly. Instead: while this node is composed,
the linker gives every composed node an `enabled: bool = true (user, last-write,
inherit)` — emitted as an ordinary field of the same `Context` struct, so it
rides the snapshot walker and the write path with no special case anywhere
downstream. `enabled` is not special in the machinery; it is only special in
that everybody has one. A node declaring its own is a link error naming the
node, because two fields of the same meaning is exactly the ambiguity the
implicit var exists to remove.

The var's columns are ash's from #p24, unchanged: `user` scope, because a
disable is one person's choice and not the group's; `last-write`, because a
tickbox has no interesting merge; `inherit`, because a per-user absence should
fall through to whatever the group or the build decided.

**Ancestor semantics are compiled, not walked.** The old expunged design asked
at runtime whether any ancestor of a node was disabled, by comparing path
strings — which is where the `':'` discriminator bug lived. The linker already
knows the tree, so the question is answered at link time: it emits one
`fn <node>_on(&self) -> bool` per composed node, whose body is
`self.<node>_enabled.value && self.<parent>_on()`, with a root node answering
from its own field alone. Unticking `tap` silences `counter`, `reset-taps` and
every other descendant through a conjunction rustc inlines. No string is
compared, no tree is walked, and the bug class is inexpressible rather than
fixed.

The parent is found by node path, so a product-local override and a shared node
sit in the same tree — `products/miso/miso/loop/tap` is the parent of
`features/miso/loop/tap/counter`, as the user would expect. And because the
conjunction is regenerated from the tree at every link, a regroup rewrites the
predicates rather than breaking them; the fields themselves are named for the
node, not the path, so a regroup does not rename anyone's stored value either.

**Gates sit at chain links that carry loop state.** A gate is injected at the
head of a composed function when two things are true: the function EXTENDS an
existing chain, and its first parameter is `state: String`. The second is the
class of function that carries the Elm loop's state — `update`, `render`,
`tools_list`, `tool_controls`, `transcribe_local` — which is to say, the class
whose behaviour a user means when they say "turn this feature off". The gate is
one line:

`if !gate_open(|c| c.<node>_on()) { return feature_<Prev>::<fn>(<args>); }`

— hand the previous link's answer back untouched. That is what "off" means for
a chain: the chain still runs, this node's contribution to it does not, and the
state the node was maintaining passes through unread and unwritten. Which is why
re-enabling finds the work intact: a disabled node does not clear anything, it
declines to look.

Chain-STARTING definitions are never gated. They are the seams the chains hang
from — `loop`'s base `update` returns the state unchanged, `shell`'s base
`render` is the page itself — and a gate there would have no previous answer to
return. Server route and message chains are not gated either, for a different
reason: they take a `request`, not a `state`, so the rule never selects them.
Enforcement is what a user's own instance does with its own state, not what the
server declines to route — this is the same boundary rungs 2 and 3 drew when
they screened `diag/context` by cookie rather than by feature.

**Reads go through the turn's frozen view.** Every gate calls this node's
`gate_open`, which calls rung 3's `with_context`, which inside a turn reads the
clone the turn opened under. So the boundary law holds for gates by
construction rather than by a new argument: a `POST /diag/context` landing while
an event is in flight cannot flip that event's gates halfway, and the event
completes under the context it arrived under. The next turn re-freezes, and that
is when the disable takes effect. On the client the turn is one `on_event`, the
Elm update itself — so "the very next event" in the user-facing sentence above
is exactly the turn boundary rung 3 built.

**The trusted base is structural, not a list.** The old design carried a
`trusted.md` naming the features that must never be disabled, and this rung
deliberately does not reintroduce one. It does not need to: the context
machinery's own functions are not of the gated shape. `held_context`,
`gate_open`, `with_context` and `edit_context` are library and startup code;
`serve`, `boot`, `route`, `context_get` and `context_set` take a `request` or
nothing at all; `on_event`'s single parameter is named `input`, because it
carries an event envelope rather than loop state. Not one of them is a
chain-extending `state: String` function, so not one of them can be gated —
which means a context can always be read, always be written, and always be
repaired, whatever the user has switched off. The trusted base is defined by
what the machinery *is*, and cannot drift out of date the way a list of names
would.

That is what makes the standing "nothing exempt" ruling (#p4) affordable. The
chooser, the panel and the toolbar gate like everything else, and a user who
disables their own chooser has genuinely lost the tickboxes — but not the way
back, because `POST /diag/context` is on the ungated side of the line by
construction. Exempting the chooser would have been a promise the tree could not
keep anyway: any feature can be the one whose absence hides the repair.

**Emission is gated on the machinery being present.** The gates need the var
family (rung 1) and the frozen read (rung 3). This node's hook token
(`fm:context-gate`) with neither is already a link error by rung 1's rule; with
the family but no `fm:context-set` asker it is a new link error naming both
halves. Failing loudly rather than degrading to no gates is the deliberate
choice: a build that silently ignores every tickbox is worse than one that will
not link, because the first is discovered by a user and the second by whoever
composed it. And with this node unticked, no hook is present, so no implicit
var, no predicate and no gate is emitted, and the source is byte-identical to a
build predating this rung.

## glossary

- **gate**: the injected head of a chain-extending, state-carrying function that
  returns the previous link's answer when the owning node is not effectively on.
- **effectively on**: a node's own `enabled` var AND every ancestor's, resolved
  at link time into one conjunction.

## code description

`enabled.lib.rs` (verbatim library): `gate_open`, the one function every
generated gate calls. It takes the generated predicate as an
`impl Fn(&Context) -> bool` — which is why it is a verbatim library rather than
a chain function — and reads through `with_context`, so the frozen-view rule
lives in exactly one place. It also carries the `fm:context-gate` hook token, so
unticking this node removes the machinery it turns on.

`tools/fmlink.py`, `parse_signature` (scaffolding, per the standing arrangement):
returns parameter NAMES alongside their types. A gate has to hand the arguments
on to the previous link, and it needs the first parameter's name to recognise
loop state in the first place.

`tools/fmlink.py`, `gate_plan` (scaffolding): the composition's tree, as the
gates need it — for each composed node, its Rust identifier (node names may
carry hyphens; fields may not), its node path, and its parent, looked up by path
so product-local overrides and shared nodes resolve into one tree. Two nodes
whose names collapse to the same identifier is a link error.

`tools/fmlink.py`, `emit_gate_predicates` (scaffolding): the `<node>_on()`
methods, one per composed node, in path order.

`tools/fmlink.py`, `emit_context` (scaffolding, extended): under the fourth
hook, prepends each node's implicit `enabled` field to whatever that node
declared, rejects a node declaring `enabled` itself, and refuses to emit gates
without the frozen-read machinery. Everything downstream — the snapshot walker,
`Clone`, `set_from_json` — sees the implicit fields as ordinary ones.

`tools/fmlink.py`, `gated` and `gate_line` (scaffolding): the selection rule
(extends a chain AND takes `state: String` first) and the one line it injects,
placed after the line the function's body opens on. A function whose body opens
on a line that already carries code is a link error rather than a mangled
injection.

## risks

The gate rule keys on a parameter NAME. A future state-carrying chain whose
first parameter is called something else is silently ungated — its tickbox
would do nothing, and nothing would say so. The honest fix is a linker report of
which functions each node gates, so an empty list is visible; rung 8's chooser
is where that would be worth showing.

`boot()` is deliberately not a turn (rung 3), so gates called during boot fall
to `with_context`'s outside-a-turn path, which clones the live context per call.
That is correct but O(gates) clones on one startup path; if it ever matters, the
answer is to make boot a turn rather than to weaken the read.
