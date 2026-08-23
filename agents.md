# agents.md — how to build here

Instructions for any agent working in this repository. The doctrine is
`fm.md` (user-authored — read it first, never edit it). This file is the
working discipline that keeps development *actually* feature-modular when
build momentum tempts shortcuts. It exists because shortcuts happened:
capabilities were once built as edits inside other features' files, and the
tree had to be repaired afterwards by audit.

## The loop: every user request follows the same five steps

**1. Place the request in the tree — before touching any code.**
Decide what the prompt is:
- a **new capability** → a new feature node. Choose its parent by what it
  extends; respect the 4–6 children cap (a regroup is itself a prompted
  event; since linearisation is provenance-ordered, regrouping never changes
  behaviour — verify with a `--chains` diff anyway).
- a **refinement or bug report** about an existing capability → a new
  *subfeature* of that node. One prompt per node, always; never fold a second
  request into an existing spec.
- a **question or design discussion** → no node; answer, and record decisions
  in `notes.md`.

**2. Write the node first.** Export the transcript
(`python3 tools/export_transcript.py --slug fm-spec --title "fm spec discussion"`),
find the prompt's `#pN` anchor, and create `<name>.md` in fm.md's spec format
with the provenance quote. Add the node to the parent's `order.md`. This costs
thirty seconds and is immune to enthusiasm; do it before the interesting part.

**3. Implement INSIDE the node.** The node owns its code, whatever the
language:
- server behaviour: `<name>.rs` — a `feature_<Name>` struct whose functions
  extend chains via redefinition + `existing.fn()`; cargo deps in `deps.toml`.
- page behaviour/appearance: fragment files (`<name>.js`, `<name>.page.css`,
  `<name>.login.js`…) composed into page skeletons at slot markers. One
  `const feature_<Name> = {…}` object per JS fragment; every cross-feature
  reference typeof-guarded — absence is the unticked state; guard DOM lookups
  so fragments survive their siblings being toggled off.
- whole files the node owns outright: `assets/`.

**Never add this feature's behaviour by editing another feature's files.**
If the parent lacks an extension point (no chain to extend, no slot to fill),
refactor the parent to create one — keeping its behaviour intact (fm.md's
refactoring rule) — then extend it from the new node. If a mechanism is
genuinely missing from the linker, say so and design it with the user rather
than working around it.

**4. Prove the toggle.** Build (`python3 tools/fmlink.py miso`), run the
tests/flows the change touches, then untick the new node in `order.md`,
relink, and confirm its code has left the composed output and nothing else
broke. Re-tick. A feature that cannot be turned off is not a feature yet.

**4a. Look at it, and ask if it is good enough.** Proving a thing works and
judging it good are different acts; only the first is a test. Anything with
a visual result gets rendered and *looked at* before it ships — a screenshot
of the real surface, not a description of it. Judge it against the composed
skillset (`products/<product>/build/skillset.md` — the tree's assembled
agent instructions; `/taste` carries the aesthetic standard). Then the
question, out loud: *is this good enough?* The bar moves over time; asking at all is the
discipline. Two things that repeatedly answer "no": a filter working hard to
correct an asset (choose a source that gives you what you want instead), and
a surface that ignores what the user's own ask history says they like.

**5. Finish the node, then ship.** Complete the code description (short
paragraphs, one per thing: entry/extension points first, then mechanics, then
helpers). Commit with a user-readable subject — commit subjects are the
changelog and the push-notification text. Deploy prints the feature nodes a
release touches and warns on nodeless releases: treat that warning as the
question "did a request go nodeless?"

## The law above the laws

**Deliver what the user asked for. Doctrine compliance is eventual, not
mandatory.** (2026-08-16, after the map tool shipped a position readout
instead of a map because imagery would have meant a third-party
dependency.) Ash: *"the user actually asked for a map — that's what we
should deliver. The doctrine is never as important as what the user
requested."* The purity concern is usually real, and usually answerable
with an hour's work rather than a refusal — proxy it, cache it, vendor it,
own it. Ship the ask; converge on the doctrine after. A node that honours
every law and not the request has failed.

## The laws (violations get repaired by audit — cheaper to obey)

- **One prompt per node**; refinements become subfeatures.
- **Only grouping nodes may be code-free.** A spec that says "the code lives
  in /other-feature" is a violation, not a style.
- **4–6 children per node.**
- **The tree owns its code**: tools/ is scaffolding infrastructure, but
  product behaviour belongs to feature nodes.
- `fm.md` is the user's voice: report errata, never edit.
- Transcripts are immutable records: regenerate, never hand-edit.

## Mechanics reference

- Chains: composition order is PROVENANCE order (notes.md proposal 9) — a
  node's position is the timestamp of the prompt its spec cites; newest is
  outermost, globally. A node may extend any chain that existed when it was
  written (causality bounds extension, not tree position). The tree carries
  grouping and selection only: regrouping cannot rewire behaviour — whose
  precise invariant is COMPOSITION ORDER (ruled 2026-08-21, hybrid #p46,
  when the context ladder gave every node an implicit `enabled` var):
  a regroup may add a grouping node's own vars to the world (a group's
  enabled flag is a feature — a per-user switch for a whole family) and
  may reroute enablement conjunctions through the new parent, but the
  chains must not move and defaults must leave behaviour unchanged. Every
  code-bearing node MUST cite a real anchor — the linker fails otherwise;
  code-free grouping nodes order by their earliest child. Inspect with
  `fmlink.py <product> --chains`.
- **Field asks are provenance too.** An ask filed from a device reaches the
  builder through the ask store, not the session log. Cite `asks#<t>` —
  the ask's filing
  timestamp, which is both its stable id and its position (the linker reads
  the time straight from the id; no lookup). Quote the ask text beneath the
  citation as transcript-cited nodes do. (`ASK_CITE_RE` in fmlink; the
  anchor was expunged by the Aug 16 rewind and rebuilt on 2026-08-21 —
  `/square-taps` and `/undo` are the shipped precedents.) **When an ask arrives, build it and
  ship it — never come back to the user with design homework**: the asker is
  expecting the feature in the next update. One question is legitimate, at
  agent discretion (2026-08-23, plans #p12): when more than one reading of
  the ask survives its context, a did-you-mean — concrete options, one
  tap — may travel to the asker's requests list; silence gets the likelier
  reading built at their scope with the hedge in the stamp. Which thing
  they *meant* is theirs to answer; everything else is yours. Use
  judgement, and document the judgement in the node (the intake discipline
  is hybrid.md's).
- Node names are TREE-GLOBAL (fm.md "tree-global names", linker-enforced):
  unique across the composed tree, self-describing without the path.
  Implementation namespaces are flat — a duplicate JS `const feature_X` kills
  every script on the page — so name the struct after the node's own name.
- fmlink parses at regex level: one `feature_` struct per node; no commas
  inside fn parameter types; braces balanced everywhere including string
  literals; `existing.fn()` only calls the enclosing function's own chain.
- Fragment targets: {index, login, install, sw}, bare = index, `page` = every
  HTML page *present in the composition* (a `page` fragment silently skips a
  page whose owning feature is excluded; an explicitly named page is a hard
  requirement), `.head.html` for the head slot. The linker removes stale
  composition-target pages from site/ when their owner is excluded.
- wasm: getrandom needs its `custom` feature (never `js`); deploy smoke-tests
  that client.wasm instantiates with zero imports.

---
*This discipline was requested at transcripts/2026-08-13-fm-spec.md#p95, after
a session in which it was learned the expensive way.*
