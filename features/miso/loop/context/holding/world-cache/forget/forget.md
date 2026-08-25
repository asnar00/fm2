# forget
*signing out forgets the device's copy of your world*

> (transcripts/2026-08-25-accounts.md#p54)
> sounds like we need to work on some kind of "upgrade code without restarting server" workflow so this doesn't happen - once we have multiple users making changes, we'll want that to be silky smooth
> *(the residual `/world-cache` named on delivery: "no eviction on sign-out — the record survives logout until someone signs in as someone else"; fixed in the run under the residuals rule, #p50)*

## user

Log out and this device forgets your world. Someone else signing in here opens on theirs, never on yours.

## spec

`/world-cache` keeps the device's copy of the world in IndexedDB and refuses a record stamped with another name — but a record left behind by a sign-out sat on the device until then. This node wipes it at sign-out: it takes the panel's logout handler at load (`/panel`'s `#logoutBtn` lives outside `#app`, so the handler is the page's, not the loop's) and calls `feature_WorldCache.wipe()` before the original runs. Untick and the record outlives a sign-out again.

## glossary

(no new terms)

## code description

`forget.index.js` — wraps `#logoutBtn.onclick` at load: wipe, then the original handler.
