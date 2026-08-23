# handover
*state of play for the next session — written 2026-08-23 at the end of
the plans-meet-terrain session (transcripts/2026-08-23-plans.md).
Discipline in `agents.md`; ops in `deploy.md`; the pipeline — now with
intake, tripwires and the replan path — in `hybrid.md`; the day's design
in notes.md ("the plan meets the terrain"); the new ledger is
`misses.md` at the repo root.*

## THE HEADLINE: plans meet terrain, and the builder learned to ask

A Sunday that was meant to be all conversation and ended with one node
shipped. Ash confirmed the summit review first: the phone walkthrough
of the feature-untick workflow worked perfectly — the contexts ladder
is proven end-to-end by its owner.

Then two interlinked designs, talked through and landed the same day:

1. **The build process self-corrects now.** Ash's diagnosis: runaway
   complexity and residual tails are both "no plan survives contact
   with the enemy" — the right move is to modify the plan and retry,
   not push through. So: briefs carry an **estimate** (nodes, vars,
   seams) and a **problem line**; workers carry a **tripwire**
   (touching the unnamed, fix-needs-a-fix, ~2× estimate → STOP) and
   return a **contact report** — a corrected map, not a failed
   delivery; review gained a depth check and the **replan path**.
   `misses.md` is the ledger that closes the loop: triage MUST read it
   before writing any brief. Its first two entries are retrospectives —
   the feature-untick ladder ("X should just work" is a foundation
   ask) and the two squares (an unwritable in-hand line is the signal
   to ask). Escalation rule: a choice must be expressible in
   ask-language or it is the agent's, decided by doctrine and recorded.

2. **The ask workflow recovers the problem.** Users ask for solutions;
   the request object now holds the reconstructed problem (with
   confirmed/edited/silent status). Intake discretion is the ambiguity
   test — "italic" with a word selected builds now; "square" inside
   taps earns one question. **`/did-you-mean` shipped** (live at build
   255, node at ask/lifecycle/did-you-mean): the bench stamps a question
   with tap-sized readings (`stamp_ask.py --question/--option/--likely/
   --note`), the asker's requests list shows a quiet row with chips,
   one tap stamps the answer and flips the ask back to `asked` so the
   monitor fires unchanged. Silence gets the likelier reading at the
   asker's scope with the hedge in the stamp — the literal ask at own
   scope is a zero-consequence floor, so the guess ladder never blocks.
   The full task-tree guesser (three-stage y/n/edit from tool-use
   history) is deliberately NOT built: it is rungs 1–4 of the
   emergent-tools ladder and leans on the open trace-privacy ruling.

The did-you-mean build was the first run under the new doctrine: brief
with problem + estimate lines, Opus worker in a worktree, delivered on
estimate, zero review returns, one honest hypothesis (below).

## FOR ASH (summit-review-sized, when convenient)

- **Fire a real did-you-mean at your phone**: file an ambiguous ask
  from the field, let the bench stamp a question, tap an answer. The
  live-arrival half is a *hypothesis*: localhost rigs verified the row
  renders after a page load, but "the question walks into an
  already-open panel" is unobserved (the relay needs a logged-in
  `_from`; localhost callers have none — pre-existing, not new).
- **The rewind experiment stays named and deferred** (your call to
  run): rewind to 501e7fe, keep attempt one as a branch, replay the
  square-tap-evening asks under the new doctrine, measure against 36
  files / ~1,400 lines / next-day fallout.
- `origin/main` still holds pre-rewind history — publishing the rewind
  is still a deliberate force-push, still pending, still yours.

## tooling state

- **Ask monitor**: `python3 tools/ask_monitor.py`, armed via the
  Monitor tool at session start. An answered did-you-mean fires it with
  no monitor changes (the answer flips status back to `asked`).
- **stamp_ask.py** grew the question mode and `--note` (the hedge), and
  honours `MISO_CONTEXT_DIR` for `--local`.
- **CARGO_TARGET_DIR advice RETRACTED** (deploy.md): fmlink reads
  `<crate>/target` literally, so the shared target dir breaks the link
  step after a successful compile. Workers build cold in their
  worktrees until fmlink honours the variable.
- **Worker worktrees can spawn stale** — one arrived 72 commits behind
  main. The preamble now orders a fast-forward before writing; keep an
  eye on it.
- Rigs: fresh `MISO_CONTEXT_DIR` + fresh user names, always
  (`/tmp/miso-broadcast.json` is process-global); port 8095 is
  hardcoded in serve.rs, so rigs and the dev server cannot run
  concurrently — parallel workers cannot both rig.
- Server state: per-user op logs in `~/.miso-context/` on the mini;
  `/tmp/miso-vars` is dead. Sole-tenant boot refusal stands
  (`MISO_ALLOW_SHARED_STATE=1` overrides; the LaunchAgent holds the
  mini's dir).
- Agent instruments unchanged: `GET/POST /diag/context[?user=]`,
  `/diag/readout` (readout is the eyes; screenshots ruled out for
  evidence, still fine for 4a taste checks).

## THE NEXT WORK (chosen, not owed)

1. The tunables conversation + the grid asks re-fired live from the
   app (redo.md item 8) — unchanged from last handover.
2. The webgpu restart; transcript mirroring + self-heal, picker fix,
   logging cluster (redo.md items 3–5) — still unredone from the
   rewind.
3. The fragment-authorship / seam-occupancy design conversation.
4. New candidates from today: the rewind experiment (ash fires it);
   the emergent-tools cheap experiment (can a model name what a person
   was doing from a day of real blackbox events? — an afternoon's
   answer, gated on the trace-privacy ruling); fmlink honouring
   CARGO_TARGET_DIR.

## standing doctrine landed today

- No plan survives contact: tripwire → contact report → replan;
  stopping is correct behaviour. `misses.md` read before every brief;
  consolidation over accumulation (the regroup law for rules).
- Never-ask, sharpened: no design homework to users, but ONE
  did-you-mean (concrete readings, one tap) at agent discretion;
  which thing they *meant* is theirs alone. Unconfirmed problems never
  license departing from the literal ask (the map guard).
- The literal ask at the asker's own scope is a zero-consequence
  floor; better-for-everyone is post-hoc, via confirmed problems that
  rhyme (rule of two at the problem level).
- `context` and `shell` sit at the 6-child cap; context's next child
  forces the holding/changing regroup (legal under #p46). `/lifecycle`
  now has two children (being-built, did-you-mean).
