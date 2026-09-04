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

**THE TOOLBAR AND THE LOGO ARE ONE SYSTEM (fm-spec-3 #p53, end of day 3):
have-now vs need-now.** User-stated, near-verbatim: the toolbar at the bottom
and the logo-button at the top are connected. If the tool exists to do a job
you need right now, chances are it's in the toolbar — *or rather, the app
evolves things that way for you*. If you need a tool that isn't on your
toolbar right now, you tap the logo at top right and get an **agent prompt
input**. You say the thing you want to do. If the tool exists, it pops up,
**introduces itself**, and adds itself to your toolbar. If not, the agent
queues a **"build this tool" request**, gets to work, and comes back with an
update when it's ready.

What this connects (all already live, none of it coincidence): `/account`
parked the logo tap *for the agent interface* — this is that interface's
spec-in-waiting. "Introduces itself" is the demo/`show me how` doctrine
becoming a tool's arrival ritual. "Comes back with an update" is the
`/policy` + `/queue` channel — a built-tool announcement is just a release
whose changes entry you already ticked in advance by asking for it. "The app
evolves the toolbar for you" makes the toolbar per-user state (today
`tools_list` is composition-global; per-user toolbars are context-manager
territory, same plumbing as `update_ticks`). And the "build this tool" queue
is the fm1 FMT vision — the kernel's surface is the agent; tool-call, then
tool-construct — now with a concrete UI: one lozenge, top right. The full
ladder (ask → surface existing → compose existing → build new) is graded
derivation applied to capability itself.

**THE NØØB BUTTON IS META-EVERYTHING (fm-spec-3 #p54): it controls how muon
works; the rest is muon.** User-stated, near-verbatim ("nøøb button" is now
ash's shorthand for the logo button — glossary-worthy when it lands). The
tickable feature list should filter by the current tool: in taps, you see
taps-related features only. That makes it **orthogonal to tools, not just
another tool** — so it belongs on the nøøb button, not in the toolbar. The
nøøb button becomes the reflective surface — feature choice, the agent
prompt (#p53), how-muon-works — while the toolbar stays the operational
surface: using muon vs steering muon, one lozenge for the meta-level.

Implementation hooks, for when this becomes nodes: filter-by-tool needs a
feature→tool mapping on changes entries — deploy already computes the
touched node paths per release (it prints them), so stamping them into
changes.json is nearly free, and "taps-related" = path-prefix match against
the open tool's subtree. The queue view then takes an optional filter;
opened from the nøøb surface inside a tool, it defaults to that tool's
subtree.

One honest tension to resolve when we get there: `/account` (#p46) moved
the *panel* — who-you-are, updates, logout — INTO the toolbar as the 👤
tool, and this doctrine reads all of that as meta ("how muon works"). If
the meta-level consolidates under the nøøb button, account may migrate
back — or split (identity stays a tool; feature-steering goes meta).
Deliberately unresolved tonight; tick storage and the queue view carry
over either way.

**ACCOUNT IS A SOCIAL TOOL; THE TENSION RESOLVES (fm-spec-3 #p55).**
User-stated, near-verbatim: account should be what it was originally
intended for — **a super simple social network**. Everyone gets a page
(*a post*), with whatever on it, connections to other people, group
membership — the standard stuff, minimal and snappy. That's how ash wants
it for "the app" we're building ("whatever that is"). And the split from
#p54 lands cleanly: the nøøb button also creates user-scoped data, but it
controls **your experience of muon**; account is a tool *in* muon.

So the #p54 tension is resolved by division: 👤 stays in the toolbar and
becomes the social surface (your page, people, groups); the system/meta
freight it currently carries (updates, feature ticks, logout?) migrates to
the nøøb button when the meta surface is built. Two loads, two buttons.

The load-bearing phrase is "everyone gets a page **(post)**": a person's
page is a post in the same store as everything else — the places doctrine's
immutable post database reaching the social layer. One substrate carries
the campaign app's geotagged posts AND profiles; "connections" are links
between posts (the additive match-link pattern); "group membership" is the
membership axis the scope machinery already models (audiences/rings). This
is also microclub's original brief re-arriving on better foundations: the
community noticeboard whose point is making the organisation's human face
visible and drawing people into deeper involvement — groups, pages,
connections — now as features over the post store rather than a bespoke
app. Minimal and snappy is the spec: the standard stuff, nothing clever.

**THE QUEUE WANTS TO BE A TREE (fm-spec-3 #p59, queued thought).**
User-stated: the tickable feature-list view could and perhaps should be a
**tree view** — which hints at a different approach to a feature tree
viewer; maybe the queue view evolves toward that, integrating with the
"source code view" button (`/panel/source`, which opens `/features/`).

The convergence this names: the queue's chronological release list and the
served feature tree are two views of the same structure, and per-feature
consent ticked *on the tree* is literally the user's own `order.md` — the
checkbox vocabulary the repo already uses for composition, become
user-scoped runtime data. Ticks by build (today) then roll up to ticks by
node (path-stamped changes entries, #p54's mapping, make each release a
delta on the tree). End state: one tree surface serving reader, chooser,
and — via `/source` — the code itself; the explorer tool and the queue view
meet in the middle. Developer view and user consent UI were never
different things, just different depths of the same tree.

**THE NØØB BUTTON IS THE WHOLE GAME (fm-spec-3 #p70): an agent-powered IDE
for end-user programming.** User-stated, near-verbatim: all the interesting
stuff is in the nøøb button — and **stop calling things "meta"** (the word
is irrevocably tainted by the unfortunate Mr Zuckerberg; vocabulary ruling —
earlier notes entries keep their as-spoken wording, new prose says
**steering**: the nøøb button *steers* muon, the toolbar *uses* muon, per
the glossary line that already existed). The glimmerings: "how do I use
this?" / "do xyz" / "I need xyz" — get the nøøb button right and it's not
far off **an agent-powered IDE for end-user programming**.

The three utterances map to machinery with names already: *"how do I use
this?"* is the explanation surface — demos narrated at the asker's level
(fm2 decision 2 + the show-me-how doctrine); *"do xyz"* is the agent
driving existing tools (fm1's tool-call); *"I need xyz"* is the build
ladder (#p53: surface existing → compose → build new; fm1's
tool-construct). The IDE framing is exact, not loose: the "source" the end
user programs is the feature tree (served at /features, provenance-anchored,
tickable), the "compiler" is the fm loop itself, and the conversation IS
the programming act — every ask becomes an anchor, every anchor a node,
every node a shipped, toggleable capability with the asker's name on the
founding quote. End-user programming where the program is the product's
own evolution.

Buildable first brick (proposed, not yet ruled on): the **ask inbox** —
the nøøb button grows a prompt box; asks are stored per user and travel
like all state; the dev loop reads the inbox at session start (deploy
could print unaddressed asks like it prints nodeless-release warnings).
The agent behind the button starts out being the dev-session agent on a
delay; the ladder automates from there. The wish arrives with provenance
born: field ask → transcript-grade quote → node.

**THE FEATURE LIST IS THE REQUEST LIFECYCLE (fm-spec-3 #p85, bedtime
addendum).** User-stated, near-verbatim: the feature-update list is also
the way to manage **requests-for-changes to the coding agent**. You ask
for something; the agent PROPOSES — and the proposal is *the `## user`
section of the prospective feature node* — you OK it, fire it off, and it
appears in the feature list as **in progress** (maybe with an ETA; "!" /
"?" marking a problem or a question needing you). "It's a very powerful
view, this chronological feature view."

What this completes: the one list then spans the whole arc of a feature's
existence — **asked → proposed → OK'd → in progress → awaiting update →
shipped → ticked** — one chronological surface, every line at some stage
of becoming. And the proposal-as-user-paragraph is node-before-code
extended one rung earlier: *user-approves-the-node-before-the-code* — the
`## user` section, already written for exactly this reader (#p73),
becomes the contract the build must honour; when the feature ships, the
paragraph you approved IS its introduction in the list. The "?" state is
the agent's questions riding the same surface (no separate chat needed for
clarifications), and the ETA/"!" states are the freshness/honesty doctrine
applied to work-in-flight: never lie about progress either. Pairs with the
ask inbox above as its display half; the awaiting-update section (#p83)
already demonstrates the pattern for the shipped end.

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

**DAY 4: THE UPDATE LADDER, TOP TO BOTTOM (fm-spec #p2, 2026-08-15).** Ash's
morning order — consent-once / review workflow / minimal disruption / agent
hookup — became four nodes in one arc (builds 122–127), and settled these:

- **Auto-vs-one-OK reconciled** (handover item 6, resolved): `/consent-once`
  makes the user's acceptance the *only* key any apply path checks; `/auto`'s
  self-apply stands down by redefinition, not deletion — untick consent-once
  and the old behaviour returns. The acceptance mirrors to localStorage so a
  sleeping device applies silently at next launch.
- **Policy re-purposed**: with consent now consent-once's business, the
  three-way picker governs only what the review pre-ticks (automatic =
  proposed additions arrive ticked). OPEN: `fixes auto` and `ask me`
  currently pre-tick identically — the picker may want to become two-way;
  ash to rule.
- **Draft-tick principle** (paid for in design, not blood): in the awaiting
  review, an addition's tick must be a *draft* — local until **upgrade**
  commits it — because the live `ftick_` event is a store-*toggle*; a purely
  visual pre-tick overlay would invert the user's intent on first tap.
- **changes.json is path-stamped** (#p54's cheap half, built): each entry
  now carries `paths` (nodes touched) and `added` (nodes introduced); the
  review workflow reads `added`, and the feature→tool mapping for context
  sensitivity remains the unbuilt half.
- **Seamless applies read their elders**: busyness (mic, speaker,
  transcriber) is typeof-guarded reads *by the newest node* — causality
  bounds extension, so the elders could never have extended a busy-chain
  the young node declared. The stash/rehydrate pattern (whole loop state,
  keyed by build, consumed once) survived a real update round-trip with
  tool, counts and tasks intact.
- **The ask inbox exists** (`noob-button/ask`, ladder grade one): find =
  toolbar labels (open-chips) + tree name/purpose/intro overlap (chooser
  rows, ticks stripped — results introduce, they don't configure); file =
  user-scoped `asks` var → `/tmp/muon-vars/user.*.asks.json` on the mini →
  deploy prints unaddressed asks beside its nodeless-release warning. Next
  rungs stay as named: proposal-lifecycle states on the feature list; the
  agent behind the box starts as the dev-session agent on a delay.

**MINIMAL UPDATES (fm-spec day 4, #p5 discussion; ash ruled "all three" at
#p6 — A and B BUILT, C's first rung BUILT, C's registry parked).**
What shipped: **B** = fmlink `SPLIT_PAGES` (index.html's js/css fragments
emitted as per-feature files under site/f/, referenced in document order —
identical semantics, index.html 85KB → 5KB; f/ is swept each link, the
first piece of hygiene #9; needed serve.rs to learn `text/css` — the site
had never served a bare stylesheet). **A** = `review/delta` (deploy
publishes hashes.json; evict = manifest diff; no-code delta ⇒ quiet apply,
no reload; the evict seam refactored out of update/panel/review call
sites). **C rung 1** = `review/patch` (wasm-only delta ⇒ live module swap,
state untouched, one nudge render; safe mid-task). Hard-won test lesson:
CDP `Runtime.evaluate` calls share the global lexical scope — a bare
top-level `const` in one eval poisons every later eval with a silent
redeclare throw; scope every scripted mutation in an IIFE. Original
analysis follows.
Ash asked: when a release is small, can we patch just what changed instead
of upgrading the whole app? Measurement first: the whole composed client is
~220KB (index.html 85KB + wasm 130KB + sw 2KB) — re-fetching *code* is
nearly free. The real costs of the ritual are (1) `caches.delete('muon')`
evicts the ONE cache, which also holds the ~133MB STT model (every update
⇒ a silent model re-fetch on next transcription — notes hygiene #10's evil
twin), and (2) every release reloads every client, even server-only
releases that change nothing a client runs. So "patch functions" aims at
the wrong 220KB; the minimal-update wins are eviction precision and
reload avoidance. The ladder:

- **A. Artifact-aware updates (recommended next)**: deploy exports a
  `manifest.json` (site path → content hash). The apply ritual diffs old
  vs new manifest and evicts only changed paths — the model survives every
  update, and a release whose manifest is identical for the client (a
  server-only fix) stamps the version *without touching the page at all*.
  Rides the path-stamping we just built; scaffolding + one node.
- **B. Split composition** (per-feature fragment files instead of one
  composed index.html) so a feature change invalidates only its own file:
  real linker work, marginal benefit at 85KB — parked until the app is big
  enough to feel it.
- **C. Hot patching / soft reload**: swapping the wasm in place with state
  carried over is nearly buildable (seamless's stash already proved the
  state carry); but true function-level patching of the JS needs
  **re-linkable chains** — chains as a registry with indirection rather
  than closures capturing `existing` — and that is the SAME missing
  mechanism runtime feature-ticks need (the context manager: tick/untick
  without reload). Minimal updates and the context manager converge on one
  linker mechanism. Parked as the research fork; when the context manager
  gets built, updates get hot-patching almost free.

**ON-DEVICE TOOL-FINDING (fm-spec day 4, #p9 research — surveyed, not
ruled on).** The ask box's find step is word-overlap; ash wants instant
semantic tool-finding on the phone, no network. Findings, sized against
muon's stack (transformers.js + ort already aboard for whisper):

1. **model2vec / potion-base-8M** (MinishLab): static embeddings — a
   token→vector table + mean pooling, no transformer at runtime. ~8MB,
   ~90% of MiniLM quality, tens of thousands of sentences/sec on one CPU
   core. No official JS port, but the runtime is ~50 lines (tokenize,
   gather, average, cosine) — verbatim-library territory, no ort needed,
   instant even without webgpu. The deploy-side twin: embed the catalog
   (tree.json name/purpose/intro) at deploy with the Python lib, ship
   vectors beside tree.json; the device embeds only the query. Parity
   between Python-embedded catalog and JS-embedded query needs a
   test-vector check (weighting/PCA are baked into the stored table).
2. **MiniLM-L6-v2 / bge-small q8 via transformers.js** (~25MB,
   ~30–100ms/query in wasm): zero new machinery — same pipeline API as
   whisper. The fallback if static-embedding quality disappoints.
3. **FunctionGemma** (Google, Gemma 3 270M fine-tuned for function
   calling, Dec 2025): browser-proven via transformers.js (official
   Physics Playground / Mobile Actions demos run fully offline);
   whisper-class download (~hundreds of MB q4). Not for finding — for
   CALLING: "do x with y" → {tool, args}. The later rung, when asks
   become parameterised commands.

The ladder maps onto the ask ladder exactly: semantic find (1, instant,
on-device) → tool call with args (3) → build new (the dev-agent inbox,
live today — and once find is instant, the inbox's ~60s latency only
ever applies to build-me asks, where minutes are inherent).

**WebGPU addendum (#p10):** the speedup is real — 10–15x over wasm in
browser benchmarks; Gemma-class models hit 20–60 tok/s typical, 255
tok/s peak on an M4 (Xenova's demo, custom WebGPU kernels). FunctionGemma
runs on webgpu via transformers.js today on desktop. On the iPhone the
gate is not the GPU (iOS 26 ships WebGPU; haze proves it from wgpu on
the same phone) but ort-web's jsep: beyond the known over-requested
limits (haze's `required_limits: adapter.limits()` recipe, phone.md's
named refinement), **onnxruntime issue #26827** reports WebKit 26 jsep
builds pinning 400%+ CPU and 1–14GB memory AFTER inference, wasm mode
included, ending in crashes — track it before betting the call rung on
ort-webgpu-on-iOS. Escape routes if it stays sick: newer ort pins as
they land; or the Rust-native path (burn's wgpu backend runs in-browser
— muon owning its inference runtime instead of riding ort would be very
fm, and haze already proves the wgpu half on target hardware). None of
this touches the find rung: potion static embeddings are sub-ms on CPU,
no gpu involved.

**MUON COMPUTES FOR ITSELF (#p12 — doctrine, ash's words).** "Being able
to implement anything we want (even potion-style search) into webgpu,
without depending on anything else." One compute substrate, owned
end-to-end: WGSL kernels as node-owned assets, dispatched by a thin
page-JS driver — and the sharpening that makes it clean: WebGPU is a JS
API, so the substrate needs NO wasm-bindgen, no burn, no ort, no
framework; the zero-import law is untouched because the engine never
enters client.wasm. The clamp-to-adapter-limits recipe (haze's) is baked
in from birth, not patched in later. Tenancy ladder: proof kernel →
potion find (ceremonially GPU, honestly CPU-trivial — the tap-counter of
kernels) → mel/matmul tiles → whisper → FunctionGemma-class. Every
kernel toggleable, provenance-anchored; absent-webgpu degrades to CPU
per tenant. First brick: `loop/compute`, built 2026-08-15.

**Roll-our-own, viewed rightly (#p11 — ash warm, not yet ruled → ruled
at #p12: the sovereign path, hand-rolled, is the direction).** The
grounded map, three tiers: **T1** — the potion static embedder in pure
Rust inside client.wasm: no framework, no GPU, ~50 lines of math over a
shipped token→vector table; deploy (Python model2vec) embeds the
catalog, the wasm embeds queries; zero-import discipline untouched.
This IS the instant find rung — rolling our own starts as a one-node
build. **T2** — a wgpu inference runtime as a muon subsystem: burn is
the vehicle (wgpu backend compiles WGSL, browser demos exist at
MNIST/image scale; candle's webgpu still unready; whisper-burn is a
stale native port — browser whisper on burn is pioneering, not
assembly). Whisper is the first tenant (mel frontend, tokenizer,
KV-cache decode — weeks of rungs, each a node). Architectural
collision to design first: client.wasm's ZERO-IMPORT law vs wgpu's
need for browser bindings — resolve as stt does today: the engine is a
SECOND wasm beside the app, and doctrinally almost a *place* (GPU as
resource in its inventory; the places vocabulary gets its first
non-network tenant). **T3** — FunctionGemma-class on the same runtime;
precedent says even hand-written WebGPU kernels are reachable (Gemma 4
at 255 tok/s in-browser was Fable-5-written kernels). whisper.cpp's
wasm port (CPU, ≤small models) noted as a pragmatic reference point,
not the path.

- `tools/export_transcript.py` — exports a Claude Code session log to `transcripts/<date>-<slug>.md` (verbatim prompts with stable `#pN` anchors + timestamps).
- `tools/explorer.py` — three-pane *feature browser* at `http://localhost:8123`: feature tree | spec + code | transcript. The tree shows feature nodes only (ordered per `order.md`, which itself stays hidden; unticked features dimmed/struck). Clicking a feature renders its spec with the `.rs` implementation(s) beneath, and opens the transcript pane at the feature's provenance prompt — tree links carry the `#pN` fragment so the browser scrolls natively. Nodes with children expand/collapse via native `<details>` toggles (no JS); by default only the path to the selected feature is expanded. Fully server-rendered plain HTML, no client JS; agents can `curl /feature/<path>`, `/view/<repo-path>`, or `/raw/<repo-path>`. Python stdlib only, incl. a built-in markdown renderer covering the fm subset; bare provenance refs (`(transcripts/…#pN)`) are auto-linkified.

**FIRST TOOL BUILD, WAR-GAMED (#p27 — "reset taps"; design, not yet
ruled).** The scenario: in the taps tool, ask "reset taps"; the system
should know the context, draft a user-description, let ash edit/OK it,
and fire — online or offline. Walked end to end, it decomposes:

- **R1 context-carrying asks** (small): ask.rs stamps `open_tool` and
  its resolved feature path into the filed ask — the wish arrives
  knowing where it was born; the builder knows the parent node before
  reading a word.
- **R2 context bias in find** (small): a lineage bonus in semantic
  scoring for the open tool — "reset" while inside taps outranks
  global hits (#p78's context sensitivity, first cash-out).
- **R3 the propose flow** (medium, THE key node): when the find can't
  answer ("taps can't do that yet"), the ask box turns proposer: a
  drafted `## user` paragraph in an editable box, OK files
  `{status: "proposed", proposal, context}` through the outbox
  (offline = queued fire; the flow is identical). Doctrine #p85 lands
  here: the proposal IS the prospective node's user paragraph,
  approved before code.
- **R3's drafter is swappable, and starts DUMB**: a template
  ("<tool> gains <capability>: <ask text made declarative>") gets 80%
  of the way with zero model — the edit box is the intelligence in
  the loop. Upgrades in place: dev-session agent when online (better
  prose, same box); local net on the compute substrate when the
  kernels grow up. FunctionGemma is tuned for calls, not prose — the
  offline drafter eventually wants a small instruct model, which is
  the T2/T3 climb, not a prerequisite. Do NOT gate the flow on the
  net.
- **R4 the build**: proposal arrives at the dev loop (the monitor
  already watches asks) → node built FROM the approved paragraph →
  ships → awaiting update → tick. Provenance: field ask → quote →
  node, exactly as the doctrine dreamed.
- **R5 lifecycle states on the feature list** (asked → proposed → in
  progress → shipped) — the display half, already specified at #p85.
- **The reset node itself**, when the game runs for real: a wrinkle
  worth savouring — tap_count is sync-escalated (shared); "reset"
  against a shared counter is a distributed op (the op-fold/CRDT note
  from #p128 becomes concrete). The first user-built tool will touch
  the deepest open question. Fitting.

**THE LOOP CLOSED (2026-08-15, end of day 4 — ash: "I think we built
something super cool here. There's something magic about being able to
modify tools from within the app itself.")** Five field asks travelled
the full lifecycle today, phone to phone: reset (built), double
(built), decrement (built, then removed by ask — removal = product
override, the node keeps its story), and the updates picker relocated.
The magic decomposes into things fm already believed: the ask box as
the programming surface (#p53's IDE glimmering, real); the proposal as
the contract (#p85 — the approved paragraph IS the shipped intro,
verbatim); provenance born in the field (#p40: the first node whose
founding quote is the wish as it reached the builder); one-second
build-side latency (the broadcast file doubling as the builder→user
channel); and the update doctrine carrying the answer back for one OK.
What made it feel like magic is that no part of it is: every link was
a node, every node toggleable, every wish traceable. The agent behind
the button is still a person-shaped session on a delay — the ladder's
next rungs (drafter upgrades, the local net, runtime ticks) now have a
working loop to land in.

**MUON → MISO (#p50, 2026-08-15, post-pizza).** Renamed everywhere:
"make it so" — ash's name-in-waiting for a self-modifying toolkit, which
is what this became the day the loop closed. Mechanics: quote lines kept
verbatim (history said "muon"); localStorage keys migrate via a one-time
shim in the shell skeleton; the cookie rename logs devices out once; IDB
recordings on the legacy origin are orphaned (blobs safe on the mini);
miso.nøøb.org canonical, muon.nøøb.org a legacy alias until retired; old
state dirs kept as backups. Lessons paid for: .gitignore paths don't
rename themselves (130MB of model briefly entered a commit — caught by
GitHub, history rewound before push); a product-dir symlink means "the
product's order.md" can silently be the SHARED order.md (the override
structure must be real dirs, learned at #p44, relearned gently here).

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

- [x] **9. stale asset *trees* in site/** — DONE 2026-08-16 (it mattered:
  `/tamed-request`'s toggle proof failed against a lingering `tame.js`, the
  exact predicted failure). fmlink now writes `build/asset-manifest.json`
  (the rel-paths it copied) and sweeps previous-but-not-current entries each
  build — only files the linker itself placed are ever deleted. Bootstrap
  caveat: files stale from before the manifest existed need one
  tick→build→untick→build cycle (or a hand-delete) to clear.
- [ ] **10. stt model download UX** — the ~130MB engine+model fetch happens
  silently on first transcription; over the network-first service worker it
  can also re-fetch per session when online. Wants: a cache-first sw rule
  for `/stt/` (needs a seam in `/pwa`'s fetch handler) and a visible
  "downloading speech model" state on the tile. Both are `/phone`
  subfeature material once real-device behaviour is observed.

## the ask–engineering gap, and privilege (2026-08-15 evening, fm-spec-2 #p12)

Ash, after five field asks shipped in one evening: *"there's a gap between
a user's request and the engineer's version of that; and different users
should have different privileges. For instance, I could change the URL or
name of the project, but one of my users shouldn't be able to do that. So
there's 'information about users' we need to keep that gates their
privilege."*

Two observations, one seam.

**The gap is already structural.** Every ask carries `text` (the user's
verbatim words) and `proposal` (what will be enacted); today the proposal
is the text echoed back, and the drafter-upgrade rung would make it the
engineer's translation. Tonight showed the translation spectrum in
miniature: "subtract 1 from the tap count" became a product-override
re-tick; "the tooltip is wrong" became a documentation repair; "+25%"
became literal CSS with the per-user-size generalisation parked in
ideas.md. The translation step is where engineering judgment lives — and
right now that judgment is supplied by the humans in the loop, nowhere
codified.

**Privilege gates the translation, per user.** The same words from
different users should enact differently or not at all: "rename the
project" from ash is the muon→miso rename; from a field user it must be
refused (or become a request-to-the-owner). Pieces already in place:

- `~/.miso-auth/users.json` is the out-of-tree user store — and the `_`
  prefix on test users is already a primitive privilege bit (PINs to the
  log, not SMS). Authority fields belong beside it, deploy-proof.
- The feature tree offers a natural authority surface: a proposal's blast
  radius is the set of nodes it touches (deploy already prints exactly
  this per release), and a user's privilege can be expressed as the
  subtrees their asks may reach — field users reach the tools they use,
  admins reach shell/serve, the owner reaches identity itself (name, URL,
  product). Enactment requires authority ⊇ blast radius.
- Per-user ticks (the rung 3 done-ness bar) is the same shape of fact —
  per-user information gating live behaviour — but preference, not
  authority. Both are "information about users"; they should not be
  conflated, only co-located.

**Why now:** the human-supervised builder IS the privilege system today.
The always-on flywheel removes that human, so the privilege model joins
provenance in the doctrine-before-code bucket: a headless builder must be
able to answer "may this user's ask touch these nodes?" before it stamps
`building`. And the target app (trust-ring campaign tool) is itself built
on graded trust — this is foundation, not plumbing.

Not built tonight; recorded as the ruling-shaped conversation it is.

## tunables: the general form of the naive ask (2026-08-15 evening, fm-spec-2 #p17a)

Ash, watching the evening's asks accumulate: *"each of these things are
actually interpretable as 'make parameter X tunable using feature-scoped
variable <blah> per-user / per-group / global'."*

The evidence, in one night: +25% buttons = a `size` parameter (shipped as
constant CSS); the button label "miso" and the placeholder
"request"→"do something" = text parameters (shipped as constant fragments,
the second churned once already); decrement's removal-and-return = a
selection parameter (which per-user ticks ALREADY models as a user-scoped
var). Every one shipped as a build because the parameter had no variable
to live in.

The engineered form: a feature declares its **tunables** — named,
feature-scoped variables with a type, a default, and a scope
(per-user / per-group / global). The machinery mostly exists: `Var<T>`
in scope's lib, user-scoped and global vars in the var store
(`user.X.feature_ticks`, `global.tap_count`), the broadcast file pushing
var updates to open panels in ~0.5s.

Three consequences worth their weight:

1. **A whole class of asks stops needing builds.** "Bigger buttons" as a
   var-write enacts in seconds, offline-queued, no deploy, no builder —
   the drafter's translation step classifies an ask as parameter-set vs
   build, and parameter-sets short-circuit the pipeline entirely.
2. **Scope is where privilege bites** (joins #p12): setting your OWN
   per-user value is self-service; setting a group's or the global
   default requires authority. The tunable declaration names the scopes
   it offers; the user record gates which scopes an asker may write.
3. **Per-user ticks (rung 3) is the first tunable**, not a special case:
   "feature on/off" is just a boolean tunable every feature carries
   implicitly. The context-manager enforcement work and the tunables
   design should be one mechanism, not two.

Open questions for a ruling: where a tunable is declared (the node's spec?
a `tunables:` stanza the linker reads?); whether a var-backed parameter
still wants a node per naming-change (provenance says the DECLARATION is
a node; values are data); and what the ask pipeline shows for a
parameter-set (instant "done" instead of building→shipped?).

**The promotion rule** (#p18, ash: "when we ask to change eg toolbutton
size *again*, you just find that feature and mod its state var"): a
parameter earns its variable on the SECOND ask that touches it. First
ask ships the literal constant — cheap, honest, no speculation. Second
ask promotes: one real node declaring the tunable (name, type, default,
scopes — the declaration is provenance-worthy), and from then on every
value is data — find the owning feature, write the var, seconds not
builds. The finding machinery already exists (semantic-find maps words
to features; birthplace says where the asker stood); what's missing is
only the declaration mechanism, and the scope-vs-authority ruling for
who a repeat ask writes for (asker's per-user value or the global
default). bigger-buttons is the standing first case: the next
size-shaped ask triggers its promotion.

## the rule of two, surface side: new tools wait to be asked twice (2026-08-15 evening, fm-spec-2 #p20)

Ash: *"if the user asks to do something, we don't give them the tool
straight away. Instead, we hold it in a 'new tools' section; and if the
user asks again, we introduce them to the tool. This prevents us
cluttering up toolspace with things that aren't really necessary."*

The promotion rule's twin. Building side (#p18): a parameter earns its
variable on the second ask. Surface side (this): a tool earns its
TOOLBAR SLOT on the second show of intent. Same instinct both times —
the first signal buys existence, repetition buys prominence.

The tension to respect: the asker DID ask, so the capability must not
feel withheld. Resolution from pieces already shipped: when an ask
ships, the open-chip in the ask results already hands the asker the
tool right there, and the lifecycle row records it. So "introduction"
means usable immediately from the ask surface; the toolbar gains it on
the second ask — or the first real use, whichever signal we trust. The
"new tools" section is the holding pen in between: likely a drawer in
the panel beside the features list, badged the way `upgrade` already
badges additions `new`.

Naturally per-user: the tool ships globally; each user's toolbar grows
only on their own repeated intent. That is per-user ticks again —
toolbar membership as a user-scoped selection var — so this, tunables,
and the rung-3 ticks enforcement keep converging on one mechanism: the
context manager. Not built tonight; joins the tunables entry as
doctrine awaiting its first case.

## the builder is a feature-modular skillset (2026-08-15 evening, fm-spec-2 #p21)

Ash, capping the evening's doctrine run: *"these sorts of ideas actually
are agent instructions — so we need the equivalent of feature-modular
agent instructions, just as we do for other languages. the part of our
system that takes user requests and drafts and implements features, is
itself a feature-modular 'skillset'."*

The recursive move. Agent instructions become a composition language
beside .rs/.js/.css: a node may carry an instruction fragment
(`<name>.agent.md`, say), and the linker composes the working skillset
the way it composes a page — a skeleton (the five-step loop is the
obvious one) with fragments landing at slots, provenance-ordered,
toggleable with their nodes. Untick a feature and the builder forgets
how to serve it; regroup and nothing rewires (grouping is not
behaviour, for instructions too).

What this reframes: tonight's notes entries are fragments-in-waiting,
each belonging to the node it governs — the promotion rule to the
tunables declaration (when built), the rule-of-two-surface to the
new-tools drawer, privilege-as-subtree-reach to `/gate`. notes.md has
been the accumulator because the instructions had no nodes to live in.
And agents.md is the monolith the skeleton comes from — the same
starting point index.html was before SPLIT_PAGES. fm.md stays outside:
constitution, not composition.

Why it matters beyond tidiness: the flywheel. A mini-resident headless
builder needs instructions from SOMEWHERE; composed-from-the-tree means
the builder's discipline ships, versions, and rolls back exactly like
the product — and a fork subagent building ask X could even receive the
skillset composed for the subtree it's touching. The builder stops
being outside the system: its behaviour gains provenance quotes,
toggles, and the audit trail everything else already has.

Open for ruling: the fragment extension and slot vocabulary; whether
the composed skillset is per-product (products/miso/build/skillset.md)
and whether the session loads it in place of agents.md; what the first
decomposed instruction should be (the promotion rule is the natural
candidate — it arrives WITH its node when tunables get built).

## field observation for tomorrow: transcripts don't mirror (2026-08-15 late, fm-spec-2 #p22)

Ash's first real test recording: transcription happened ON THE IPHONE
(the on-device whisper path worked in the field — worth celebrating),
but the transcript did not propagate to the laptop. This is the known
day-3 gap ("no transcript mirroring") now confirmed by field use:
`/mirror` moves the AUDIO between instances, and each instance
re-transcribes locally — but the laptop reseeds RecList from IndexedDB
and restarts re-transcribe (the other day-3 pending item), and a
transcript made on one device never rides the mirror. Tomorrow's shape,
probably: transcripts join the mirrored record (a `/mirror` or
`/transcript` subfeature), so the phone's words appear under the
laptop's tile the way the audio already does — with the better-server-
transcript-replaces-rough rule already specced in `/phone` deciding
collisions.

## the sovereign turn: ort is being removed (2026-08-16, fm-spec #p15–16)

The T2 rung mapped on day 2 has been taken deliberately. Ash, after the
shim experiment's third failure in a row: *"let's remove the ort
dependency and stand up our own sovereign webgpu runner, then get
whisper working on it. It'll be a heavy lift but SO much more
satisfying."*

The ledger that decided it. Two days spent on ort produced: a device
request iOS refuses (`/tamed-request`), a `webgpuInit is not a function`
that was really a MIME type (`/module-mime`), a memoized-failure trap, a
q8 kernel bug, a tokenizer that vanishes when a path has a scheme, and a
26MB binary nobody here can read. None of it reusable. Meanwhile
`/compute` and `/semantic-find` — both ours — went in clean and ran on
the first phone we tried.

The plan lives in **`sovereign.md`** at the repo root: four layers (GPU-
resident tensors → the WGSL op library → whisper's graph → the
`/dictate` rung), an eleven-rung ladder with a numeric acceptance test
per rung, and the verification discipline that makes it survivable — a
numpy twin (`tools/whisper_ref.py`) dumping golden tensors at every
boundary, because a transformer that is subtly wrong produces fluent
nonsense and timing tells you nothing.

Two structural commitments worth repeating here: **ort stays a reachable
rung until the last one**, so the app never stops transcribing mid-climb;
and **feature-modular WGSL finally has its first real user**, which is
exactly the condition `compute.md` set for building it.

Five rulings are queued for ash in `sovereign.md` §10 — the load-bearing
one being node placement (reusable math under `/compute`, the model
under `/dictate`), which reads against compute.md's letter that tenants
carry their own kernels.

## per-feature logging, switchable at runtime (2026-08-16, fm-spec #p23–24a)

Ash, after a day in which nearly every question became an archaeology
expedition: *"I feel like we need more pervasive logging as part of the
black box. A lot of these questions devolve to 'what actually ran and
what did it return'"* — and the shape: *"logging statements are
pervasive, but enabled at runtime on a per feature basis. So if we're
working on transcription, we enable logging for transcription and have
at it, then silence it once we're done (except for basic stuff)"*. Plus
(#p24a): **device IDs**.

**Why now: the promotion rule fires (#p18).** `/engine-receipts` was
built this afternoon as one feature's private telemetry, and its own spec
names the general form as "deliberately not built". This is the second
ask for the same mechanism, so the parameter earns its variable: build
the general form, and let receipts become a thin user of it.

The day's evidence for the need, all of it real: the phone transcribed
and we could not tell whether the GPU had run it; a failure arrived that
we could not attribute to a device; a picker vanished and only a
DOM-by-DOM reproduction found it; an OOM appeared whose cause was
arithmetic nobody had done. Every one of those is "what actually ran and
what did it return".

### The shape

**1. The call site is free of bookkeeping.** `log(...)` in Rust and in
JS fragments, with **the linker injecting the node path**, exactly as it
already injects node paths for `/context-manager`'s tick gates. Nobody
hand-writes a path, so nothing drifts when a node is regrouped. For JS,
composition injects a `FM_PATH` constant per fragment; for Rust, the
existing rewriting pass covers it.

**2. The switch is the context manager wearing a different hat.** A
user-scoped `feature_log` var mapping node path → level, with the same
prefix semantics ticks already use: enable `miso/loop/dictate` and
everything beneath it speaks. **Absent means off** — the mirror image of
`feature_ticks`, where absent means on. Levels: `always` (the few events
a feature emits regardless — the "basic stuff" ash wants left on) and
`verbose` (everything, on demand). Two levels until a third is missed.

This is #p17a's prediction landing exactly: ticks, tunables and logging
are one mechanism with three defaults.

**3. The transport already exists.** `/blackbox` batches, bounds by age
and count, survives offline, ships on visibility/reconnect/page-hide with
a keepalive request, and the server half ingests into a size-rotated log
on the mini. Log lines become another entry kind beside the event deltas
— which also means **they replay with the events**, which is precisely
the "what ran and what did it return" story, reconstructable after the
fact. Console in dev; nothing new for delivery.

**4. The control has two ends.** On the device, the chooser already
lists every feature with a tick — logging is a second control on the same
line (the `/sub-tool-cards` long-press idiom). From the terminal, the
urgent one: a `tools/` script writes the user-scoped var and the existing
broadcast reaches the device in ~0.5s. **Turn on transcription logging on
ash's phone from my terminal, watch it, turn it off** — that is the
capability this whole day lacked.

**5. Instance identity (#p24a).** A short, stable, per-install id in
localStorage, carried by every diag report, blackbox batch and log line.
Today's stopgap was to add `ua` to receipts (build 193), which cannot
distinguish two iPhones and is verbose in every line. The tree already
says *instances* (`/mirror`, `/scope`), so the vocabulary exists: call it
an **instance id**. Losing it when storage is cleared is honest — it is a
new instance.

**Cost when off** is a prefix check against a small cached list, using
the same thread-local cache `/context-manager` already established. The
zero-import law is untouched: Rust `log()` accumulates into a
thread-local that the wasm entry drains into state, the way `_send`
already carries outbound messages — no signature changes anywhere in the
tree.

### Rulings wanted

1. **Privacy, the load-bearing one.** Log lines can carry user content —
   transcripts, ask text, names. Receipts set the precedent (`chars`,
   never the words). Is that the rule everywhere (log shape, never
   content), or may verbose logging carry content when a user explicitly
   enables it on their own device?
2. **Does verbose ship to the server by default**, or is remote delivery
   a separate opt-in from local capture? (Battery, bandwidth, and 1
   above.)
3. **Two levels or a number?**
4. **Does `/engine-receipts` get subsumed** into the general mechanism,
   or stay as a specialised node that uses it?
5. **Instance id visible to the user?** (It would make "which device is
   misbehaving" answerable in the panel, not just in my terminal.)

## the flywheel's two rulings, settled in the field (2026-08-16, asks#1786892582635)

An ask arrived mid-session — *"set the background grid size to match the
tool icon size, and center it around screen center"* — and settled two
things that had been queued since the 15th.

**Ruling 1: never come back to ask.** Ash, on being asked how to cite it:
*"when an ask comes in, the user expects a feature in the next update. So
you must never ask me about it here - just build using your best
judgement, document properly in the feature node, and ship."* Now in
agents.md. The asker is a user waiting for their feature, not a
collaborator in a design conversation; a question aimed back at the chat
is a broken promise. Judgement belongs in the node's spec, where the
asker can read it.

**Ruling 2: the ask store is a provenance source.** The blocker named in
the 15th's handover, hit for real: fmlink refuses a code-bearing node that
cites no anchor, and an ask reaching the builder through the ask store has
no `transcripts/…#pN`. Settled by taking the judgement rather than
deferring it — **a field ask is provenance in its own right**, and a
better record than a chat message quoting one: it is the human's actual
request, timestamped to the millisecond, carrying its own recorded OK.
Specs cite `asks#<t>`; the linker reads the node's position straight from
the id, since the id *is* the timestamp. Known gap:
`tools/audit_prompts.py` still inverts only transcript citations, so
ask-cited nodes read as uncited in that audit until it learns the form.

**And the promotion rule's first case landed with it** (#p18). The ask was
size-shaped, which `ideas.md` had named as `/bigger-buttons`' trigger —
and it turned out to be a stronger trigger than a plain resize, because
two features now had to agree on one number, which is exactly the
duplication the rule exists to prevent. `/tools` now *names* its two sizes
and derives its own rules from them; `/bigger-buttons` *sets* those names
instead of restating rules; `/aligned-grid` reads them. Proven by toggle:
untick `/bigger-buttons` and the background grid drops to 40px along with
the buttons. What was promoted is the **name** — one declaration, several
consumers. Binding that name to a per-user var, so size is tunable without
a build, is the next rung and waits for an ask that wants it.

## the tickbox that commissions: suggested subfeatures (2026-08-16, fm-spec #p35a)

Ash, watching a tool ship with its limits named in the spec where nobody
would ever read them: *"ship minimal function, but anticipate subfeatures,
and consider how you'd build them. It would be great to suggest a laundry
list of subfeatures to the user, let them tick the ones they want once the
first one has shipped. Can be part of an 'intro new feature / tool'
workflow."* Noted for later, with the design as far as it goes.

**The move underneath it is new: a tickbox that commissions a build.**
Every tick in miso today enables or disables something that already
exists — `feature_ticks` steers composition, `/review`'s awaiting section
steers an update. A suggested subfeature is a line for something that does
*not* exist, and ticking it doesn't switch anything on: **it files an
ask**. The builder proposes, the user chooses, and the flywheel starts
without anybody typing a wish. That closes a real gap — asks presently
require the user to think of the thing and find the words, which is the
slowest part of the loop and the part that most needs the app's help.

**Most of the surface exists.** `/chooser` already renders tickable
feature lines with intros and build numbers; `/review` already prepends a
section of not-yet-yours features to that same list. "Suggested" is a
third section in the same idiom, with the tick meaning *commission* rather
than *enable*, and a build number that hasn't happened yet. The distinction
must be visible — a promise and a fact should never look alike.

**Where the suggestions come from.** The honest source is the spec: this
session's nodes already end with named limits — `/map` names panning,
antimeridian wrapping and offline vendoring; `/country-icon` names the
border imprecision; `/logging` names the levels it deferred. Today those
are prose an asker never sees. A `## suggested` section (or a stanza the
export reads) would carry them into the tree export, and the chooser would
render them. Then "anticipate subfeatures, consider how you'd build them"
becomes a documented obligation of writing a node, not a private habit —
and the cost of thinking ahead is paid once, by the person best placed to
pay it.

**Open questions for the build:** does ticking file the ask immediately or
gather a batch; does a suggestion expire if never ticked; may the builder
suggest against *another* node's subtree (the `/dictate` engine suggesting
a `/compute` rung); and does a shipped suggestion keep its line, becoming
an ordinary feature row, or vanish and reappear as one.

## quality: the map was withdrawn, and what it taught (2026-08-16, fm-spec #p36–39)

The map tool shipped in four builds across an hour, satisfied every ask
literally, passed every toggle test — and was withdrawn, because it looked
bad. Ash: *"This was a poor quality ship. Image is blown out, doesn't feel
'good'. How can we do better? I'd like to grope towards some basic
principles that deliver quality, rather than blame."* Then: *"delete the
map tool. We need to do some foundational work before we're ready for a
request like that."* Deleted at build 208; the work survives in git if it
is ever wanted.

**The root failure was structural, not careless: nothing ever looked at
it.** Every check answered *did the mechanism work* — 25 tiles loaded, the
filter applied, the chain resolved to the override. None answered *is this
good*. There was no way to take a screenshot, so the test that could be run
silently replaced the test that mattered. Tooling absence becomes a quality
ceiling, invisibly. (`shot.py` now exists in the session scratchpad — CDP
`Page.captureScreenshot`, whole viewport or one element. It belongs in
`tools/` properly.)

### The principles, as far as they go

1. **See what you ship.** Anything with a visual result gets rendered and
   looked at before it goes. "It loaded" is not "it looks right".
2. **Self-criticism as a step, not a mood** (ash, #p37a): before shipping,
   look at the thing and ask *is this good enough?* The bar moves over
   time; the discipline is asking at all. It belongs in the five-step loop
   beside "prove the toggle" — proving it works and judging it good are
   different acts, and only one of them was being done.
3. **Choose a source that gives you what you want; don't hack a filter over
   one that doesn't** (ash, #p38). The map's tiles came with baked-in
   labels, so a style was swapped — correct. But the swapped style was too
   dark, so a `brightness(1.75)` was piled on top, which clipped the
   brightest elements (pavement dashes) into glare while leaving buildings
   near-black: the information hierarchy inverted, decoration louder than
   substance. **A filter working hard to correct an asset is a smell.** The
   real answer was a source that lets us say what to draw — vector tiles we
   style ourselves, which is exactly the foundational work not yet done.
4. **Read the user's aesthetic from their ask history** (ash, #p37b). It was
   all there and unread: *"make the dots brighter"*, *"halve the dot
   spacing"*, *"match the tool icon size"*, *"no extraneous labels"* — plus
   a shell that is black, monochrome, restrained, with `#d8d8d8` as its
   brightest note. The map arrived with a **blue** marker and a blue
   accuracy disc, the only colour in the entire app, imported unthinkingly
   from every other map anyone has seen. Nobody asked for miso to look like
   Google Maps. The ask history is a style guide that is already written.
5. **A premature yes is worse than a considered wait.** Four builds went out
   before the foundations existed. Shipping the ask is right (agents.md's
   law above the laws) — but *foundations first* and *ship the ask* only
   conflict when the foundation is genuinely missing, and then the honest
   answer is to say so, not to ship something that technically answers.

### What the foundations actually are, before a map is attempted again

- **Tiles we control.** Vector tiles plus our own style, so "buildings and
  streets, no commercial names" is a rule we write rather than a basemap we
  hunt for. This is the same sovereignty argument as `sovereign.md`, and it
  should be planned the same way rather than improvised.
- **A visual bar.** What does a miso surface look like? Monochrome, dark,
  quiet, `#d8d8d8` at the top of the range. Written down, it stops being
  something each node re-invents.
- **The screenshot in the loop**, so the bar is checkable.

### The remedies — what to build before that ask is attempted again

Ash (#p40): *"write up the lessons and some proposed remedies - those are
the features we need to build before we try that ask again."* Sized and
ordered; the first three are general quality foundations, the fourth is the
map's specific prerequisite.

**R1 — `/shell/look`: the palette and scale, named once.** *(small; do
first)*
The evidence is worse than the map incident suggested: **52 distinct
colours** across the tree's CSS, including six near-identical greys
(`#333`, `#3a3a3f`, `#3c3c3c`, `#26262c`, `#23282f`, `#1a1a1d`) and colour
that had already drifted in unnoticed — a blue (`#9db7d8`, `#48628a`) and a
green (`#9fdba4`) predate the map entirely. The map's blue marker was not
an aberration; it was the same disease with a witness.
Declare the app's ink, paper, dim, line and accent as tokens on `:root`,
exactly as `--tool-size` was named by the promotion rule, and let nodes
derive. Migration is eventual, not mandatory: declare now, convert each
node as it is next touched. What this buys is not tidiness — it is that
*"does this look like miso"* becomes a question with an answer, and
inventing a colour becomes a visible act rather than a silent one.

**R2 — `tools/look.py`: the visual smoke sheet.** *(small)*
`tools/shot.py` now takes one picture; the missing piece is taking *all* of
them without being asked. One command that drives the app through its
standard surfaces — home, each tool open, the panel open, the feature list
— and writes a contact sheet. Then "look at what you ship" costs one
command instead of a bespoke rig each time, which is the difference between
a discipline and an intention. Deploy could refuse a release touching CSS
whose sheet has not been regenerated; worth trying, easy to make naggy.

**R3 — the visual bar, written down.** *(small; a doc, not a node)*
What a miso surface is: black ground, monochrome, restrained, `#d8d8d8` the
brightest note, information louder than decoration. Derived by *reading the
ask history* — "brighter dots", "halve the spacing", "match the icon size",
"no extraneous labels" — which is a style guide already written by the
person who has to look at it. Belongs beside R1 so the tokens have stated
intent rather than only values.

**R4 — map data we author: the ask's actual prerequisite.** *(large; plan
it like `sovereign.md`, do not improvise it)*
The failure was reaching for a raster basemap and then arguing with it.
Raster tiles are somebody's finished opinion; what the ask needed was
**vector tiles plus our own style**, so "buildings and streets, no company
names" is a rule we write rather than a basemap we hunt for. Three rungs:
a vector source (self-hosted extracts, or a source whose licence permits
restyling), a renderer (canvas first, `/compute` WGSL later — the same
sovereignty argument as speech), and a style we author. **The tile proxy,
cache, Web Mercator projection and country outlines from the deleted tool
are all reusable from git** — that hour was not wasted, it was just
sequenced wrong.

**R5 — `withdrawn` in the ask lifecycle.** *(small)*
Four asks still read *shipped*, with build numbers, for a tool that no
longer exists. The lifecycle has no way to say a feature was delivered and
then taken back, so the panel is currently lying to the person who asked.
Directly caused by today, and the fix is a `/ask/lifecycle` subfeature plus
a `--status withdrawn` in `stamp_ask.py`.

**The gate.** The map ask becomes attemptable again when R1 exists (so it
can look like miso), R2 exists (so we can see whether it does), and R4 has
its first rung (so we can say what is drawn). R3 and R5 are cheap and
should not wait for any of it.

### One real bug the investigation surfaced, worth keeping

Tile URLs carried no style name, while the server cache did. So a style
change would have been invisible to anyone holding week-old cached tiles —
and it fooled this very investigation for two rounds, showing a "light"
basemap that was the old dark one inverted. **When a resource's content
depends on a parameter, that parameter belongs in its URL.** Applies to any
future tile, model, or asset route.

## the model comparison: where judgment lives (2026-08-21, hybrid #p2–p6)

The Aug 16 session was a natural experiment nobody designed: p1–p8 ran on
Fable 5 (high effort), the credit cutoff fired mid-p8, and p9–p43 continued
on Opus 5 — same session, same context, same rulebook, same user, same day.
Ash's felt experience ("with Opus I wasted a ton of time correcting poor
judgment calls; with Fable things just worked" — #p6) was checked against
the transcripts by two forensic readers, one on the Fable baseline (both
Aug 15 sessions plus the Aug 16 morning), one on the Opus stretch.

**The numbers.** Fable, ~80 prompts across the baseline: essentially one
substantive rework (reset as canvas button vs sub-tool), roughly 1.3
prompts per surviving feature. Opus, ~30 substantive prompts: eight or
nine corrective interventions, two doctrine rules written mid-session to
constrain the model (the law above the laws, never-ask), four builds
withdrawn — about 2.5 prompts per surviving feature, flattered by counting
the map arc's negative yield as zero rather than minus four.

**The split.** Opus followed the written discipline faithfully — anchors,
toggle proofs both ways, parent refactors to extension points, the
promotion rule correctly fired — and its root-cause debugging (the MIME
type, the DOM loan, the double-load OOM) was excellent, its candor
exemplary. Every major failure was a *judgment call in the space the rules
didn't yet cover*: what the ask really wanted (the non-map), what
"verified" means (the unseen `brightness(1.75)`), when to hold an ask for
a ruling (the provenance stall), which hostile case a new mechanism must
survive (three latent defects in one afternoon: tamed-request's failing
fallback, logging evicting the flight recorder, logging polluting replay).

**The asymmetry that decides the architecture.** Each Opus failure became
a rule — 4a, the law above the laws, never-ask all date from that
afternoon. Fable, running the same rulebook *before* those rules existed,
never needed them. That is the signature of a capability difference, not
a prompt gap: the prompt-fixable surface is now largely written into
agents.md, and the residue — seeing that a blown-out image is bad without
a rule saying to look, reading the asker's taste unprompted — is why
judgment seats get Fable. Hence `hybrid.md`: Fable triage (ask → brief),
Opus worker (the five-step loop in a worktree), Fable review (evidence
against brief), main session serialising integrate/deploy/stamp. The
hybrid is the flywheel with the seats named, and it is a bridge — the
goal remains a Fable-only workflow that fits the fixed plan.

**A note on fairness.** The comparison is one afternoon, mid-session
handoff, on work (the map) that was genuinely the week's hardest ask; and
Opus wrote two of the repo's best notes entries in the post-mortem. The
finding is not "Opus is bad" but "the two models fail in different places,
and fm2's discipline was written by watching Fable, so it patches Opus's
gaps *after* they cost something." The hybrid puts the patching before.

## contexts: the world-object (2026-08-21, hybrid #p21–p26)

The design conversation the contexts redo was waiting for (redo.md item 6
— the one that didn't happen before the first build). Ash's crux (#p21):
the runtime enabled-check should not be a scan of anything — turn the
context into an **actual object whose methods are the composed
functions**. Then ash pushed it further (#p23): the object holds not just
enable/disable and settings but the in-flight Elm state too. The Context
is **the world** — one value that is a user's entire situation.

**The slot model is flat** (#p24, ruled #p26). No config/state taxonomy
in the object: a Context is one namespace of feature-scoped slots, each
declaring its own attributes — type, default, **scope**
(global/group/user/device), **merge** (last-write, crdt-sum,
better-replaces-rough, none), **inherit** (overlay through
user → group → global, absent-means-inherit; never for owned state like
drafts). `enabled` is not special: it is the one slot every feature has.
"Config" and "state" survive only as preset attribute-bundles a
declaration can invoke as shorthand; the odd slot (the counter's
crdt-summed count) spells its columns out.

**Lifecycle is enforced through generic vars** (ash's ruling, #p26): the
attributes live in `Var<T>`-style wrapper types (scope's verbatim lib is
the seed), so a mis-declared slot — an inheritable draft, a group-scoped
open-tool — is a type error, not a doctrine violation found in review.
The linker emits the Context struct from per-node slot declarations; a
compose-time untick removes the slots from the struct, a runtime disable
flips the `enabled` slot — the same toggle at two speeds, kept.

What falls out by construction: the old raw-scan bug class (the `':'`
discriminator lesson) becomes inexpressible — gates read typed fields.
Per-user isolation is the server holding one Context per user. Instant
toggle without losing work: disable gates a feature's slots, re-enable
finds them intact; toggle is an in-place edit, **switch** is swapping
which object is live, between loop turns (the Elm boundary satisfied by
an atomic pointer swap). Update stays conceptually pure —
`(context, event) → context` — so replay and the blackbox stay truthful,
and snapshot/divergence-proof/hypothetical-worlds are all "construct
another object". The trusted-base lesson is restated structurally: the
machinery that builds and syncs Contexts lives *beneath* the Context and
is not a slot on it. The promotion rule lands cleanly: a constant earns
its variable by gaining a declaration line — name, type, default,
scope, merge — and values are data forever after.

**Open for the build phase**: the declaration stanza's grammar and home
(spec stanza vs sibling file, linker-read either way); context
versioning across builds (an update must migrate live Contexts);
default scope for undeclared legacy state during the transition; whether
the header/hypothetical contexts need a lifetime discipline. Sync "the
context syncs" = each slot merges by its declared discipline — this is
also the machinery the tunables conversation (redo item 8) needs, so
that conversation starts from here.

## the absorption ladder: SyncVar merges into Var by migration (2026-08-21, hybrid #p31–p32)

Ash asked the right question at #p31: isn't the old SyncVar just replaced
by the new Var — one entity? Yes — they are the same entity seen from two
eras, and the column mapping is exact: `Scope::Local/User/Group/Global` ↔
the scope markers (local ≈ device); `.set()` ↔ `MergeLastWrite`; `.add()`
↔ `MergeCrdtSum`; a key into the JSON state ↔ a field on the Context. The
end state is one `Var`: the runtime sync behaviour lives on the marker
impls (`add()` exists only where `MergeCrdtSum` was declared), so the
declared discipline selects the machinery — today nothing stops a caller
using `.set()` on a counter.

They cannot be welded together mid-flight: SyncVar's callers live in
composed functions that hold the JSON state string and have no path to
the Context yet. Welding sync methods onto Var while values still live in
the string would hand Var a key-into-JSON identity the design exists to
kill. The merge is by **absorption**: migrate callers feature by feature
once functions can reach the context; SyncVar is deleted when its caller
count hits zero. Ruled at #p32: keep going until all rungs are built and
working, testing as we go — the run is autonomous under the hybrid
pipeline from here.

**The rung ladder of record** (each rung provable, toggleable, and
changing only what it claims):

1. ✅ declarations → emitted typed Context (`.vars` sidecars, the Var
   family, `Permits`; build 187, renamed 188).
2. the Context comes alive: constructed and held per place;
   `GET /diag/context` snapshot route (readout precedent).
3. functions reach the context: an accessor threads the held Context to
   every composed function (the methods-on-Context destination from #p21,
   arrived at incrementally); zero behaviour change.
4. `enabled` — the goal rung: every feature's implicit
   `enabled: bool = true (user, last-write, inherit)` var; linker-emitted
   gates at chain heads read the typed field. The old design's lessons
   apply structurally: the var-delivery machinery lives beneath the
   Context (no trusted.md needed); the gate reads the incoming context
   (Elm boundary); nothing exempt (#p4 ruling stands).
5. per-user Contexts on the server: the process-wide object becomes a
   table keyed by user; dispatch selects; client unaffected (its one
   Context is its user's).
6. var sync by declared merge: the broadcast channel speaks
   node-path-keyed var ops; marker impls emit them (set/add semantics
   arrive on Var).
6a. persistence + eviction (added on the rung-5 worker's recommendation,
   2026-08-21): a user's world survives a server restart, and idle
   worlds can be reclaimed. Sequenced after sync because a var op on the
   wire is the same shape a disk record wants; eviction lands here too —
   it is the same question as the table-only-grows leak.
6b. the overlay chain: global (and group) scopes become real — a shared
   `_global` layer that inherit-resolving reads fall through to, ops
   whose audience is everyone, and the linker's scope refusal lifted.
   Added 2026-08-21 when rung 6 proved `SyncVar::global` (the shared
   tap counter) cannot migrate without it — rung 7's zero-callers goal
   blocks on this rung. Transport hardening lands here too: op ids +
   a seen-set, because the first crdt-sum declaration makes the
   demonstrated replayed-add double-count reachable.
7a. the bridge (added 2026-08-21 when rung 7 returned to triage): six of
   seven SyncVars are read from the state string by JS fragments, which
   cannot call with_context — so migrated vars republish their resolved
   values into the loop payload at declared legacy keys (a `js:` column
   in the declaration). Fragments stay untouched; #p23's full arrival
   (JS reading a ctx object directly) is a later cleanup. Bridged
   declarations without the bridge node are a loud link error.
7b. the counter merge kind (same triage return): `crdt-sum` forbids
   `set`, and three shipped tools set the tap counter — a latent
   set-races-add bug that exists TODAY under SyncVar. `counter` = an
   epoch-reset counter: set bumps the epoch and assigns, adds within
   the epoch sum, adds carrying a stale epoch drop (reset wins). The
   tap/sync scope duality (local when sync unticked, global when
   ticked) is preserved by two declarations and a seam sync extends.
7. migration: feature by feature, each SyncVar use becomes a `.vars`
   declaration + field access; per-feature toggle proofs cover both
   eras. From this rung, loop/context stops being optional for migrated
   features — untick becomes a loud link failure, and the byte-identical
   baseline era ends, by design.
7c. the context join (added 2026-08-21 when worker 2 measured the hole):
   migrated user-scoped vars drop out of `/join`'s snapshot — a fresh
   device learns them only from the shared 50-entry broadcast backlog,
   and a flooded backlog hands it defaults (a user who chose "ask me"
   would get automatic updates on a new phone). The fix is the exact
   VarJoin analogue: on join, the server sends the user's present
   context values (and the global layer's) as id-bearing ops the client
   applies between turns — dedupe-safe, boundary-safe. Rules before
   rung 8 may claim absorption complete.
   ✅ BUILT as `loop/context/converge/parity`. One deviation from the
   brief, argued in the spec: the records are **resolved values in the
   `CtxUpdate` shape, not id-bearing ops**. Rung 6 chose that shape for
   the relay precisely because assignment is idempotent, so a join needs
   no id and no seen-set — and an id-bearing op would be strictly weaker,
   since the seen-set is bounded and a join replayed past the bound would
   double-apply (for a counter, wrongly). Everything the brief asked for
   comes with it: counters carry `[epoch, sum]`, device vars are excluded,
   absent vars send nothing, and a join queues no op at all.
   The rig found one defect in the first cut: the layer is a `Context`
   like any other, so it carries a present bit for `own` user vars whose
   resolver never reads it — five records of nobody's value in every
   parcel. Fourth silence added; parcel 11 → 6 records.
   FOR RUNG 8: the trigger is `/join`'s `Join` message and the reply is
   typed `VarJoin`. Deleting either name silences the context join
   SILENTLY. Keep both, or rename both halves in one commit.
8. ✅ absorption complete (build 212, 2026-08-21): SyncVar deleted, the
   chooser's tickboxes drive `enabled`. THE DONE SENTENCE WAS PROVEN on
   the real UI, nine stages: untick a feature in the chooser, it is off
   for you only, on all your devices (0.5s, no reload), it survives a
   server restart and reaches a never-seen device, and re-tick finds
   your state intact — the count resumed at 3 and counted to 4. The
   ladder ran builds 187–212 in one day under the hybrid pipeline: two
   Opus workers, eleven rungs (three added mid-climb by triage returns
   and worker findings), zero review returns, one correct refusal.
   Remaining post-ladder queue: sender_of full-phone migration (now
   URGENT — rung 8's rig reproduced a cross-user settings leak under
   colliding last-fours), fragments obeying enabled (the half-off
   surface), the gate-coverage report, rung 3's POST as edit_op,
   deploy's one-server-per-state-dir assert.
   ✅ **THE LADDER IS TOPPED OUT.** Built as
   `shell/panel/noob-button/chooser/enforced`, which contains no mechanism
   of its own — every rung beneath it supplies one, and this node is a
   translation: a click on one side, an `enabled` edit on the other, and a
   DERIVED `feature_ticks` map that cannot disagree with the gates because
   the map and the gates are two readings of one field.
   `chooser.index.js` was not touched.
   Two arguments worth keeping. **Re-tick is `clear`, not set-true** —
   `enabled` is `inherit`, and the old stored map's absent-means-on is
   exactly what `clear` restores; writing `true` would look identical
   today and would silently pin that user off the layer forever. And the
   map carries a node's OWN answer, not its resolved one, because
   `reflect()` already derives ancestor shading by prefix walk — so an
   unticked parent shades its children without unticking their boxes, and
   re-ticking the parent restores the shape the user had.
   **The sentence was proven end to end through the real UI** (nine
   stages, a genuine click on the rendered `.ctick`, no synthesized
   `ftick_` event): A1 counts 3 taps → unticks the counter → its tool
   leaves A1's toolbar and A2's within a moment, no reload → B, a
   different person, is untouched throughout → server restart, A's untick
   replays from the log → a device A has never used joins and sees it →
   A1 re-ticks → both instances recover and the count resumes at 3, then
   counts on to 4.
   SyncVar, `queue_var_op`, the `/tmp/miso-vars` store and its VarUpdate
   relay, and `/join`'s value snapshot are all deleted; `scope` is a
   grouping node holding the joining flow now, and its `serde` dependency
   moved to `/context`, which is what uses it. The `Join` message and the
   `VarJoin` reply type were KEPT, per 7c's warning — `/parity` rides the
   envelope and `/veil` waits on the type.

## the migration drops a var out of `Join`: rung 7 needs a context join (2026-08-21, rung-7 worker)

**Named and measured, not worked around — this is the wall rung 7's second
half hit.** A user-scoped SyncVar lived in the var store, and `/join` answers a
booting instance with a snapshot of that store, so a device seeing this user for
the first time was told their values before it decided anything. A declared
`/var` is not in that store, and nothing replaces the snapshot: the only thing
that carries a migrated var to a never-seen-before instance is `messaging`'s
broadcast backlog — **fifty entries, shared by every user**, so a var whose last
write has aged out arrives as its declared default.

Measured on the two-instance rig (test server on 8097, its own state directory,
one test user, a third browser profile that has never seen the server):

- **before** the update migration, with the backlog flooded past fifty:
  `update_policy` = `"fixes"` on the new device — `VarJoin` delivered it.
- **after**: `update_policy` = `""`, and `policy.index.js` falls back to
  `auto`. A user who chose *ask me* would get automatic updates on their new
  phone.
- the hole is **not** this cluster's: flood the backlog with tick writes and
  the already-shipped `asks` migration (6427208) fails identically — a fresh
  device reads `[]` where the user has a list. It only looked fine before
  because the flood *was* asks writes, and a `last-write` relay carries the
  whole resolved value.

Device-scoped vars are unaffected by construction — nothing was ever supposed
to travel — so the tools cluster is clean.

What is missing is a rung: **a context join**, the exact analogue of `VarJoin`.
A booting instance asks; the server answers with its world; the values arrive as
the ordinary events every other arrival already is (`set_from_json`, idempotent,
republished by the bridge on the way out). It is small, it reuses the existing
message path, and it should be ruled and built **before rung 8 declares
absorption complete** — otherwise "off for you only, on all your devices" is
true only for devices that were listening at the time.

Two honest workarounds were considered and rejected: declaring `auto` as
`update_policy`'s default (hides the gap behind a value that looks right and is
not the user's) and leaving the three update vars on SyncVar (fails rung 7's
zero-callers goal and leaves the hole in `asks` anyway).

## found in passing: the four-digit tag collision (2026-08-21, rung-5 worker)

`comms/messaging`'s `sender_of` keys users by the LAST FOUR DIGITS of
their phone number, and `dictate/mirror`'s blob storage inherits the
key — so two guests whose numbers share a last-four share a blob
namespace today: one could see, or overwrite, the other's mirrored
audio. Pre-existing, unrelated to the contexts ladder, surfaced while
rung 5 was deriving cookie→user (it deliberately keys contexts on the
full phone number for exactly this reason, and does not call
`sender_of`). Two nodes now independently derive user-from-cookie — the
rule-of-two signal that a shared `sender_of` belongs in `miso/users`,
keyed on the full number, with mirror's blob namespace migrating to it.
Queue as a redo-adjacent fix; it touches real user data isolation.

**Seen live, 2026-08-21, rung-8 worker.** The rung-8 rig minted two test
users whose numbers happened to share a last-four, and the "a different
person is unaffected" stage failed: person B's feature list showed A's
untick and B's toolbar lost the tool A had switched off. The *authority*
was never wrong — the server wrote A's world and only A's, exactly as
rung 5 promised — but `publish("user.<sender_of>")` addressed the relay by
the four-digit tag, so B's long-poll heard it and applied it. That is the
collision, reproduced end to end on a real surface, and it is now a
cross-user leak of a *setting* rather than only of a blob namespace. It
cost an hour of a rig chasing a phantom. The fix is unchanged and now
overdue: `sender_of` moves to `miso/users`, keyed on the full number.

✅ **FIXED, 2026-08-21** — two nodes, both cited `#p32`. `users/whole-number`
redefines `sender_of` to the whole number, spelled `phone:+44…` exactly as
rung 5's context table spells it, so the server has one key per person;
`dictate/mirror/adopt` moves recordings already filed under a four-digit
name onto their owner. Three findings worth keeping:

- **The relay is addressed by a token, not by the identity.** Numbers in
  audience strings would have moved the leak rather than closed it:
  `/tmp/miso-broadcast.json` is a shared file that outlives the request.
  `publish` and `wait_filter` translate through an HMAC of the identity
  under the signing secret, so callers keep writing `user.<identity>` and
  the buffer never sees a number. Durable state keeps the identity — a
  lost secret costs one relay round, not a recording.
- **Migration is on first touch, not at boot.** A directory named `…0123`
  names a *tag*; the map from tag to person is the ambiguity being repaired,
  and only a request carries a proven identity. The rename is the claim, so
  it is atomic and the collision resolves itself: first claimant takes the
  store, the second starts clean, and the log says in as many words that
  some of those recordings may belong to somebody else.
- **The rig proves the bug, not just the fix.** Untick `whole-number` and
  the same script fails three checks — B hears A's context op, A and B read
  each other's blobs, B's index lists A's recording — which is the rung-8
  reproduction, now on a switch. Scripts in the worker's scratchpad; the
  shape is worth re-creating if this ever regresses.

Still keyed by the tag, deliberately: `diag/blackbox`'s log lines, where the
tag is an annotation in an operator-only file rather than an isolation key.
Two colliding guests interleave there and cannot be told apart — a
readability limit, named rather than fixed.

## the parked-residuals register (2026-08-21, hybrid #p61)

Under the zeno rule (hybrid.md checklist 7, #p57) a run ends with
residuals fixed or deliberately parked. Ash delegated park judgment
(#p61: "if you consider them necessary, build them, otherwise don't");
these six are parked, each with its reason and revisit trigger:

1. **Relay entries dropped at the identity switch** — one-time event,
   already past; parity heals rejoining devices. Nothing to build.
2. **Audience-token/signing-secret coupling** — losing the secret
   re-keys audiences, costing one relay round in a scenario that
   already logs everyone out. Revisit: never, unless secrets rotate.
3. **Local epoch minting** — two racing resets collide into the same
   zeroed counter both users asked for; adds in the collision window
   are lost either way by reset-wins semantics. Revisit: if a counter
   ever means money.
4. **Seen-set FIFO bound (4096)** — a replay slips through only after
   4096 intervening ops outrun a retry; growable constant, log-primed.
   Revisit: raise the constant if op rates grow 100x.
5. **Whole-value join parcels** — bounded by what a person touched;
   the watch-trigger is a chatty list var (asks is the candidate).
   Revisit: per-var versioning when a parcel measurably drags a join.
6. **Group scope refused** — awaits a membership model; future feature,
   not leftover. Revisit: when groups exist.

7. **Seam occupancy** (assignment 2's triage-return): a node reached
   only through `typeof` checks wants ABSENCE when unticked, not a
   no-op — which needs window-bound fragment objects and an authorship
   convention (53 unguarded `feature_Loop` references say so). Parked
   with the census attached. Revisit: at the fragment-convention /
   builder-skillset design conversation, or when a seam-occupied
   feature's untick becomes a real user path (today's only case is the
   chooser's own teaser seam, behind the self-lockout repair path).
8. **Fragment-obedience coverage limits**: obey gates the four census
   shapes on index.html; document-level listeners/timers, post-load DOM
   outside claimed containers, and the auth/install pages (which have
   no per-user world) are outside it, named in obey.md. Revisit: same
   conversation as 7 — they share the convention.

9. **The sweep's own tails** (final assignment, judged at close):
   the eviction-mid-request window (a user idle past threshold whose
   request lands during the sweep finishes against an emptied world —
   writes reach the log, next touch rebuilds; revisit if eviction
   thresholds shrink); `context_evicted`'s rebuildable-from-log rule
   enforced by prose (revisit: type-enforce when a third implementor
   appears); S5's console line unobserved by rig (static inspection
   only; revisit: first JS-eval-capable rig) and its seamless-upgrade
   noise left deliberately loud; boot's first-turn making a foreign
   mid-boot edit invisible — recorded as the boundary law working, a
   design note rather than a residual.

10. **Undo's stack does not survive a reload** (undo worker, asks#1787346956331):
    it is thread-local in wasm memory, so a refresh empties it and the button
    is dimmed until the next edit. Persisting it means choosing a home — a
    `device`-scoped var does not persist server-side either — so it is
    explicitly unbuilt rather than half-built. Revisit: if anyone asks to undo
    something from before a reload.
11. **An inverse op that errors is silent** (same): a step whose var has left
    the composition, or whose prior no longer deserialises, spends the step and
    changes nothing. Right shape for a button, wrong shape for a diagnosis.
    Revisit: the first time it happens to anybody.
12. **Undo is one level deep by design** (same): the ask asked for
    undo-then-undo-to-redo, so the inverse is itself recorded and pressing
    twice oscillates rather than walking further back. The ten-step stack is
    what lets several tools' histories coexist and what bounds memory, not a
    depth of history the button can reach. Revisit: when someone asks to go
    back further, which is a different ask and would need a redo stack.

13. **`/payload`'s republish is still position-dependent** (turn-end worker,
    #p56): the bridged page keys are read during the paint, by Rust as well as
    by JavaScript, so republishing at the turn's end would be a frame too late
    and it stays at `/payload`'s own link. A node newer than `/payload` that
    edits a **bridged** var would paint one stale frame. There is no such node —
    all six bridged vars (`open_tool`, `tools_catalog`, `asks`,
    `update_policy`, `update_accepted`, `update_ticks`) are written only by
    nodes older than it. Revisit: the first node newer than `/payload` that
    writes one; the structural answer is a pre-paint moment emitted by the
    linker into the `render` entry, which is a bigger change than the trap
    currently earns.

THE CAMPAIGN CLOSED 2026-08-21, build 238: every residual from the
contexts ladder and its own assignments is fixed (isolation, fragment
obedience, coverage report, same-door, sole-tenant, unmixed, nested
turns, boot-turn, single broadcast, eviction-frees-memory, the bridge
complaint) or on this register with a reason and a revisit trigger.
The ledger is empty. Done means done (#p57).
## fragments obey enabled: the census, and the half that is a design question (2026-08-21)

Built as `loop/context/enabled/obey` (#p56). The page-side twin of rung 4:
a fragment's chain links fall through to what they replaced when their node
is off, and its furniture is marked with its owner and hidden. Four things
worth keeping:

- **The census made the mechanism honest.** 105 fragments (68 script, 32
  style, 4 body, 1 head); 62 script + 32 style reach `index.html`, the only
  page with a per-user world. Every script fragment is one or more of four
  shapes — object definition (65 of 68), chain link (41 files, 75 patched
  functions), load-time side effect (34), handler registration (12) — and
  there is no fifth. The mechanism covers shapes two and three, plus body
  and style; it says so, with counts, in its own spec.
- **No JS parser is needed to gate a chain link.** The linker notes what a
  function looks like above a fragment and wraps it below, only if it
  changed. A patch inside a `typeof` guard that did not fire is left alone
  for free, and the rule "a new method is a chain's start, not a link" falls
  out of the same comparison — which is Rust's gating rule verbatim.
- **A claim must ride the element, not its position.** The first cut marked
  only the root a fragment added; `/build-row` re-parents `#featuresBtn` out
  of `/features-button`'s row and deletes the row, so the button escaped its
  owner. Marking every element in what a fragment adds fixes it. Expect this
  class wherever one node rearranges another's DOM.
- **The object half is returned to triage, not shipped.** A node whose only
  page effect is a method reached through `typeof feature_X !== 'undefined'`
  is still untouched — `panel.index.js` picks the chooser's list or the
  changes teaser exactly that way, and says in a comment that unticking the
  occupant should restore the teaser. Absence is what that seam wants, and
  it needs every `const feature_X` rewritten to a window binding; the census
  found 53 unguarded references to `feature_Loop`, so absence there throws
  rather than degrades. That is a design conversation with a fragment
  authorship convention in it, not a residual fix. The runtime carries an
  empty `extra()` seam for it.

**A rig lesson that cost an hour, again.** The rig's login route derived the
phone number from every digit in the query string, so `who=a2` minted a
different user than `who=a` — and "a device this user has never used" looked
like a broken context join, on a build with and without the change. Two
readings saved it: the same failure with the node unticked (so: not mine),
and a hand-fed `VarJoin` that worked (so: not the client half). **When a rig
says a shipped invariant broke, suspect the rig's identities first** — print
the server's view of who each request is before believing the surface.

## the silent-tickbox and tooling cluster (2026-08-21, #p56)

Four fixes, four commits. Three arguments and one limitation worth keeping.

- **The coverage report measures what the LINKER emitted, and that is not
  quite "what a tickbox reaches".** `fmlink.py --coverage` counts rust gates,
  fragment wrappers, marked stylesheets and marked body roots per node, and
  writes them beside the build for the export to stamp. 50 of 120 nodes gate
  nothing at runtime today — including all four nodes built in this cluster,
  correctly: a route, a boot hook and a log label are machinery, not behaviour
  a person ticks. The limitation: a node that honours its own tickbox in its
  own code reads as under-covered — `/obey` shows `1 style` because its script
  fragment is deliberately ungated (it is the thing that answers "is this node
  on?") while its runtime neutralises its own map when unticked, which the rig
  proves and the counter cannot see. A second-generation report could ask a
  node to declare self-enforcement; today the number is a floor, not a truth.
- **A second door is where invariants die.** `POST /diag/context` predated ops,
  so it assigned into a world: no merge check, no id, no relay, and a log
  record minted by a seam of its own. It is now a translation into the same
  `CtxOp` a client sends, and everything downstream happens by itself. The
  visible payoff is not the tidiness: a repair typed at the server now reaches
  the person's phones without a reload, and a counter refuses a bare value
  with a message naming both verbs instead of a serde error about arrays of
  length two.
- **A guest must not take the claim.** `sole-tenant` claims the state
  directory at boot with a pidfile and refuses a second live miso. The first
  cut let `MISO_ALLOW_SHARED_STATE` write the pidfile too — so the deliberate
  second server, on dying, left a corpse's pid behind and silently disarmed
  the check for the next boot. The override now warns and leaves the claim
  with the server that actually holds the directory. Boot is the honest place
  for this (deploy can only look at one machine at one moment, and the port
  only ever caught the same-port case); deploy asserts the outcome instead.
- **What a log is for decides how it names people.** The blackbox is read by
  an operator who then usually talks to that person, so its label needs to
  separate people AND identify them; the last-4 tag did only the second, badly.
  It now carries the guest-list name plus the first 48 bits of the same opaque
  id the relay uses — no phone number in a `/tmp` file, and `replay.py --who`
  selects on either half. The seam it hangs on was a two-line refactor of
  `/blackbox` (the tag was inline), which is the sanctioned move and which the
  toggle proof shows is behaviour-neutral.

## the small-residuals sweep (2026-08-21, #p56)

Five fixes, five commits, the campaign's last assignment. What is worth
keeping is mostly about measurement.

- **Two readings before a performance claim.** Boot-as-a-turn drops the
  per-gate world clone from 15 to 1, and the first timing said the fix made
  boot *slower* — which would have been a real finding if it were true. It
  was a cold machine: warm, it is 363µs against 369µs, a 1.6% difference at
  the noise floor. A `Context` clone turns out to cost ~0.4µs because most
  of its 121 vars are `Copy` scalars. The fix is still right (the count is
  one clone per gate call, and the tree only grows) but it buys allocation
  pressure, not latency, and the spec says so. **A number from one run is
  not a measurement.**
- **The eviction proof needed the right instrument.** RSS does not move when
  200 worlds are freed — macOS's allocator keeps its arena — so RSS would
  have said the fix did nothing. A counting `GlobalAlloc` patched into the
  rig binary says the truth to the byte: 147 KB in for 200 worlds, 99.9% of
  it back after the sweep, against 1.2% before. When a measurement can only
  disprove, find a better one before believing it.
- **A refactor's last 35% was somebody else's state.** The `Arc` migration
  freed the worlds and still returned only 64.5%, because eviction left two
  things behind: the maps' retained buckets (a map that has held 200 users
  keeps room for 200 users) and `/overlay`'s per-user dedupe state. The
  second could not be reached from `/remember` without a backwards
  dependency, so `/remember` grew a `context_evicted` seam and `/overlay`
  hung its own forgetting on it — with the rule written down: anything hung
  there must be rebuildable from the log, because that is all an evicted
  user leaves behind.
- **The bridged key set was free all along.** The one-way bridge's complaint
  looked like it needed the linker to emit a list of bridged keys. It does
  not: `Context::republish` writes its keys unconditionally, so republishing
  into an EMPTY object is the list, and the values with it. A mechanism that
  looks like it needs a declaration sometimes only needs to be run against
  nothing.
- **A guard for a case that cannot happen yet is still worth its ten lines.**
  Nothing nests turns today; the probe that proves the depth counter also
  documents what the bug would have been (an inner begin re-freezing from
  live, an inner end clearing the outer view) and it took one rig route to
  show both, before and after.

## the late link's ops: a trap every node newer than /converge falls into — FIXED (2026-08-21, undo worker; fix the same evening)

Found while building `shell/tools/undo` (asks#1787346956331), measured on the
two-instance rig, and worth a rung of its own.

`/converge`'s `update` link drains the op outbox into `state["_send"]`, and
`/payload`'s re-freezes the shared layer before the paint. Both were written
when they were the outermost links on `update`. They are not any more:
composition order is provenance order, so every node authored after them —
`chooser/enforced`, `tap/counter/square-taps`, and now `tools/undo` — wraps
them. **An op minted by one of those links is minted after the drain and
after the re-freeze**, so it neither reaches the wire nor the frame until
some later event happens to flush the outbox.

This is not hypothetical. `/square-taps` shipped with it two hours before
undo was built. Measured with undo unticked: three taps, then n² — the count
stays at 3 on the device that pressed it, and the second instance never hears
about the square at all until something else happens. With undo ticked (it
calls `ctx_ship_ops`, `ctx_stamp_outbox` and `context_layer_begin` at the true
end of the turn) the square shows in the frame the finger caused and the
other device has it a moment later. So the defect is real, and it is
currently masked by whichever node happens to be newest.

The honest fix is not "the newest node pays for everyone" — that evaporates
the moment `tools/undo` is unticked or a newer node arrives that does not know
to do it. The turn's end has to belong to the turn, not to a link.

**BUILT, as `loop/context/edit/turn-end` (#p56).** `edit`'s `on_event` link
gained one named moment, `context_turn_close`, between the event and the drop of
the freeze, and the new node fills it with `/converge`'s drain and `/overlay`'s
stamp. The position is structural rather than provenance-ordered: `update` is
called from inside `on_event`, so every `on_event` link is outside every
`update` link by construction, and no future node can get in front of it.
Measured with a probe spliced into the client's outermost `update` dispatcher —
a link no real node could be newer than: on main it stranded and shipped only on
the next unrelated event; with the phase it ships in its own turn, stamped, and
the server's world has it.

The paint's half needed a different answer, because the phase is after the
render, not before it. `edit_layer` gained the read-your-own-writes replay
`edit_context` has had since rung 7, so a layer edit made at any depth is
visible to the rest of its own turn including the paint, without anybody
re-freezing anything. `/payload` keeps its layer re-freeze only for records
arriving from the server, which are written straight to the live cell.

One dependency in the family cannot be caught by rustc — the phase removes no
generated function, it moves *when* something runs — so the linker catches it:
`fm:turn-end-required` on `/converge`, `fm:turn-end-phase` on the new node, and
a composition with the first and not the second fails by name instead of
building an app that looks fine and syncs nothing.

What is NOT fixed, and is on the parked register: `/payload`'s republish still
runs at its own link, because the bridged page keys are read during the paint.
A node newer than `/payload` that edits a bridged var would paint one stale
frame. No such node exists — all six bridged vars are written by nodes older
than it.

**Related, and fixed in the same motion:** `/payload`'s `update` link called
`context_turn_begin()` without a matching end. The depth counter added
by `edit/first-turn` makes that a no-op re-freeze and leaves the depth one
higher after every event, so the client's own frozen view is taken once at
the first event and never retaken — the boundary law holds only because
`edit_context`'s read-your-own-writes mirror keeps that view current. The
layer half of `/payload`'s re-freeze does work (`context_layer_begin` is not
depth-counted). Nothing observable misbehaved and `eprintln!` goes
nowhere in the wasm place, so the eight-deep warning was silent. The call is
simply gone — the user's own world never needed a re-freeze there, because
everything reaching it during a turn goes through `edit_context`. `edit.lib.rs`
now counts begins, ends and freezes (`context_turn_stats()`), so the balance is
a reading rather than an argument: over a rig session of 32 turns — 32 begins,
31 ends (read from inside the still-open turn), 32 freezes, depth 1. Before the
fix it was two begins and one freeze per event, with the depth climbing.

## the ask conversation, and the trace behind it (2026-08-23, asks #p8–p10)

Two conversations that turned out to be one machine. Ash brought Tom's
challenge to the never-ask rule (#p8), and then the wider vision the ask box
has always been a door into (#p9). Recorded as design, not as a plan: ash's
own framing is *"groping at the big idea — measure the design against reality
and modify as we go"* (#p10), so what follows is a hypothesis with named
places where it will meet evidence.

### the never-ask rule was about WHEN, not whether

Tom's challenge: a user asking for a feature has usually already jumped from
their perceived problem to a solution, and a developer who understood the
problem would often propose something better. So a context-understanding
dialogue would drastically improve the proposal — and "never ask" appears to
forbid exactly that.

It does not, and the distinction is the whole answer. The rule was ruled at
2026-08-16 (asks#1786892582635, notes.md above) in these words: *"when an ask
comes in, the user expects a feature in the next update. So you must never ask
me about it here."* **Here** is the builder's channel, later, asynchronously,
in a place the asker is not watching. That is a broken promise.

A question asked *at the moment of asking, in the app, while the person is
still holding the phone* is the opposite act. Their attention is never cheaper
than in that second, and `/propose` already does a weak version of it — the
drafted paragraph in an editable box IS a dialogue. It is simply a stupid one,
because the drafter is a template with no idea what the person was doing.

So the rule sharpens rather than bends:

- **at ask time, in the app, synchronously**: questions are welcome, and
  the flow already has the surface for them.
- **after the ask, in the builder's channel**: never. Unchanged.

### what the drafter would need to know

Tom's developer knows the code and the other users. Most of that is already
exported and can be handed over:

- the feature tree — every node's name, purpose and `## user` paragraph,
  already baked into `tree.json` and embedded for `/semantic-find`;
- the birthplace — `/birthplace` already stamps the open tool and its node;
- **the last minute of the person's own events** — `/blackbox` records every
  tap, always. If somebody pressed reset five times before asking, the real
  problem is in the record and not in their sentence. This is the strongest
  signal available and today it is thrown away;
- their own ask history, and other people's open asks — which is where the
  rule of two (dedup, promotion) can fire at the moment of asking rather than
  weeks later.

### the shape of the conversation

1. Local find runs first — instant, offline, free. A tool that already does
   it ends the ask, as today.
2. Otherwise the ask goes out with the context pack above.
3. Three things come back at once: a proposed paragraph (as today), **at most
   two questions** — only where the answer changes what gets built, offered as
   taps rather than open prompts — or, sometimes, an answer instead of a
   proposal ("that exists, here"; "that is a setting, changed").
4. Pressing propose immediately skips all of it. Today's behaviour must stay
   one tap.
5. What is settled is the contract, unchanged (#p85's doctrine).
6. **The whole exchange is filed, not only the final sentence** — so a node's
   provenance records the problem, not just the request.

Offline, step 3 does not happen and the template drafts as it does now. The
flow is never gated on the net (`/propose`'s standing rule).

Two constraints held firm. **Questions must be cheap** — two maximum, tappable;
more is an interrogation and people stop asking. **A better idea is offered,
never substituted** — the law above the laws was paid for by the non-map, and
the fix is not to propose worse things but to put the alternative in front of
the person as a choice. Silent substitution stays forbidden; the difference
here is that the asker is present to say yes.

### the wider machine: traces, tasks, emergent tools (#p9)

Ash's vision: a person is pursuing a goal ("book a trip to China") and
sub-goals ("find a hotel in Shanghai"), using tools in aid of them. From the
trace of tool use we should be able to (a) infer the task tree, or (b) let
them represent tasks as first-class objects — and then, from accumulated
history, synthesise **emergent tools** ("book a hotel in [city]") that by and
large do what the person would have done.

**An ask and a trace are the same machine from two ends.** An ask is the user
telling us the task; a trace is the user showing us it. The context pack above
is the first rung of this ladder, which is why it pays before anything
ambitious exists.

**Three substrates are already built**, for other reasons, on days two and
three: `/blackbox` (every event recorded, always, offline-first, shipped when
possible), `/replay` (a recorded session re-driven through the same update
chains — a trace is already an executable program), and `/drive` (an agent
tapping the real UI). The `/loop`'s `(state, event) → state` purity is what
makes traces replayable at all.

**Infer, then offer — never make them file paperwork.** Ash's (a) and (b) are
one thing joined by a move the ask flow already uses: a task becomes a
first-class object when the user confirms a guess. Nobody creates task objects
by hand; almost everyone will tap yes on "booking a trip to China?".

**The second occurrence reveals the variable.** One trace is a recording and
cannot say which parts are the point. A second trace — Osaka rather than
Shanghai — puts the hole exactly where the two differ. So parameterisation is
a diff, not a generalisation problem, and it is the promotion rule (#p18)
arriving in a new domain: a constant earns a variable on its second showing.

**An emergent tool should be a real node**, with a description, a toggle, and
provenance — and the provenance is *the traces it came from*, a third citation
kind beside `transcripts/…#pN` and `asks#<t>`. It then rides the machinery that
exists: awaiting list, one OK, all devices, untickable if wrong.

The ladder, roughest form: (1) traces the server can read back per user;
(2) episode segmentation — tool opened, tool closed, long pause, no model
needed; (3) naming an episode, offered for confirmation; (4) grouping episodes
into goals; (5) spotting episodes that rhyme; (6) proposing the tool on the
second rhyme, with the diff as parameters; (7) shipping it as a node. Rungs
1–2 improve the ask box immediately; rung 3 is the first thing a user notices;
5–6 are the payoff.

### where this meets reality, named in advance

**The current tools are toys.** Taps and dictate generate no trace worth
mining. The mechanism can be proven on them — the precedent is `/compute`
shipping a multiply kernel it did not need — but the value waits for tools
that do things with consequences.

**Replaying is safe for looking and dangerous for acting.** Searching,
filtering and reading can be re-driven; booking, paying and messaging cannot.
An emergent tool should run to the last step and stop. This belongs in the
design from birth, not after the first bad surprise.

**Traces are private, and ash has ruled on it (#p10): only you can see your
traces.** That is a stronger constraint than the ask conversation's, and it
bounds the design rather than decorating it — an ask is a sentence somebody
chose to write, a trace is everywhere they went. Whether the ask text itself
may leave the device for the drafter is a separate and still-open decision,
and for the campaign app's trust ring it may be no.

**The cheap experiment, before any of it.** Real blackbox logs exist on the
mini. Take a day of recorded sessions and ask a model to say what the person
was doing, from the events alone. If a task cannot be named from a real trace,
rungs 3–6 do not exist yet — and that is an afternoon's answer, not a
quarter's.

### open, and ash's to rule

- Does the ask text (and any trace context) leave the device to a third-party
  model? For everyone, or per user, or only ash?
- Who pays, and is the template fallback honest enough when the model is
  unreachable?
- Does a conversation that ends in an answer rather than a build count as
  done? Proposed: a new lifecycle state, `answered`, distinct from `shipped`.
- The fourth moment nobody uses: after shipping. The awaiting list already
  shows what was built; "is this what you meant?" is one tap, and a no could
  reopen the conversation instead of becoming a fresh ask.

## permission, escalation, and features that spread by consent (2026-08-23, asks #p11, #p13)

Still forming — ash stopped the build deliberately ("more I want to chew
over"), so this is the state of the thinking, not a plan. Follows the ask
conversation above; the two belong to one design.

### where permission actually stands today

Nothing is enforced. `users.json` is `{name, phone}` with no authority field,
and the only privilege check in the composed source is
`overlay.rs`'s `ctx_may_write_layer` — localhost tooling may write the shared
layer, a logged-in user may write their own world and nobody else's. No ask
can reach another person because no path exists for one.

So **the human-supervised builder IS the permission system**, exactly as
notes.md predicted when it filed this as ruling-shaped. That holds at two
users and fails at a campaign team.

### permission gates REACH, not asking

Asking stays open to everyone on the guest list. What permission decides is
how far the answer travels — and miso already has the vocabulary for that, in
scope. **An ordinary person's ask lands in their own world by default**, so
the common case raises no permission question at all: bigger buttons, for
you, is nobody else's business. Permission only bites when the effect widens
past the asker.

Three things the word muddles, kept apart: *may you ask* (guest list, one
bar); *what may it touch* (blast radius against authority); *how far does it
land* (scope). The second and third are the real ones.

**Authority may not live in the asker's own world.** A context is the user's
to write — that is the ladder's whole design — so a permission expressed as a
var is a permission the user can raise. It belongs beside the guest list,
where only the server writes it. This is the sharp form of the standing note
that ticks are preference and authority is not: co-located, never conflated.

**Blast radius is a guess before and a fact after.** Triage can only estimate
what an ask will touch; the build discovers it. So the check runs twice, and
the second instrument already exists — deploy prints the feature nodes a
release touches. A finished thing that reaches further than what was
authorised does not ship; it escalates.

### escalation routes to the nearest person who can say yes

Not everything reaches ash. The adjudicator is the smallest authority
covering the ask's blast radius — a field volunteer's group-wide ask goes to
their campaign lead; only asks touching miso itself climb to the owner.

Two details that keep it cheap: **the adjudicator's inbox is the requests
list from the other side** (they already have that surface in the panel —
approve and decline, no new screen, works on a phone in a field), and **the
asker hears a state, never a question and never silence** — a lifecycle stamp
naming who it waits on. The never-ask rule survives intact: the question goes
to the person who can answer it, not back to the person who asked.

**Partial grant is the move to reach for.** If the ask would widen past the
asker and they may have it themselves, build it at their own scope NOW and
let the wider version wait for the ruling. They get their thing today, nobody
else is touched, and approval later is a change of scope rather than a build.
Three people separately granted their own copy is also evidence for the
adjudicator — the rule of two, arriving as data instead of argument.

Two failure modes to design against: **over-refusal** (a system that asks
permission often teaches people to stop asking — self-scope-by-default is
what prevents it) and **invisible authority** (a refusal that does not name
who could say yes feels arbitrary; the app should say "Sam can approve
that").

**A bin we do not have**: asks whose subject is another person ("remove Dave",
"let Priya see my posts"). Not a tunable, selection, feature or bug — an act
with consequences for somebody else. Always an adjudicator, never a builder.

**The model should bound the builder too.** An agent shipping to everyone's
phone should not be able to exceed what the requester could have authorised.
That is the flywheel's safety catch and it matters most unsupervised.

### consent from beside: features that spread (#p13)

Ash's move, and it is a bigger one than permission-from-above: two people
collaborating, one asks for something affecting their interaction, and they
**send the feature to the other**, who accepts. Then the pool widens by
adding more people. Always consensual; good features spread quickly to the
groups that need them.

Three consequences worth recording:

**The group is the accept-set.** `group` scope has been refused at link time
since rung 6b for exactly one reason — nothing in this system can say who is
in a group (overlay.md). This answers it: a group is not a list somebody
administers, it is the set of people who accepted, and the consent trail IS
the membership record. The one hole the absorption ladder named and could not
fill.

**Spreading a feature is not distribution, it is enablement.** Composition is
global and enablement is per-user, so a feature built for one person is
already sitting, switched off, on every other person's phone. "Send it to
Priya" is an offer to flip a switch she already holds — the chooser does the
flipping and `/messaging` does the offering. Both exist. This makes the whole
mechanism small.

**And it exposes a leak**: if every phone carries every feature, every
feature's name and `## user` paragraph appear in everyone's list. Harmless
for taps and dictate; not harmless for a campaign tool where a node's name
may reveal what somebody is doing. Whether the tree is public within a
product, or filtered per person, is a decision the account work owes.

### the account ladder (sketched, NOT started)

`/panel` is at the six-child cap; `/account` has none and its spec has been a
declared placeholder for the profile page since #p58. So the family hangs off
`/account`:

1. **profile** — the 👤 tool gets a body: who you are, what this device is,
   log out moving in from the panel. Assembles what exists.
2. **display name** — the first piece of a user's world other people can
   read. A public corner of a private world is a new idea and needs one.
3. **directory** — who else is here, and the privacy decisions begin.
4. **link** — a mutual connection, consensual both ways. The campaign app's
   trust ring, arriving early.
5. **share** — offer a feature along a link; accepting flips the tick; the
   accept-set becomes the group.

**Rung 4 is the first user-to-user action in miso's history.** Everything
until now is you-to-you or server-to-you. Crossing that line opens unwanted
contact as a possibility, so the constraint belongs in the design at birth:
reachable only by someone you have linked with, and a link needs both sides.

## the plan meets the terrain (2026-08-23, plans #p4–#p14)

A Sunday conversation, no code until ash said go (#p13–#p14). Two areas,
which turned out to be one architecture seen at two moments.

### the build process: no plan survives contact (#p4–#p7)

Ash's two observations: simple asks blow up into deep trees and long
builds; and the residual tail ends in permission questions he is not
placed to answer, having not been in the code. His diagnosis (#p5): the
plan was made with inadequate information, and the right move on
detecting runaway complexity is to *modify the plan and try again* —
not to push through. That reframes a blown-up build as reconnaissance:
the plan is made from the map, the build is first contact with the
terrain, and what comes back is a better map. Pushing through on a
wrong plan is exactly what generates the residual tail — every
improvisation made to keep a doomed plan alive becomes a loose end.

The machinery (now in hybrid.md): briefs carry an **estimate** (nodes,
vars, seams); workers carry a **tripwire** — touching what the brief
never named, a fix needing a second fix, actuals crossing ~2× estimate
→ stop, and stopping is correct behaviour, not failure; what comes back
is a **contact report** (what the plan assumed, what turned out false,
what the tree needs), which is a **replan**, not a failed delivery —
triage writes a new brief from the corrected map, and the worktree may
be discarded: code is cheap, the map is the asset. The rewind precedent
at per-ask scale.

**The self-improving loop (#p6):** every miss is a labeled example —
plan, estimate, actuals, what was missed. `misses.md` is the ledger;
triage reads it before writing any brief (the loop only closes if the
record is *consulted* — a report nobody reads is a diary). Calibration
is measurable: estimate-vs-actual per ask, tripwires firing too late
or too early, returns getting rarer, tails getting shorter. Rule count
going up is not improvement. And doctrine that only accumulates goes
sclerotic — the ledger's lessons get periodically consolidated into
fewer, deeper principles; the regroup law for rules.

Escalation routing got its rule too (#p4): **an escalation must be
expressed in ask-language or it isn't ash's** — a choice statable as
what a user experiences is his; a choice that exists only in code terms
is the agent's, decided by doctrine and recorded. Each code-level
question that reaches ash and finds him unable to judge is a missing
rule, not a needed answer.

**The deferred experiment (#p7–#p8):** rewind to the pre-square-tap
boundary (501e7fe), keep the first attempt as a branch, retry the same
asks under the modified plan, and measure — wall time, files, returns,
residual count — against the recorded 36 files / ~1,400 lines / next-day
fallout of attempt one. Deliberately not run today; it stays a named,
ready experiment.

### the ask workflow: recovering the problem (#p10–#p12)

The XY problem, named for miso: users meet a problem, imagine a
solution, and ask for the solution. The brief template held the ask
verbatim and nowhere the problem. Ash's design (#p11): three guesses,
each offered y/n/edit — the task ("we think you were doing T", from
history), the problem ("the problem looks like P"), the amended request
("we'd propose R; *n* means build what I asked"). The keystone is that
**the literal ask at the asker's own scope is a zero-consequence
floor** — only they see it — so the whole guess ladder is enrichment
that never blocks, silence is a valid answer meaning "build what I
said, for me," and the better-for-everyone version is post-hoc, riding
confirmed problems that rhyme (the rule of two at the problem level;
the problem line is the dedup key). The map-lesson guard survives: an
unconfirmed problem never licenses departing from the literal ask.

**Discretion decides whether to ask at all (#p12).** The criterion is
the ambiguity test: how many readings survive the ask's context?
"Italic" with a word selected has one — build, any question is noise.
"Square" inside taps had two, and the record shows both got built six
minutes apart — ambiguity resolved with tokens. The in-hand line is the
forcing function: unwritable means undisambiguated means ask. A
did-you-mean (two concrete readings, one tap) is doctrinally clean
where an open question is not — never-ask forbade design homework, and
which thing *you meant* is the one fact only the asker holds. The
threshold itself learns, with two measurable failure modes: wrong
builds (immediate unticks, corrective re-asks) and question fatigue
(confirms going unanswered — the over-asking failure the permission
notes warn of). Both areas converged on the same architecture: a few
named judgment points (bin, scope, ambiguity, tripwire), each with a
floor that makes wrong calls cheap, each leaving a trail that tunes it.

**Stage 1 is the emergent-tools ladder's foundation.** "Guess the
task-tree from tool-use history, offer it for confirmation" is rungs
1–4 of that ladder almost verbatim — one build serves both designs, and
the cheap experiment (can a model name what a person was doing from a
day of real blackbox events?) is still the gate. Parked today, not
built: it leans on the open privacy ruling (#p10 of the asks
transcript: traces are private; whether ask text and trace context may
leave the device to a third-party model is ash's open call). Today's
guesser context is deliberately narrower — birthplace tool, selection,
ask history — which existing machinery already carries.

**Every y/n/edit is a labeled example** — the confirms are the
guessers' contact reports, and they land in the same ledger. The two
areas are one loop: guess → contact → correction → better guesser.

### what got built on it (same day)

Doctrine: hybrid.md gained the intake section (request object,
ambiguity test, silence default), the brief's problem and estimate
lines, the preamble's tripwire and contact report, the review's depth
check and replan path; agents.md's never-ask bullet sharpened to admit
the one legitimate question. `misses.md` opened with two retrospective
entries — the feature-untick ladder (the tripwire would have fired at
rung 1; the lesson: "X should just work" is a foundation ask) and the
two squares (the lesson: an unwritable in-hand line is the signal to
ask). Code: the did-you-mean node under `/lifecycle` — the question
status, the option chips, the answer event, the stamp tool's question
mode — built through the hybrid pipeline itself; its record is in the
node and the session transcript.

### the third language (#p25–#p29, landed the same evening)

Ash asked for an aesthetic standard "so visual stuff doesn't have to
keep being re-litigated" (#p25); nine principles were extracted from
the shipped surface and reviewed live. Then the reframe (#p26): the
standard is *a per-user feature that affects agents only* — and the
recall (#p27) of the 2026-08-15 skillset design ("the builder is a
feature-modular skillset", fm-spec-2 #p21). Ash's ruling (#p29) cut
the design smaller than the Aug 15 sketch: no slot vocabulary, no
skeleton — **agent instructions are simply the tree's third language**,
`<name>.agent.md` beside `.rs` and `.js`, governing build-agent AND
future in-app agent behaviour, assembled per product like everything
else.

Landed: fmlink collects `.agent.md` from included nodes and emits
`products/<product>/build/skillset.md`, provenance-ordered,
provenance-commented, toggle-obeying (a node carrying only agent
instructions counts as contributing and needs a real anchor). Three
fragments prove it: `/taste` — the first agent-only node in the tree's
history, the nine principles as its entire implementation, under
`/shell` (which is now genuinely at the six-child cap) — plus
`/did-you-mean` carrying the ambiguity-test discretion and
`/attention` carrying the nothing-rings-about-nothing rule, each
instruction living with the node it governs, exactly as the Aug 15
entry predicted. Toggle proven: untick `/taste` and the standard
leaves the skillset. Consumers wired: agents.md 4a judges against the
composed skillset; the brief template points workers at it; CLAUDE.md
says to read it at session start.

Deliberately not built (foundations named, not owed): per-user
selection of agent instructions (the world-object makes it possible —
overlay chains over doctrine); exchange-by-consent of agent features
(#p13's design applies unchanged); the decomposition of agents.md and
hybrid.md into the tree (the monolith-to-fragments move, index.html's
precedent). The old "open for ruling" items from Aug 15 — slot
vocabulary, whether the session loads skillset in place of agents.md —
are half-answered: no slots (ruled #p29), and the session loads it
*alongside* agents.md until the decomposition happens.

Then ash, reading the ship report, stated the rule the delivery was
missing (#p17–#p19): a builder message reaches you in place if the
panel is open, as a gentle lozenge pulse if the app is foreground, as
a notification if backgrounded — and nothing rings about nothing. That
shipped the same afternoon as `/to-owner` (the relay's audience is the
edited world's owner — the fix that makes "panel open → it updates"
true for bench stamps at all) and `/attention` (the three-channel
rule, the sw-side foreground fork, targeted push). The worker's three
map corrections became misses.md's third entry — the ledger fed by
the pipeline it governs, on its first day.

## ideas parking lot

Superseded — passing whims now live in `ideas.md` at the repo root.

## the auth red-team, and what it became (2026-08-23, #p3)

Ash asked for a feeling about how secure the login scheme was, then to fix
everything found. The crux the read surfaced: the crypto is sound (HMAC token,
correct WebAuthn), so nobody forges their way in — they get in by **network
position, one readable file, or a taken phone**, and once in there was almost
no authorization layer. That shaped the fixes (all shipped, build 261):

- The real trust boundary was `gate.rs`'s `if !r.tunnel` — network position,
  not identity — over a `0.0.0.0` bind. Binding loopback makes the kernel
  enforce "not-tunnel = same host". The single highest-value fix.
- The signing secret + `users.json` were the whole kingdom (forge-anyone +
  the confidential supporter list). Perms are the mitigation the code can
  reach; the file being the game is inherent to good crypto. Secret rotated on
  the mini (had been 0644 since Aug 15).
- Sessions were un-revocable one-year bearers. Issued-at + a revoked-before
  epoch + a guest-list recheck in `token_valid` gives both a per-person cutoff
  (delist) and a mass one (epoch), without discarding the key.

**Deliberate non-fixes, recorded so they aren't mistaken for oversights:**
the WebAuthn signature counter (Apple passkeys are iCloud-synced, always 0 —
clone detection would be dead weight); the `auth/request` *timing* side channel
(the blatant name+403 oracle is closed; a send-takes-longer timing difference
remains); the push `/tmp` file (it's RFC-8291 ciphertext, not plaintext — the
red-team note was wrong on that).

**The authz foundation, and the honest gap.** `/authority` grades the one
enforcement point that existed (`ctx_may_write_layer`) into member/support/admin
beside the guest list. It is NOT the full sketched model (notes.md 966-984:
authority as reachable subtrees, "enactment requires authority ⊇ blast radius")
— that needs enactment machinery the tree doesn't have, and is the next rung.
What shipped is the authority datum + a graded gate + `may_write_shared` as the
first, coarsest blast-radius test (own-world vs shared). Built on this, the
subtree model is a data-and-check extension, not a rewrite.

`users` hit the 6-child cap doing this (harden, authority added) — next child
regroups.

## cards: the universal object, owned and exchanged (2026-08-25, accounts #p7–#p12)

Tara (the customer) visits 2026-08-26; ash chose to spend the day on user
accounts. The morning's critique had named the gaps: two guest-list entries
(ash + `_test`), adding a person means SSH, the 👤 tool an empty placeholder.
The conversation moved the design three times in twenty minutes, each move
smaller and more general than the last:

1. **The proposal (#p7):** a user page — name, picture, mission statement,
   all editable — then *projects*: "something we're trying to get done", from
   "book a trip to China" to "campaign for MP in 2029"; "person P has role R
   in project X", stored and interrogable; projects filter posts and tools.
   Triage's first read: the page is the first *public* record and cannot live
   in the private world-object, so it wants a server-side store; projects are
   the post store arriving — estimate the foundation, not the feature.

2. **Users own their data and exchange it (#p8).** Ash's redirect, and it
   dissolved the "public store": a profile lives in its owner's world like
   every other var; other people see it because the owner *sent a copy*.
   A directory is the cards you've been handed; a link is a mutual exchange;
   a project is somebody's card, its members the accept-set. Consequences:
   no new store, the exchange primitive is `share`'s mechanism arriving
   early, unwanted contact is structurally impossible, and the campaign's
   special-category-data story is one sentence ("only what you were given").
   Honest caveat recorded: the server still holds every world; ownership
   means sole write + copies-by-consent + withdrawable, not a blind server.

3. **The card (#p9–#p11).** One object for post, profile, group, project,
   recording: `{id, owner, type, created, edited, blocks[], links[]}`. `type`
   is a field, never a subclass (#p10 — "profile" is a value). Cards are 1–5
   phone screens: a page, not a row — so a card has two renderings, *tile*
   (grid/list thumbnail, the `.crow` form) and *page*. Blocks are text or
   blob references; the dictaphone's grid of recordings (audio + transcript,
   IndexedDB + `/mirror`) is the standing prototype of both the tile grid
   and the blob path (#p11) — it migrates by the absorption ladder later,
   not now.

**Placement:** `loop/cards` (loop's sixth child — cards are state + render,
the loop's business; no root regroup needed today) with `cards/me` taking
the 👤 tool's open seam. Built through the hybrid pipeline; brief in the
session scratchpad, worker in a worktree. Parked and named: exchange /
send-card, the people list, links, projects, dictate's migration,
per-card merge (the `cards` list is last-write whole today, like `asks`).

**Open, ash's:** whether a received card needs an accept tap (triage's
default: no — only guest-list members can send); who may create a project
(default anyone); who asserts "P has role R in X" (default: either side,
no confirmation until `link`). Tara's guest-list entry and role (support
or admin) still wanted before tomorrow.

**Seed data, ash's (2026-08-25 #p14):** "miso" is a project — this project —
and ash is *lead dev* of miso. So the first project card is `miso` (type
`project`) and the first link is `ash.profile —role: lead dev→ miso.project`;
Tara's will be `tara.profile —role: candidate→ sevenoaks-2029.project`. The
`projects`/`link` brief should build exactly these as its worked example and
the 👤 page should read "lead dev of miso" under the mission.
## the apply-wrapper race: `/account`'s watch is orphaned on today's build

*Found 2026-08-25 while proving `/me`'s toggle (transcripts/2026-08-25-accounts.md#p10).
Filed here because it belongs to nobody this brief built.*

Several page fragments extend `feature_Loop.apply` from inside an `init()` that
a 100ms `setInterval` calls once the loop has state — `/account`, `/phone`,
`/mirror`, and now `/me`. They are supposed to chain: each captures the current
`apply` and installs its own, so the last one installed is outermost and calls
the rest.

They do not chain. On this build the outermost wrapper is `/phone`'s or
`/mirror`'s — it varies between page loads — and `/account`'s link is not in the
chain at all: `feature_Account.wasOpen` stays `true` for the life of the page,
`watch()` never runs again after the first paint, and **tapping 👤 does not open
or close the system panel**. Reproduced at the tree baseline with `/cards` and
`/me` both unticked and `account.js` untouched, so it predates both.

Two things make it possible: the guard (`feature_Loop.state !== null`) is not
the same instant for every fragment, and a fragment that captures `apply` on an
earlier tick than another fragment installs on will drop that fragment's link
when it installs. It is the JS half of the problem chains solve in Rust, and it
wants the same answer: one place that owns the extension, rather than four
copies of a capture-and-replace idiom racing each other.

`/me` is not affected — it is the newest index fragment, so it installs last and
is outermost by construction — which is exactly why the bug was invisible until
its toggle was proven. Anything else that wraps `apply` from a timer is a
coin-toss.

### the answer's first half: a named seam instead of a wrapper (2026-08-25, #p20)

`/keep` needed to see the repaint, and took `feature_Loop.paint(html)` — a new
one-line extension point in `loop.js` whose default is what `apply` used to do
inline — by **replacing the property at load**, the way `/me` replaces
`feature_Account.openTool`. Load-time replacement of a named seam cannot race:
there is no window in which two fragments both hold the old value, because the
value they capture is the file's own, not another timer's guess.

That is the shape the note above is asking for, one seam at a time. `apply`
itself still has four timer-installed wrappers and still coin-tosses; the fix
for those is the same move — give the thing they want a name, and let them
replace it at load.

### a repaint DOES fire focusout, and that is a hazard, not a safety net

Measured while building `/keep` (`t6-focusout-probe`): Chrome fires `focusout`
synchronously when `$('app').innerHTML = …` destroys the focused
contenteditable. So `/cards`' focusout save runs from **inside** a repaint and
re-enters `apply` — a nested `innerHTML` assignment to the same node, once per
repaint, whose result the outer assignment then overwrites. It has been doing
this since `/cards` shipped; it is why the words usually survive today and the
caret never does. `/keep` swallows its own repaint's focusout in the capture
phase and sends the draft itself when the block does not come back. Anything
else that saves on focusout should know that a repaint looks exactly like a
tap-away unless it asks.

## the accounts day, in one paragraph (2026-08-25, end of session)

Five workers, four direct fixes, builds 261→279, all pushed. The card
object exists and 👤 is your own page; invite works from the app; two
field asks came off the phone, one through a live did-you-mean, and both
shipped stamped. Two rulings from live misbehaviour became nodes the same
hour (`fresh-words`, `present`). The ceiling to watch is the whole-list
`cards` var (one op per edit, 56KB): var-per-card + a blob path is now the
rung before projects. Tomorrow's first act is a phone: pinch the frame.

## the lost card, and the guard (2026-08-25, #p47–#p48)

Build 292 dropped ash's profile picture and mission: the update reloaded
the page during the server restart, `/veil`'s join timed out (which also
sets `fm-joined`), `/me`'s ensure ran against an empty world and made a
blank card, and the `cards` var's last-write merge sent that one-card list
over the real one — the failure `me.md` had described and parked. Recovered
from the op log (line 91 → the diag door). Fixed structurally the same
hour: `/guard` extends `handle_msg` ahead of `/converge` and merges every
`cards` set into what the server holds — union by id, newer `edited` per
card, blank profile duplicates discarded — so **a set cannot delete a
card** (deletion, when it comes, is its own op); `/me/patient` makes the
ensure wait for a real join (`feature_Veil.joined`, not the timeout's
class) or do nothing. misses.md carries the lesson: a documented way to
lose user data is a defect, never a residual. Also today: `/glyphs`, the
fourth agent-instruction node (toolbar icons are ink — filtered emoji or
drawn SVG — never an emoji-presentation character), after the undo arrow
shipped as a colour bitmap on iOS while a desktop rig called it black.

## a regroup CAN rewire behaviour, and now cannot (2026-08-25, #p54)

The holding/changing regroup that made room for `/world-cache` produced a
`--chains` diff, which is supposed to be impossible: `dictate/mirror/adopt`
moved through `converge`'s `handle_msg` links. The cause is proposal 9's
tie-break rather than proposal 9 itself. Nodes citing the SAME prompt have
identical provenance keys — twelve of them cite hybrid #p32, the "keep going
until all rungs are built" ladder — and fmlink broke those ties by
`(depth, path)`. Pushing six nodes one level down therefore moved every one of
them past every tied node in the rest of the tree.

The fix is `tie_break()`: ties resolve by how deep a node is in **contributing**
nodes and by its path **with code-free grouping ancestors removed**. A grouping
node contributes nothing to the composition, so inserting or dissolving one is
now order-neutral by construction rather than by luck — including for
yesterday's `users/login` and `cards/page` regroups, which the new rule
retroactively neutralises (both moved a tied node; neither happened to move one
that shared a chain with it). Chains before and after the regroup are identical,
modulo paths.

The residue worth remembering: **a regroup is free for chains and not free for
stored state.** Vars are addressed by node path, in `/remember`'s op log and now
in the device's cache, so moving a node that DECLARES a var orphans that var's
stored value. Nothing declared a var inside this regroup, so nothing was lost —
but `context`'s open question about versioning across builds now has a second
reason to exist, and the answer probably has to be a rename map that a moved
node carries.

## 2026-08-25 — what the handover leaned on, and what it found there

`serve/reuseport` + `/handover` (accounts #p54) rest on one invariant, and it
is worth writing down because it is now load-bearing rather than advisory:
**two servers may be bound to the port, but only one may ever be accepting.**

The reason is `context_log_append` in `remember.lib.rs`. It is not an append —
it reads the whole log, pushes one record, and rewrites the file through a
temp and a rename. Two processes doing that concurrently lose each other's
records outright, compaction or no compaction. `/sole-tenant`'s refusal is
therefore the *only* thing standing between a second process and silent data
loss, which is why the handover sequences that refusal (evict, wait for the
pid to actually be gone, then claim) rather than relaxing it. The rig proves
the property it can prove — 100 card edits straddling two handovers, 100
records in the log, none missing — but the proof is "the writers never
overlapped", not "overlapping writers are safe".

The foundation this wants, when something needs genuinely concurrent writers
(a second place, a read replica, a hot-swap that keeps both halves live): a
cross-process lock — `flock` on the log, or an O_APPEND single-line append with
compaction behind that lock. It belongs to `/remember`, not to `serve`.

Two smaller things found in passing, neither fixed:

- `/tmp/miso-broadcast.json` is a fixed path with no `MISO_CONTEXT_DIR`
  equivalent, so every server on one machine shares one broadcast slot —
  including two workers' rigs, which is a real source of confusing rig
  results. The handover *wants* the two processes to share it; a rig does not
  want to share with a stranger.
- `/serve`'s port is now a named seam (`serve_port`), which is what the
  next-work item about rig ports asked for. It is still a constant: making it
  read an environment variable would be a behaviour change and wants its own
  node.

## seeing each other: exchange, rung one (2026-08-25, #p69–#p71)

The alice/bob walkthrough (pretend people, admin-invited, logged in via
the mini's log; #p63–#p68) proved the invite loop end to end with no SSH,
and surfaced the next rung in one sentence: alice and bob each have a card
and cannot see each other's, or ash's, or the `miso` project. Ash's ruling
(#p71): **invite makes people visible to each other automatically**; the
other visibility cue is **shared membership of a project — later**. Rung
one, briefed: a per-person inbox on the server; *send a card to a person*
(by number, or from the people you hold); received cards land in your own
`cards` var as copies (foreign ids, `from`, `received`) and appear in the
cards tool; a copy stays fresh when its owner edits (re-offer to the
sent-to set); invite seeds both directions; other people's cards are
read-only; no accept tap, no directory beyond the cards you hold. Parked:
withdrawing a card, an accept tap, links, rings, project membership.

## the people surface, and two things it left behind (2026-08-25, #p76)

`/people` re-aims `/browse`'s surface at 👤 and retires the cards tool. Two
things worth carrying forward:

- **Which tool is open is read two ways, and they can disagree.** Most render
  links ask the live context (`open_tool_read()`); `/me` and `/under-account`
  read `open_tool` out of the loop *state string* they are handed. `/people`
  exploits that: it hands the chain beneath a copy of the state with
  `open_tool` cleared so `/me` does not draw its own-card page as 👤's landing
  surface, and puts the live value back in `tool_controls` so the invite plus
  is unaffected. It works and is proven, but it is a contract rather than a
  seam: **the honest form is a seam in `/me`** — "am I the landing surface?",
  two lines, default true — and until it exists, a new node that decides what
  to render by testing the state string for `account` must be told about the
  mute.
- **The focusout repaint eats the very next tap.** Type in a card block, then
  tap a toolbar button: the `focusout` fires first, `/cards` sends `CardEdit`,
  the loop repaints `#app`, the button element is replaced between mousedown
  and mouseup, and no `click` ever fires. The second tap works. Rig-proved to
  **predate `/people`** — it reproduces on the cards tool with `/people`
  unticked — so it is `/cards`' or `/loop`'s to fix, not this node's. The
  cheap fix is for `/cards`' focusout to send nothing when the text did not
  change (`/keep` has usually saved it 600ms earlier already); the general one
  is for the loop's delegated click to survive its own repaint.

## the map fitted without a tweak: the skillset's first clear win (2026-08-26, #p99)

Ash: "particularly loved how the map fitted our aesthetic without any
tweaks — a sign that the feature-modular agent stuff is working." What
happened: the `/map` worker read `/taste` from the composed skillset,
and made two calls nobody briefed — it chose CARTO's dark render of
OpenStreetMap over plain OSM rather than filter bright tiles into the
house ground (principle 9, and the exact move that got the 2026-08-16 map
withdrawn), and it caught Leaflet's white attribution pill by computed
style because principle 1 says nothing arrives white. The instruction
was a node; it composed; the worker obeyed it as it would code. That is
the third language paying for itself, and the argument for moving more
of agents.md/hybrid.md into the tree.

## two clocks in provenance order (2026-08-26, found building `/post-time`)

`node_key` reads a transcript anchor's timestamp from the line
`*YYYY-MM-DD HH:MM*` that `export_transcript.py` writes, and that line comes
from the session log's ISO timestamp — **UTC**. A field ask's anchor is
`asks#<ms>`, and fmlink turns it into a stamp with
`datetime.fromtimestamp(ms/1000)` — **local**. In August that is an hour
apart, so a prompt and an ask a minute apart in real time can linearise an
hour out of order.

Caught, not suffered: `/post-time` cites #p103 (01:12 BST, written into the
transcript as 00:12) and composes *before* three asks filed at 01:02–01:10.
Its links touch none of the same chains, so the composed behaviour is
identical either way — but the next collision may not be so lucky, and it
would present as a chain silently in the wrong order rather than as an error.

The fix is one line in one of the two readers — either export the transcript
stamp in local time, or read the ask's ms as UTC — and it reorders nodes
across the whole tree, so it wants a `--chains` diff and a deliberate run of
its own rather than a drive-by.

## vocabulary ruling: "extensible function", not "seam" (2026-09-01, saturday #p2ff)

Ash: from now on say **extensible function** for a joining point a feature
leaves for others to plug into, and **function extension** for the code that
plugs in. "Seam" is retired in conversation, briefs and new spec text; older
documents keep their wording until rewritten anyway.

## proposal: the toggle proof is implied by a confined diff (2026-09-02, agent)

Ash asked whether the enable/disable check (agents.md step 4) could be
skipped by enforcing patterns in the work. It can, exactly, for one
pattern.

The linker never reads an unticked node's files, so the composition
without a node is a function of every file *outside* the node. If a
commit's diff is confined to the new node's directory plus one added,
ticked line in the parent's `order.md`, then the change cannot alter the
composition without the node — the untick cannot observe it — and for a
new node that composition is byte-identical to the previous commit's,
which was built, smoke-gated and shipped (ship-as-built). Code outside
the node, a back-reference from an older file, a build that breaks
without the node: each needs an edit outside the node, which the pattern
forbids. The toggle proof re-proves the last release. (Sharpened while
building: an order.md *inside* the node's subtree leaves with the node
and counts as inside; the argument covers a modified existing node too,
since the change is invisible to the untick either way.)

Where the pattern does not fit — a parent refactored to open an
extension point, a refinement that touched the parent, a regroup, a
sibling unticked — the full toggle proof stays mandatory; those are the
commits it was written for.

Enforcement, if adopted: (1) deploy.sh already computes the node dirs a
commit touched and the specs it added — classify each commit as
conforming or not, and refuse a release whose non-conforming commit
carries no proof record; (2) `fmlink.py <product> --prove <node>` runs
the same classification from the working tree so a worker knows in a
second which kind of commit it is making; (3) agents.md step 4 becomes
"fit the pattern, or prove the toggle". Premises to check in the gate:
the parent commit is main's tip (stale worktree base), and the previous
commit was released (the deploy stamp names it).

Saves two links and a flow rerun per node, and retires the transient
untick of a shared order.md (the "restore in the same breath" hazard).
Removes nothing from the ticked-state proof.

## the learning loop (2026-09-03, housekeeping #p31–#p32)

Ash asked for a self-improvement process: look at how asks were refined
after shipping, so the next build is shaped the way the asker likes by
default. Decision: the tree already holds the data — every node carries its
ask, and a refinement is a child born after its parent — so `tools/tweaks.py`
walks it (git dates each node) and prints the digest over all history; the
distillation is a builder's act at session end, written into
`taste/learned/learned.agent.md`, which the skillset carries to every
builder. First run: 169 refinements of 76 asks, one asker; thirteen defaults
extracted, each with its precedent nodes. A rule lives while its precedent
holds; an ask that contradicts it amends it. Per-asker rules are the next
rung once a second asker files tweaks (`--user`).


## feature flow (ruled 2026-09-03, invite-test #p157)

Two users asked for features on the same evening (ash as admin, ash as
Tara), which raised how features flow from one person to another. The
discussion (invite-test #p157–#p157): the project ladder — admin → candidate
→ team → volunteer → supporter — is already a following graph, and a
feature could ship to its asker and flow down the ladder, with adoption
above; but the build spend is the reason everyone cannot build whatever
they want — anyone may ask, and the person paying for tokens decides what
gets built.

**The ruling, for now:** an ask from anyone who is not admin or support is
stamped `proposed` (tools/ask_ack.py reads the asker's authority from the
guest list); ash accepts and prioritises proposals by hand, they are built
in a batch, and everyone gets them. A person can switch a feature off in
the chooser. Later, chosen but not built: the proposal queue with seconds
("wanting is cheap"), the payer's sheet with the budget beside it, and the
flow down the ladder with the asker's rank as the feature's floor.

## the audience floor, ruled (2026-09-04, morning before the field walk)

Ash's rulings, taken in conversation: his own test posts stay admin-only;
volunteers, once they sign up, see only campaign posts made by Tara, the
leader; a newcomer sees only posts promoted to their level; the promote
workflow is what today's walk looks at. This is what `/audience` already
does — a post is stamped at its author's grade, promote lowers the floor
one rung and only the author may, a reader sees floors at or below their
grade — so no node was cut. Test users are reset before the real session
(`tools/reset_user.py`); an ethernet cable for the mini is being bought.
