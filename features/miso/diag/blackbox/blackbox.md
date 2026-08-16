# blackbox
*every event is recorded locally, always; shipped to the server when possible*

> (transcripts/2026-08-13-fm-spec.md#p100)
> Remember though: we want blackbox to work even when not connected to the internet; so if we're connected, events should travel to the server, but they should also always go into some kind of rotating log (to avoid memory blowouts, maybe a few minutes in duration).

## user

Nothing to do. Your recent interactions are always recoverable for debugging: the last few minutes live on the device even with no signal, and reach the server whenever a connection exists. Tap-by-tap replay of a problem is possible from either copy.

## spec

The always-on flight recorder, offline-first by design. Every turn of the `/loop` loop appends a lean `{t, event}` delta to a **rotating local log** (localStorage): bounded by age (last 5 minutes) and count (500 entries). Full state lives in a keyframes array — boot state is keyframe zero, `/keyframes` adds periodic ones — trimmed so the newest keyframe at-or-before the window start always survives: the retained window is always replayable, with no unbounded growth, ever, connected or not. When the network permits, unsent entries **also** ship to the server in batches (every 10s while visible, on regaining connectivity, and on page-hide with a keepalive request); the send watermark survives restarts, so a session that ended offline — or crashed — ships its final minutes on the next launch. The server half (same node) ingests batches, cookie-gated, into a size-rotated log on the mini.

## glossary

- **rotating log**: a bounded record that discards its oldest entries as new ones arrive, holding the recent window only.
- **baseline keyframe**: the newest state snapshot at or before the retained window's start, from which the logged events replay.

## code description

`blackbox.js` wraps the `/loop` loop in the JS extension idiom — reassigning `feature_Loop.send`/`apply` around the originals: each event appends a lean `{t, event}` to the ring; the first `apply` records boot state as keyframe zero. Trimming bounds entries by age and count, and keeps the newest keyframe at-or-before the window start. The log persists under `localStorage.misoBlackbox`; `flush()` posts keyframes and entries past the `sentT` watermark to `blackbox/events` (keepalive on page-hide), advancing the watermark only on success; boot flushes any previous session's leftovers. A `paused` flag lets `/replay` suspend recording during playback.

`blackbox.rs` is the server half: a `route` /extension/ ingesting `POST blackbox/events` — cookie-gated (event streams are user data), body-capped, appended as `<ms> <who> <batch>` lines to `/tmp/miso-blackbox.log`, size-rotated like `/diag`'s log.
