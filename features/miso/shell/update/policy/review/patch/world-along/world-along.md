# world-along
*an in-place update carries the world with it*

> (transcripts/2026-09-03-invite-test.md#p39)
> the taps button came back

> (transcripts/2026-09-03-invite-test.md#p41)
> force-quit and reopen makes it go away

## user

An update that lands without a reload leaves everything as you had it: a tool
you switched off stays off, your current project stays current, the map keeps
its place. Before this, such an update quietly put every setting back to its
default until the next relaunch — the taps button reappearing was the visible
one.

## spec

`/patch` hot-swaps `client.wasm` when an update changes only the Rust, and
its spec said *"the loop state is not stashed or rehydrated — it is simply
never lost."* That was true of the Elm state string, which lives in JS. It
stopped being the whole truth when `/context` moved the world — every
declared var, the user's switches, the current project, the device's open
tool — into the wasm module's own memory. A fresh instance is a fresh world:
every var at its default, the global layer at epoch 0. Nothing rehydrates it,
because `/world-cache` hydrates once at boot and the join ran long ago. So
after each of today's three in-place updates ash's phone showed the taps
tool their world had switched off, and sent tap counts minted under epoch 0
that the server dropped (#p39, diagnosed by the black box and the diag log:
a launch line for 572, none for 573). A force-quit fixed it (#p41) because a
boot hydrates.

**The world rides along.** This node wraps `feature_Patch.swap`: it keeps
the newest world records the page has seen (every applied turn's payload
carries them — the same records `/world-cache` writes down), holds the paint
seam across the swap exactly as boot does, and once the new module is in
place sends `WorldHydrate` with those records — `/world-cache`'s own event,
whose Rust link assigns each record through `set_from_json`, queues no op,
and paints. The first frame the new module draws is the world, never the
default. Then `feature_Resume.join()` rejoins, so the server confirms the
world and the global layer comes back at its real epoch.

**Why the payload's records and not the cache.** The IndexedDB record trails
by 300 ms and is read asynchronously; the last payload is what the page has
right now, and it is the same shape. Absent and global vars are not in it,
which is right: an absent var must stay absent, and the layer is the join's
to bring.

## hostile cases

- **No world seen yet** (a swap before the first turn). Nothing to hydrate;
  the swap proceeds as before and the join is asked for anyway.
- **`/world-cache` unticked.** `WorldHydrate` has no Rust link; the event is
  an unknown type and falls through the chain, harmless; the join still
  repairs the world. The hold/release seam is guarded by `typeof`.
- **`/resume` unticked.** No rejoin; the hydrate alone restores every
  cached-shape var; the layer waits for the next foreground.
- **The swap fails.** The wrapped function returns false, `/patch` falls
  through to the full reload, and this node sends nothing.
- **This node unticked.** `/patch` behaves as before: a fresh world until
  relaunch.

## code description

`world-along.index.js` — wraps `feature_Loop.apply` to remember the last
payload's `world`; wraps `feature_Patch.swap` to hold the paint seam, run the
swap, and on success send `WorldHydrate` with the remembered records and ask
`feature_Resume.join()`.
