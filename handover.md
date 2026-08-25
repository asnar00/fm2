# handover
*state of play for the next session — written 2026-08-23 at the end of
the auth red-team session (transcripts/2026-08-23-fm-spec.md#p3).
Discipline in `agents.md`; ops in `deploy.md`; the pipeline in `hybrid.md`;
the ledger is `misses.md` at the repo root.*

## THE HEADLINE: the auth scheme was red-teamed and hardened, live at build 261

Ash asked for a red-team of the login scheme, then "fix all the weaknesses"
(fold authz in; apply the mini ops over SSH). Eight toggleable nodes shipped
and deployed, each with a transcript anchor (`#p3`), toggle-proven, and the
SMS/token/authz paths exercised against a running server:

- **serve/loopback** — binds `127.0.0.1`, so `/gate`'s `!r.tunnel == trusted`
  is now backed by the kernel; the LAN can no longer skip the login wall. Live:
  the mini's server is bound to loopback and `185.96.221.52:8095` is
  unreachable from the internet (cloudflared is outbound-only).
- **users/harden** — sessions are revocable now: the token gained an issued-at
  (four-part `digits.issued.expiry.hmac`), and `token_valid` additionally
  requires issued ≥ a `~/.miso-auth/revoked-before` epoch AND still-a-guest.
  Drop someone from users.json → their cookie dies next request; bump the epoch
  → mass sign-out keeping the key. Also: signing secret forced 0600, urandom
  short-reads are hard errors, and a process-global `with_store_lock`
  (`harden.lib.rs`) ends the flat-file races. **The four-part token invalidated
  every old cookie once — the whole guest list re-logs-in; passkeys and the
  guest list survived.**
- **users/pin/code-guard** — uniform 6-digit codes (were biased 4-digit), no
  membership oracle (stranger and member get identical opaque `{ok:true}`,
  proven live), race-free 3-strike verify (proven: 6 parallel wrong guesses →
  exactly 3 tries then lockout).
- **users/passkey/login-guard** — consume the challenge before the credential
  lookup, re-check the guest list, race-free. (NOT toggled live — needs a real
  device; compiles + follows proven patterns.)
- **users/authority** — graded authority beside the guest list
  (member/support/admin) gating `ctx_may_write_layer`, the one shared-write
  choke point; least-privilege default, proven live (member refused, support &
  localhost admitted). This is the FOUNDATION only — the richer
  reachable-subtrees / "authority ⊇ blast radius" enactment model
  (notes.md 966-984) is the **named next rung**, built on this authority datum.
- **users/pin/vonage/off-argv** — Vonage creds + the login code off the curl
  argv (via `-K -` on stdin). (Not exercised live — no test creds.)
- **comms/push/private-vapid** — VAPID key to 0600.

Mini ops applied over SSH (ash-authorised): every `~/.miso-auth` file → 0600,
the dir → 0700, and the signing key **rotated** (it had been world-readable
0644 since Aug 15). `~/.agent-config.json` (Vonage creds) was NOT touched —
worth a look.

**`users` is now at the 6-child cap** (pin, gate, passkey, whole-number,
harden, authority) — its next child forces a regroup (like context/shell).

## (historical) prior session: plans meet terrain, and the builder learned to ask

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

3. **The attention ladder shipped the same afternoon** (ash's rule,
   plans #p17–#p19): every builder message reaches the user through
   exactly one channel chosen by the app's state — panel open → the
   row appears in place (`/to-owner` fixed the real bug underneath:
   the relay now publishes to the *edited world's owner*, so bench
   stamps reach open panels at all); foreground, panel closed → the
   nøøb lozenge pulses gently (its own class beside `/update`'s,
   cleared on panel open, never for this device's own edits — read
   from the outbox, not guessed); backgrounded → a targeted push to
   that user's devices (subs already carried the user; no re-enrol
   needed — an earlier triage claim to the contrary was wrong). And
   nothing rings about nothing: no-op stamps and wordless changes
   send zero wire traffic, proven at the wire.

Both builds ran under the new doctrine: briefs with problem + estimate
lines, Opus workers in worktrees, delivered on estimate, zero review
returns. The attention brief's three map errors are `misses.md`'s
third entry — the ledger fed by the pipeline it governs, same day.

4. **Agent instructions are the tree's third language** (ash's ruling,
   plans #p29, simplifying the Aug 15 skillset design): a node may
   carry `<name>.agent.md`, and fmlink assembles the included nodes'
   fragments into `products/<product>/build/skillset.md` — provenance
   order, provenance comments, toggles obeyed. Three fragments live:
   **`/taste`** (the first agent-only node — the nine-principle
   aesthetic standard, extracted at #p25, its whole implementation one
   `.agent.md`; shell is now genuinely at the six-child cap),
   `/did-you-mean` (the ambiguity-test discretion) and `/attention`
   (nothing-rings-about-nothing). Consumers: agents.md 4a, the brief
   template, CLAUDE.md's session-start read. NEXT SESSION: read the
   composed skillset alongside agents.md. Named foundations, not owed:
   per-user skillset selection, exchange-by-consent of agent features,
   decomposing agents.md/hybrid.md into the tree.

## FOR ASH (summit-review-sized, when convenient)

- **Fire a real did-you-mean at your phone**: file an ambiguous ask
  from the field, let the bench stamp a question, watch which channel
  it takes (open panel / lozenge pulse / notification), tap an answer.
  Rig-proven end to end on localhost; the two device-only hypotheses:
  whether iOS/Chrome tolerate the suppressed-when-foreground push
  without showing a default card, and the lived feel of the pulse.
- **A status flip on an ask still carrying a question re-sends the
  question text as a notification.** Defensible (the question is still
  open) but it is a re-notification — say if you want it narrowed.
- **The rewind experiment stays named and deferred** (your call to
  run): rewind to 501e7fe, keep attempt one as a branch, replay the
  square-tap-evening asks under the new doctrine, measure against 36
  files / ~1,400 lines / next-day fallout.
- **Published 2026-08-25**: `origin/main` is fast-forwarded to the
  current work (it had sat at the rewind point `431b39a` all along — the
  earlier claim that it held the Aug 16 history was wrong; that history
  only ever lived on the local archive branch). The Opus-written Aug 16
  session is now on GitHub too, as `origin/archive/aug16-pre-rewind`.

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
5. Parked residuals from the attention build: the broadcast-backlog
   replay on page load (`feature_Messaging.lastV` starts at 0 — the
   attention node works around it with a 1s awake window; the real fix
   belongs to `/messaging`); `/attention`'s hard dependency on `/push`
   is BY DESIGN and PROVEN loud (untick `/push` → clean link failure
   naming the node, the `loop/context` precedent); the `asks` list
   still grows unboundedly (the standing lifecycle-archive rung).

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
