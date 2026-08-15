# consent-once
*the one OK is the only key: no build applies anywhere without the user's acceptance*

> (transcripts/2026-08-15-fm-spec.md#p2)
> I think in order: 1) update by consent once, update all devices belonging to user; 2) update workflow (review proposed feature additions auto-ticked by policy, press "upgrade" button; 3) minimal disruption upgrade [retains system state / place / tasks underway]; 4) agent hookup ("do x" -> find + introduce/use tool, or build tool for next update)

## spec

`/review` built the one-OK path but left the old self-apply paths running
beside it: `/auto` still updates an instance by itself mid-session, and
launch still self-applies, both gated by `/policy`'s `consentNeeded()` —
per-device decisions with no acceptance recorded. This node closes them:
**an instance applies a build only when its user has accepted it.** The
acceptance (`update_accepted`, stamped by the update button, carried by
`/scope`) becomes the single key every apply path checks.

Concretely: the launch-time consent question changes from "does policy
allow this?" to "has this user accepted this build?"; `/auto`'s
mid-session self-apply stands down (the node stays in the catalog —
untick this one and the old behaviour returns). The acceptance mirrors
to localStorage on every state change (the `/policy` mirror idiom),
so a device relaunched after an OK arrived elsewhere applies silently
at launch instead of booting stale and reloading after `/join`.

An unaccepted newer build behaves as `/review` already provides: the
handle pulses, the feature list opens with the awaiting section, and
the update button is the one OK — for this device and all the user's
others. `/policy`'s picker keeps its place; its new meaning (what the
review pre-ticks rather than whether to ask) is the next rung's node.

## user

No device updates itself behind your back, and no device nags you twice.
When a build is waiting, any one of your devices can show you what's in
it; say OK once, and every device you own takes the update — the ones in
your pocket the moment they hear, the ones on your desk the next time
they wake.

## glossary

- **acceptance**: (sharpens `/review`'s awaiting update) the user-scoped
  record `update_accepted` — the sole authority for whether any instance
  of that user may apply a build.

## code description

`consent-once.index.js` composes after `/policy` and `/review`
(provenance order) and owns three moves.

It replaces `feature_Update.consented` (the seam `update` declares and
`/policy` previously claimed): a build is consented exactly when the
user's acceptance covers it — `update_accepted`, read from loop state
with a localStorage fallback (`muonAccepted`) for launch, which runs
before `/join` delivers state.

It stands down `/auto`'s self-apply by redefining `feature_Auto.act` to
a no-op (typeof-guarded; `/policy`'s wrap of the same function composes
inertly around it). Mid-session application is `/review`'s watch — the
acceptance arriving over sync — and nothing else.

It mirrors `update_accepted` into `localStorage.muonAccepted` by
wrapping `feature_Loop.apply`, so the next launch knows what this user
last accepted even before state arrives.
