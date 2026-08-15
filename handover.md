# handover
*state of play for the next session — written 2026-08-15, end of day 3's marathon (transcripts/2026-08-14-fm-spec-3.md, 89 prompts, builds 91→120). Discipline in `agents.md`; ops in `deploy.md`; this file is only what's current.*

## where things stand

Live: **build 120** at muon.nøøb.org, 77 nodes. The day in one line: dictate
learned to transcribe itself on the device, the nøøb button became the
steering surface with the feature list living inside the system panel, and
updates became one-reviewed-OK-for-the-whole-fleet.

The arcs, in order:
- **account / noob-button**: the panel moved into the toolbar as the 👤 tool
  (#p46), then the doctrine matured and it moved back (#p58): the lozenge
  (top-right since #p49) is the **nøøb button** — it *steers* muon; the
  toolbar *uses* muon ("meta" is banned vocabulary, #p70). 👤 remains, empty,
  awaiting the **profile page** (account = super-simple social network,
  #p55 — everyone gets a page *(post)*, connections, groups).
- **transcription, phone rung** (`dictate/phone`, #p47): whisper-tiny q4 via
  transformers.js, self-hosted under /stt/ (pinned by `tools/fetch_stt.py`),
  main-thread engine (ort won't init in a worker), webgpu probed then
  wasm-fallback-with-fresh-module (v4 memoizes failures — see the five
  hard-won facts in phone.md's code description). Field-verified: "the
  first message recorded on the dictaford". Scheduler + slots live in
  dictate.rs; `transcript` (tap a playing note → scrollable panel) and
  `transcript/honest-panel` (waiting/transcribing/failed states, err
  stamped as `t_err`) complete the loop.
- **the chooser** (`noob-button/chooser`, #p71–#p82, nine prompts of draft
  churn): the feature list IS the system panel's centrepiece — one line per
  feature, newest first, numbered by **the latest build that touched it**
  (computed from git at export; `tree.json` carries name/purpose/intro(=
  `## user` para)/ts/build), tap-line expands intro + child chips, `‹` up,
  intro-tap opens the served node page in-place, tick per node path
  (`feature_ticks`, user-scoped, inert until the context manager). Panel:
  who → list (height-capped, scrolls) → policy → logout; `source` unticked;
  `queue` composed but dormant (entry stood down, signposted).
- **review** (`policy/review`, #p83 — the doctrine reversal in ash's words):
  when the server is ahead, the list opens with an **awaiting update**
  section (pending features = live tree.json builds > running), one
  **update** button stamps `update_accepted` (user-scoped) → every instance
  applies on sync. One OK, fleet-wide.
- **aesthetics + toolbar feel**: ember 3400K-Dark categorical colours per
  tool (black icon on colour, selected brightens; `tool_colour` seam on
  /tools), centred toolbar, dot-grid background (`logo/dots`). "It's got
  attitude." Late churns (#p86-88): in a tool, the lit tool button owns the
  LEFT edge and is itself the way back (the `‹` retired; tools_home stays
  for programmatic use), controls centre in the free space (auto-margin
  trick in ember.css — beware :first-of-type matches element TYPE).

## NEXT SESSION (in rough order of pull)

1. **Transcription server rung** (`dictate/server`): whisper.cpp + small
   model on the mini, invoked as subprocess on blobs already there (mirror
   uploads them). Consent given in principle — **confirm before installing**.
   The scheduler's upgrade logic is already live: grade-2 results replace
   grade-1 stamps in place ("dictaford" → "dictaphone" is the demo).
   While there: **persist transcript stamps** (currently in loop state only —
   RecList reseeds from IndexedDB on boot, so every restart re-transcribes).
2. **`final` rung**: external batch API — **OPEN: provider is ash's pick**;
   key goes in the mini's `~/.agent-config.json` (the Vonage pattern).
3. **ort-webgpu on iOS** (named refinement in phone.md): the GPU is fine
   (haze proves it on the same phone); ort's jsep requests more than Safari
   grants — haze's recipe is `required_limits: adapter.limits()`
   (ftr repo, haze/src/renderer.rs:536-556). Prize ~5-10x; clear the
   localStorage.muonSttDevice pin when it lands.
4. **The nøøb surface's ask** (#p53/#p70 — agent-powered IDE for end-user
   programming): first brick is the **ask inbox** (prompt box; asks stored
   per user, travel via exchange; dev loop reads them; deploy warns on
   unaddressed). Enriched at #p85 (bedtime): the feature list is the whole
   REQUEST LIFECYCLE — ask → agent proposes (the proposal IS the
   prospective node's `## user` paragraph, approved before build) → in
   progress (ETA; "!"/"?" for problems/questions) → awaiting update →
   shipped. Notes.md has the full doctrine + ladder.
5. **Profile page** (account's social future, #p55) and **context
   sensitivity** for the panel/list (#p78 names it).
6. **auto-policy vs one-OK reconciliation**: 'auto' still self-applies
   without review; the doctrine now says one OK always — decide with ash.

## today's doctrine additions (all in notes.md)

- **The nøøb button steers muon; the toolbar uses muon** — "meta" is retired
  (tainted; #p70). The button's destiny: "how do I use this?" / "do xyz" /
  "I need xyz" — an agent-powered IDE for end-user programming; the ladder
  (surface existing → compose → build) is graded derivation over capability.
- **The queue wants to be a tree → it became one** (#p59→#p71): reader and
  consent surface are one tree at two depths; ticks-on-nodes are a
  user-scoped order.md.
- **Account is a social tool** (#p55): page-as-post unifies with the places
  doctrine; the system freight moved to the nøøb button.
- **One OK, fleet-wide** (#p83): per-device consent was the chore, not
  consent itself.
- **Never touch another feature's show/hide lifecycle from a stylesheet**
  (#p81, paid for): chooser's `#panel{display:flex}` leaked the sheet at
  boot and broke sizing.

## tooling state

- **`tools/panel_drive.js`** — NEW: full-stack CDP rig driving the real app
  (real login via _test PIN from the server log, real taps, DOM asserts).
  Caught two bugs headless-wasm tests can't see. Chrome + dev server needed;
  usage in its header.
- `tools/export_features.py` — exports `tree.json` (name/path/purpose/
  intro/ts/build) beside the static pages; latest-build via git (children
  excluded from a node's own files).
- `tools/fetch_stt.py` — pinned STT artifacts (transformers 4.2.0, ort
  1.26.0-dev..., whisper-tiny.en q4 @ pinned sha) into phone/assets/stt/
  (gitignored except engine.js).
- `tools/export_transcript.py` — two bugs fixed: log dir now derived from
  repo path (was hardcoded one segment short — wrong-project exports), and
  a session keeps its transcript filename across midnight (was forking by
  mtime date).
- Headless-wasm test pattern still the workhorse; the sim (`xcrun simctl`
  iPhone 17 Pro, iOS 26) + beacon-server pattern verified the STT engine
  end-to-end (real speech via `say` → correct transcript).

## small print

- Laptop dev server on 8095 runs the build-114 binary (log:
  /tmp/fm2-devserver.log); local traffic is ungated (gate passes !tunnel);
  _test/_test2 users in ~/.muon-auth/users.json. Sim shut down.
- STT gaps deliberate (notes.md #10): ~130MB model fetch is silent and can
  re-fetch per online session (wants a cache-first sw rule via a /pwa seam
  + a "downloading model" state); >30s notes truncate; transcripts don't
  mirror between devices.
- notes.md hygiene #9: fmlink copies asset *trees* but never sweeps stale
  ones from site/ when their feature unticks.
- shell/update still at the 6-child cap (policy's children took tonight's
  growth); muon root at 6 — next root child forces a regroup.
- The queue node is dormant-but-composed; update_ticks (by build) and
  feature_ticks (by path) are separate stores, both inert until the context
  manager; `update_accepted` joined them tonight (review).
- changes.json path-stamping (#p54, feature→tool mapping for a context-
  sensitive list) remains cheap and unbuilt; deploy already computes the
  paths.
- ideas.md holds: ember palettes (spent tonight), plus whatever the morning
  brings. "There's a million ideas, but they'll keep." — #p84
