# handover
*state of play for the next session — written 2026-08-15, end of day 4
(transcripts/2026-08-15-fm-spec.md, 46 prompts, builds 120→155). Discipline
in `agents.md`; ops in `deploy.md`; this file is only what's current.*

## THE RENAME (#p50, post-pizza): muon is now MISO ("make it so")

Everywhere: tree root features/miso, product products/miso (override
structure rebuilt), crate miso_server, cookie miso_auth (devices
re-login once), localStorage misoX keys (one-time migration shim in the
shell skeleton copies muonX ->), IDB miso-blobs (legacy-origin local
recordings orphaned — they live on the mini via mirror; named cost),
mini dirs ~/miso, ~/.miso-auth, /tmp/miso-vars, /tmp/miso-broadcast,
agent com.noob.miso, log /tmp/miso.log. Old state dirs KEPT as backups.
**miso.nøøb.org is canonical; muon.nøøb.org still serves as a legacy
alias** (installed devices keep working; retire whenever). Provenance
quotes keep "muon" verbatim — history said what it said. Loose ends: a
stray doubled CNAME (miso.xn--nb-lkaa.org.xn--nb-lkaa.org) deletable
only in the Cloudflare dashboard; a pre-existing com.noob.miso-auditor
agent on the mini (not ours, untouched); laptop dev auth copied to
~/.miso-auth.

## where things stand

Live: **build 155** at miso.nøøb.org, 102 nodes. The day in one line: the
update pipeline became one-OK-and-deltas, miso grew its own WebGPU compute
substrate and an 8MB semantic find, and then **the loop closed** — five
field asks travelled phone → proposal → build → awaiting-update → phone,
including one removal. The app modifies its own tools from inside.

The arcs, in order:
- **the update ladder** (#p2, morning): `review` now has six children and
  sits AT the cap — `consent-once` (acceptance is the ONLY key; auto's
  self-apply stood down by redefinition), `upgrade` (additions badged
  `new`, pre-ticked by policy, DRAFT ticks committed only on the button),
  `seamless` (busy tasks defer the apply; whole-state stash/rehydrate),
  `delta` (deploy ships `hashes.json`; evict only the diff; no-code delta
  = quiet apply, no reload), `patch` (wasm-only delta hot-swaps the module
  live, state untouched), `live-panel` (an open panel re-renders when news
  arrives or a quiet apply lands).
- **minimal updates** (#p6): fmlink `SPLIT_PAGES` emits index.html's js/css
  as per-feature files under `f/` (85KB → 5KB skeleton; f/ swept each
  link); serve.rs learned `text/css` (browsers discard mistyped
  stylesheets); gate hotfix: `f/` + `hashes.json` joined the public shell
  list — logged-out visitors were briefly frozen (build 130, fixed 131).
- **miso computes for itself** (#p12, doctrine in ash's words):
  `loop/compute` — ~90 lines of page JS driving WebGPU, no ort/burn/
  bindgen, adapter-clamped limits from birth (haze's recipe), proof kernel
  0.7ms warm. First tenant `semantic-find`: potion-base-8M as int8 table
  (7.5MB, fetch_find.py pins it), WordPiece+mean embedder mirrored in
  Python (tools/potion_embed.py) and JS with **measured parity 5e-7**;
  deploy embeds the catalog (embed_catalog.py → features/vectors.json,
  QUOTED SPANS STRIPPED — spec examples were outranking their subjects);
  the device embeds only queries; GPU cosine with CPU fallback (CPU wins
  at 87 entries — the kernel is ceremonial by design). Feature-modular
  WGSL (chains in shaders) is NAMED in compute.md, to build when the
  speech pipeline arrives.
- **the ask pipeline** (#p27 war-game → #p30 go): `ask/birthplace` (asks
  carry `tool` + `at`), `semantic-find/context-bias` (+0.08 for the open
  tool's family), `ask/propose` (editable draft = the ask verbatim, #p33;
  OK fires {text, proposal, context} through the outbox — offline = queued
  fire), `ask/lifecycle` + `being-built` (requests ride the feature list:
  status pill in the number slot, tap-to-expand, headerless #p39),
  `ask/open-chip` + `tools-first` (results ARE tools: open chip + the
  registering feature's readout; bystanders drop; no-tool asks keep the
  reading path). Tool ground truth: export_features stamps `tool:` per
  registering node; `tools_catalog` state var replaced DOM-scraping (the
  toolbar only renders the open tool in open mode — view, not truth).
- **THE LOOP CLOSED**: reset-taps (#p27→field ask, toolbar sub-tool via
  tool_controls after the #p32 correction), double-taps (#p33's own
  example, asked for in earnest), decrement-taps (#p40 — **first node
  whose founding quote is the field ask itself**; the event got its own
  transcript anchor), decrement REMOVED by ask (= product override; see
  small print for the structure), updates picker tucked behind the
  features button (#p44a; #p81 law kept via an owned container). Builder
  ritual: stamp `building` (live to the panel via the broadcast file) →
  build → deploy → stamp `shipped --build N`.
- **panel calm** (#p14/#p15): ask box first, build line under it,
  awaiting/building/requests, policy (now tucked), features button
  folding the 100-row list (fold surrenders the height budget), who+logout
  sharing the last row, panel top-tied BELOW the nøøb button's row with a
  height bound. Plus `chooser/build-order` (list strictly descending by
  shown build) and `tools/steady` (toolbar slide plays only on mode
  change).

## NEXT SESSION (ash's pick: whisper on webgpu first)

0. **IN THE INBOX, status proposed** (arrived as the handover was being
   written, t=1786816107134): "A long press on a tool button should pop
   up a tooltip with user documentation" — no birthplace tool (filed
   from the launcher). Stamp `building` on pickup; the tooltip content
   is ready-made (tree.json intros; the registering node's `## user`
   para); mind iOS long-press vs the click-delegation in loop.js.

1. **Whisper on the substrate** — the climb begins. Two probes, cheapest
   first: (a) the **ort shim experiment**: engine.js wraps
   `GPUAdapter.prototype.requestDevice` clamping requiredLimits to
   adapter.limits (haze's recipe; catches BOTH pinned bundles), clear the
   `localStorage.misoSttDevice` failure memo, sim then device, watching
   for onnxruntime **#26827** (WebKit-26 jsep: 400% CPU / 1-14GB after
   inference — if it bites, ort is off the table); (b) the **sovereign
   path**: mel spectrogram → matmul tiles → attention WGSL kernels on
   `/compute`, feature-modular WGSL landing with it. notes.md has the
   T1–T3 map and the 10-15x / 20-60 tok/s numbers. The prize pays twice:
   whisper ~5-10x AND the FunctionGemma call-rung door.
2. **THE FLYWHEEL** (#p47a — ash: "a flywheel that's always on, that
   creates subagents to build requests in parallel"). Half exists (the
   1s monitor intake, stamp_ask status channel, the codified ritual).
   The design, two stages:
   - **In-session parallelism (adopt immediately)**: on each NEW
     PROPOSED event, spawn a fork subagent in a WORKTREE (forks inherit
     the whole discipline context); it runs the five-step loop on its
     ask; the main session serialises integration — merge, single
     deploy, stamp shipped. Parallel where subtrees don't collide;
     serial at the deploy (one mini, one build line). Watch: two asks
     targeting the same parent's order.md collide — integrate resolves.
   - **Always-on (the real flywheel)**: a mini-resident builder. The
     `claude` CLI is already installed there (2.1.56, not logged in);
     the sanctioned route is an API key (`--bare -p`, per-token, ~$1-5/
     month at this scale — subscription programmatic use is against
     policy, researched #p31). A watcher on the mini invokes it per
     ask; the human session becomes reviewer, not builder. OPEN
     mechanism question first: provenance — field asks currently get
     anchors by landing in a session transcript (#p40); a headless
     flywheel needs the ask-store itself accepted as an anchor source
     by fmlink. That's a doctrine conversation with ash before code.

3. **Drafter upgrade**: dev-session agent writes the proposal paragraph
   when online (same textarea, better prose) — the seam is
   `feature_Propose.draft`.
3. **Lifecycle polish**: builder→user status channel is the broadcast
   file (works mid-session); asks store on-device still shows stale
   status until relaunch when stamps happen while an instance is closed.
   "!"/"?" states and ETA remain unbuilt (#p85).
4. **Still pending from day 3**: `dictate/server` (whisper.cpp on the
   mini, consent given in principle — confirm before installing) and
   persisting transcript stamps (RecList reseeds from IndexedDB, restarts
   re-transcribe).
5. **Regroup pressure**: review, panel, miso root, shell/update all at
   the 6-child cap; counter at 3 and growing by ask. The first regroup is
   itself a prompted event.

## tooling state

- **1s ask monitor** — re-arm each session (it dies with the session):
  persistent ssh to the mini, remote 1s loop over
  `/tmp/miso-vars/user.*.asks.json`, local dedupe vs
  scratchpad/asks_seen.txt, fires on status asked|proposed with tool/at/
  proposal in the event. See the Monitor call in this transcript (#p28ff).
- **tools/stamp_ask.py** `--text X --status building|shipped [--build N]
  [--local]` — updates the var store AND appends the per-user VarUpdate to
  `/tmp/miso-broadcast.json` (the server's own publish file) → open
  panels update in ~0.5s. Named risk: two writers, one file.
- **tools/fetch_find.py** — pins potion-base-8M → semantic-find/assets/
  find/ (gitignored except PINNED). **tools/potion_embed.py** — the
  Python twin embedder. **tools/embed_catalog.py** — runs in deploy after
  the tree export.
- **deploy.sh** additions: hashes.json (content hash per site file,
  data files excluded), changes.json now carries `paths` + `added` per
  build, prints unaddressed asks (asked|proposed) after shipping.
- **export_features.py**: stamps `tool:` on registering nodes; order.md
  no longer bumps a parent's build (#p41).
- CDP testing lessons, paid for: `Runtime.evaluate` calls share the
  global lexical scope — bare top-level `const` in one eval silently
  breaks all later evals (IIFE everything); the toolbar DOM is view not
  truth; restore reopens the last tool so blind toolbar clicks toggle
  wrong; headless Chrome needs `--enable-unsafe-webgpu --use-angle=metal`
  for the compute tests.

## today's doctrine additions (all in notes.md, day-4 entries)

- **One OK always** (auto-vs-review resolved); policy now means
  "what the review pre-ticks"; fixes-auto vs ask-me currently identical —
  picker may want to become two-way, ash to rule.
- **Draft ticks are drafts**: the live ftick event is a store-toggle; a
  visual overlay would invert intent — commit on the button only.
- **Miso computes for itself** (#p12): WGSL kernels as node assets, thin
  JS driver, no dependencies; the zero-import law untouched because the
  engine never enters client.wasm.
- **Specs are data**: quoted example phrases embed into the catalog and
  magnetise their own documentation — stripped at embed time (learned
  twice).
- **A parent doesn't age when it gains a child** (order.md excluded from
  build stamps) and **an update never lists nothing** (release-line rows
  for scaffolding builds).
- **In the panel, overrides must carry the #panel id** or silently lose
  the cascade (paid for twice in one hour).
- **THE LOOP CLOSED** entry: what the magic decomposes into.

## small print

- **products/miso/miso is no longer a symlink**: it's a real-dir override
  path (real dirs miso→loop→tap→counter with glob-symlinked siblings and
  a local counter/order.md unticking decrement-taps). The old single
  symlink meant writing "product order.md" wrote THROUGH into the shared
  tree — paid for at #p44. Unticked children may simply be absent from
  the local dir. hello_only remains the small exemplar.
- Client localStorage keys grown today: misoAccepted (acceptance mirror),
  misoHashes (delta baseline — seeded only when running==server),
  misoStash (seamless, consumed once). caches.delete('miso') no longer
  happens on updates (delta evicts precisely) — the STT model survives.
- Dev server on 8095 runs the build-155 binary; _test login via PIN from
  /tmp/fm2-devserver.log; local traffic ungated. Headless Chrome may
  still be running on 9222 (scratchpad profile).
- STT gaps unchanged from day 3 (silent 130MB fetch, >30s truncation, no
  transcript mirroring); ort-webgpu blocked on the shim experiment +
  #26827; localStorage.misoSttDevice memoizes the wasm fallback.
- asks var grows unboundedly (shipped entries never pruned); fine at
  current scale, a lifecycle-archive rung eventually.
- find/* is gated (data, not shell) — correct while ask sits behind
  login; revisit if ask ever goes public.
- ideas.md: ember palettes (spent), panel reorder (#p14 — BUILT same
  day). notes.md hygiene items #9/#10 still open (stale asset sweep
  beyond f/, stt cache-first sw rule).
