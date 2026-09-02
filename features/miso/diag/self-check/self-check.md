# self-check
*after every launch the device reports which fragment versions it really runs, and whether the basics work*

> (transcripts/2026-09-02-self-check.md#p4)
> ok, let's kick off the next rung: the /diag boot self-check - could you explain in simple terms (I'm a "simple mind" ;-)

*(The rung was earned on Saturday — transcripts/2026-09-01-saturday.md#p31, where the readout proved the deployed build correct while the phone showed something else: a stale stylesheet in a mixed service-worker cache that nothing could see. The report's place on the phone is `/engineer`'s, the later prompt #p7 of the same session.)*

## user

Nothing on the app. After each launch your phone quietly tells the builder exactly which pieces of the app it is running and whether the basics work, so a "my phone shows something different" can be answered without guessing. To see the same report yourself: tap the nøøb lozenge, then the small gear.

## spec

Three phone-only divergences on Saturday (the safe-area gap, projects blank mid-update, a video post text-first) could not be diagnosed because nothing says what an installed PWA is actually running after rapid updates leave the service-worker cache mixed. `/diag`'s launch report carries only the build number. This node is the **boot self-check**.

After paint — once the loop has state, a beat later — the page fetches the live `hashes.json` (through `/delta`'s `fetchLive` when present; deploy.sh writes the manifest) and, for every path the manifest lists that is code (`/delta`'s `code()`: `index.html`, `client.wasm`, `sw.js`, the pages, `f/*`), hashes what this device holds: the service worker's cache entry (`caches.match`, what the app runs offline or under `/fresh`'s deadline); a path the cache lacks is fetched only when a worker controls the page (the cache should have had it, and the fetch passes through the worker and repairs it) — with no controller, a plain tab, it is counted **uncached** and left alone, so the check never downloads the app a second time. SHA-1 via `crypto.subtle`, cut to the manifest's 16 hex. Each **mismatch** (hash differs) and each **missing** path (neither cached nor fetchable under a worker) is named.

Three basics join it: the tap seam — `feature_Panel.buttonTap` reaches `feature_Panel.open` (probed with a stand-in, then restored) and the lozenge carries its handler; the boot veil lifted (`/veil` gone, `body.fm-joined`); no orphaned update wrapper — the seams `/delta` replaced at load (`feature_Update.evict`, `feature_Update.launch`) still resolve through it.

The result is posted as a second `diag/report` line, kind `self-check`: device id (a random 8-hex stamp the device keeps in `localStorage.misoDevice`), running build, server build, service-worker controlled, pwa, manifest present, fragment count, how many came from the cache and how many were uncached, the mismatch and missing lists **cut to their first 12 names** with the full counts (`nmismatched`, `nmissing`) beside them — `/diag` keeps 2KB per report and a wholly stale cache would name ~220 paths, the very phone the report exists for — the three basics, `ok`, and the time it took. The engineer text keeps the whole lists. The server keeps the latest per device beside the diag log (`/tmp/miso-self-check.json`) and answers `GET diag/self-check?n=N` with the newest N — open on localhost, **owner-only** (admin authority) through the tunnel. `text()` renders the same report as plain text; `/engineer` (the later node) puts it in the engineer section under the gear, so the phone's owner can read it there — nowhere else does it appear. This node itself draws nothing.

## hostile cases

- No `hashes.json` (a dev build, a rig, or offline): `manifest:false`, count 0, and the report says so; the basics still run and post.
- Cache API absent or refused, worker controlling: every path is hashed from a fetch through the worker (`cached 0`); no controlling worker: every path is `uncached`, nothing is fetched, `ok` is unaffected. `crypto.subtle` absent: hashes are `null`, counted as `unhashed`, never as mismatches.
- A fragment fails to fetch under a worker and is not cached: it is named under `missing`, and `ok` is false.
- Every fragment stale at once: the posted report names the first 12 and carries `nmismatched` = all of them, serialising under the server's 2KB cap (rig-proven); the log line stays valid JSON.
- The check is asked to run while running: the same promise is returned; it never runs twice at once and runs once per page load on its own.
- The report's POST fails: silent — the engineer section still shows the local result.
- Server: a report that is not JSON, or not kind `self-check`, is left to `/diag`'s log alone; a full store evicts the oldest device; an unwritable file is ignored.
- On a rig (`MISO_RIG=1`) the page drops its service worker and caches at load, so every fragment is hashed from the network: a clean rig reads `cached 0 · mismatched 0`. The rig evidence poisons the Cache API entry directly and re-runs the check in place.

## parked

- **Self-healing**: refetch a mismatched fragment and repair the cache (the next rung — "fix it for me" in the engineer section extends this report).
- The mini-side view of every device's last self-check extends the GET.

## glossary

- **self-check**: the report a device posts after each launch — what it runs and whether the basics work.
- **mismatch**: a code path whose held bytes hash differently from the live manifest's entry.

## code description

`self-check.index.js` — `feature_SelfCheck`: `run()` (once, re-entrant-safe) gathers `fragments()` and `basics()`, posts through `feature_Diag.report`, and refreshes the engineer section; `hashOf(path)` (cache, then fetch only under a controller), `posted(r)` (the wire shape: lists cut to `few` = 12), `manifest()` and `code()` (the delta's, when present), `text()` the plain rendering, `last` the latest full result (a later "fix it for me" reads it). At load it arms one boot timer.

`self-check.rs` — `route` /extension/: a `POST diag/report` of kind `self-check` is kept per device in `self_check_file()` (beside `diag_file()`) before the base handler logs it; `GET diag/self-check` returns the newest N, gated through the tunnel by `authed` and `authority_rank(context_user_of(…)) >= 3`.
