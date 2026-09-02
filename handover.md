# handover
*state of play for the next session — rewritten 2026-09-02, morning, at the
end of a short settings session (transcripts/2026-09-02-settings.md, 6
prompts) after Saturday's 20-ask day (transcripts/2026-09-01-saturday.md).
Discipline in `agents.md`; ops in `deploy.md`; the pipeline in `hybrid.md`;
the ledger is `misses.md`. Read the composed skillset alongside this — it
carries nine agent-instruction nodes now; the newest are /retrofit and
/confined.*

## TODAY (2026-09-02): build 453 is live; two changes to how we work

- **Subagents run on Fable 5.1.** `CLAUDE_CODE_SUBAGENT_MODEL=fable` in
  ash's user settings; the hybrid worker seat is the named agent
  `.claude/agents/worker.md` (model fable, effort medium, the preamble as
  its system prompt). Spawn with `subagent_type: "worker"`, `isolation:
  "worktree"`. Effort has no global subagent switch — an unnamed subagent
  inherits the session's (high). hybrid.md carries the dated note; its
  Opus text is history. Ash restarted the session to make this live —
  **check `worker` appears in the Agent tool's type list.**
- **The toggle proof is implied for a confined change** (`/confined`,
  agents.md step 4): a commit whose feature-tree footprint is one node
  (subtree and own order.md included) plus ticks added to its parent's
  order.md cannot alter the build without that node. `fmlink.py miso
  --prove` says so from the working tree; deploy.sh refuses any other
  shape that lacks a `Toggle-proof:` trailer, checking from
  `products/miso/build/released.sha` (written when a ship lands;
  `PROOF=skip` overrides). First real run: build 453's gate, green.
- Saturday (build 411 → 450): 20 field asks from ash's phone — video
  posts with poster/flip/square, project audience, safe-area floor,
  launcher order, stale update notices dropped (#p33). Three phone-only
  divergences that day earned the **boot self-check on the device via
  `/diag`** its place as the top next rung: one report from the phone
  saying which fragment versions it runs would have answered all three.
- Seen and cleaned: a smoke-gate server (port 8169, its own scratch home)
  was still running sixteen hours after Saturday's gate — killed by PID.
  smoke.py's teardown can leave its server behind; worth a look.

## EARLIER HEADLINE (2026-08-25/26): Tara's morning — a live user, ~20 asks shipped from the phone in real time

*Updated 2026-08-26 evening.* Build 398 is live. THE SIMULATOR RIG exists
(deploy.md, `tools/simrig.py`, `tests/sim/`): the installed app on an
iPhone 17 simulator, real touches by selector, eyes through /readout+/rects,
hands through /drive (+js), four tests green (pencil on post/profile/project;
‹ and the picker after writing). The pencil bug was found by /touches — the
phone's black box: the finger lands on the glyph's <svg>, the face swap
detached it, the swallow disarmed — and its exact sequence is a gate step.
Build 377 was live at noon. Afternoon additions:
`posts/titled/above` (title over photo), `page/editing/toolbar` (edit/save
are toolbar buttons — pencil/tick, nothing floats over a card),
`page/keep/lands` (a tap while writing still lands: ‹ and the picker on
the first press), `chooser/arrives` (the nøøb sheet opens on the tap; the
gate's flake was this — see below), and `being-built/announced` (a global
`builds` list on everyone's sheet, fed by `stamp_ask.py --announce` at
build start and ship; its agent.md is in the skillset — use it for every
conversation ask). Since the evening
handover: the smoke gate (`tools/smoke.py` in deploy.sh — waits for the
loop to boot, three passes), the deploy rule **ship as built** (`/ship-as-
built`, an agent-instruction node), `/own-slot` (each world's broadcast
slot under `context_dir()` — before it every server on a machine shared
`/tmp/miso-broadcast.json`, which is why the gate cried wolf), `/urgency`
(urgent / whenever on the ask box), ticks in the ask box's results
(`/everywhere`), and the morning's field asks: posts picture-first,
tile-words, plus-at-home, post-time (EXIF date orders posts), delete
(tombstones; undo restores), name-first, map-location → map, backdrop
(tap the ground to close), ‹ (`/back`), lead (projects, posts, people
first), reorder (hold-then-drag, per user, `tools_order_chosen()` seam),
quiet, build-below, and **manual save** (`/keep/manual`: autosave off —
it was losing keystrokes on the phone; a save pill, or tap away).

**The gate is green** (`tools/smoke.py`, three passes). Its morning of
crying wolf had four causes, all the gate's or triage's, none the app's:
rigs talking into its stream (fixed by `/own-slot`), a relink of the
shared build dir mid-run (deploy.md rule), a fixed boot wait too short
under load (now waits for the loop), and fixed 2-second waits on the
panel and the map on a fresh world's first page (now polled; they open in
~200 ms). Six deploys shipped with `SMOKE=skip` while this was found; each
said so. A fifth (build 365): the cold pass's lozenge poll timed out
while a rig's cargo build on the same laptop was still running — a rerun
with the machine quiet was green. So: a gate failure on a quiet machine
means the app — and the five first-attempt failures of 2026-08-26 WERE the
app: a `no-store` re-fetch of `features/tree.json` that hangs under a fresh
service worker, holding the nøøb sheet shut (the phone's "doesn't press"
of that morning, too). Fixed by `/chooser/arrives`; the gate's failure
dump and full log (`products/miso/build/smoke.log`) are what found it
(deploy.md).

**Also open:** transcript anchors are stamped UTC and ask anchors local
(post-time worker) — a one-line fix in one reader plus a whole-tree
`--chains` diff, its own run; `/kinds/new` writes after `/undo/late`'s
scan, so making a post is not undoable (the `/late` → `/turn-end` rung);
`/guard/singleton` vs tombstones for a deletable singleton (not reachable
today).

## FOR ASH (tomorrow morning, before Tara)

- Update the phone. 👤 → the map glyph: pins for you, alice, bob. The flag
  tool: **new** → "miso", add yourself as lead dev, add alice as
  canvasser → alice's 👤 card reads "canvasser for miso". The bubble tool:
  **+** → a post from where you stand.
- With Tara: invite her (real number → SMS); she installs, logs in, sees
  you; make a project "sevenoaks 2029", add her as candidate. She is
  `member`; make her `support` on the mini if she should invite her team.
- Two rulings you may want to make: CARTO dark tiles vs plain OSM (one env
  var, `MISO_TILE_URL`); a project reaching only its members (not your
  whole invite tree).

## THE NEXT WORK (chosen, not owed)

0. **The smoke gate is in deploy.sh** (`tools/smoke.py`, accounts #p96):
   nine steps × three passes must be green or nothing ships. Next rungs:
   (a) tree-owned steps — each node carries `<name>.smoke.py`, fmlink
   composes them; (b) a boot self-check on the device reporting through
   `/diag` (the tap seam is `open()`, the veil lifted, no orphaned
   wrapper) — the only layer that sees the real phone; (c) check the
   update-policy default a NEW user gets (`update_policy` is the empty
   string — find what that means) so a dead control can never trap
   someone on an old build: the lozenge was the only road to an update
   when it died (#p95).
1. **Known bug, fix first:** `/keep` — typing right after a fresh repaint
   can land a character one place early ("buildin — v2g"); seen in two
   rigs; the keystroke races the caret restore. Reproduce with
   `scratchpad/invite-rig/caret.py`'s pattern and a keystroke inside the
   600ms debounce window.
2. **Project membership as the second visibility cue** (#p71 "later"):
   members of a project see each other (`people_order`, `users/near`,
   `exchange_give` are the seams). Posts in a project (`links:[{kind:"in"}]`
   reserved). Current-project filtering.
3. **Exchange stage two** only when asked: send to a number, withdrawal,
   an inbox.
4. **Var-per-card + blob path**: every edit resends the whole list to every
   invite-linked person and project member (four world reads per write).
5. Named foundations: a var rename map when a declaring node moves;
   `/remember`'s append is read-modify-write (single writer); the fixed
   `/tmp/miso-broadcast.json`; vector tiles we style ourselves; a `loop`
   agent-instruction ("no clock inside update — time rides on the event");
   a singleton/`guard` note for new types.
6. Older: tunables, webgpu restart, redo.md items 3–5, the rewind
   experiment.

## standing doctrine landed today

- **Residuals are fixed in the run, never listed for signature** (#p50);
  a documented way to lose user data is a defect, not a residual (misses.md
  "the lost card"). Recovery move: the op log holds every prior value;
  replay one through `POST /diag/context?user=<raw key>`.
- **Anticipation** (#p74, `/anticipation`): ship the literal ask, shaped
  for the next three asks — seams, not builds. Its two failure modes are
  in the ledger (the exchange brief that built the foundation; the cards
  blob built with none).
- **Toolbar glyphs are ink** (`/glyphs`): filtered emoji or drawn SVG in
  currentColor; never an emoji-presentation character; undo stays last in
  every row — a newer node inserts before it.
- hybrid.md: the check-in (fixed cadence — estimates are dropped, #p157 — from outside, diagnosis as
  hypothesis); tear down rigs by PID; **`set -o pipefail` and assert the
  fragment composed before reading any evidence** (deploy.md) — four
  broken commits today came from a `| tail` hiding a link error.
- Ask tooling honours `MISO_HOST` (`.local` does not resolve from ash's
  laptop; use `microserver@185.96.221.52`); `?user=` takes the RAW key.

## tooling state

- **fm2 lives on the mini** (2026-08-28): `~/fm2` there, tmux session
  `fm2`, sessions start there; from ash's Mac type `mini`. deploy.sh on the
  mini ships to localhost. See deploy.md "Working on the mini". Tailscale
  login on both ends and the mini's GitHub key were left for ash.
- Ask monitor via the Monitor tool; ~15 field asks today, every one
  stamped shipped with its build; one live did-you-mean answered on the
  phone.
- Rigs: `serve_port()` is a seam — retarget its body in the emitted
  main.rs (the `8095u16` literal is gone); one rig dir per worker; the
  invite/exchange/people rigs in the scratchpad are reusable patterns.
- Worktrees: ~12 agent worktrees under `.claude/worktrees/` — prune when
  convenient (`git worktree prune` after removing the dirs).
- Local `~/.miso-auth/users.json` carries test users from rigs; harmless.

