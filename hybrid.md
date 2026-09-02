# hybrid — fable judgment, opus hands

*A working strategy for running the fm2 loop when Fable credits are scarce.
Commissioned 2026-08-21 (transcripts/2026-08-21-hybrid.md#p2, #p6) after the
forensic comparison of the Aug 16 session's two halves. A living plan, freely
editable, like sovereign.md. The evidence behind it is in notes.md ("the
model comparison", 2026-08-21).*

*2026-09-01 (ash, this session): the worker seat moved from Opus to
**Fable 5.1 at medium effort**. The seat is now a named agent —
`.claude/agents/worker.md` carries the model, the effort and the standing
preamble — spawned with `subagent_type: "worker"` and `isolation:
"worktree"`; `CLAUDE_CODE_SUBAGENT_MODEL=fable` in ash's user settings
makes Fable the default for every other subagent too (effort has no global
subagent switch: an unnamed subagent inherits the session's effort). The
Fable-vs-Opus text below stays as the record of why the seats were split;
the economics section is superseded.*

## the finding this rests on

On 2026-08-16 one session ran p1–p8 on Fable 5 and p9–p43 on Opus 5, same
rulebook, same user, same day. Fable: zero corrections in eight prompts,
five surviving builds. Opus: eight or nine corrective interventions in
~30 substantive prompts, two doctrine rules written mid-session to constrain
it, four builds withdrawn. Opus followed the written discipline faithfully
and debugged excellently — the MIME bug, the DOM loan, the OOM arithmetic
are first-rate work — but every major failure was a judgment call in the
space the rules didn't yet cover: what the ask really wanted, what
"verified" means, when to look, when confidence was earned. Each failure
then *became* a rule (4a, the law above the laws, never-ask). Fable, on the
same rulebook minus those rules, never needed them.

Conclusion: the prompt-fixable surface is already mostly written into
agents.md. The residue — recognising a blown-out image as bad *without* a
rule saying to look, reading the asker's taste unprompted — is a capability
difference. So judgment seats get Fable; execution seats get Opus.

## the shape

Three seats per ask; one Fable session holds two of them.

**Triage (Fable, main session).** Reads the ask, writes a short brief (the
template below), spawns the worker. This is where ask-translation lives —
the step that failed hardest under Opus (the non-map). The brief is small;
writing it costs little Fable budget.

**Worker (subagent, worktree — Fable 5.1 at medium effort since 2026-09-01; Opus before).** Receives the brief plus the standing
preamble below, does the whole five-step loop inside an isolated worktree:
placement, node, implementation, toggle proof, 4a evidence. Returns a diff,
the evidence artifacts, a one-paragraph outcome, and named open risks. This
is where the tokens go, and it is the seat the Aug 16 evidence says Opus
fills well: mechanical discipline held all afternoon; the failures were
upstream (translation) and downstream (acceptance) of it.

**Review (Fable, main session).** Gates the ship on the brief's named
acceptance evidence: looks at the screenshot with 4a eyes, checks every
claim against an observation, probes the hostile case. Verdict is ship or
one return-with-notes; a second failed round escalates to ash rather than
looping. Then the main session serialises integrate → deploy → stamp, as
the flywheel plan already prescribes — the hybrid is the flywheel with the
seats named.

This runs inside one Claude Code session: the main session is Fable, and
each worker is an Agent call with `subagent_type: "worker"` (the
definition in `.claude/agents/worker.md` sets model, effort and preamble)
and worktree isolation. Parallel asks = parallel workers; integration stays serial.

## the economics (superseded 2026-09-01 — both seats are Fable now)

Implementation is the bulk of any ask's tokens — the file reads, builds,
test runs, screenshots, retries. All of that burns Opus. Fable burns only
on the brief (a few hundred words over context it already holds) and the
review (reading a diff and looking at evidence). Rough expectation: 80–90%
of spend moves to Opus, which is what makes the loop affordable on the
fixed plan between credit refreshes.

Two degraded modes, so the loop never fully stops:

- **Fable exhausted**: Opus-only with the post-Aug-16 agents.md, and ash
  takes the review seat live — every ship message must present the
  acceptance evidence for a human yes before deploy. Slower, safe.
- **Opus exhausted / trivial ask**: Fable does the whole loop solo, as in
  the baseline sessions. This is the proven path; the hybrid exists only
  to ration it.

## intake: from ask to request (2026-08-23, plans #p10–#p12)

Users meet a problem, imagine a solution, and ask for the solution. The
ask is the solution as imagined; the *request* is what triage builds
from it, and the difference is the problem recovered. "Request = ask +
user edit" was the old formula; the request object is now richer:

**ask verbatim + problem guess (with its evidence) + amended request,
each carrying a status: confirmed / edited / silent.** The problem line
is the dedup key — different users asking for different features are
often reporting the same problem — and the promotion unit: two
confirmed problems that rhyme are the rule of two firing where it
means something.

**The ambiguity test decides whether to ask.** Before writing a brief,
triage reads the ask against its context — the birthplace tool, any
selection, the asker's history — and counts the readings that survive.
One reading ("italic", said with a word selected): build now; any
question would be noise. More than one with comparable weight
("square", said inside taps: button shape, or square the count?): ask
first, via a did-you-mean — two concrete readings, one tap. The record
holds the cost of guessing instead: 2026-08-21, both squares built six
minutes apart. The forcing function is the in-hand line: **if you
cannot write one in-hand sentence, you have not disambiguated, and
that is the signal to ask.** By the time a brief exists, ambiguity is
dead — this discretion lives in the triage seat, never the worker.

**Never-ask, sharpened (#p10):** what the rule forbids is design
homework travelling toward the user. Which thing *you meant* is the
one fact no context can fully settle, and it is genuinely the asker's
— so a disambiguation (concrete options, one tap) is doctrinally
clean where an open question is not. Guesses are shown for
confirmation, never asked as questions: "were you trying to X?" with
y/n/edit, not "what were you trying to do?".

**Silence is a valid answer and it means "build what I said, for me."**
The confirm ladder never blocks: the literal ask at the asker's own
scope is the floor, ships regardless, and is zero-consequence — only
the asker sees it (#p11). A did-you-mean left unanswered gets the
likelier reading built at their scope, with the hedge in the stamp:
"read it as X — tap if you meant the other." A wrong guess costs one
untick and a re-ask, which the context ladder made cheap.

**An unconfirmed problem never licenses departing from the literal
ask** (the map lesson's guard): only a problem the asker confirmed
lets the request say "the build may solve X even where that means
not-literally-Y." The developer's private theory of the real need
never overrides the words. Post-hoc is where "better for everyone"
lives: the literal thing ships to the asker today; the generalised
solution follows confirmed, rhyming problems through the escalation
ladder.

## the triage brief (Fable writes one per ask)

Short — ten lines, not a spec. **Before writing one, read `misses.md`**
— the ledger of plans that met terrain and lost; a brief written blind
to the recorded misses repeats them. It must contain:

- **the ask, verbatim**, with its anchor (`asks#<t>` or transcript `#pN`).
- **in-hand**: one sentence saying what the asker literally has on their
  screen or in their hands when this ships. The map ask's line would have
  been "streets and buildings drawn around their position, on the phone" —
  a sentence the non-map could not have survived.
- **the problem, as reconstructed**: what the asker was trying to do
  and where it broke, with the evidence (birthplace, rhyming asks) and
  its status — confirmed, edited, or silent. Silent means the literal
  ask is the whole contract.
- **placement**: proposed node and parent, with the cap check done.
- **the footprint** (no estimate — ash, 2026-08-26 #p157: "I've never
  found them to be accurate"): nodes touched, new vars, seams crossed —
  what the plan expects to change, which is the tripwire's baseline. An
  ask whose honest footprint dwarfs its apparent size is a foundation
  ask; say so here, before the build discovers it. No hours, no days.
- **acceptance evidence, named**: which artifacts the reviewer will demand —
  a `/diag/readout` assertion on the real surface (DOM-as-JSON is the
  agent's instrument for seeing the screen; pixel-reading is not — ash's
  ruling, hybrid #p16, and readout's own spec since Aug 13), an on-device
  receipt, a two-instance divergence proof, the toggle proof both ways.
  Named per ask, not generic. Questions of *appearance* — is it
  beautiful, is it on-brand — are not agent-verifiable evidence at all:
  they go to ash interactively.
- **scope**: the literal ask; generalisations parked, with the promotion
  rule cited if a tunable is in play. Then the anticipation test
  (`/anticipation` in the skillset, accounts #p74): name the next three
  asks this user's task will produce, and check each would extend a seam
  the brief creates rather than change what it builds.
- **taste notes**: anything the asker's history says they like or dislike
  that touches this surface — on top of the standing standard: the brief
  points the worker at the composed skillset
  (`products/<product>/build/skillset.md`), where `/taste` and every
  other agent-instruction node land in provenance order.
- **standing law restated**: deliver the ask; doctrine compliance is
  eventual; never hand the ask back as a question.

## the worker preamble

The preamble is the system prompt of `.claude/agents/worker.md` — the
worker gets it by being spawned as that agent type, nothing is prepended
by hand. Edit it there. (Until 2026-09-01 it lived here as a blockquote
and triage pasted it ahead of every brief.)

## the check-in (triage, while a worker runs) — 2026-08-25, accounts #p16

A worker that has gone quiet is not evidence of anything, and there is no
estimate to have run past (ash dropped estimates on 2026-08-26, #p157).
**Triage checks from outside on a fixed cadence — about every 45 minutes
while a worker runs** — never by reading the worker's transcript, which
would flood the main context:

- `git -C <worktree> log --oneline -3` and `status --short`: has it
  committed? is it mid-edit?
- `lsof -i :8095` and `ps` for `miso_server` / headless Chrome with their
  elapsed times: is it in the evidence phase, and is the server on the port
  actually *its* server?
- the age of anything new under the scratchpad's evidence dir.

Then one message to the worker with the diagnosis, or nothing — and
**the diagnosis is a hypothesis until the worker confirms it**. The case
that wrote this rule cut both ways: the `cards` worker had been running
37 minutes; from outside, a two-day-old dev server still held 8095 and
the worker's own server was gone, which read as "its readouts are of a
build without the feature". Triage killed the stale process and said so.
The worker's reply: it had already routed round the busy port (its own
server on 8096, its own `site/`), its evidence was sound, and the nudge
cost it a full re-run on 8095 to prove it. So: a long run with a commit
in the worktree and a rig alive is most likely *working*; the check-in's
first job is to find out, its second to unblock, and a kill or a nudge is
worth one line saying "hypothesis — tell me if I'm wrong". Hazard entries
go to `misses.md` when the miss was the plan's; this one was triage's.
The port hazard is real regardless: a port the rig can choose is the fix.

**Tear down by PID, never by pattern.** With workers rigging in parallel,
`pkill -f miso_server` kills every rig on the machine (triage did exactly
this on 2026-08-25 while proving `/roomier`). Keep the PID you started and
kill that.

## the review checklist (Fable, before any ship)

1. Every artifact the brief named is present. Read the readout — not
   "does the mechanism work" but *does the surface say what the ask
   wanted* (4a at the reviewer's desk: the in-hand line, checked against
   the DOM's actual structure and content). Where the ask is aesthetic,
   the reviewer's job is to route it to ash, not to judge pixels.
2. Calibration: every claim in the worker's report traces to an
   observation. Anything device-shaped claimed from a desktop measurement
   is a return.
3. The hostile case was stated and tested (fail / fill / re-enter).
4. The literal ask is delivered — no doctrinal substitute, no scope creep.
5. Node discipline: real anchor cited, toggle proven both ways, placement
   and cap sound, spec paragraphs complete.
6. Verdict: **ship** (integrate, deploy, stamp) or **return** with notes,
   once; twice → escalate to ash with both rounds' evidence.
6a. **Depth check**: did the delivery's footprint match the brief's?
   Excess that the terrain demanded is essential — record it in
   `misses.md` as a footprint miss. Excess the terrain did not demand is
   accidental — that is a return.
6b. **A contact report is not a failed delivery — it is a replan**
   (#p5). It skips the return-with-notes path entirely: triage reads
   the corrected map, writes a *new* brief (often "build the missing
   foundation first"; sometimes "this is two asks"; occasionally
   "park it — the terrain says no"), and the second attempt starts
   fresh rather than inheriting the first's half-built compromises.
   Every contact report lands in `misses.md`: the guessers and the
   estimators improve only if their misses are recorded and read
   (#p6) — a report filed where no future triage looks is a diary,
   not a feedback loop.
7. **A task is not done while residuals stand** (ash's ruling, hybrid
   #p57; sharpened 2026-08-25, accounts #p50: "always fix residuals before
   calling a job done"). A residual is fixed in the run — not listed for
   signature. The only things that may be parked are ones ash parks by
   name, and a documented way to lose or corrupt a user's data is never
   parkable (misses.md, "the lost card"). Workers naming residuals mid-run is the pipeline's best
   behaviour — but a run does not end with a queue attached: every
   named residual is either fixed within the run or explicitly parked
   by ash as accepted-and-recorded. Without this, every request leaves
   a tail that becomes the next request's opening cost, which leaves
   its own tail — Zeno's paradox as a workflow. "Done" means the
   residual ledger is empty or every entry carries ash's signature.

## triage at scale: many askers (#p7)

miso is being built for a real user group, so the steady state is not one
ash firing asks at a session — it is N users firing parallel asks at the
system, and triage is what stands between that and chaos. The seats above
don't change; what scale adds is a **classification step before any seat
is occupied**, and an escalation ladder with named rungs. Most of the
machinery already exists as doctrine; this section just wires it into the
intake.

**Every ask is classified before anything is built.** Triage reads the ask
against the tree and the ask history and sorts it into one of six bins:

1. **Duplicate / rhyme** — the same want in different words. Matched
   against open and shipped asks first; a duplicate becomes a +1 on the
   existing ask, and the +1 *is* the promotion signal (rule of two,
   fm-spec-2 #p20: second show of intent earns the toolbar slot; #p18:
   second ask touching a parameter earns the variable). At N users,
   dedup-first is what turns a flood into a ranked queue.
2. **Tunable** — "make X bigger/brighter/faster", the general form of the
   naive ask (fm-spec-2 #p17a). If the parameter is already promoted,
   this is a data change at the asker's scope: no build, no worker, no
   Fable beyond the classification itself. If not yet promoted, ship the
   literal constant per-user where scoping exists, and count it toward
   promotion.
3. **Selection** — wants a thing that exists turned on or off. Product- or
   user-level order.md override; nodeless by standing doctrine.
4. **Feature** — genuinely new behaviour. This is the bin that enters the
   three-seat pipeline above, with one addition: the brief records which
   nodes the work will touch, so concurrent feature asks with overlapping
   footprints get sequenced or assigned to the same worker instead of
   colliding in integration.
5. **Bug** — a shipped thing misbehaving. Jumps the queue: a broken
   promise to an existing user outranks a new want, and the worker's task
   is diagnosis-first (the seat Opus demonstrably filled well; Fable 5.1 holds it since 2026-09-01).
6. **Ruling-shaped** — the ask is really a design decision, a privilege
   question, or a doctrine conflict. Never enters the pipeline; goes
   straight to the escalation ladder.

**Privilege bounds blast radius** (fm-spec-2 #p12): an ask's effect may
not exceed the asker's subtree reach. The default that makes N users safe
is: **an ordinary user's ask lands at the asker's own scope** — per-user
var, per-user tick — and only convergence (rule of two, or ash) promotes
it to group or global. Users experimenting on themselves cannot break each
other; the system converges on what several people actually want rather
than what one person just said. Ash's asks are the standing exception:
global by default, because the privilege bit says so.

**The asker hears state, not questions.** The never-ask rule generalises:
no asker is ever handed their ask back as a design conversation. The
lifecycle stamps (asked → proposed → building → shipped, plus `withdrawn`
once R5 lands) are the user-facing protocol; a triaged-out ask still gets
a stamp and a one-line reason. The only questions that travel toward a
user are choices genuinely theirs to make ("on just for you, or for
everyone?" — and rule of two usually answers even that).

**The escalation ladder** — what reaches ash, and nothing else does:

- ruling-shaped asks (bin 6), batched, not one-at-a-time;
- any ask whose honest blast radius exceeds the asker's privilege;
- a review that fails twice (both rounds' evidence attached);
- a doctrine/ask conflict the worker could not satisfy both ways;
- anything irreversible or outward-facing (data deletion, messaging
  other users, external services);
- capacity: when the queue is deeper than the session can drain, the
  ranked remainder — dedup counts attached — so ash spends decisions,
  not attention.

**Ship as built (ash, 2026-08-26 #p107).** A proven node deploys at once — never held for a batch; the user decides urgency, not the queue. Deploys serialise through one build dir, so triage's own rigs live in a separate checkout and never touch it while a deploy runs.

**Serialisation stays sacred.** Workers parallel in worktrees; integrate →
deploy → stamp single-file through the main session (the flywheel rule).
Two extra habits at scale: deploys batch quiet data-only deltas rather
than reloading the fleet per ask, and every stamp names the build that
actually carries the change (the 84babe3 lesson).

What scale does *not* get to change: the classification step is judgment —
which bin, whose scope, what really got asked — and it stays in the Fable
seat. It is also cheap there: six bins over context the session already
holds is exactly the brief-and-verdict shape the economics section wants
Fable turns to be.

## what this is not

Not permanent. The goal remains a proven Fable-only workflow that fits the
plan; the hybrid is the bridge. And not a fourth document of laws: the
worker preamble and review checklist are `<name>.agent.md` fragments in
waiting (notes.md, "the builder is a feature-modular skillset", fm-spec-2
#p21) — when the skillset composition lands, this file decomposes into it.

## open rulings for ash

- Does the reviewer see the worker's full transcript, or only diff +
  evidence + report? (Cheaper and cleaner: the latter; the Aug 16 evidence
  says candid reports can be trusted, it was judgment that failed.)
- Should field asks under some size threshold skip the worker entirely and
  run Fable-solo (wording changes, CSS overrides), spending a little more
  Fable for zero round-trip latency?
- Where do review returns get recorded — notes.md, or a per-ask file the
  next worker sees?
- Scale: what the one-line reason on a triaged-out ask may say (tone is
  user-facing product surface, not engineering); and whether queue rank
  beyond bug > duplicate-count > age needs any policy at all yet.

**2026-09-02, evening (ash):** workers are back on **Opus at effort high** (`CLAUDE_CODE_SUBAGENT_MODEL=opus`, `.claude/agents/worker.md` model opus). One day of Fable 5.1 workers at medium — a typical day, ash said — cost a third of the week's Fable budget (3% → 39%). Fable stays the triage and review seat.

**Switching a seat's model mid-session (2026-09-02):** `.claude/agents/worker.md` and `CLAUDE_CODE_SUBAGENT_MODEL` are read at session start; edited mid-session they change nothing until a restart — three workers spawned right after the edit ran on Fable. The Agent call's own `model` parameter takes precedence and is live at once: spawn with `model: "opus"` until the session restarts, and check `tools/usage_log.py --seats` (or the subagent transcript's `model`) after every spawn that follows a seat change.
