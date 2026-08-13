# notes
*working notes on fm — discussion, ideas, open questions. (fm.md remains the user-authored source of truth; this doc is co-written and freely editable.)*

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

## ideas parking lot

(empty — add freely)
