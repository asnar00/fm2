# deadline
*slow networks degrade to last-known-good*

> (transcripts/2026-08-13-fm-spec.md#p54)
> we could add a wrinkle to that: sometimes we'll be in very low-bandwidth mobile-network situations, so large data would come from the cache and small stuff from the network?

## spec

What matters on a bad network is time, not size — and iOS offers no bandwidth API, while a clock measures size×bandwidth directly. So `/fresh` gains a 1.2-second freshness deadline: a network response that arrives in time always wins; past the deadline, a cached copy serves while the fetch completes in the background and refreshes it for next time. Offline is the deadline missed instantly; nothing-cached waits on the network however long. One rule covers fast, slow, and offline.

## user

On terrible mobile signal the app stays responsive with the last good copy, quietly updating itself for the next visit.

## glossary

- **freshness deadline**: the time (1.2s) the network is given to deliver before a cached copy is allowed to answer.

## code description

In `assets/sw.js`: `Promise.race` between the network fetch (with cache refresh) and a deadline timer resolving to the cached copy; no cached copy means the race is skipped and the network is awaited.
