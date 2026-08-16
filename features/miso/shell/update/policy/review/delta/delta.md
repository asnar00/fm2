# delta
*updates fetch only the delta: evict what changed, keep what didn't, reload only for code*

> (transcripts/2026-08-15-fm-spec.md#p6)
> let's do all three :-)

## user

Updates only download what actually changed — a small fix costs a few
kilobytes, not the whole app, and never re-costs the big speech model.
And when a release only touches the server, your device just notes the
new build number: no reload, nothing to wait for.

## spec

The apply ritual's eviction was a sledgehammer — `caches.delete('miso')`
threw away every cached file, the ~130MB speech model included, to
refresh a few kilobytes of app. This node makes the update a **delta**.

Deploy publishes `hashes.json`: a content hash per site file
(`version`, `changes.json` and `hashes.json` itself excluded — they are
always-fresh data, not cached app files). Each instance remembers the
manifest it's running. An apply diffs the two and evicts exactly the
paths that changed or vanished (`index.html` also evicts the cached
`/` navigation; query-keyed entries evicted with `ignoreSearch`) — the
model survives every update it isn't part of. Either manifest missing
degrades honestly to the old full eviction.

And when the diff contains **no code** — nothing among `index.html`,
`client.wasm`, `sw.js`, `login.html`, `install.html`, `f/` — the
update is applied **quietly**: changed data files evicted, version
stamped, the pulsing handle becalmed, no reload at all. A server-only
release costs a client nothing but a version number.

The seam this claims is `/update`'s `evict` (refactored out of the
three ritual sites for this node — behaviour preserved), so the review
button, the acceptance arriving over sync, launch, and the standing
update button all become delta-precise together. Pairs with the split
composition (fmlink `SPLIT_PAGES`, same prompt): per-feature fragment
files mean a typical release's delta is a few small files, not the
whole app.

## glossary

- **delta**: the set of site files whose content hash differs between
  the running build and the server's.
- **quiet apply**: taking a build whose delta contains no code — stamp
  and evict data, skip the reload.

## code description

`delta.index.js` keeps the running manifest in
`localStorage.misoHashes`. An instance that has never seen one learns
its own right after `/update`'s launch settles — and only while it
runs the build the live manifest describes: seeding with a pending
build's manifest would make the next delta read empty against old
code.

It replaces `feature_Update.evict`: fetch the live `hashes.json`
(no-store), diff against the stored manifest, `cache.delete` each
changed/removed path (`ignoreSearch: true`; plus `/` for
`index.html`), store the new manifest. Missing either manifest: full
`caches.delete('miso')`, then store what was fetched.

It wraps `feature_Review.apply`: the diff is computed first; if both
manifests are known and no changed path is code, `quiet()` runs —
`feature_Update.evict()` (the precise one), version stamped into
localStorage and `feature_Update.running`, the handle's update pulse
cleared — and the wrapped chain (`/seamless`'s stash, the base
reload) is never called. Any code in the delta falls through to the
full ritual, whose eviction step is already delta-precise via the
seam.
