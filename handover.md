# handover
*state of play for the next session — written 2026-08-25 at the end of
the user-accounts day (transcripts/2026-08-25-accounts.md). Discipline in
`agents.md`; ops in `deploy.md`; the pipeline in `hybrid.md`; the ledger is
`misses.md`. Read the composed skillset alongside this.*

## THE HEADLINE: Tara visits 2026-08-26 — the account surface exists now

Ash's customer (Tara) sees work in progress tomorrow. The day built the
first user-facing surface and the object under it, through the hybrid
pipeline (five Opus workers, all on estimate, zero returns) plus four
direct fixes. Live at **build 279**, all pushed to `origin/main`.

**The design (notes.md "cards: the universal object"):** one object, the
**card** — `{id, owner, type, created, edited, blocks[], links[]}` — for
profile, post, project, group, recording; `type` is a field, never a
subclass (#p10). **Users own their data and exchange it** (#p8): a card
lives in its owner's world; other people hold copies only by the owner's
act. Cards have two renderings, tile and page. The dictaphone's grid is
the tile's prototype (#p11). Seed data (#p14): `miso` is a project, ash is
its *lead dev* — the first project card and first link, not yet built.

**Shipped today, in order:**
- `loop/cards` + `cards/me` (261→268): tap 👤 → your profile card (name,
  picture, mission), edited in place, in your world, on all your devices.
- `comms/messaging/roomier` (271): `/msg` body cap 16KB→64KB (a seam,
  `msg_body_cap`); pictures 384px/24KB; list 56KB. Fixed ash's "too big
  to keep" on a real photo. Cost: a text edit still resends the whole
  list, picture included.
- `attention/fresh-words` + `attention/present` (276): a stamp rings only
  with words the entry didn't carry before; a user whose page holds a live
  `/msg/wait` gets the screen and no push at all (server-side presence,
  30s window). Both rig-proven from the server log. Both were ash's
  rulings on live misbehaviour (#p26, #p26a).
- `users/invite` (276): support/admin add a person from under their 👤
  card; `users.json` under the store lock, never written from a failed
  read; `joined` stamped by `auth_verify`; ✕ takes back an unused invite;
  names starting `_` refused (review fix). **Ash's mini entry is now
  `admin`.** Tara gets in tomorrow through this.
- `users/invite/invite-someone` (278): one pill, then the form — ash's
  field ask, built and stamped shipped within the hour.
- `cards/keep` (277): save-as-you-type (600ms), a repaint keeps the focused
  block's text and caret (one seam: `feature_Loop.paint`), Enter finishes
  the title, long-press picture → remove. Found in passing: a repaint DOES
  fire `focusout` in Chrome, so cards' old save was re-entering the loop
  mid-paint; `keep` swallows its own repaint's focusout.
- `cards/frame` (279): the zoom ask — filed from the field, answered by a
  real did-you-mean tapped on ash's phone (the first live one), built:
  pinch/drag to frame the square, keep/cancel. **Touch gestures are
  untested on a device — ash's to try first thing.**
- Regroup: `users/login` (pin + passkey) — chains unchanged. `users` is
  back at the six-child cap (login, gate, whole-number, harden, authority,
  invite). `loop` is at six (cards was the sixth). `cards` has me, keep,
  frame.

## FOR ASH (tomorrow morning, before Tara)

- Update the phone; tap 👤: card, invite pill, your name. Try the photo
  framing (pinch/drag) — the one untested-on-device piece.
- Invite Tara from the app: name + number, then she installs and asks for
  a code. Her entry will be `member`; make her `support` on the mini if
  she should invite her team.
- Two parked residuals awaiting your signature: (1) the whole-list
  `cards` var — one op per edit, last-write, 56KB ceiling: **a second
  card with a picture will not fit**; the var-per-card + blob rung is now
  ahead of projects, not behind. (2) the apply-wrapper race: 👤 doesn't
  open the system panel on any recent build (harmless since the panel
  lives behind the lozenge); the fix is a listener seam in `loop.js`
  replacing four timer-installed wrappers. `keep`'s `paint` seam is the
  first half.

## THE NEXT WORK (chosen, not owed)

1. **Var-per-card + blob path** (the ceiling above) — before projects.
2. **Projects + links**: `miso` project card, `ash —lead dev→ miso`,
   Tara `—candidate→ sevenoaks 2029`; a project page = the card plus
   linked cards you hold. Then "current project" as a per-user var
   feeding the contexts machinery (ash: projects filter posts and tools).
3. **Exchange**: send a card to a person; received cards make the people
   list; a link is a mutual exchange. Open ruling: accept tap or not
   (triage default: none — only guest-list members can send).
4. The dictaphone → cards migration (absorption ladder).
5. A rig port the worker can choose (8095 hardcoded in serve.rs cost
   real time today); one rig dir per worker.
6. Older: tunables, webgpu restart, redo.md items 3–5, the rewind
   experiment.

## standing doctrine landed today

- hybrid.md **check-in**: at ~2× a brief's implied wall time, triage
  inspects from outside (worktree commits, ports, processes) — and the
  diagnosis is a hypothesis until the worker confirms it (misses.md).
- **Tear down rigs by PID, never by pattern** (`pkill -f miso_server`
  hit every worker's rig).
- Ask tooling honours `MISO_HOST` (`ask_monitor.py`, `stamp_ask.py`);
  `.local` does not resolve from ash's laptop today — use
  `MISO_HOST=microserver@185.96.221.52`. `?user=` takes the RAW world key
  (`phone:+44…`), not percent-encoded.
- Workers may not wrap `feature_Loop.apply` from a timer (the race);
  react via Rust `render`, `MutationObserver`, or a named seam replaced
  at load.

## tooling state

- Ask monitor: `MISO_HOST=… python3 tools/ask_monitor.py` via the Monitor
  tool. Worked all day: two field asks arrived, one did-you-mean fired
  and was answered on the phone, both asks stamped shipped with builds.
- Rigs: workers ran on 8096–8099 via post-link sed on the emitted
  `main.rs`; the invite rig scripts (`scratchpad/invite-rig/`) are a
  reusable pattern (scratch `HOME`, `_`-users, playwright headless).
- Local `~/.miso-auth/users.json` gained test user `_cards`
  (+15550007777) — harmless, delete when convenient.
