# by-policy
*under automatic, the instance says OK for you: the one key is stamped by policy, never asked for*

> (transcripts/2026-09-02-self-check.md#p24)
> another thing: I want to make the update policy work better. Under "auto", I shouldn't be asked to OK updates, they should just happen (I should still be notified when the app isn't in focus). Otherwise, the app should just be up to date at all times without any action from me. The only exception should be that we shouldn't update while the user is recording or editing. As long as update doesn't switch the UI state, it should be completely seamless.

## user

With updates set to **automatic**, you never press anything. A build ships;
a moment later you are on it — same tool, same post, same place — and no
handle pulsed, no button appeared, nothing asked. **Fixes auto** does the
same for a build that only fixes things and still asks when one brings new
behaviour; **ask me** always asks, as before. If the app was in the
background when the build shipped, the phone shows one notice that it
updated; open it and you are current a moment later.

## spec

`/consent-once` made the acceptance — `update_accepted`, stamped by the
update button and carried to every instance by `/scope` — the single key
every apply path checks. It closed the self-apply paths for every policy at
once, so `automatic` began asking, the opposite of its name. This node
keeps the one-key architecture and gives `automatic` its meaning back: **when
the policy allows a build to proceed without review, the instance stamps
the acceptance itself**, the moment the build is known, exactly as the
update button would. Everything downstream is untouched and runs as it
always did: `/review`'s watch sees the acceptance and applies, `/seamless`
lets a task finish first and brings the instance back where it was,
`/delta` and `/patch` keep the apply as small as the change allows, and the
acceptance reaches the user's other instances over sync so they take the
build too. No new apply path exists; the key is stamped, not bypassed.

The policy question is `/policy`'s own, asked through `consentNeeded()`:
`auto` accepts every build; `fixes` accepts a build whose pending changes
are all fixes and asks otherwise; `consent` never accepts here — the
handle pulses and the update button is the only OK, unchanged. The empty
string — the value a user has before they ever touch the picker, since
`/policy` declares its default as `""` and the fragment falls back to
`localStorage.misoPolicy || 'auto'` — is read as `auto`, so a new user's
app keeps itself current from the first day.

Nothing new appears on screen. `/watch` lights the handle whenever the
server is ahead; that pulse means "a build is waiting for you", and once
the acceptance covers the build nothing is waiting for anyone, so this node
takes the pulse off again wherever the policy let the build through. Under
`consent`, and under `fixes` when a build needs review, the pulse stays and
the review is exactly what it was.

"Known" means what `/watch` reports: the foreground and online events, the
minute poll, and the launch-time compare (which `/consent-once` turned into
a decline that hands over to `/watch`; the instance now stamps and applies
through the same road, one state change later). A push announcing a build
to a visible window arrives as `/attention`'s page message; this node
treats that message as a reason to check, so an instance in the foreground
does not wait for the poll to notice a build the server already told it
about. An instance out of focus is not on this road at all: the phone rings
`/push`'s notice — "updated to build N — <subject>", news of a build, not a
request — and the check runs when the app comes back.

Nothing is stamped during a `/replay` (a ghost's acceptance would be a real
op), before the loop has state (the next state change retries), or twice
for one build (the acceptance already covering the server build is the
guard, and the stamp in flight is a second one).

## parked

- *pause updates for an hour*: a control that holds the acceptance back for
  a while — a sibling of this node under `/consent-once`, reading a
  user-scoped "not before" time in `allowed()`.
- per-feature consent, `/policy`'s named refinement: the ticks `/queue`
  stores start steering what runs; not this node's concern.
- *tell me what changed after an auto update* extends `/push`'s notice or
  `/attention`'s flash with the release subjects; not built.
- *let me undo an update* is not an update-policy matter at all: it is a
  version pin, which `/policy` names as a known limit of the freshness
  cache.

## glossary

- **stamped by policy**: an acceptance the instance records for its user
  because the update policy allows the build without review; identical in
  every downstream effect to one the update button records.

## code description

`by-policy.index.js` composes after `/consent-once`, `/review`, `/policy`
and `/watch` (provenance order) and owns one move: `feature_ByPolicy
.accept()` sends `AcceptUpdate {build: server}` — the update button's own
event, claimed by `review.rs` — when `/watch` knows a newer build, the
loop has state and an instance, no `/replay` is running, the acceptance
does not already cover the build, no stamp is in flight, and `/policy`
answers that no consent is needed.

`allowed()` asks `feature_Policy.consentNeeded()` and remembers the verdict
per (server build, policy): under `fixes` the question costs a fetch of
`changes.json`, and the answer for a given build cannot change, so a
pending review does not re-fetch on every state change; picking another
policy changes the key and re-asks.

`quiet()` removes the handle's `update` class. It runs from the `/watch`
wrap whenever the policy allows the server build and the acceptance covers
it — this instance's stamp or another's — so the pulse never shows for a
build nobody is being asked about; the class is left alone under any
policy that is asking.

Three hooks call `accept()`: a wrap of `feature_Watch.check` (after the
original), a wrap of `feature_Loop.apply` (after the chain — the launch
decline and a policy picked mid-session both reach here), and a
`serviceWorker` `message` listener for `/attention`'s `{fm: 'attention'}`
hand-off whose body is `/push`'s build notice, which calls
`feature_Watch.check()` so the wrap does the rest. Every reference to
another feature is typeof-guarded; with `/policy` absent nothing is ever
stamped.
