# keep-worker
*a rig that keeps the service worker, so the cache path itself can be under test*

> (transcripts/2026-09-02-self-check.md#p19a)
> hm ok. Does that mean it would be useful to have a real phone hooked up to the mini instead of the simulator, so we can debug service worker issues?

> (transcripts/2026-09-02-self-check.md#p20)
> ok, do the keep-the-worker rig mode now

## user

Nothing on the app. This is a switch for the builder's simulator: with it on, the simulator keeps the app's offline copy the way your phone does, so the builder can replay a mixed-cache problem on the simulator instead of asking you to try again.

## spec

A rig drops the page's service worker and caches at load — `/rig`'s rule that a rig runs the code it was given. That rule made Saturday's failure untestable on the simulator: the mixed service-worker cache after rapid updates was the fault, and no rig could hold one. The first run of `/self-check` on the iPhone simulator said so — `sw none`, every fragment `uncached`, nothing hashed. Ash's question (#p19a) was whether a real phone on the mini was the answer; it is not: the simulator runs the worker fine, the rig was removing it.

This node is the switch. A rig server started with `MISO_RIG_KEEP=1` answers `diag/rig` with `{"rig":true,"keep":true}`; the page's arming reads `keep` and leaves the worker and the caches alone. Everything else a rig does — readout, drive, the fast black-box flush, `js` — is unchanged. Off (the env var absent, or this node unticked) a rig behaves as before: the answer carries `keep:false` or no `keep` at all, and `/rig` drops both.

The seam: `feature_RigPage.arm(answer)` now takes the rig's answer and sets `feature_RigPage.keep`; the drop is guarded by it. `/rig` gains the seam; this node is the only thing that sets it true.

## hostile cases

- `MISO_RIG_KEEP=1` on a server that is not a rig (`MISO_RIG` unset): `rig:false`, `keep:false` — nothing arms, nothing is kept beyond what a plain server keeps.
- Through the tunnel: a rig never answers `rig:true` there, so `keep` is never true either.
- Node unticked: the answer is `/rig`'s plain `{"rig":…}`; `arm` sees no `keep` and drops the worker as before.
- A stale worker from a previous rig build stays installed across a relink: that is the point — the self-check under this mode names the stale fragments; `MISO_RIG_KEEP` unset for one launch clears it.

## glossary

- **keep mode**: a rig started with `MISO_RIG_KEEP=1`, which keeps the page's service worker and caches.

## code description

`keep-worker.rs` — `route` /extension/: answers `diag/rig` itself with `rig` (localhost and `rig_on()`, as `/rig` does) and `keep` (`rig` and `MISO_RIG_KEEP=1`); everything else goes to `existing.route(r)`.

`/rig`'s `rig.page.js` carries the seam: `keep: false`, `arm(answer)` sets it from the answer and only drops the worker and caches when it is false; the load-time fetch passes the answer through.
