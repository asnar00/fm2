# auto-export
*the bake checks its sources; the catalog checks its bake*

> (transcripts/2026-08-15-fm-spec-2.md#p6)
> we should make it so that if I go change the text file of a node in the feature, it auto-updates everywhere. In other words, the cache entry should be able to check whether its source file has changed, and update if so

## user

Change what a feature's documentation says, and everywhere those words
appear — the feature pages, the features list, the little cards —
says the new thing on its next look, without waiting for a redeploy on
your dev machine or an app update on your phone.

## spec

Every surface that speaks a node's documentation — the served
`/features/` pages, `tree.json`, a device's held catalog, the
long-press cards it feeds — is derived from the node's own files, and
each derivation now validates against what it derives from instead of
waiting for the right kind of update event.

On the server, `/features/*` answers from a bake. Where the source
tree sits beside the server — a dev machine running from the build
dir — a request under `/features` first compares the newest mtime
anywhere in `features/` against the bake's `tree.json`: sources newer
means the bake re-exports before the request is answered (about five
seconds, paid once per edit), and the embeddings follow along off the
request's clock. On the mini the sources are absent by design — a
release is a committed state — so the check finds nothing and the
deploy's bake stays the truth, at zero cost.

On the device, the export writes a `stamp` beside the bake — a short
hash of `tree.json`'s content. The chooser's held catalog revalidates
against it on each read: a tiny fetch, a refetch of the full tree only
when the stamp actually moved. Words reach live devices as soon as the
bake changes, no reload and no update-apply required; offline, the
stamp fetch fails quietly and the held catalog answers as before.

Named risks, both dev-only by construction and healed by the next
read: two requests racing the same re-export both run it, and the
export clears and rewrites its output dir, so a reader in that window
can catch a 404; and because the re-export runs inside whichever
request noticed first, a concurrent read (`/threads`) can pair a
mid-export tree with the other side's stamp — the held-vs-served
comparison then refetches one read later than ideal, never not at all.

## glossary

- **bake**: the static export of the feature tree the server answers
  `/features/*` from.
- **stamp**: the bake's content fingerprint, written beside it; the
  cheap question "did the words change?"

## code description

`auto-export.rs` extends `route`: a path under `features/` (paths reach
the chain slash-stripped by `clean_path`) first calls
`refresh_if_stale`, then hands to `existing.route(r)`. `refresh_if_stale`
resolves sources and tools relative to the server's working dir
(`../../../features`, `../../../tools`) and returns immediately when
either is absent — the mini's whole participation. Staleness is
stateless: `newest_under` (a recursive mtime walk) against the baked
`tree.json`'s mtime. Stale runs `export_features.py` synchronously —
the request that noticed waits for fresh words — then spawns
`embed_catalog.py` detached; a failed export logs once and serves the
stale bake rather than nothing.

`auto-export.js` is the device half: it wraps `feature_Chooser.load`
(redefinition + the saved original), fetching `features/stamp`
(`no-store`) before delegating; a stamp that moved since the last read
nulls the held catalog so the original refetches, and the new stamp is
remembered after. No stamp — an old server, offline — means no
revalidation, exactly the prior behaviour. Guarded on the chooser
existing.

The stamp itself is written by `tools/export_features.py` at the end
of `main()`, beside the tree it fingerprints.
