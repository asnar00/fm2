# notes
*working notes on fm — discussion, ideas, open questions. (fm.md remains the user-authored source of truth; this doc is co-written and freely editable.)*

## the spirit of the enterprise (#p125)

> "it's not to build apps, per se, but it's to evolve our understanding of what 'feature modular architecture' could actually be, and why."

This is a research project whose method is building. The apps (muon, the tap counter, whatever comes) are apparatus — probes that force the architecture to answer questions it could otherwise evade. A long gestation across multiple re-spins wasn't repeated failure to build a product; it was an inquiry iterating toward a sharp enough question. Success is measured in *understanding gained*, and the working tree is the lab notebook.

What the inquiry has established so far (evidence: two days of practice):

- **Composition can be one primitive.** Redefinition + `existing.fn()` dissolved annotations, methods, middleware, and framework registries alike. Every "we need a mechanism for X" so far has reduced to a chain, a fragment, or a message.
- **The unit of intent can be the unit of code AND the unit of control.** One prompt → one node → its own toggleable implementation. When that holds, the tree is simultaneously spec, changelog, test surface, and product configurator; when it slipped, audits could repair it — the discipline is enforceable, not aspirational.
- **Provenance can be first-class.** Every capability traces to the sentence that requested it. Software becomes a conversation with an audit trail — which, in the agent era, may be the point: when code is cheap to produce, *intent and structure* are the scarce assets worth engineering.
- **Placement belongs to products, not code.** One placeless tree already yields a native server and a wasm client; single nodes own behaviour on both sides of the network; the same code could ship as standalone, thin-client, or fat-client. The full placed-data vocabulary is still ahead — it is the largest open question.
- **Observability falls out of the architecture.** Explicit state + explicit events made recording, keyframing, replay, drive, and readout nearly free. They weren't features added to an app; they were properties the architecture already had, waiting to be surfaced.
- **The instruments should be shaped like the subject.** The tree browser serves the tree; deploys narrate which nodes shipped; demo scripts are both demonstration and regression test. When the meta-work and the work share a shape, each improves the other.

**Framing candidate (2026-08-14, agent observation):** the doctrine's headline
is user-controllable feature sets, but what two days of practice has actually
demonstrated may be better named *provenance-modularity*: the unit of code is
the unit of conversational intent, and the tree is the transcript's table of
contents. The toggle test is what keeps that falsifiable. Offered as possible
fm.md framing when the author next revises; not a decision.

**The method, named (#p126):** the monolithic-code episode resolves under this frame — it was less a discipline failure than an articulation failure: the laws it "broke" were discovered by colliding with their absence, and nearly every principle now in force (tree owns its code, one prompt per node, shell-public-data-gated, honest checks, readout-over-pixels) was born as a repair. So the working method is the cycle: build → notice the wrongness → articulate the principle it reveals → encode it (laws, mechanisms, tools) → continue. Errors are experiments; the repair is the result. When something feels wrong, ask "what principle is trying to be born?"

**CONVERGENCE (#p128): binary formats and Shared<T> are one question — typed, linker-generated state.** JSON currently earns its keep as observability (readable blackbox/readout/replay/wire); serde's format-agnosticism means binary is a swappable backend *once state is typed* — representation becomes a product policy (dev = JSON for the instruments, production/hot paths = postcard-style binary), at the cost of explicit schema-evolution discipline. `Shared<T>` is fm.md's day-one `@shared` variables realised as a policy type: the tap demo hand-rolled (~60 lines) exactly what the declaration should generate — outbox op, authoritative server copy, broadcast, merge. Staged path: (a) typed state — linker flat-merges feature-declared fields into one derived `State` struct (unlocks binary); (b) typed messages — the v2 signature-derived router; (c) policy types — `Local<T>`, `PerUser<T>` (=@user), `Shared<T>` (LWW register), `SharedCounter` (op-fold — the demo already proved registers lose concurrent increments; counters are op-based, quietly CRDT-shaped); (d) representation per product. This IS the placed-data vocabulary arriving as types; strongest candidate for the next probe: "make tap_count a declared shared variable and generate what we hand-wrote." Parser note: `Shared<u64>` fields parse today; comma-carrying generics in fields don't yet.

**SCOPE (#p129): "shared across what?" is the real axis, and it's a lattice.** device ⊂ user ⊂ group ⊂ everyone — fm.md's day-one `@user`/`@shared` annotations were already two points on it; `Local<T>` and groups fill the ends. Scope is irreducible domain complexity: hiding it converts it into leaks (an unscoped shared value is a privacy bug awaiting a second user), so the vocabulary makes it explicit as scope-named types: `Local<T>`, `PerUser<T>`, `PerGroup<T>`, `Global<T>`. Mechanically scope is a key: authoritative store keyed by scope-instance (user = the cookie-proven phone — identity is already built), broadcast filtered per scope key, long-polls subscribing to their keys (global + user:me + my groups) — sessions/rooms fall out of scoping rather than existing separately; the scope boundary IS the access-control boundary (placement and permission unify). Cost gradient: global trivial (done), per-user nearly free, device-local free, per-group the real homework (groups are themselves data: membership, invitation — its own probe, not a type parameter smuggle). Next probe candidate, smaller than Shared<T> generation: key taps by phone — the same three demo instances then show phone+tab (one user) converging while the simulator (_test) counts alone: scoping made felt.

**JOIN, SESSIONS, PRESENCE (fm-spec-2 #p19–22).** The restart bug (a
relaunched instance shows zero taps until the next write anywhere) exposed the
missing half of the authority model: broadcast covers steady-state deltas but
not the *catch-up* — and a fresh boot is just a maximally-stale replica, so
boot-join and reconnect-join are one mechanism. Named **join** (#p21;
"hydrate" rejected as jargon). Session-object question resolved: no new
primitive — a session is (scope key) × (presence). The durable axis (who MAY
hear) is the scope lattice; the ephemeral axis (who IS here) is presence,
which is necessarily *server-derived* state (a crashed replica can't write
its own departure; the server holds the long-polls) published as an ordinary
scoped var. The one genuinely new noun is **instance identity** (a durable
per-instance id — also the missing key for true Local scope); deferred with
presence until something renders them. Built now: `loop/scope/join` — init()
queues a Join through the outbox; the server replies with a snapshot of the
sender's hearable vars (global + user.<me>, the same audience rule as
wait_filter); the VarJoin reply applies through the update chain, so joins
are blackbox-visible and replay-safe. Offline boot leaves the Join queued —
reconnect catch-up falls out free.

**PLACES CONVERSATION (fm-spec-2 #p34–37): the app, the store, and graded
derivation.** The real app (#p35): a spatial database of conversations and
information — add + match, patchy network (canvassing), with agentic
interrogation of the dataset that also works offline. What fell out:

- **Posts are immutable, append-only facts** — no write conflicts; two
  reports of one pothole are two true facts joined by an additive match-link.
  Authority migration largely dissolves: a post is born true on its device;
  the server is the *exchange*, not the owner. Mutable state (aggregates,
  retractions-as-links) is the minority case.
- **Replication = scope ∩ interest**, both as key-set subscriptions on the
  existing audience machinery: scope (team.X — who MAY hear) intersected with
  interest (tile.A..F — what this place cares about). Tiles behave like
  ad-hoc groups: the group-membership rung and spatial subscription are
  plausibly one design.
- **Enrich at the exchange, consume at the edge**: heavy derived data
  (corpus embeddings, tile digests) computed server-side at sync time and
  replicated with the patch; the device does only light compute at use time
  (query embedding, kNN). Derived stores: computed at a compute-rich place
  from a source store, replicated like their source.
- **GRADED DERIVATION (#p36–37) — the central pattern, user-named as the next
  build.** Derived data declares *ranked rungs* (implementations via multiple
  dispatch over resource types, best first), each with needs; the standing
  rule: **run the best rung whose needs are reachable now (reachability has a
  budget — the freshness-deadline principle generalised to places); stamp the
  result with its rung; when a better rung comes into reach, re-derive and
  upgrade in place.** Provisionality is honest metadata: never lie about
  quality, the twin of never lying about freshness. One pattern covering:
  transcription (server whisper / device / pending), agent synthesis (cloud /
  local model / retrieval floor), embeddings, splat building — and join
  itself (local state now, joined state moments later = graded derivation of
  state).
- Canonical worked example (#p36): dictaphone-style voter conversation —
  stream audio up for live server transcription when bandwidth allows; fall
  back to on-device (or pending) when not; the immutable audio replicates
  later and the transcript *upgrades*. Even fully online the pattern applies:
  per-chunk streaming text is the draft, whole-recording re-transcription is
  the final.

**MUON IS AN OS (fm-spec-2 #p41): tools on a launcher.** The muon/apps
grouping question resolves with better vocabulary: muon runs **tools**
(user's preferred term over "apps"), organised into **toolsets** (pages) when
one screen overflows. The form (revised in draft, #p42 — first live use of
two-phase churn-in-place: tools.md now carries both prompts): a **toolbar**
of small icon buttons along the bottom edge beside the corner stamp, the
whole screen above it the **display surface** for the open tool, a `‹` at
the toolbar's left to close. Which tool is open is per-instance Local state
(navigation never syncs across devices). A tool
registers on the `tools_list` chain from its own node. Discovery while
building: **provenance ordering forces old features to register on new
chains via new subfeatures** (tap predates the chain, so `tap/counter` does
the registering — causality made visible in the tree), and the linker
correctly refuses `counter` ticked with `tools` unticked: chain dependencies
are real dependencies. First registered tool: taps. Next: transcribe
(`loop/dictate` — the graded-derivation probe registers itself, being newer
than the launcher).

**RULE (fm.md "tree-global names", fm-spec-2 #p32–33): node names are unique
across the tree and self-describing** — you shouldn't need the parent path to
know what a node does. Grounds: implementation namespaces were always flat
(struct `feature_Gate` collided across `users/gate` and join's paint gate;
duplicate JS consts are a page-killing SyntaxError the linker never saw), and
provenance ordering made the path pure presentation — identity is name +
timestamp, and a name must survive regrouping. Linker now fails on duplicate
node names (code-free nodes included). Renames executed while free
(behaviour-neutral post-proposal-9): `diag/pwa` → `standalone`, `join/gate` →
`veil`. fm.md carries the rule (author-added).

**RULE (transcripts/2026-08-14-fm-spec.md#p132a): tick/untick is a product property, not a core-tree property.** The shared tree's order.md is catalog + ordering and stays fully ticked; a product switches features off via its own order.md override (the mechanism products/hello_only has used since day one). Persisted policy — like disabling `update/auto` — belongs in the product; the shared tree never records anyone's selection. (fm.md's ordering section still describes unchecking as exclusion in general — the author may want to relocate that semantic to products when next editing.)

**The reverse index (transcripts/2026-08-14-fm-spec.md#p135).** Nodes already cite their originating prompt; `tools/audit_prompts.py` inverts those citations, giving prompt → node(s) and three audits: prompts no node cites (missed / coalesced / conversational), many-to-one coalescing both ways, and nodes with no provenance at all. Building it surfaced three transcript-integrity fixes: mid-turn messages now get rider anchors (`p132a`) so the main numbering never shifts; edited-and-resent prompts keep their anchor but carry a do-not-cite mark; and snapshot files of the same session are aliased by session id so a citation of an old snapshot counts toward the same anchor. First run: 131 live prompts, 43 reached nodes, zero orphan nodes, zero dangling citations — the genuine gap is the features-browser interaction cluster (#p9–#p22), which lives in tool templates awaiting migration into the tree.

The open questions ARE the agenda: cross-cutting features, contexts and runtime dynamism, the placement/data vocabulary, typed message routing, extension chains for assets, chain semantics under regrouping, what happens at 1,000 nodes, multi-user permission. Each will be answered the same way — by building the smallest thing that forces the question, inside the discipline, with the answer recorded here.

## restatement (checking understanding)

fm organises a codebase as a tree of features under `features/`, where each feature node modifies the behaviour of its parent. A *linker* composes selected features into executable *products* (under `products/`, expressed as symlink trees into `features/`). Composition happens at two levels:

- **functions**: a subfeature can extend a parent's function (`@before`, `@after`, `@replace`, `@on`), and the linker weaves the calls into a single composed function.
- **structs**: a subfeature can add fields; the linker aggregates them into one global struct.

Every feature carries a spec (`C.md`) in a fixed format, anchored to the user prompt that caused it — provenance is first-class. Features declare `@shared`/`@user` variables, and *contexts* let users enable/disable features and change settings dynamically, per user.

## decisions (2026-08-13)

- **Extension form is the sole composition primitive**: a new definition of `fn` supersedes the previous one in the chain and may call it via `existing.fn(x)`. The original vocabulary (`@before`/`@after`/`@replace`/`@on`) collapsed into this: before/after = where you place the `existing` call; replace = don't call it; parallel (né `@on`) = call `existing.fn()` and the new behaviour concurrently *in ordinary code* (threads/rayon/async — author's choice). Consequence: the linker needs **no function annotations at all** and no built-in concurrency story; it only rewrites `existing.fn()` references and chains definitions.
- **Flat struct merge accepted** (proposal 3): the linker generates one flat composed struct; no nested sub-structs, no type-aware access rewriting. Field-name collisions are link errors. User will update fm.md accordingly.
- **Cross-cutting features are in scope**: logging, distribution, networking, login, UI. A mechanism for expressing them is needed (see proposals below).
- **Rust-first**: performant, one codebase for browser (wasm) + server, good safety. Minimal ts/py scaffolding, which does *not* need to be feature-modular. Keep the door open for other languages (esp. WebGPU/WGSL) but don't complicate now.
- **Contexts**: dynamic enable/disable implies a guard per woven call site; acceptable, but the dynamic subset is restricted to higher-level code, not deep in loops → dynamism should be *opt-in*, static composition the default.
- **Previous experiments are not to be excavated**: integrating them proved counterproductive — detail from incorrect approaches pollutes fresh thinking.
- **Ordering = creation time, recorded in `order.md`**: each feature node keeps an `order.md` listing its subfeatures in composition order. Maintained by appending on feature creation (so the default order is chronological: newest composed last, outermost extension), but deliberately editable — reordering is an explicit, diffable act. `order.md` also serves as a static include/exclude switch: de-listing (or unchecking) a subfeature temporarily drops it and its subtree from composition.

## proposals (for discussion)

### 1. extension form as the primitive — ACCEPTED (and annotations eliminated)

`existing.func(x)` subsumes the whole annotation vocabulary; decision is to drop the annotations entirely rather than keep them as sugar. Redefining a function *is* the composition act; whether/where/how it calls `existing` is ordinary visible code. Return values need no special rule — the outermost extension's return is the return. Concurrency needs no special rule — extensions use ordinary Rust concurrency and Send/Sync checks it.

### 2. ordering & the `existing` chain — RESOLVED: `order.md` per node

When several features redefine the same function they form a chain; `existing` refers to *the previous definition in the chain*. Each feature node keeps an `order.md` listing its subfeatures in composition order (appended on creation → chronological by default; editable when chronology isn't the right order; durable across clones/copies/sharing, unlike filesystem timestamps).

Proposed mechanics:

- **Format**: markdown checklist, one line per subfeature —

      - [x] colour
      - [x] alpha
      - [ ] hdr        ← present but excluded from composition

- **Maintenance**: creating a feature appends a checked line to the parent's `order.md`. The linker validates: a subfolder not listed is an error (or auto-appended with a warning) — no silent drift.
- **Linearisation**: depth-first pre-order over the tree — parent first, then each listed subtree in order. The last definition visited is outermost.
- **Consequence to be aware of**: cross-*subtree* order is now traversal order, not global chronology. With A → [B, C] and B → [D]: linearisation is A, B, D, C — so C wraps D even if D was written later. Deterministic and visible, but different from the pure "newest is outermost" rule; the fix when it matters is editing `order.md` at the common ancestor.
- **Three-tier toggle story** (each mechanism has a distinct job):
  1. `order.md` unticked — author-side static exclude, for development/experimentation
  2. product composition — per-executable subset, for shipping variants
  3. contexts — runtime enable/disable, per user, opt-in (`@dynamic`)

### 3. flat struct merge (linker simplification) — ACCEPTED

fm.md nests sub-structs (`colour.alpha.a`) and rewrites accesses (`col.a` → `colour.alpha.a`). That rewrite requires *type-aware* source transformation — the linker must infer that `col: Colour` — which in Rust means something approaching rust-analyzer-level analysis.

Alternative: the linker generates one **flat** composed struct:

    // generated
    pub struct Colour {
        pub r: f32, pub g: f32, pub b: f32,   // from colour
        pub a: f32,                            // from colour/alpha
    }

Then `col.a` in feature source is *already correct* — zero rewriting, no type inference needed. Name collisions across features become link errors (probably desirable anyway). Per-feature methods become separate generated `impl` blocks on the composed struct. The linker drops from "type-aware transformer" to something much closer to "syntax-aware concatenator", which makes Rust-first far more tractable.

Cost: we lose the per-feature grouping inside the struct layout. Is that grouping load-bearing (e.g. for serialisation, or per-feature state save/restore), or was it a means to an end?

### 4. cross-cutting via qualified targets

Keep "a feature modifies its parent" as the *default*, and let an annotation name a target elsewhere in the tree:

    impl feature_Logging {
        // @after server/request::handle
        fn log_request(req: &Request) { ... }
    }

- No path → same-named function in the ancestor chain (the fm.md base case).
- Qualified path → advise any named function in the tree.
- Wildcards (`server/*::handle`) possible later; explicit names first, since pattern-based pointcuts are the fragile part of AOP.

So the tree stays the primary organiser; cross-cutting features are just features (probably living high in the tree) whose extensions carry targets.

### 5. dynamism is opt-in (`@dynamic`)

A feature (or an individual extension) marked `@dynamic` gets a context guard at its woven call sites; everything else is composed statically with zero overhead. This makes the performance boundary explicit and auditable — you can grep for what's dynamic.

### 6. the product is the compilation unit

Individual features aren't independently compilable (their code references composed structs and siblings' contributions). Consequences:

- Development happens "against" a product — plausibly an *everything* dev-product that links the whole tree, so all code type-checks during editing.
- The linker emits a generated build directory; compiler errors and debug info must map back to feature source (e.g. rewrite paths in rustc's JSON diagnostics; keep generated code line-stable with the original where possible).
- Feature tests run in (at least) two modes: the feature linked with only its ancestor chain (refactoring safety), and within each product that includes it (feature-interaction bugs).

## open questions

- **Concurrency in extensions** (background, no longer a linker concern): authors spawning `existing.fn()` in parallel use ordinary Rust; Rust's Send/Sync rules make unsafe compositions fail at compile time. Wasm products constrain executor choice (web workers / wasm threads, no blocking the main thread) — becomes relevant when a browser product first uses parallel extensions, and again for *distribution* (the "run elsewhere" generalisation of "run in parallel").
- **`existing` inside closures**: `existing.fn()` will legitimately appear inside closures/spawns (parallel extensions) — the rewrite must handle that; also decide whether `existing` may be referenced anywhere other than a same-named function (e.g. can `helper()` call `existing.main()`? probably no — restrict to the redefining function for legibility).
- **Struct initialisation**: who constructs the composed struct? Do features declare field defaults so the linker can generate a constructor, with features optionally extending construction logic?
- **Products/symlinks**: symlinks are filesystem-native but awkward on Windows and in git. Manifest-per-product as alternative or complement? Also: when a product overrides a feature folder, what happens to that feature's subfeatures — re-symlinked individually or owned wholesale?
- **Disabled-feature state**: when a context disables a feature, do its struct fields / `@user` variables persist (inert) or disappear?
- ~~**Provenance**: where do transcripts live?~~ RESOLVED: `transcripts/` in the source tree (see proposal 7).
- **Combined glossary**: generated by the linker from all feature glossaries?
- **wasm split**: one codebase, but presumably some features are server-only / browser-only — is target-selection just product composition (a `server` product and a `browser` product from the same tree)?

### 7. transcripts in the source tree

Decision: conversations live in `transcripts/` as part of the repo; feature-spec provenance references point into them. Implemented mechanics (first cut, working today):

- **Capture**: `tools/export_transcript.py` converts a Claude Code session log (JSONL, which has verbatim text + exact timestamps) into `transcripts/<date>-<slug>.md`. User prompts and assistant reply text are kept verbatim; thinking and tool traffic are omitted. Re-running regenerates deterministically.
- **Anchors**: each user prompt gets a heading `### pN` with its timestamp beneath. Prompts are append-only within a session, so anchors never move. A feature spec references its prompt as:

      > (transcripts/2026-08-13-fm-spec.md#p4)
      > actual prompt text, quoted verbatim

- **Ordering fallback**: the prompt timestamp under the anchor is the "timestamp drawn from the conversation reference" that fm.md's ordering section falls back to when `order.md` is absent.
- **Immutability**: transcripts are generated, never hand-edited — they are the evidentiary record of user intent that fm.md's provenance requirement rests on.
- Open: whether to also capture tool-level activity (file edits made during the conversation) — probably no; git history covers that.

### 8. global functions only — no methods; overloads via signature-keyed chains — IMPLEMENTED

Implemented in fmlink.py (2026-08-13, #p29): chains keyed on (name, all param types); `impl feature_X` blocks serve as natural namespaces (no mangling — `feature_Colour::add` and `feature_Vec::add` coexist); per overloaded name the linker generates the `fm_<name>` trait + generic dispatcher; unique names get a plain delegate fn; operator names (add/sub/mul/div/rem/neg) get `std::ops` glue, so `v1 + v2` works. `existing.fn()` may only call the enclosing function's own chain (enforced). Same-name-different-arity and mid-chain return-type changes are link errors. Trait-bound failures are translated to "no definition of add(vec2, colour) in any linked feature". Demo: `features/vec` + `features/sums` exercise colour/vec dispatch incl. alpha's extension and operator form. Practice note that emerged: features constructing mergeable structs should use `..Default::default()` in literals, since later features may add fields.

Original decision rationale (2026-08-13): computation lives in global functions, never authored methods — `col = col + col` preferred over `col.sum(col)`; one composition primitive, uniformly applied. Two gaps, both linker-owned:

- **Generated trait glue**: operators and trait conformances (`Add`, `Display`, …) are emitted by the linker as delegation impls over free functions matching name+shape conventions (`fn add(a: colour, b: colour) -> colour` ⇒ `impl Add for colour`). Authored code: structs + global functions only; impl blocks exist only as generated artifacts.
- **Overloaded names — full-signature keying (multiple dispatch)**: chain identity = **(function name, all parameter types)**, read syntactically from the signature (zero-arg fns key on name alone — `main` is the degenerate case). So `add(colour, colour)`, `add(colour, vec2)`, and `add(vec2, colour)` are three independent chains; same name + same param types across features = same chain, composed in linearisation order (by design). Concrete links are emitted under mangled names (`add_colour_vec2__x`); call sites stay unrewritten because the linker emits one generic dispatcher per name — a generated trait with a type parameter per argument slot and an associated `Out`, exactly the pattern of Rust's own `Add<Rhs, Output=…>` — and **rustc's type system does the dispatch**: no linker type inference, zero runtime cost via monomorphisation. Each overload keeps its own return type (associated type), so the earlier shape-unification rule dissolves; the only remaining constraint is **same name ⇒ same arity** (one generic dispatcher per name), else link error. Non-overloaded names emit as plain functions (no trait noise). Undefined combos surface as unsatisfied trait bounds, mapped by diagnostics to "no add(colour, vec2) defined". Heterogeneous operators come free: `impl Add<vec2> for colour` glue gives `col + vec`. Prior art: this is Julia/CLOS-style multiple dispatch over global generic functions, realised statically via Rust traits.
- `existing.fn()` resolves within the chain of the *enclosing* function's signature — an alpha `add(colour, colour)` extension can never accidentally reach the vec2 chain.

This supersedes the earlier "impl blocks on merged structs" idea — methods are dropped entirely. Constructors are just global functions; subfeatures extend construction via the same chains.

### 9. provenance-ordered linearisation — IMPLEMENTED (2026-08-14, same day)

*Implementation (fm-spec-2 #p17):* fmlink now linearises by provenance
timestamp — `chronologise()` reads every anchor time from transcripts/, takes
each node's first spec citation as its position, ties resolve by
(containment, path), and a code-free grouping node takes the earliest key in
its subtree so a late regroup (e.g. shell/pwa, cited today) never displaces
its older children. A code-bearing node without a citable anchor is now a
LINK ERROR — provenance became load-bearing, which is the doctrine enforced
mechanically. The migration diff matched the experiment's prediction exactly:
only `route` and `update` rewired (plus the corresponding fragment slots),
and both were verified commutative by inspection — every route member guards
disjoint paths, every update member guards disjoint event types, all
delegating via `existing` otherwise. All products build and run; hello_only's
subtraction still works. The per-node "linearise before X" override remains
unimplemented — add it the day chronology is wrong for a real node. fm.md's
ordering section now differs from practice twice over (order.md as ordering;
timestamp only as fallback) — noted in errata.

Original proposal follows.

### 9a. original proposal — USER-ENDORSED DIRECTION (2026-08-14)

> (transcripts/2026-08-14-fm-spec-2.md#p9)
> Hm, that's an interesting oversight on my part. It means regrouping features will change their ordering, which will obviously change behaviour […] I wonder if there's a way we can stabilise ordering even in the face of regrouping?

**Problem.** Chain order = DFS position, so tree shape carries behavioural
weight: a regroup can silently rewire who wraps whom. Proposal 2 noted the
ambient version (cross-subtree order is traversal order, not chronology);
regrouping is the acute version, and the 4–6 cap *forces* regroups.

**Options considered:** (1) discipline + evidence — keep DFS, require regroups
to show an empty chain-dump diff; stabilises by vigilance, and forbids regroups
that should move things. (2) chain lockfile — a checked-in manifest of each
chain's member order; behaviour regroup-proof by construction, but a second
ordering artifact. (3) **linearise by provenance timestamp** — every node
already carries the timestamp of its originating prompt; make that the
composition order. The transcript becomes the true program order; the tree
becomes a pure grouping/selection view with zero behavioural weight.

**Decision leaning: option 3** (#p10). The user's rationale: composition is an
agent's job, and in the long run humans won't read the tree much — a human
wanting to understand the code is better served by an agent-produced ad-hoc
tutorial or interactive session than by tree inspection. So the tree should be
freely regroupable for whatever organisational purpose, with no behavioural
consequence.

**What falls out:** "newest is outermost" becomes globally true (restoring the
original creation-time intent that DFS broke); the #p91 constraint ("tree
position bounds what a node can extend") dissolves into causality — a node may
extend anything that existed when it was written, which un-blocks cross-cutting
extensions like serve/features → gate; order.md's role shrinks to catalog +
selection, completing the #p132a trajectory; composition order = order of
intent, the provenance-modularity thesis made mechanical.

**Costs / open points:** chronology is occasionally wrong — needs a rare,
explicit per-node override ("linearise before X") to replace today's implicit
order.md editing; the linker must read timestamps from provenance anchors
(transcripts record them under every #pN; grouping nodes are code-free so
their skip-provenance allowance doesn't matter); tie-breaking within one
prompt (multiple nodes from one request) needs a rule — anchor order, then
order.md as tiebreak, is the obvious one. Migration: build the chain dump
first, switch linearisation, diff — an empty diff proves the current tree
already agrees with chronology; any non-empty diff is reviewed as a real
(latent) behaviour difference.

**Experiment run (2026-08-14, fm-spec-2 #p11): DFS vs chronology on muon.**
`fmlink.py --chains` built (prints every chain's contributors in linearisation
order; stable sorted output for diffing). A scratch comparator resolved every
muon node's provenance timestamp and checked the 8 multi-member chains:

- **6 agree** with chronology already (handle, handle_msg, is_public, render,
  send_sms, serve).
- **route REWIRES**: diag (16:32) linearises after comms/messaging (21:49)
  because the diag subtree sits later in the tree; chronology would move
  diag's route link from position 6 to 3. Also *within* diag's own order.md,
  readout (21:38) is listed before blackbox (21:13) — the tree is already
  non-chronological inside a single node, so "order.md = creation order" has
  in practice drifted.
- **update REWIRES**: scope (08-14 07:52) sits innermost-but-one in DFS but is
  the newest link; chronology would make it outermost (loop → tap → tap/sync
  → scope).

Both rewires look behaviourally commutative — route links dispatch on disjoint
paths and delegate otherwise; update links dispatch on disjoint event types —
which is why the divergence has been invisible. But "looks commutative" is
exactly what migration must verify per chain, and the dump gives the review
list. Tie-break data point: all same-timestamp ties in muon are parent/child
pairs sharing one prompt (shell+logo #p38, loop+tap #p97), resolved by
containment (parent composes first) — so the ordering rule is
(timestamp, containment, anchor-rider order), no order.md tiebreak needed
so far.

**TWO-PHASE FEATURE LIFECYCLE (transcripts/2026-08-14-fm-spec-2.md#p16) —
resolves the depth question.**

> One way to get around the depth issue is to have a "two-phase" approach to features. When we first create a feature, we'll expect a bunch of churn, and nobody else is using the feature, so it's safe to make big changes to it. Once we ship/publish and there's other people using the feature, that's when the feature should become immutable (except for refactoring). That means that a string of tweaks won't create big feature subtrees unless we decide they need to.

**The reframe**: subfeature-per-refinement is a *compatibility* mechanism, not
a history mechanism. History lives in transcripts and git; a refinement
subtree exists so that *other consumers* can decline the change by toggling
it. While nobody else uses a feature, a subtree serves no one — so the
discipline was paying compatibility costs with zero consumers.

**The lifecycle**: a **draft** feature churns in place — tweaks amend the
node's own spec and code; provenance accumulates as a list of prompt
citations in the spec (the reverse index already handles many-prompts-to-one-
node as "coalesced"). **Publication** — the moment others depend on it — is
the freeze point: the spec becomes the immutable contract; the implementation
may still change *toward* the spec (bug fixes) or via behaviour-preserving
refactoring (the existing rule, backed by tests); behaviour *changes* become
subfeatures, individually toggleable, which is exactly when toggleability
earns its keep. Prior art: semver's pre/post-1.0 line, applied at feature
granularity.

**Consequences**: (1) the one-prompt-per-node law refines to *one prompt per
published change* — draft nodes may coalesce prompts; (2) absorb (collapsing
a refinement stack) becomes mostly unnecessary, and where a draft grew
internal structure, publish is the natural absorb point — squash before
freezing; (3) this is also the sharing contract fm.md's intro promises: what
you share is frozen, your later changes are additions the recipient can
decline; (4) open: what marks publication concretely — a spec field, presence
in another user's product, a version stamp? With one user today, everything
is effectively draft, which is why depth pressure felt artificial.

## muon (the real feature space — started 2026-08-13, #p32)

Shared infrastructure for all apps/tools: a Rust/wasm PWA with four base capabilities. Root node at `features/muon/`; apps will be subfeatures of muon; products = muon + an app subtree.

1. **offline** — works with no internet ("local server cache")
2. **users** — authentication and users built in
3. **blackbox** — always-on recording + reproducible replay to catch errors
4. **feature UI** — feature management through a simple UI (contexts made real)

Decisions (2026-08-13):

- **Offline = service worker** + Cache Storage/IndexedDB; app logic in wasm; minimal JS shim (scaffolding, not feature-modular).
- **One feature defines behaviour across client AND server roles in the same code** — easier for people and agents to understand than a tree split.
- **REFINEMENT (#p34): code is placeless; data has places.** Any function can potentially run anywhere — the only constraints are (1) data locality and (2) authority/trust. Model: feature state/resources carry placement policies (authoritative-on-server, replicated-to-device, local-only; screen/camera = client-only resources; a *place* = an inventory of stores/resources). A function's runnable-set is derived from its typed store/resource parameters — syntactic, no inference. The message layer routes to wherever the needed data lives (possibly right here); offline shrinks reachable places but functions with local data keep running. **Authority axis**: functions running against replicas produce *provisional* results reconciled on reconnect — this is both the offline-writes sync story and the honest limit of offline auth (a device-held sealed verifier lets reauthentication protect local data offline — cf. Windows cached credentials, passkeys — but can't mint server-trusted sessions; those re-establish on reconnect). Prior art: **local-first software** (Ink & Switch, CRDTs/Automerge); fm's twist is that placement policy lives on feature-scoped data declarations, so a feature carries its own distribution story in-file. Supersedes rigid `client`/`server` role types below (first-param typing survives as "what this needs", not "where this goes").
- **CAPSTONE (#p37): placement lives in the product, not the feature.** Feature code is fully place-agnostic: it declares *needs* (typed store/resource params) and *constraints* ("must live in a trusted place", "never leaves the device") — never locations. The product description grows a topology section: which places exist, each store's authoritative home, replicas, transports. Same code → standalone desktop (one place), thin client (all stores on server), fat client (server = data store), or any mix — four product files, zero code changes. **The commitment that makes it sound**: semantics are pinned at the distributed end regardless of topology — functions always interact with a local replica, always communicate by async message, reconciliation always exists. Colocation is purely optimisation (linker compiles same-place messages to direct calls, no serialization) — placement changes performance/freshness, never meaning. This dodges the classic location-transparency trap (Waldo et al., *A Note on Distributed Computing*: latency/partial-failure/concurrency leak through "remote = local" abstractions; we invert to "local = remote, but fast"). Consequences: (1) **distributed bugs replay locally** — a blackbox stream recorded on the full constellation replays in the single-process build under a debugger; (2) **security posture is a product property**, so the linker must validate product topology against feature constraints (feature says *must*, product says *is*, linker refuses violations). Prior art: Electric Clojure (compiler-managed client/server splicing), Erlang location transparency, local-first.
- **USE CASE (#p35): multi-device single surface.** Laptop = main tool screen, phone/iPad = auxiliary interface (e.g. colour picker), all one surface. Falls out of the placeless model: N places, one shared state; picker runs where a touch surface is; picked colour = write to replicated store; canvas reacts. Forces four additions: (1) **sessions** — places join a session; same-user authentication = membership; pairing (QR scan) is an auth flow owned by the users feature; (2) **per-place presentation via dispatch** — surface kinds are resource types, `render(s: main_surface)` vs `render(s: touch_surface)`; a place renders what its inventory satisfies; (3) **multi-writer event log** — blackbox replay needs deterministic ordering across places; v0 = server-sequenced relay; (4) **transport**: v0 relays device↔device messages through the server (websockets); WebRTC data channels later as another route for the same messages. Freebie: N places/one user ≡ N places/N users mechanically — multiplayer collaboration is the same substrate + permissions.
- Original representation proposal (superseded in part by the refinement above):
  - **Role = first parameter type.** Muon defines `client` and `server` structs (also the home for per-role state). `fn handle(s: server, req: GreetRequest)` is server-side by signature — no annotations, read syntactically, same keying the dispatch machinery already uses. Functions with neither role type are shared.
  - **Cross-role calls are messages, not RPC**: `send(c, GreetRequest{…})` posts; the function taking `(server, GreetRequest)` receives; the reply lands in the function taking `(client, Greeting)`. Routing = multiple dispatch by (role, message type). Avoids async coloring and hidden network magic.
  - **Messaging is the one crossing point**: client/server boundary, offline queue (service worker holds messages while unreachable), and blackbox recording/replay are all the same event stream. One event core serves capabilities 1 and 3 and the role split.
  - **Linker work implied**: split emission by first-param role type into wasm + native products; serde derives on merged structs; message-routing glue.
  - Prior art: tierless languages (Eliom, Links, Ur/Web), React Server Components ("one file, compiler splits"); message-passing core à la Elm/actors (also what makes deterministic replay tractable).
  - Open: request/response correlation (one message type, several possible repliers?); is `send` fire-and-forget only; every `(server, T)` fn is de facto API surface — users/auth feature must gate callability; `send` behaviour when offline (queue = capability 1 expressed in the message layer).

**FIRST LIGHT (2026-08-13, #p38)** — hello-muon PWA, mobile format, showing the nøøb logo `ᕦ(ツ)ᕤ`, live at **https://muon.nøøb.org** off the mac mini via cloudflare tunnel (ingress muon.xn--nb-lkaa.org → :8095; LaunchAgent `com.noob.muon` — NB distinct from `com.noob.muon-server`, which is the dev surface). Features: `muon/serve` (stdlib static server, no crates), `muon/shell` (`render()` chain base + PWA assets: loader html, stale-while-revalidate service worker for offline, manifest + placeholder icons), `muon/shell/logo` (extends render with the logo). Product `products/muon` is the first **two-place product**: `places.md` declares `server: native, entry=serve` and `client: wasm, entry=render`. Linker gained: places.md parsing, per-place crate emission from one composed body, wasm cdylib target with a generated `fm_entry()` export (String → packed ptr/len, hand-rolled JS glue in the loader — zero crates, no wasm-bindgen), feature `assets/` copied into `build/site/`. Deploy: `tools/deploy.sh` (ftr-style: LAN/public host pick, refuses dirty tree, ships arm64 binary + site via rsync, kickstarts the agent). Placeholder black-square icons — proper logo icon later. Client wasm: ~20KB.

**LOGIN (2026-08-13, #p39-40)** — SMS-PIN auth live, ported from ftr's battle-tested `auth-gate.ts` (itself a port of earlier nøøb muon — the flow came home). Feature tree: `muon/users` (guest list `~/.muon-auth/users.json` outside the deploy tree, read per request; stateless HMAC-SHA256 session cookies, 1 year, survive deploys), `users/pin` (4-digit PIN, 5-min TTL, 3 attempts, 5 SMS/hr rate limit, pending persisted to disk across restarts; base `send_sms` = console), `users/pin/vonage` (real SMS via the mini's `~/.agent-config.json` creds, TLS via `curl` subprocess; missing creds → `existing.send_sms` console fallback — swappable delivery as an extension chain), `users/gate` (extends the serve `route` chain: tunnel traffic needs the cookie, login page on 401 no-store, local/LAN ungated via `cf-connecting-ip` detection; login page carries ftr's iOS-autofill and Safari-401-cache fixes). **Architecture moved**: serve refactored to `request`/`response` structs + a `route()` extension chain (fm's middleware = feature redefinition + `existing.route()`); the linker gained per-feature **`deps.toml`** cargo dependencies (merged, version conflicts = link error) — first deps: `sha2`, `serde_json`. `_`-prefixed names are test users (PIN to server log, no SMS) — the whole flow is testable by curl without spending credit. Next auth step (deferred): passkeys/Face ID via WebAuthn — needs P-256 + CBOR crates.

**SHELL-PUBLIC FIX + DIAG (2026-08-13, #p44)** — gating the app shell froze logged-out installed PWAs (sw only caches 2xx; even sw.js updates got the 401 login page). Policy corrected: *shell is public, data is gated* (`gate::is_public`); the shell asks `auth/whoami` and routes logged-out visitors to login.html. `muon/diag` added: every client launch posts a one-line report (running/server version, authed, sw state, ua) and JS errors to public `POST diag/report` → `/tmp/muon-diag.log` on the mini — remote debugging of installed devices, first step toward blackbox. On-screen version stamp bottom-right. Confirmed end-to-end from the user's iPhone (launch report received, authed, sw controlling, self-update armed). PWA update rule: deploys arrive on the next launch automatically.

**DESIGN PRINCIPLE — PWA-only (2026-08-13, #p50):** "let's just focus on the PWA experience - the browser adds all kinds of bullshit we don't want to deal with if possible." The installed app is *the* product; browser contexts are scaffolding. Mobile browsers only ever see the install screen (logo + add-to-home-screen steps — `muon/shell/install`). Consequences embraced: one storage/cookie context per device (no Safari-tab/PWA double-login confusion), standalone display assumptions throughout (no browser chrome to design around), diag reports distinguish `pwa:`. Dev-only affordances that remain: desktop browsers pass through (the dev surface), and `?browser=1` is an undocumented session-scoped bypass for testing — both removable if they ever leak into the product experience.

**CACHING PRINCIPLE — freshness deadline (2026-08-13, #p52-54):** evolved across three prompts: (1) online always means current — cache is a fallback, never a substitute for freshness; (2) refined for low-bandwidth mobile: what matters is *time*, not size (and iOS has no bandwidth API; a deadline measures size×bandwidth directly). Final rule, in `sw.js`: **the cache serves only when the network can't deliver within the deadline (1.2s)** — fresh-in-time always wins; slow networks degrade to last-known-good while the fetch completes in the background and refreshes the offline copy; offline is the deadline missed instantly; nothing-cached waits on the network. `/auth/*` and `version` never touch the cache. This is the local-first authority model at the HTTP layer: network copy authoritative, local copies for availability, staleness bounded by choice.

**SYSTEM PANEL + UPDATE AWARENESS (2026-08-13, #p55-60):** the corner build stamp is muon's first system UI — tap for: logged-in name (whoami returns it), running build + update state, what's-changed list (`changes.json`, commit subjects tagged with build numbers, generated at deploy — commit subjects are now a product surface, write them user-readable), log out (`POST auth/logout`, clears cookie; stateless tokens can't revoke server-side — deferred), update button. Update detection: silent auto-update on launch; foreground/online re-check; 60s poll while visible; panel re-checks live and distinguishes "up to date" from "can't reach the server" (**rule: a failed check must surface as uncertainty, never as the hoped-for answer**). Verified on device incl. deliberate build-22 logo-move test. Chrome must respect safe-area insets (corner-clipped stamp lesson). **Next: `muon/push`** — Web Push for installed PWAs (iOS 16.4+): panel toggle for permission (user gesture required), per-device subscription store, VAPID/ES256 server-side (the same P-256 crypto passkeys need — build push first, Face ID walks through free), deploy.sh pings server → notify all devices even when the app is closed.

**FACE ID / PASSKEYS — LIVE, CONFIRMED ON DEVICE (2026-08-13, #p63):** `muon/users/passkey` — WebAuthn with hand-rolled server verification (`p256` ECDSA + `ciborium` CBOR, ~300 lines, no webauthn framework): enrol via the system panel while SMS-logged-in, sign in with Face ID from the login page; same year cookie as PIN; SMS remains bootstrap + recovery; passkeys sync via iCloud Keychain. Verified by synthesized assertions (genuine sig accepted, tampered rejected) then by the real ceremony on the user's iPhone. Battle scars → permanent guards: (1) getrandom's `js` feature poisoned the wasm with wasm-bindgen import demands → black screen, diagnosed in one diag line, fixed with `custom` feature (wasm back to 17KB — unused crypto strips fully); **deploy.sh now smoke-tests that client.wasm instantiates with zero imports before shipping**. (2) fmlink `deps.toml` lines now pass through verbatim (feature flags etc). Auth stack is now complete: install wall → SMS bootstrap → Face ID daily → logout → recovery.

**NEXT UX STEP (#p68): auto-enrol on first login.** Face ID and notifications should just happen automatically on first login on a device — no panel buttons to discover. Mechanism: iOS requires user gestures for both `credentials.create` and `pushManager.subscribe`, but the login flow's final tap qualifies — so first-login onboarding chains: PIN verify (or Face ID sign-in on a synced device) → passkey create → push subscribe → enter app. Panel buttons remain as fallback/retry. Push notifications LIVE (2026-08-13, #p66): muon/push, Web Push from the RFCs up (VAPID ES256 + RFC 8291 ECDH/HKDF/AES-GCM via curl), deploy announcements by extending serve() — verified by synthetic browser decrypt + real device.

**LAW (#p91): pointer nodes are forbidden — only grouping nodes may be code-free.** Legalised immediately with existing machinery: `icon` owns its PNGs (feature assets); `gate/public` owns the `is_public` policy as a chain extension over gate's now-empty base. Discovered constraint while trying the same for `serve/features`: **an extension must linearise after its base** — features (DFS position 3) cannot extend gate's chains (position ~29), so a node's tree position bounds what it can own; public-ness of /features/ stays in gate/public's list. Remaining illegal nodes fall into two mechanism-shaped groups: CSS/markup owners (pinned, corner, lozenge, steps, drawers…) await **asset composition** (features contribute page fragments assembled in linearisation order under order.md control); behaviour owners (honest, watch, fresh, deadline, pwa, enrol, button, everypage…) await the **event core** (client logic as Rust chains). The /features/ page-rendering nodes (drawers/place/tidy/fmdoc) additionally raise the tool-boundary question: their code is explorer/exporter scaffolding — the likely resolution is the page template/CSS migrating into the features node's assets so refinements own fragments of it.

**NAMED DEBT — pointer nodes (#p88):** the node audit completed the tree of *intent*, but ~⅓ of muon's behaviour (555 lines of JS/HTML in shell/gate/install assets + deploy.sh's stamp/changes/tree-export) is monolithic: 20 nodes are spec-only pointers into it, so unticking e.g. `update/honest` currently changes nothing — fm's toggle promise doesn't yet hold client-side. Root causes: the wasm client is render-a-string-only (no event loop), and the linker has no asset-composition mechanism. Payoff path, in order: (1) **event core** — client update/render/event loop in Rust; pointer-node behaviours become real feature_ objects on chains; loader shrinks to a generated bootstrap; (2) **asset composition** — features contribute CSS/JS fragments assembled in linearisation order under order.md control; (3) deploy.sh's product behaviours fold into the linker. Until then, a spec-only node whose behaviour lives in another feature's files is *debt by definition* — the tree must eventually own its code.

**THREE SCREENS, ONE NUMBER (2026-08-14, #p111 proven):** the shared-tap demo ran live — iPhone + iOS simulator + browser tab, all logged into the mini, nine taps in ten seconds converging on one global counter through the full messaging stack (optimistic local bump → _send outbox → POST /msg → handle_msg → file-backed total → publish → three long-polls → TapTotal as events → update chains). The tree restructure that shipped with it: `events`→`loop` (the app core, not a log), observability united under `diag`, `comms{push, messaging}`, `serve/threads` (concurrency as one toggleable chain link). Multi-device single-surface is now working engineering; sessions/per-user scoping, WebSockets, and typed message routing are the named next rungs.

Open design questions (pre-code):
- **Blackbox shapes the core**: deterministic replay requires every nondeterministic input (user events, network responses, time, random) to flow through recordable channels — which argues for an event-driven core where inputs are injected, record = log the stream, replay = re-inject it. This is the first true cross-cutting feature and must be designed in from day one, not bolted on.
- **Feature UI pulls contexts forward**: runtime feature toggles need the `@dynamic` guard machinery, per-user settings (`@user` vars), and the linker embedding feature metadata into products so a running app can introspect its own tree.
- **Linker gaps muon hits immediately**: per-feature cargo dependency declarations (merged into the generated Cargo.toml); a wasm/cdylib product target (+ index.html, manifest.json, service-worker shim emission); parser robustness against real Rust (async, generics, use statements).

**Spec style convention (2026-08-13):** code descriptions are written as short paragraphs — one per thing described (entry/extension points first, then mechanics, then helpers) — never a single dense block.

## tools (scaffolding — not feature-modular)

- `tools/export_transcript.py` — exports a Claude Code session log to `transcripts/<date>-<slug>.md` (verbatim prompts with stable `#pN` anchors + timestamps).
- `tools/explorer.py` — three-pane *feature browser* at `http://localhost:8123`: feature tree | spec + code | transcript. The tree shows feature nodes only (ordered per `order.md`, which itself stays hidden; unticked features dimmed/struck). Clicking a feature renders its spec with the `.rs` implementation(s) beneath, and opens the transcript pane at the feature's provenance prompt — tree links carry the `#pN` fragment so the browser scrolls natively. Nodes with children expand/collapse via native `<details>` toggles (no JS); by default only the path to the selected feature is expanded. Fully server-rendered plain HTML, no client JS; agents can `curl /feature/<path>`, `/view/<repo-path>`, or `/raw/<repo-path>`. Python stdlib only, incl. a built-in markdown renderer covering the fm subset; bare provenance refs (`(transcripts/…#pN)`) are auto-linkified.

## v0 linker — BUILT (2026-08-13)

`tools/fmlink.py` (Python, ~250 lines, regex-level parsing — quick first pass; a Rust rewrite using `syn` is the obvious v1). Usage: `fmlink.py [product] [--run]`.

Working end-to-end:

- linearises `features/` depth-first with sibling order + include/exclude from `order.md` checklists (validates: unlisted folder = error, listed-but-missing = error; unticked = excluded with notice)
- chains same-named functions in linearisation order; rewrites `existing.fn()` to the previous definition; emits global `main` calling the outermost link
- flat-merges same-named structs; duplicate field = link error citing both source locations
- emits a cargo project at `products/<name>/build/src/main.rs` with a per-line source map; rustc errors are reported as `features/<path>/<file>.rs:<line>` (verified: a planted type error mapped exactly)
- demo product runs: `Hello, world!` + `Goodbye...`; unticking `goodbye` in `hello/order.md` removes the farewell

The demo features live under **`features/test/`** (moved 2026-08-13, #p30, to keep the root clear for the real feature space): `test/hello[/goodbye]`, `test/colour[/alpha]`, `test/vec`, `test/sums` — each with a spec in the fm.md format. `test` itself is a spec-only container node (no code). The `demo` product is now a single symlink to the `test` subtree.

**Product subsetting added (2026-08-13, #p20):** the linker now composes from `products/<name>/` — a feature tree of symlinks into `features/` plus a root `order.md`. A symlink to a feature folder imports it with all subfeatures (`products/hello_goodbye`); a product-local folder with symlinked files and its own `order.md` overrides the shared node — `products/hello_only` unticks `goodbye` this way and prints only the greeting. An unticked `order.md` entry no longer needs a local folder (that's what subtraction looks like). Diagnostics resolve symlinks, so errors still cite `features/…` real paths. All three products (`demo`, `hello_goodbye`, `hello_only`) build and run correctly.

**Glossary term links (2026-08-13):** convention settled — glossary terms are written backticked with a leading slash, `` `/term` ``, in specs. The explorer resolves them against the combined glossary (every spec's `## glossary` section, definition bullets `- **term**: …` get `#term-<slug>` anchors) and renders them as links to the defining feature's bullet; a `/name` matching a feature resolves to that feature's page instead; unresolvable ones stay plain code spans. Hovering a term shows a pure-CSS popup card with the definition (or the feature's one-line description) plus a clickthrough to the source — the definition text is embedded server-side, so still no client JS. fm.md's older `/term/`-style examples may want updating to the backtick form.

Known v0 limitations (deferred, not forgotten):

- one `feature_` struct per feature node (all `.rs` files in a node are treated as one feature)
- ~~`impl` blocks on merged structs~~ — superseded: methods dropped by design (proposal 8); computation is global functions + generated dispatch/operator glue
- contexts, `@shared`/`@user` variables, cross-cutting targets, wasm: out of scope, per plan
- parsing is regex + brace-matching, not a real Rust parser — adequate for fm-style code, will misread exotic syntax (strings containing braces, macros defining items)

## prior art (vocabulary to borrow, pitfalls to dodge)

- **FOSD / AHEAD (Batory)** — feature-oriented software development; step-wise refinement by feature composition. Closest ancestor to fm's tree model.
- **AspectJ / AOP** — `before`/`after`/`around` advice, pointcuts; the weaving-order and fragile-pointcut problems are well documented here.
- **Scala stackable trait modifications / mixin layers** — function extension via linearised `super` calls; their linearisation is one answer to sibling ordering.
- **Delta-oriented programming** — features as deltas (add/modify/remove) over a core; close to fm's product-override model.
- **Context-oriented programming (ContextL, COP)** — dynamically scoped layer activation; directly relevant to fm contexts.
- **Software product lines** — the products/ concept; the "optional feature problem" (two optional features whose interaction needs glue belonging to neither) is a known trap worth designing for early.

## fm.md errata (for the author to fix — noting only, not editing)

*(updated 2026-08-13 after the existing/flat-struct revision)*

- **Stale line ~98**: "the linker automatically converts them to `colour.colour.r` or `colour.alpha.a`" — leftover from the nested-struct design; with flat merge no conversion happens (`col.r` / `col.a` are already correct). The sentence can simply become: both features refer to `col.r` or `col.a` directly.
- **Line ~48 explanation vs code**: text still says the linker "adds the call to `goodbye()` to the end of the existing definition of the global `hello()` function", but the mechanism is now the extension: `feature_Goodbye::main` *calls* `existing.main()`. The shown output (two sequential calls in global `main`) is the *inlined* result; the more literal lowering is: global `main()` calls `feature_Goodbye::main()`, inside which `existing.main()` is rewritten to `feature_Hello::main()`. Worth deciding which the doc shows (see discussion).
- Naming drift: "the global `hello()` function", shell shows `./main` then `./hello`, code composes `main()`.
- "print our greeing" → greeting (~32); "turn this into the a single global function" (~26).
- Rust syntax: struct fields separate with `,` not `;`; `existing.main()` missing `;` (~43).
- `some_fun` vs `some_func` (~166/174).
- *(added 2026-08-14)* **Ordering section** (~187): describes unchecking in
  order.md as exclusion in general; per the #p132a rule, selection now lives in
  products (shared-tree order.md is catalog + ordering, stays fully ticked; a
  product unticks via its own order.md override). The section may want to
  relocate that semantic to products.
- *(added 2026-08-14, proposal 9)* **Ordering section** (~187): order.md is no
  longer the composition order and the timestamp is no longer the fallback —
  it's the rule. Composition order is the provenance timestamp of each node's
  cited prompt; order.md is catalog + selection. The section would invert:
  timestamps primary, order.md for grouping/selection only.

## hygiene todos (2026-08-14, from the fresh-eyes review)

*Repo-health items, distinct from the feature rungs in handover.md. Ordered:
cheap insurance first, then the debt it protects, then doctrine. Tick as done.*

- [x] **1. chain dump** — DONE 2026-08-14: `fmlink.py <product> --chains`
  prints each chain key with its contributors in linearisation order and
  exits; stable sorted output for diffing. First run fed the proposal-9
  chronology experiment (see there: 6 of 8 muon chains already chronological;
  route and update rewire).
- [x] **2. shell regroup** — DONE 2026-08-14 (fm-spec-2 #p13–14):
  `shell/pwa{icon, install, pinned}` groups "being an installed app"; shell is
  at 4 children. The three were contiguous, so linearisation was preserved —
  proven by an empty `--chains` diff (which now also dumps fragment order per
  page/slot; fragment order is behaviour: cascade + script wrap order). The
  group toggle test surfaced the **optional-feature problem** live: diag's
  `page`-targeted fragments failed when the install page's owner was excluded,
  and a stale install.html lingered in site/ from the previous build. Linker
  fixed: `page` now means "every HTML page present in the composition" (soft;
  explicitly named pages stay hard requirements), and stale composition-target
  pages are removed when their owner is excluded. Toggle then passed cleanly
  both ways.
- [~] **3. features-browser template migration** — PINNED by user decision
  (2026-08-14, fm-spec-2 #p15): the feature browser and linker may live
  outside the tree as scaffolding for now. The #p9–#p22 audit gap stays a
  known, accepted gap rather than debt-to-clear.
- [x] **4. errata consolidation** — DONE 2026-08-14: the #p132a
  ordering-semantic note is folded into the "fm.md errata" section above.
  (Struct extension confirmed flat — doctrine and merge_structs agree; the
  sole vestige is the stale `colour.colour.r` sentence already listed.)
- [x] **5. lib/chain ratio** — DONE 2026-08-14: `--chains` now ends with a
  summary line (currently 1403 chain / 84 lib lines, 5% lib). A steady climb
  means typed code is escaping the composition model — the early-warning gauge
  for when the regex parser must grow up (or the syn-based v1 arrives).
- [x] **6. export_transcript collision guard** — DONE 2026-08-14: the tool now
  refuses to overwrite a transcript recording a different session id (the
  header's ``session `<id>` `` line), telling you to pick a distinct --slug;
  same-session regeneration is unaffected. Motivated by the same-day collision
  that briefly clobbered the session-1 transcript (recovered from git).
- [x] **7. depth doctrine** — RESOLVED IN PRINCIPLE 2026-08-14 (fm-spec-2
  #p16): the two-phase feature lifecycle — see "TWO-PHASE" below. Draft
  features churn in place (no subtree growth from tweaks); publication is the
  freeze point, after which changes become subfeatures. Absorb becomes mostly
  unnecessary; where wanted, publish IS the natural absorb point.

- [x] **8. export_transcript log-dir bug** — FIXED 2026-08-14 (session 3):
  `PROJECT_LOG_DIR` was hardcoded to `…-Users-asnaroo-Desktop-experiments`
  (missing `-fm2`), so it silently exported whichever session was newest in
  the *parent folder's* log dir — one wrong transcript was written and
  deleted before anything cited it. Now derived from the repo's own path
  (`str(REPO_ROOT).replace("/", "-")`), the exact fix ftr's feature_common
  established for this bug class. Tooling fix, no node (per the taxonomy).

## ideas parking lot

Superseded — passing whims now live in `ideas.md` at the repo root.
