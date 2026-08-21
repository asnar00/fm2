# deadline
*slow networks degrade to last-known-good*

> (transcripts/2026-08-13-fm-spec.md#p54)
> we could add a wrinkle to that: sometimes we'll be in very low-bandwidth mobile-network situations, so large data would come from the cache and small stuff from the network?

## user

On terrible mobile signal the app stays responsive with the last good copy, quietly updating itself for the next visit.

## spec

What matters on a bad network is time, not size — and iOS offers no bandwidth API, while a clock measures size×bandwidth directly. So `/fresh` gains a 1.2-second freshness deadline: a network response that arrives in time always wins; past the deadline, a cached copy serves while the fetch completes in the background and refreshes it for next time. Offline is the deadline missed instantly; nothing-cached waits on the network however long. One rule covers fast, slow, and offline.

## glossary

- **freshness deadline**: the time (1.2s) the network is given to deliver before a cached copy is allowed to answer.

## code description

This node owns `deadline.sw.js`: `const feature_Deadline = { ms: 1200 }` — the entire feature is one composable constant. `/fresh` races the network against it only when it exists; unticking this node genuinely reverts to pure network-first.
