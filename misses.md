# misses — the ledger of plans that met terrain

*Commissioned 2026-08-23 (transcripts/2026-08-23-plans.md#p5–#p7). Every
entry is a labeled example: a plan, what it assumed, what the terrain
said. Triage reads this file before writing any brief (hybrid.md);
contact reports, estimate misses, and confirm-ladder edits land here.
Append-only in spirit — entries may be corrected, never quietly removed.
When the lessons pile up, they get consolidated into fewer, deeper
principles and dead entries retired to the bottom — a ledger that only
accumulates makes a worse planner, not a better one (#p6).*

Entry shape: **the ask** (verbatim, anchored) · **the estimate** (what
the plan thought the footprint was) · **the actual** · **what the plan
could not see** · **the lesson** (one sentence, the part a future brief
must read).

---

## the feature-untick ladder (2026-08-21) — retrospective, filed 2026-08-23

**The ask:** that the feature chooser's ticks mean it — untick a
feature and it is off, for you, on all your devices, surviving
restarts. (transcripts/2026-08-21-hybrid.md; the ladder's record is in
notes.md, "the absorption ladder".)

**The estimate, as it would have been written:** a handful of nodes
around `/chooser` and `/enforced` — read the ticks, filter the
composition. An afternoon.

**The actual:** eleven rungs, builds 187–212 — `.vars` declarations,
typed per-user worlds, the turn boundary, implicit `enabled` gates,
merge-disciplined ops, persistence, the overlay chain, a migration with
epoch counters, the context join, SyncVar deleted. Then a same-day
residuals campaign of comparable size (builds 217–238).

**What the plan could not see:** per-user anything requires a place for
per-user state to live, and no such place existed. The ask was not a
feature; it was a demand for the world-object. Every rung after the
first was this one discovery propagating.

**Where the tripwires would have fired:** immediately — rung 1 was
already "modifying things the brief never named" (a state model no
chooser brief would mention). The contact report would have said: *the
plan assumed ticks could filter composition; false — composition is
global and enablement needs a per-user world; the tree needs the world
built before this ask is buildable.* The replan — build the foundation
as its own named ladder — is what happened anyway, but by
discovery-in-flight over a full day rather than a one-hour first
contact. Same destination; the map would have been bought cheaper.

**The lesson:** an ask phrased "X should just work" is a foundation ask
until proven otherwise — estimate the foundation, not the feature.

---

## the two squares (2026-08-21) — retrospective, filed 2026-08-23

**The ask:** "square", filed from the field, from inside the taps tool
(asks anchor in the `/square-taps` and `/tap/square` nodes).

**The estimate:** trivial — one node either way.

**The actual:** two readings survived the context — the tap *button*
made square, and the tap *count* squared — and both were built, landing
six minutes apart (commits 512edec 22:00, 0a69170 22:06). Ambiguity was
resolved with tokens instead of a question.

**What the plan could not see:** nothing in hand broke the tie — the
birthplace said "taps," which fit both readings. The miss was not the
estimate but the intake: no in-hand line could have been written for
this ask, and that unwritable sentence was the signal to ask, unread.

**The lesson:** when more than one reading survives the context, the
cheap move is a did-you-mean (two concrete options, one tap), not a
build — and an in-hand line you cannot write is how you notice
(hybrid.md, "intake"; plans #p12).

---

## the attention brief (2026-08-23) — filed the same day, by the pipeline itself

**The ask:** the attention ladder (plans #p17–#p19) — panel open
updates in place, foreground flashes the lozenge, backgrounded rings a
notification, and nothing rings about nothing.

**The estimate:** 2 nodes, ~8 files — held. The misses were in the
brief's *map*, not its size; the worker corrected in flight and named
all three rather than building on them.

**What the plan could not see (three things, one root):**
1. It said `push-subs.txt` lines were anonymous and would need a user
   field plus a migration — false; field 4 has carried the subscriber's
   phone since the miso rename, so no migration existed and the
   "ash must re-enrol his devices" warning given at triage was wrong.
2. It implied the relay audience was tag-shaped — false; since
   `/whole-number` the audience is `user.phone:+44…`, the whole world
   key. The worker's first build published to an audience nobody
   listens on; the rig caught it.
3. It never mentioned that a joining page replays its broadcast backlog
   from `v = 0` — so the flash would have fired on every page load. The
   worker added an awake-window; the tree-level fix belongs to
   `/messaging` (parked).

**The lesson:** a brief's claims about wire formats, keys and file
layouts must be read from the composed source at brief-writing time,
never from specs or memory of them — the specs describe births, the
composition describes now. And any feature that reacts to "an arrival"
must ask what join-replay looks like before believing its trigger.

## the stuck worker that wasn't (2026-08-25) — triage's miss, filed the same day

**The ask:** ash, of the `cards` worker at 37 minutes: "seems like it's
been running a rather long time. maybe it's got in a muddle?"

**The estimate:** triage read the machine from outside — a two-day-old
dev server holding 8095, the worker's own server gone — and diagnosed
"its readouts are of a build without the feature". Killed the stale
process, told the worker so.

**The actual:** the worker had already routed round the busy port (own
binary on 8096, own `site/`), its commit was in the worktree, its
evidence was sound. The nudge cost it a full re-run of the acceptance
list on 8095 — same commit, same results.

**What the plan could not see:** a worker that runs long *with a commit
and a live rig* is most likely working. The outside view showed a
hazard (the stale server) and triage read it as a failure.

**The lesson:** the check-in's diagnosis is a hypothesis, sent as one
line the worker may refute; unblock (free the port) without
prescribing (re-run everything). Now in hybrid.md's check-in section.

---

## the picture cap (2026-08-25) — an estimate miss, filed the same day

**The ask:** the profile picture (#p7), briefed at "downscale to 256px,
~25KB, state a hard cap".

**The estimate:** a picture is a block; the budget is the disk's.

**The actual:** the whole cards list travels as one `/msg` op, and
`/messaging` truncated bodies at 16KB — an oversized message becomes
invalid JSON, is answered `400 untyped`, and is retried forever, jamming
every op the device will ever send. The worker found it live at a 40KB
cap, dropped to 8KB; ash's first real photo then failed; `/roomier`
raised the wire to 64KB the same afternoon.

**What the plan could not see:** the brief sized the picture against
storage and never asked what carries it. The attention brief's lesson
("read wire formats from the composed source") applied here too and was
not applied.

**The lesson:** any brief that puts bytes into a var must name the
op that carries the var and the cap on that op's wire; a var that is a
list is one op, whole, per edit.

## the lost card (2026-08-25, build 292) — a named risk that was allowed to ship

**The ask:** none — ash's profile card (picture, mission) vanished after the
build-292 update (#p47). Ruling (#p48): "let's make sure that never happens
again — data loss is a sure way to kill user trust."

**The estimate:** `me.md` had described this exact failure at build time —
an ensure before the join makes a blank card and last-write sends it over
the real one — and parked it as "the offline duplicate; per-card identity is
the rung that closes it". Triage read that paragraph, listed it as a
residual for ash's signature, and shipped eleven more releases on top.

**The actual:** an update reloads the page while the server restarts; the
join times out at two seconds; `fm-joined` is set by the timeout too; the
ensure fires against an empty world; one op replaces the whole list. Line
127 of the op log. Recovered from line 91 through the diag door; fixed the
same hour with `/guard` (server-side merge: a set can never drop a held
card, blank duplicates discarded) and `/me/patient` (ensure waits for a real
join or does nothing).

**What the plan could not see:** nothing — it saw it and priced it as a
later rung. The miss is a category error: a *documented* way to lose user
data is not a residual, it is a defect, and "parked pending signature"
does not apply to it.

**The lesson:** any write path that can replace a user's stored value with
less than the server holds is a bug to fix before the next deploy, never a
residual to record — and a store whose merge is "last write wins on the
whole list" must be guarded at the server the day it holds anything a
person would miss.

## the hidden link error (2026-08-25, twice) — triage's rig discipline

**The ask:** none — two direct builds (`map-pin`, `world-cache/forget`).

**The estimate:** trivial nodes; link, rig, commit.

**The actual:** both first links FAILED — a tree-global name collision
(`pin` vs `users/login/pin`) and a child citing an anchor older than its
parent's — and both failures were invisible: `fmlink … | tail -1` hides
the exit status, so the rig ran the previous binary, printed plausible
numbers, and a broken tree was committed (once amended, once re-committed).

**What the plan could not see:** nothing — the linker said "fm link
error" in plain text one line above the one I kept.

**The lesson:** a rig that does not assert the link succeeded proves
nothing about the change; `set -o pipefail` and an explicit "the fragment
is in the composed output" check before any evidence is read. In
deploy.md.

## the exchange brief (2026-08-25) — the foundation built where the feature was asked

**The ask:** "let's talk about making people able to see each other" →
"invite should automatically make things visible" (#p69, #p71).

**The estimate:** triage briefed an inbox file, a send-to sheet, freshness
to a sent-to set, and the invite seed — about a day.

**The actual:** ash (#p72): "a *day*? just to make two users see each
other? I'd have thought a few minutes!" The cut-down build — server copies
a card along invite links on every write, foreign cards read-only — took
one worker a couple of hours and is what shipped.

**What the plan could not see:** nothing; it saw the whole design (#p70)
and briefed the design instead of the ask.

**The lesson:** the mirror of "estimate the foundation, not the feature":
when the ask IS the feature, build the feature, shaped for the foundation
— seams (`exchange_copy`, `exchange_give`), not builds. Now `/anticipation`.

## the clock in wasm (2026-08-25) — a trap with no sign on it

**The ask:** `/projects`' ✕ (remove a role) — the first cut stamped the
time inside `update` with `SystemTime::now()`.

**The actual:** `SystemTime` panics on `wasm32-unknown-unknown`; the client
loop died silently. Fixed in the run: the page half sends `t: Date.now()`
with the event, as every other card event does (`/revert` had met the
same wall and used "newest stamp + 1").

**The lesson:** inside the loop's `update`/`render` there is no clock —
time arrives on the event from the page. Nothing in the tree said so;
now the ledger does, and a `loop` agent-instruction should when one exists.
