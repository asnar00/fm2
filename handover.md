# handover
*state of play for the next session — written late 2026-08-15, end of the
evening session (transcripts/2026-08-15-fm-spec-2.md, 22 prompts, builds
162→175). Discipline in `agents.md`; ops in `deploy.md`; this file is only
what's current.*

## THE HEADLINE: the loop ran all evening

Seven field asks travelled phone → inbox → node → build → phone in one
sitting, each one stamped building/shipped live to the panel:

- **decrement restored** (165) — "subtract 1 if >0" arrived by ask; the
  removal had been a product override, so the restore was the symmetric
  re-tick + symlink in products/miso. Selection changes stay nodeless.
- **sub-tool-cards** (166) — long-press cards extend to control buttons
  (reset, ×2, −1, record). Ground truth: `export_features.py` now stamps
  `subtools` (the control's data-ev ids) per registering node — the
  sub-tool twin of the `tool:` stamp. The node wraps
  `feature_LongPress.contentFor` and arms the parent's own timer state.
- **honest 👤 tooltip** (167) — the card was right, the account node's
  `## user` para was stale (pre-noob-button prose). Doc repair in place
  with a cited revision note; no node (code-free subfeatures are illegal).
- **fresh-catalog** (168) — 167 shipped as a data-only delta = quiet
  apply = no reload, and the chooser's memoized catalog outlived it (ash
  hit this live: "still showing the old tooltip"). New chooser subfeature
  wraps `feature_Delta.quiet`, forgets flat/byPath before live-panel
  re-renders. The wasm `patch` path shares quiet's ending, so it's covered.
- **bigger-buttons** (170) — +25%: 40→50px squares, 19→24px icons, back
  chevron in proportion. Pure CSS override node under shell/tools.
- **miso-button** (173) — the ask button says **miso**: type a wish,
  press the name, make it so.
- **request-box** (174, churned to 175) — the placeholder became
  "request", then same-evening wordsmithing landed on **"do something"**
  (revised in place; two-phase lifecycle blesses draft churn). Box and
  button now read *do something → miso*.

Plus one typed design ask that became a node:

- **auto-export** (169, serve/features) — "change a node's text file and
  it auto-updates everywhere". Server half: any `features/*` request
  compares newest source mtime against the baked tree.json and re-runs
  the export first (~4.5s, once per edit; mini has no sources so the
  deploy bake stays the truth there). Device half: export writes a
  `stamp` (tree.json content hash); the chooser's held catalog
  revalidates against it on every read — words reach phones without
  reload, apply, or deploy. PAID-FOR LESSON: route paths arrive
  slash-stripped (`clean_path`) — match `features/`, not `/features`.
  Named risks (both dev-only, self-healing next read): racing re-exports
  can 404 momentarily; threads can pair a mid-export tree with a
  mismatched stamp, deferring the refetch one read.

Live: **build 175** at miso.nøøb.org, 110 nodes. (171/172 were doc-only
commits — build = commit count.)

## THE DOCTRINE RUN (all in notes.md, all anchored, fm-spec-2)

The evening's asks kept generalising; five entries landed, converging:

1. **the ask–engineering gap + privilege** (#p12): text vs proposal is
   where translation lives; authority = subtree reach (blast radius ⊆
   asker's privilege); user records in ~/.miso-auth/users.json (the `_`
   prefix is already a privilege bit). Gates the headless flywheel.
2. **tunables** (#p17a): every naive ask is "make parameter X tunable at
   scope per-user/per-group/global". Var<T>, the var store, and the
   broadcast channel already exist; parameter-set asks could skip builds.
3. **the promotion rule** (#p18): a parameter earns its variable on the
   SECOND ask that touches it. First ask ships the literal constant; the
   declaration (name/type/default/scopes) is the one node; values are
   data forever after. bigger-buttons is the standing first case — the
   next size-shaped ask triggers its promotion.
4. **rule of two, surface side** (#p20): a tool earns its TOOLBAR slot on
   the second show of intent; until then it's usable from the ask
   surface (open-chip) and held in a "new tools" drawer. Naturally
   per-user — toolbar membership as a user-scoped selection var.
5. **the builder is a feature-modular skillset** (#p21): agent
   instructions become a composition language (`<name>.agent.md`
   fragments, skeleton + slots, provenance-ordered, toggleable).
   Tonight's entries are fragments-in-waiting for the nodes they govern;
   agents.md is the monolith the skeleton comes from (index.html before
   SPLIT_PAGES); fm.md stays constitution. The flywheel's headless
   builder would receive its composed skillset like devices receive
   index.html.

Entries 2–4 + per-user ticks all converge on ONE mechanism: the context
manager (rung 3 below).

**Rulings queued for ash**: where a tunable is declared (linker-read
stanza?); whose scope a repeat ask writes (asker's per-user vs global
default); what graduates a tool from the new-tools drawer (second ask vs
first real use); skillset fragment extension + slot vocabulary, and
whether the composed skillset replaces agents.md as what a session loads.

## NEXT SESSION

0. **Transcript mirroring** (#p22, field-confirmed): ash's first real
   test recording transcribed ON the iPhone (the on-device STT path
   works in the field!) but the transcript never reached the laptop —
   /mirror moves audio only, and the laptop reseeds from IndexedDB and
   re-transcribes. Shape: transcripts join the mirrored record
   (/mirror or /transcript subfeature); /phone's better-replaces-rough
   rule decides collisions. Also still open from day 3: persisting
   transcript stamps, silent 130MB model fetch, >30s truncation.
1. **Whisper on webgpu** — unchanged standing rung: (a) ort shim
   experiment (clamp requiredLimits to adapter.limits; clear
   localStorage.misoSttDevice; watch onnxruntime #26827), or (b) the
   sovereign path (mel → matmul → attention WGSL on /compute,
   feature-modular WGSL lands with it). notes.md has the T1–T3 map.
2. **THE FLYWHEEL** — in-session parallelism (fork subagents in
   worktrees per ask, main session serialises integrate/deploy/stamp)
   ready to adopt; always-on mini builder still blocked on the
   provenance ruling (ask-store as anchor source) AND now the privilege
   doctrine (#p12) — both doctrine-before-code.
3. **Per-user ticks / THE CONTEXT MANAGER** — the convergence point:
   ticks enforcement (untick gates live behaviour, ancestor patterns
   survive), tunables, and the new-tools drawer are one mechanism.
   `feature_ticks` is user-scoped, absent-means-on; `reflect()` already
   shades; enforcement is what's missing. `_test`/`_test2` machinery
   ready for the two-user divergence proof.
4. **Regroup pressure**: `ask` joined the at-cap list (open-chip,
   birthplace, propose, lifecycle, miso-button, request-box) — its next
   child forces a regroup; miso-button + request-box are natural
   "wording" group candidates. Also still at cap: review, panel, miso
   root, shell/update. counter at 4.
5. ideas.md gained: bigger-buttons → per-user "size" var (the promotion
   rule's first target, with the naive-asks lesson attached).

## tooling state

- **1s ask monitor** — re-arm each session (scratchpad is
  session-specific, so recreate `ask_filter.py`): persistent ssh to the
  mini streaming `/tmp/miso-vars/user.*.asks.json` once a second, local
  python dedupe on t:status against a seen-file, fires on
  asked|proposed. **Wrap the ssh in a `while true` reconnect loop** —
  lid-sleep kills the stream otherwise (learned twice tonight). An ask
  fires twice: `asked` (pre-OK, no birthplace) then `proposed` (OK'd,
  with tool/at) — act on `proposed`.
- **tools/stamp_ask.py** `--text X --status building|shipped [--build N]`
  — unchanged, used seven times tonight; open panels update in ~0.5s.
- **export_features.py**: now stamps `subtools` (control data-ev ids per
  tool_controls-bearing node) and writes `site/features/stamp`
  (tree.json content hash) after export.
- **export_transcript.py**: same-day sessions need distinct slugs (the
  collision guard refuses politely) — tonight is `fm-spec-2`.
- Dev server on 8095 runs the build-175 binary from products/miso/build
  (auto-export ACTIVE locally: editing features/** re-exports on the
  next /features request). Headless Chrome lives on 9222; CDP lessons
  from day 4 still apply (IIFE everything; toolbar renders only the
  open tool in open mode — go home before pressing launcher buttons;
  restore reopens the last tool).

## small print

- fresh-catalog + auto-export overlap benignly: quiet applies forget
  the catalog; every read revalidates the stamp anyway. Old builds
  without the stamp file 404 and behave as before (fetch guarded).
- request-box holds "do something" under its founding name — revised in
  place per the two-phase draft-churn rule, revision note cites #p17.
- The account node's ## user para now describes the placeholder truth;
  its second revision footnote cites the field ask (#p4a).
- asks var still grows unboundedly (11 entries, all shipped); the
  lifecycle-archive rung remains someday-material.
- notes.md hygiene items #9 (stale asset trees) and #10 (stt cache-first
  sw rule + download UX) still open; #10 now has field evidence.
