# handover
*state of play for the next session — written 2026-08-14, end of day 2's second (marathon) session. Discipline in `agents.md`; ops in `deploy.md`; this file is only what's current.*

## where things stand

Live: **build 90** at muon.nøøb.org, 65 nodes. Since the mid-day handover:
muon became an **operating system** — a toolbar of tools along the bottom
edge (`shell/tools` + the `tool_controls` chain: the open tool docks leftmost
after `‹` and contributes its own toolbar buttons) — and grew its first real
tool, **dictate** (`loop/dictate` 🎤): record voice notes to the device
(IndexedDB blob store, state-driven Elm-style effects, replay touches no
hardware), tap a note to play it, and **mirror** (`dictate/mirror`): notes
appear on all the user's instances — *metadata eagerly* (RecShared through
the outbox; per-user index + broadcast + RecIndex boot catch-up), *audio
lazily* (upload when connected, marked per-file, retried on reconnect;
fetched by other instances on first play; dimmed tile until the audio is
local). Confirmed working phone → laptop. Blobs live in
`~/.muon-blobs/<user>/` on the mini, outside the deploy tree. `serve`'s
`request` now carries raw body bytes (binary-safe, 16MB) beside the String
view. Earlier in the session: join/veil/resume (never see stale state),
update policies (automatic / fixes auto / ask me, releases self-classify),
tree-global names (linker-enforced; `standalone`, `veil` renames).

## NEXT SESSION: transcription (the graded-derivation probe, continued)

The design is settled and recorded (notes.md "PLACES CONVERSATION" +
#p36–39): **two guarantees** — *live* (a transcript now, deadline-bound,
quality-flexible) and *posterity* (the best transcript eventually). Concrete
first, generic later: three slot functions `transcribe_local()` /
`transcribe_server()` / `transcribe_api()` as base "unavailable" chains in
dictate, a `transcribe()` scheduler on top, each rung a subfeature that
redefines its slot: `dictate/phone`, `dictate/server`, `dictate/final`.
Build order: **server** first (whisper.cpp + a small model on the mini,
invoked as a subprocess — the curl pattern; user consented in principle,
confirm before installing), then **final** (external batch API — **OPEN:
which provider**; key goes in the mini's `~/.agent-config.json`, the Vonage
pattern), then **phone** (wasm/WebGPU STT — heaviest, last; Web Speech API
disqualified: iOS routes it to Apple servers, not honestly offline).
Everything transcription needs already exists: blobs reach the exchange
(mirror), local blobs are readable (dictate), transcript values flow as
events, and provisional-quality stamping is the veil/honesty pattern.

## today's doctrine additions (all in notes.md)

- **Provenance-ordered linearisation IMPLEMENTED** — composition order = the
  prompt timestamp each node cites; tree = grouping + selection only;
  provenance is load-bearing (link error without it). Old features extend new
  chains via new subfeatures (causality) — `tap/counter` is the worked case.
- **Two-phase lifecycle in practice**: draft features churn in place, specs
  accumulate prompt quotes (tools.md carries three). The deploy "no new
  nodes" warning now has a taxonomy of honest answers: new capability →
  node; rule/tooling → none; draft churn → none, spec amended.
- **Join / veil / resume**: catch-up is one act (boot, reconnect,
  foreground); never lie about staleness OR quality.
- **The places design** (from the app: spatial database of conversations —
  see ideas.md): posts immutable/append-only; replication = scope ∩ interest
  as key-set subscription; enrich at the exchange, consume at the edge;
  graded derivation as the central pattern. mirror is its first working
  instance (eager metadata / lazy payload).
- **muon is an OS**: tools (user's term), toolbar, display surface,
  toolsets when the row overflows. Colour discipline: white on dark grey,
  black on light grey selected; emoji forced monochrome via CSS filter.

## tooling state

- `fmlink.py`: `--chains` (chains + fragment slots + lib ratio: 4%),
  chronological linearisation, tree-global name enforcement, optional-feature
  fix, stale-page removal.
- Headless wasm testing pattern: node instantiates `client.wasm` directly
  and drives `fm_entry`/`fm_event` — every dictate/tools state transition was
  verified this way before deploy. Cheap and fast; use it.
- `export_transcript.py` refuses cross-session overwrites. This session is
  `transcripts/2026-08-14-fm-spec-2.md` (50+ prompts).
- deploy stamps changes.json entries with `kind` (feature/fix) from the tree
  diff; update policies consume it.

## small print

- `shell/update` at the 6-child cap; muon root at 6 (serve, shell, users,
  comms, loop, diag) — next root child forces a regroup (regroups are
  behaviour-free now).
- Dictate gaps, deliberate: no delete, no per-file upload progress, remote
  tiles after restart re-dim until RecIndexed arrives. Fine for now.
- Mic/audio quirks to watch on iOS: MediaRecorder yields audio/mp4 (AAC) —
  whisper eats it happily.
- The laptop's dev server on 8095 runs the current binary (restarted during
  testing); `~/.muon-blobs` on the laptop was test junk and was removed.
- Local `_test` user +15550001111; mini's users.json differs. getrandom stays
  on `custom` for wasm; deploy smoke-tests zero-import instantiation.
