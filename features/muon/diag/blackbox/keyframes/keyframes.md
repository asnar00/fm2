# keyframes
*periodic state snapshots, so replay never starts from the beginning*

> (transcripts/2026-08-13-fm-spec.md#p101)
> it strikes me that besides sending events, we should also occasionally send the system state (in this example, the tap count) so that if we want to reproduce a fault, we don't have to start right from the beginning of a (potentially multi-hour) run. "keyframes", if you will.

## spec

The video-codec model applied to the /blackbox/: event entries are small deltas; every 30 seconds or 50 events (whichever comes first) a **keyframe** — the full state — is captured alongside them. Reproducing a fault then starts from the nearest keyframe before it, not from the start of a multi-hour run. Keyframes live in the same rotating log, ship in the same batches, and are trimmed with the window (keeping the newest one at-or-before the window start, so the retained window always replays). Untick this node and the blackbox honestly degrades to replay-from-boot.

## user

Nothing to do — faults are reproducible from within seconds of where they happened, however long the session ran.

## glossary

- **keyframe**: a full state snapshot between event deltas; replay starts from the nearest one before the moment of interest.

## code description

`keyframes.js` wraps `feature_Blackbox.record` (the JS extension idiom): after the original runs, if 30 seconds or 50 events have passed since the last snapshot, it appends `{t, state}` to the log's keyframes. Capture cadence lives in this node (`everyMs`, `everyEvents`); storage, trimming and shipping of keyframes are `/blackbox`'s machinery.
