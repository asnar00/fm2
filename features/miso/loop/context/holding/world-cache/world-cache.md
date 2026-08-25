# world-cache
*the world survives a reload: the device keeps its own copy and starts from it*

> (transcripts/2026-08-25-accounts.md#p54)
> sounds like we need to work on some kind of "upgrade code without restarting server" workflow so this doesn't happen - once we have multiple users making changes, we'll want that to be silky smooth

## user

Reload the app — an update, a swipe-away, a crash, a train tunnel — and your world is already there: your card, your settings, your tools, exactly as they were a moment ago. There is no blank moment while the server is asked. If you are offline the app opens anyway, on yesterday's world, and the moment the server answers it quietly agrees with it; anything you changed in between is sent on.

## spec

A reload used to throw the world away. The page came up empty, asked the server for everything, and lived in an *empty world* until the answer came — which on a restarting server is seconds, and offline is forever. That window is where the build-292 card was lost (`/guard`, `/me/patient`) and it is the second of the three pieces ash's #p54 answer named: piece 1 is the server handing over instead of restarting; this is piece 2, the device's half. With both, an upgrade lands between two keystrokes.

The device writes its world down on every applied turn. The record is the whole var table as one value — one write of one consistent moment rather than a var at a time — kept in IndexedDB under `miso-world`, keyed by the person it belongs to, ~200KB at today's sizes. The write is trailing by 300ms, because typing is many turns and only the last is worth the disk.

The world reaches the page through the **payload**, not the state: `event_payload` is the single wrapper every turn's answer passes through, so one link catches boot and every event, and the var table never crosses back into the wasm on the following event the way a state key would.

On boot the record is read **before the first paint**. The paint seam is held shut across the fresh world's turn, the records go in through the door a join uses — `set_from_json`, assignment, no op queued, idempotent — and the hydrate's own turn does the first paint the user sees. So the empty world is never rendered, not even for a frame, and not by luck of the veil covering it.

`/veil` is told the truth in two parts: the world is **shown** (`fm-cached` lifts the hidden `#app`) but not **joined** (`fm-joined` still waits for the real join). That distinction is what keeps `/me/patient`'s rule intact — no card is ensured against a world the server has not confirmed — and it turns that rule from a guard into a belt-and-braces, because there is no longer an empty world to ensure against. If the join times out, `/veil`'s own banner appears over the cached world and says exactly what is true: showing local state.

Reconciliation needs no new mechanism, and that is the point of hydrating through the join's door. The join arrives and assigns the server's value over the cached one wherever the two differ — the server is the authority for `user` scope, per-card newest-edit-wins is `/guard`'s business on the way in — and the ops made while offline are in `/messaging`'s persistent outbox, which replays them when the connection returns. A hydrate queues nothing, so it cannot echo, and applying the same record twice is applying it once.

Two kinds of var are not cached. An **absent** one has never been touched on this device, and writing it back would set its presence bit and stop it inheriting from the layer for good. A **global** one's authority is the layer, so the device's own field for it is nobody's value. **Device**-scoped vars, which a join skips because the server is not an authority about anyone's phone, are cached — after a reload this is the only place they can come back from.

## hostile cases

- **Server stopped, then started.** The card is on screen from the cache; the join lands when the server returns; no second card is minted, because `patient` waited for the real join and the world was never empty anyway. An edit made while the server was down is in the outbox and reaches it.
- **The device changed hands.** The record carries its owner. A boot that knows a different name wipes it and hydrates nothing. A boot that knows no name — offline — hydrates, exactly as the localStorage-backed settings on that device already do.
- **An update dropped a var.** The setter refuses a `(path, name)` the composition no longer declares and the record is skipped; the rest of the world still arrives.
- **A node was regrouped.** Records address vars by node path, so a moved node's cached value is orphaned in the same way its op log is — see the risk below.
- **No IndexedDB, or a broken one.** Every path is wrapped: the boot falls through to what it did before this node existed.

## open risks

- **Path-keyed state and regroups.** A var is addressed by its node's tree path, here and in `/remember`'s op log. Moving a node that declares a var therefore orphans that var's stored value — the regroup that made room for this node moved no declaring node, so nothing was lost, but the general case is now a real cost of regrouping and belongs to `context`'s open question about versioning across builds.
- **The cache is not evicted on sign-out.** Logging out reloads into the login page and the record stays on disk until somebody signs in as somebody else. It is the same exposure the localStorage settings already carry, and it wants the same answer: one place that clears a device's local state.

## glossary

- **hydrate**: to fill a fresh world from the device's own record, before the first paint and before the join.

## code description

`world-cache.rs`, `event_payload` /extension/: adds `world` to the payload every turn carries — the generated snapshot, filtered — beside `state` and `html`.

`world-cache.rs`, `world_cache_records`: the filter and the record shape (`path`, `name`, `value`), and the argument for what is left out.

`world-cache.rs`, `update` /extension/: claims the `WorldHydrate` event and applies its records through `set_from_json` **before** delegating to the chain beneath, so `payload`'s republish carries them into the very paint that follows.

`world-cache.index.js`, `feature_WorldCache`: the store (`open`, `load`, `wipe`, `note`, `write`), the ownership check, the held paint seam (`hold`, `release`), and `reveal`.

`world-cache.index.js`, the load-time block: wraps `feature_Loop.boot` to hydrate around it, and `feature_Loop.apply` to write each turn's world down.

`world-cache.index.css`: lifts `/veil`'s hidden `#app` for a cached world and hides the veil itself.
