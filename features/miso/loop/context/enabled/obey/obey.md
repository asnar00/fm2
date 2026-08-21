# obey
*a tickbox governs the whole feature, page half included*

> (transcripts/2026-08-21-hybrid.md#p56)
> let's fix all residuals next.

## user

Untick a feature whose visible half lives in the page, and it stops. Its
behaviour stops on the next thing you do, and its furniture — a button, a row,
its styling — leaves the screen without a reload. Re-tick it and all of it
comes back where it was.

Before this, unticking such a feature only silenced its server half: the page
kept filtering, kept drawing, kept its buttons. A feature that is half Rust and
half page is now off in both halves or on in both, never half-off.

Everything else is unchanged: your choices are still yours alone, still on all
your devices, and a choice made for everyone still reaches you.

## spec

Rung 4 put a gate at the head of every state-carrying Rust chain link. About a
third of the tree's visible surface is not Rust: it is page fragments composed
into `index.html`, and they ran regardless. Untick `/bookkeeping` and the
release-list filter — pure JS — kept filtering.

**The census first, because the mechanism has to name what it covers.** 105
fragment files under `features/miso`: 68 script, 32 style, 4 body, 1 head. 62
script and 32 style fragments reach `index.html`, which is the only page with a
per-user world to obey. Every script fragment is at least one of four shapes,
and there is no fifth:

- **object definition** (65 of 68) — `const feature_X = {…}`, a namespace other
  fragments call, usually behind `typeof feature_X !== 'undefined'`.
- **chain link** (41 files, 75 patched functions) — captures a function on
  another node's object and replaces it. This is the page's `existing.fn()`.
- **load-time side effect** (34) — top-level code that makes DOM, registers a
  document listener, or starts a timer.
- **handler registration** (12) — a click handler on an element that exists.

**Chain links gate exactly like Rust chains.** The linker wraps each index
script fragment in two generated blocks: above it, a note of what each function
it is about to replace looks like now; below it, a wrapper on each function it
actually did replace — *off, the previous link answers untouched; on, this one
does*, which is `gate_line`'s rule verbatim. A fragment that adds a NEW method
to another object is not wrapped: it is starting a chain, and Rust does not gate
those either. Detection needs no JS parser — the wrapper is installed only when
the function changed across the fragment, so a patch inside a `typeof` guard
that did not fire is correctly left alone.

**Furniture is marked, because a runtime cannot delete what a build baked.**
Three sources, one mark: body fragments get `data-fm-node` on each top-level
element (3 in this composition), each stylesheet's `<link>` gets it (32), and
DOM a fragment makes at load is claimed by a MutationObserver the generated
blocks drain, mark included on the elements inside it. The mark travels with
the element, so a later fragment that re-parents somebody's button — and drops
the row it came in, which `/build-row` does — cannot take that button out of its
owner's reach. On each paint a marked element is hidden and a marked stylesheet
is `disabled`; both are reversible and neither touches the element's own
styling.

**One truth, and no resolution gap.** The tick map the chooser already reads is
published from the server's resolved values: a var this user never wrote falls
through to the shared layer, so a node switched off for everyone appears in the
map like any other. What the map does not carry is the ancestor conjunction, and
that is a prefix walk — the same one `reflect()` does to shade a row. So the
gates and the shading are two readings of one field, exactly as the Rust gates
and the map are.

**The view is frozen per paint.** This node's fragment composes last, so its
link is the outermost of `feature_Loop.apply`: it reads the map out of the
payload being applied — before any gated fragment runs, and from the state being
painted rather than the one before it — and settles the furniture after the last
one. Nothing can change its mind mid-paint. An unchanged map does no work at
all, which is the common case.

**Nothing is exempt, including this.** The mechanism's own fragments are not
gated by the linker — the read that answers "is this node on?" cannot itself
stop running — but the runtime honours its own tickbox: unticked, it holds
nobody to the map, and because the freeze keeps running, re-ticking is noticed
on the next paint.

**What this does not reach, named rather than implied.** A node whose only page
effect is a method another fragment calls through a `typeof` seam is untouched:
`panel.index.js` chooses the chooser's list or the changes teaser by asking
whether `feature_Chooser` exists, and an unticked chooser still exists. The
honest fix is to make the object absent, which is what the seam was written for
(the tree does this 121 times) — but it requires the linker to rewrite every
`const feature_X` into a window binding, and the census found 53 unguarded
references to `feature_Loop`, so absence there throws rather than degrades. That
half is a design question, not a residual, and is returned to triage with the
census. Also uncovered: top-level side effects that are neither DOM nor a patch
(a document-level listener, a timer, a one-line edit of another node's element),
and every page but `index.html` — `login`, `install` and the service worker have
no per-user world to read.

## glossary

- **fragment gate**: the generated wrapper that makes a fragment's chain link
  fall through to the function it replaced when its node is off.
- **claim**: the `data-fm-node` mark that says which node made an element.
- **frozen view**: the tick map as read once at the head of a paint, held for
  the whole of it.

## code description

`obey.index.js` is the runtime, and carries `fm:fragment-gate` — the linker's
hook. Untick this node and none of the wiring below is emitted.

`on` (line 20) answers effective enablement: the node's own answer from the map
and every ancestor's, by prefix walk, memoised per frozen view.

`freeze` (line 37) reads `feature_ticks` out of the payload being applied,
answers whether it changed, and applies this node's own tickbox to itself.

`paint` (line 61) walks `[data-fm-node]`: a stylesheet link is `disabled`, any
other element gets `data-fm-off`, which `obey.index.css` hides with
`!important` so the mark beats the element's own rules.

`extra` (line 56) is an empty seam for the object half, if it is ever built.

The outermost `feature_Loop.apply` link (line 77) is the paint: freeze, then the
chain, then the furniture.

In `tools/fmlink.py`, `js_patches` finds the functions a fragment replaces on
other nodes' objects; `js_watch_block` and `js_gate_block` are the two generated
halves around each index script fragment; `html_mark_roots` stamps body
fragments' top-level elements; `compose_assets` marks the stylesheet links and
skips the hook-bearing node's own fragments.
