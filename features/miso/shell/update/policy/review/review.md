# review
*one reviewed OK updates every instance: the awaiting build, itemised at the top of the feature list*

> (transcripts/2026-08-14-fm-spec-3.md#p83)
> earlier in the conversation (maybe yesterday even) I made a quip like "let's make updates automatic because having to select updates on each of my devices is a chore" - but I was wrong. what we actually need is a single update workflow that shows me the contents of the awaiting update" (which may integrate more than one), with things ticked or unticked according to policy; the awaiting would be in a section of its own at the top of the feature-list, and having reviewed it you hit the "update" button to accept the changes and update the build, across all your instances. So there's one "OK" but there still needs to be one.

## user

When an update is waiting, the top of your feature list shows exactly what's in it — every feature it changes, each readable and tickable, even if several releases arrived while you were away. Look it over, untick anything you'd rather not have (choices take effect when choices become live), and hit **update** once: all your devices take the new build. One OK, and it's yours everywhere.

## spec

The per-device chore that motivated automatic updates was the wrong cure; the right one is **one consent for the whole fleet**. When the server is ahead of this instance, the feature list opens with an **awaiting update** section: every feature the pending build(s) touched — however many releases the gap spans — as normal chooser lines (build number, intro on tap, tickbox honouring the usual rules). Having reviewed, one **update** button accepts: the acceptance is recorded *per user*, travels to all their instances, and each instance applies the build on arrival — one OK, everywhere. The awaiting set is computed from the same unified numbering the list already speaks: the server's live `tree.json` versus the running build — no new bookkeeping. The old always-visible update button stands down while the section is showing. `/policy`'s modes keep their meaning for defaults and for what may proceed *without* review; the doctrine shift is that "automatic" was answering the wrong question — the chore was per-device consent, not consent itself.

## glossary

- **awaiting update**: the set of features touched by builds newer than the one running — the reviewable content of the next update.

## code description

`review.rs` claims the `AcceptUpdate` event: it stamps `update_accepted` (a user-scoped var) with the accepted build number — `/scope` carries it to the user's other instances.

`review.index.js` composes after `/chooser` (provenance order) and wraps `feature_Chooser.mount`: after the list renders, it compares `feature_Update.server` against `running`; when ahead, it prepends the **awaiting update** section — a header naming the pending build, the pending features' rows (`feature_Chooser.row()` reused, ticks and taps included: any node whose `build` in the freshly-fetched live `tree.json` exceeds the running build), release-line rows for any pending build no feature represents (its changes.json subject — scaffolding and out-of-tree fixes; an update never lists nothing), and the **update** button — and hides the panel's standing update button while present. The button sends `AcceptUpdate {build}` and applies locally (the version-stamp + cache-clear + reload ritual). Every instance also watches `update_accepted` on apply: an acceptance newer than the running build applies the update there too — the one OK arriving over sync. Replay-guarded; failure to fetch the live tree degrades to the plain update button.

Two seams carry the release lines, refactored in for `/bookkeeping` and permissive by default: `releases(changes, running, server, covered)` chooses which changes.json entries earn a line, and `count(running, server)` says how many releases the header claims.
