# replay
*any recorded session stands up in a test browser and plays back with real timing*

> (transcripts/2026-08-13-fm-spec.md#p104)
> I think replay would be nice. Ideally, we could take any session and stand it up in a test browser (ooh, maybe even a proper iphone simulator running the pwa), and then replay user events with the proper timing.

## spec

Turns a /blackbox/ recording back into a live session. With `?replay=1`, the app seeds its state from the best /keyframe/ at-or-before the window's first event, then dispatches each recorded event through the `/events` loop at its original inter-event delay (`&speed=` scales time). Recording pauses during replay so a playback never records itself; the login redirect is suppressed (a replay is not a user session); a REPLAY badge marks the screen. The scaffolding launcher (`tools/replay.py`) pulls a user's batches off the mini over ssh, assembles the window into `/replay.json`, ensures the local server is running (localhost is ungated by design — no login needed in the test browser), and opens the URL — in a booted iPhone simulator when Xcode provides one, else the default browser. Replay covers the event-loop surface; as more of muon moves into `/events`, replay coverage grows with it.

## user

`python3 tools/replay.py` — your latest recorded phone session opens on the laptop and re-performs itself, tap for tap, pause for pause. `--simulator` stands it up in an iPhone simulator; `--speed 4` hurries it along; `--minutes 10` picks the window.

## glossary

- **replay**: re-driving recorded events through the same update chains from a seeded state — a reproduction, not a recording.

## code description

`replay.js` is the in-app driver: when `?replay=1` is present it pauses `/blackbox`, suppresses `/gate`'s login redirect (wrapping idiom, deliberate no-op), waits for the `/events` loop to boot, fetches `/replay.json`, seeds `feature_Events.state` from the chosen keyframe (a synthetic `replay-seed` event triggers the first render — unknown events pass harmlessly through the update chain), then schedules each entry with `setTimeout` at its scaled original offset, and shows the badge.

`tools/replay.py` (scaffolding) does the standing-up: ssh-pull the mini's blackbox log, filter by user tag, merge batches, cut the window, write `site/replay.json` (never deployed — deploy removes it), start the local server if absent, and open browser or simulator.
