# handover
*state of play for the next session — written 2026-08-21 at the top of the
contexts ladder (transcripts/2026-08-21-hybrid.md, builds 182→215).
Discipline in `agents.md`; ops in `deploy.md`; the pipeline in `hybrid.md`;
the day's ledger in `redo.md` and notes.md.*

## THE HEADLINE: the rewind, the hybrid, and the ladder

Three things happened in one day, each built on the last:

1. **The rewind.** The Aug 16 session's code (Fable morning and Opus
   afternoon) was expunged on ash's ruling — main rewound to the Aug 15
   handover, archive on `archive/aug16-pre-rewind`, doctrine and records
   kept. The forensic model comparison that motivated it is in notes.md;
   `redo.md` is the ledger of what came back and how.
2. **The hybrid pipeline** (`hybrid.md`): Fable triage writes per-ask
   briefs, Opus workers build in isolated worktrees, Fable reviews
   against named acceptance evidence before integrate → deploy → stamp.
   Run all day: ~15 worker deliveries, zero review returns, one correct
   triage-return, two workers (the first retired honestly at its context
   budget after seven rungs). Persistent workers (one warm instance,
   briefs as follow-up messages) cut the per-rung cold-start.
3. **THE CONTEXTS LADDER — DONE.** Eleven rungs, builds 187–212, the
   full design in notes.md ("the world-object", "the absorption
   ladder"): `.vars` declarations → typed per-user `Context` worlds →
   the turn boundary → implicit `enabled` gates → merge-disciplined ops
   → persistence → the overlay chain → the migration (payload bridge +
   epoch counters) → the context join → **the chooser's ticks mean it
   and SyncVar is deleted**. The done sentence was proven nine stages
   on the real UI: untick a feature, it's off for you only, on all
   your devices, survives restarts, reaches new devices, and re-tick
   finds your state intact. ASH: the phone walkthrough is your summit
   review — untick something in the feature list and watch.

Live: **build 212** at miso.nøøb.org (local head 215+, tools-only
commits — next deploy carries them). 122 nodes, 123 vars per world.

## THE RESIDUALS CAMPAIGN — CLOSED (builds 217–238, same day)

Under ash's zeno rule (a task is not done until residuals are done —
hybrid.md checklist 7, #p57), everything the ladder left behind was
fixed the same evening by a third persistent worker: the sender-tag
isolation leak (opaque relay tokens, blob adoption), fragments obeying
`enabled` (census-led, 105 fragments, four shapes), the gate-coverage
report (`fmlink --coverage`; 71/122 nodes gate something, the silent
rest announced), the tooling POST through the one op door, sole-tenant
state dirs (boot refuses a second server, crash-safe), un-mixed
blackbox streams, nested-turn safety, boot-as-a-turn, single-broadcast
global ops, eviction that genuinely frees memory (99.9% back, counted
by allocator), and the bridge's lost-write complaint. 57-check
regression green including the done-sentence rig. The parked-residuals
register (notes.md) holds nine entries, each with a reason and revisit
trigger — the only legitimate leftovers. NOTE for hand-runs: a server
now refuses to start on a claimed state dir (MISO_ALLOW_SHARED_STATE=1
overrides); the LaunchAgent holds the mini's.

## THE NEXT WORK (queue emptied — these are chosen, not owed)

1. The tunables conversation + the grid asks re-fired live from the
   app (redo.md item 8) — the promotion rule's machinery all exists
   now; a tunable ask is a `.vars` line and data forever after.
2. The webgpu restart (`webgpu.md` from scratch — redo.md item 7);
   transcript mirroring + self-heal, picker fix, logging cluster
   (redo.md items 3–5) — still unredone from the rewind.
3. The fragment-authorship / seam-occupancy design conversation (the
   register's items 7–8 revisit here, census attached in notes.md).

## tooling state

- **Server state moved**: per-user context op logs live in
  `~/.miso-context/` on the mini (`MISO_CONTEXT_DIR` overrides).
  `/tmp/miso-vars` IS NO LONGER WRITTEN — the old 1s ask monitor must
  read `~/.miso-context/<user key>.log` (asks are ops in the log) or
  poll `GET localhost:8095/diag/context?user=<key>`.
- **Agent instruments**: `GET/POST /diag/context[?user=]` (localhost
  open, tunnel cookie-gated) — the world as JSON, and the repair path
  for a user who unticks their own chooser. `/diag/readout` — DOM as
  JSON (screenshots are ruled out; readout is the eyes).
- **Speed** (deploy.md "Speed"): `fmlink.py --quick` = debug builds
  (1.2s warm) for proof cycles; export_features skips an unchanged
  bake; workers should share `CARGO_TARGET_DIR` across worktrees.
- Dev server on 8095 runs the summit binary from products/miso/build.
  `/tmp/miso-broadcast.json` is process-global and survives state-dir
  wipes — rigs should mint fresh user names, not reuse.
- `.claude/worktrees/` is gitignored (persistent workers live there).
- `origin/main` still holds pre-rewind history — publishing the rewind
  is a deliberate force-push, ash's call, still pending.

## standing doctrine landed today

- The regroup law's invariant is COMPOSITION ORDER (ash, #p46):
  grouping nodes may carry vars (a group's enabled is a feature);
  chains must not move; defaults must not change behaviour.
- The model comparison, the absorption ladder, the world-object design,
  the tag-collision finding, and the ladder's completion record are all
  in notes.md with anchors.
- From rung 7 on, `loop/context` is a dependency of migrated features:
  unticking it is a loud link failure, not a degraded app — by design.
- `context` and `shell` sit at the 6-child cap; context's next child
  forces the holding/changing regroup (now legal under #p46).
