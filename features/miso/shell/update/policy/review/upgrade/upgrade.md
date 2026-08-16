# upgrade
*the review workflow: proposed additions arrive pre-ticked by policy; one upgrade button*

> (transcripts/2026-08-15-fm-spec.md#p2)
> 2) update workflow (review proposed feature additions auto-ticked by policy, press "upgrade" button

## user

When an update is waiting, brand-new features in it are marked **new**.
Whether they arrive ticked is your updates setting: on **automatic**
they're on unless you untick them; on the other settings they wait for
your tick. Press **upgrade** once and your choices are kept and every
device you own takes the build.

## spec

The awaiting update section becomes a real review: rows that are
**proposed feature additions** — nodes that did not exist in the running
build — wear a `new` badge, and their tick arrives set by `/policy`:
**automatic** pre-ticks them, **fixes auto** and **ask me** leave them
unticked for the user to opt in (an explicit tick the user has already
stored always wins over the policy default). Rows for existing features
that a pending build merely touched keep the user's standing tick — the
tick means "this feature, on or off for me", so a fix never changes it.

While the section shows, an addition's tick is a **draft**: tapping it
flips the shown state locally and stores nothing. The button reads
**upgrade**; pressing it commits the draft — any addition whose shown
tick differs from the user's stored choice is stamped into
`feature_ticks` (durably queued, so the reload cannot lose it) — then
accepts the build as `/consent-once` provides: one OK, all devices.

Which rows are additions comes from the release record: deploy now
stamps each changes.json entry with the feature node paths the commit
touched (`paths`) and the node specs it added (`added`) — the
path-stamping named at #p54, built here. Pending builds the record
doesn't cover degrade honestly: their rows just keep today's behaviour.

`/policy`'s enforcement duties are gone (consent is `/consent-once`'s);
its picker now answers only this pre-ticking question. That `fixes
auto` and `ask me` currently pre-tick identically is recorded as an open
question for the picker's future shape.

## glossary

- **proposed feature addition**: a node a pending build introduces —
  present in the awaiting update, absent from the running build.

## code description

`upgrade.index.js` wraps `feature_Review.section`: after the section
renders it runs `dress()`, and a `feature_Loop.apply` wrap re-runs
`dress()` after every state change (composing after `/chooser`, so it
lands after `reflect()` re-asserts stored ticks).

`dress()` fetches changes.json once per section life and collects
`added` paths from entries with `build > running`. Each matching
awaiting row is badged `new` and its tick taken out of the live-toggle
loop (`data-ev` removed — a review is a draft): the shown state is the
user's in-review choice (`chosen`, session-local), else their stored
explicit tick, else the policy default — `automatic` pre-ticks. A tap
on a draft tick flips `chosen` and re-dresses; nothing reaches the
store. `dress()` also renames the accept button to **upgrade** and
wraps its click: `stamp()` first sends an `ftick_` click — the tick's
own event, riding `/comms`'s durable outbox across the reload — for
every addition whose shown tick differs from the stored effective
state, then `/review`'s accept runs.

`upgrade.index.css` styles the `new` badge.
