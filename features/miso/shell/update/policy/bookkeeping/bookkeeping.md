# bookkeeping
*diary commits are counted builds, not release items*

> (transcripts/2026-08-16-fm-spec.md#p3)
> I notice the update contains "new releases" that aren't actually fixes or changes, stuff like "handover:" and "notes:" - those shouldn't appear in the release item list.

## user

The awaiting-update list now shows only things that actually changed. Session
notes, handover records and formatting passes — commits whose subject says so —
no longer appear as releases you are asked to review, and the "N releases"
count in the header counts what the list actually shows.

If an update turns out to be nothing but bookkeeping, you see one line saying
so rather than an empty list. The full ledger behind the panel's changes
teaser ("every update") is unchanged: it is the record of every commit, and it
still holds them all.

## spec

A build number is a commit count, so every commit is a build — including
commits that changed nothing on any device. Those rode into `/review`'s
awaiting list as release lines, which is what the ask objects to.

Classification is by **declared subject prefix** (`notes:`, `handover:`,
`idea:`, `ideas:`, `format:`) — one convention with two consumers, the other
being `tools/export_features.py`, which already refuses to stamp a feature's
build from a diary commit. The author declares the class; nothing infers it
from file paths.

Filtering is at **display** time, not deploy time. `changes.json` stays a
complete, gap-free record of every build because `/policy`'s `fixes` mode
reads it to decide whether an update may apply unasked, and treats a build the
list does not cover as reason to ask: removing entries there would silently
turn a would-be-silent update into a prompt. This node touches only what the
awaiting list renders; the update semantics for real builds are untouched.

## glossary

- **diary commit**: a commit whose subject declares it repo bookkeeping
  (`notes:`, `handover:`, `idea:`, `ideas:`, `format:`) — counted in the build
  number, absent from the release list.
- **release line**: (from `/review`) the changes.json row shown for a pending
  build that no feature row represents.

## code description

`bookkeeping.index.js` wraps `feature_Review.releases`, the release-line seam
refactored into `/review` for this node: it drops entries whose subject is
diary-class, and records how many it dropped.

It also wraps `feature_Review.count`, the header's release-count seam,
subtracting that same number — so the header claims exactly what the list
shows.

`diary(c)` matches the subject prefix case-insensitively after trimming
leading space. An entry with no readable `text` is *not* diary: a malformed
line stays visible rather than being silently swallowed.

`summary(server)` covers the case where the whole gap is diary and no feature
row stands in for it: rather than an awaiting section with no items (`/review`
promises an update never lists nothing), one line reads "housekeeping — notes
and records only".

Both wraps are typeof-guarded on `feature_Review`; with `/review` absent the
node does nothing. Unticked, the seams keep their permissive defaults and
diary builds reappear as release lines.
