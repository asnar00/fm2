# buildnum
*builds are named by a simple increasing integer*

> (transcripts/2026-08-13-fm-spec.md#p51)
> instead of a long build code, can we have a simple increasing integer build number? Like Claude Code does, actually?

## spec

The /deploy stamp/ is the repo's commit count at deploy — a plain increasing integer needing no counter file (a release is always a committed state, so every deploy is at least one commit ahead), and each /build number/ still names an exact commit for debugging. The what's-changed list tags each entry with its build number the same way.

## user

Builds read as "build 51", counting up by however many commits a release contained. The `/panel` maps any build number back to its changes.

## glossary

- **build number**: the commit count at deploy — a monotonically increasing integer naming each release.

## code description

One line in deploy.sh: `git rev-list --count HEAD > site/version`; `changes.json` generation derives per-entry numbers as count-minus-offset.
