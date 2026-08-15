# seamless
*upgrades without disruption: tasks finish first, and you come back to where you were*

> (transcripts/2026-08-15-fm-spec.md#p2)
> 3) minimal disruption upgrade [retains system state / place / tasks underway]

## spec

Applying a build is a reload, and a reload today interrupts whatever the
instance is doing and forgets its local state. This node makes the
apply polite in both respects.

**Tasks finish first.** An accepted build does not apply while the
instance is mid-task: recording, playing back, or transcribing a note.
The apply defers, and fires on the first state change after the work
completes. Being the newest node it knows its elders — busyness is read
from the features that can be busy (`/dictate`'s mic and speaker,
`/phone`'s transcriber), typeof-guarded so any of them may be absent;
causality bounds extension, so an older feature could not have extended
a busy-chain this node might have declared instead.

**The instance comes back where it was.** Just before the reload, the
whole loop state is stashed (keyed by the build being applied); on the
first apply after boot, a stash matching the now-running build merges
under the fresh state — fresh keys the stash lacks keep their new
defaults, everything else resumes — and a nudge event re-renders so
the screen shows the resumed state, open tool included (`/restore` sees
the place already open and stands down). The stash is consumed once;
a stash for any other build is discarded. `/join`'s later arrival
still wins for user-scoped state — convergence is unchanged.

## user

Updating never barges in: if you're recording, listening, or a note is
still transcribing, the update waits for the task to finish. And when
it lands, you're back exactly where you were — same tool, same counts,
same tasks — not at a fresh front door.

## glossary

- **mid-task**: recording, playing back, or transcribing — states an
  apply must not interrupt.

## code description

`seamless.index.js` wraps `feature_Review.apply` (the one ritual every
accepted build passes through — the upgrade button and the acceptance
arriving over sync both land here). `busy()` reads the elder features'
live flags. Busy: the wanted build parks in `deferred` and the wrap
returns. Idle: the loop state is stashed to `localStorage.muonStash`
as `{v, state}` and the original ritual runs.

A `feature_Loop.apply` wrap does two jobs. It retries: a parked
`deferred` fires on the first state change that finds the instance
idle. And it rehydrates: on the first apply after boot (state was null
before the call), a stash whose `v` equals the running build is merged
beneath the fresh state — stash values win only for keys the fresh
state also knows nothing fresher about, via `Object.assign(stashed,
fresh-init-keys-absent-from-stash…)` semantics: fresh keys absent from
the stash keep their defaults, stashed keys return otherwise — then a
`seamless_resume` event (unknown to every update chain, so a no-op)
nudges a re-render of the resumed state. The stash is deleted in the
same breath, matching or not.
