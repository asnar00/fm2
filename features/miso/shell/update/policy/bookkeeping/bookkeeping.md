# bookkeeping
*commits that change no product are not release items*

> (transcripts/2026-08-16-fm-spec.md#p3)
> I notice the update contains "new releases" that aren't actually fixes or changes, stuff like "handover:" and "notes:" - those shouldn't appear in the release item list.

## user

The update review now lists only things that actually changed: features and
fixes. Notes-to-self, diary entries and session records no longer appear as
"releases" — they were never updates to your app in the first place.

## spec

The release classifier (specced at `/policy`: releases classify themselves
from the tree discipline) gains a third kind, **docs**: a commit whose every
touched file is repo bookkeeping — root-level markdown (notes.md,
handover.md, ideas.md, deploy.md, agents.md…) or `transcripts/` — changed
nothing on any device, so it is not a release item. Deploy still writes an
entry per commit (build numbers are commit counts, and `/policy`'s
fixes-mode coverage check needs the numbering gap-free); the entry simply
carries its honest kind, and the awaiting-update list skips it. A docs
commit is never `feature`, so under the `fixes` policy a doc-only build
auto-applies — correctly, since there is nothing to review. Anything
touching `features/`, `products/`, or `tools/` keeps its existing
classification: scaffolding and out-of-tree fixes still show their release
line (`/review`'s "an update never lists nothing" stands — for commits that
did something).

## glossary

- **docs commit**: a commit touching only repo bookkeeping (root markdown,
  transcripts) — counted in the build number, invisible in the release list.

## code description

The classifier half lives in deploy scaffolding (the standing arrangement
noted at `/policy`): `deploy.sh`'s changes.json writer stamps `kind: "docs"`
when every path a commit touched matches root-`*.md` or `transcripts/`;
`feature` and `fix` classify exactly as before.

`bookkeeping.index.js` owns the display half: it redefines
`feature_Review.releaseWorthy` (the seam `/review`'s release-line filter
routes through, refactored in for this node; base accepts everything) to
refuse entries of kind `docs`. Typeof-guarded; without `/review` in the
composition it does nothing. Unticked, docs entries reappear as release
lines — the old behaviour, intact.
