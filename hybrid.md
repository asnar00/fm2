# hybrid — fable judgment, opus hands

*A working strategy for running the fm2 loop when Fable credits are scarce.
Commissioned 2026-08-21 (transcripts/2026-08-21-hybrid.md#p2, #p6) after the
forensic comparison of the Aug 16 session's two halves. A living plan, freely
editable, like sovereign.md. The evidence behind it is in notes.md ("the
model comparison", 2026-08-21).*

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

**Worker (Opus subagent, worktree).** Receives the brief plus the standing
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
each worker is an Agent call with a model override to Opus and worktree
isolation. Parallel asks = parallel workers; integration stays serial.

## the economics

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

## the triage brief (Fable writes one per ask)

Short — ten lines, not a spec. It must contain:

- **the ask, verbatim**, with its anchor (`asks#<t>` or transcript `#pN`).
- **in-hand**: one sentence saying what the asker literally has on their
  screen or in their hands when this ships. The map ask's line would have
  been "streets and buildings drawn around their position, on the phone" —
  a sentence the non-map could not have survived.
- **placement**: proposed node and parent, with the cap check done.
- **acceptance evidence, named**: which artifacts the reviewer will demand —
  screenshot of the real surface, on-device receipt, two-instance
  divergence proof, toggle proof both ways. Named per ask, not generic.
- **scope**: the literal ask; generalisations parked, with the promotion
  rule cited if a tunable is in play.
- **taste notes**: anything the asker's history says they like or dislike
  that touches this surface.
- **standing law restated**: deliver the ask; doctrine compliance is
  eventual; never hand the ask back as a question.

## the worker preamble (prepended to every Opus worker)

> Follow agents.md steps 1–5 including 4a; the brief you carry is the
> contract. Deliver its in-hand line. If doctrine and the ask conflict,
> find the move that satisfies both — proxy it, cache it, vendor it,
> refactor the parent; if that is genuinely impossible, return to triage
> with the conflict named rather than shipping a substitute.
>
> A claim about anything you have not observed is a hypothesis, not a
> result — label it as one. If the evidence the brief demands needs
> tooling you don't have, that is a blocker to report, not a step to skip
> silently.
>
> Before declaring a new mechanism done, state what happens when it fails,
> fills, or re-enters — and test that case. The fallback that also fails,
> the buffer that evicts what it exists to protect, the replay that
> re-triggers: these are where your defects will live.
>
> Report outcomes plainly: what shipped, what was proven, what remains.
> No victory prose — calibrated claims are what make celebration safe.
> Return: the diff, the evidence artifacts, one paragraph of outcome,
> and open risks by name.

## the review checklist (Fable, before any ship)

1. Every artifact the brief named is present. Look at the screenshot —
   not "does it exist", but *is it good* (4a at the reviewer's desk:
   blown-out filters, ignored taste notes, and asset-fighting are the
   known "no"s).
2. Calibration: every claim in the worker's report traces to an
   observation. Anything device-shaped claimed from a desktop measurement
   is a return.
3. The hostile case was stated and tested (fail / fill / re-enter).
4. The literal ask is delivered — no doctrinal substitute, no scope creep.
5. Node discipline: real anchor cited, toggle proven both ways, placement
   and cap sound, spec paragraphs complete.
6. Verdict: **ship** (integrate, deploy, stamp) or **return** with notes,
   once; twice → escalate to ash with both rounds' evidence.

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
   is diagnosis-first (the seat Opus demonstrably fills well).
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
