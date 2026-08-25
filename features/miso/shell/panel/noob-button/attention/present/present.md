# present
*if you are looking at the app, the screen updates and nothing rings*

> (transcripts/2026-08-25-accounts.md#p26a)
> if the app is focused, we shouldn't send a notification - it should just update the screen

## user

While miso is open in front of you, builder messages arrive on screen — the row updates, the lozenge pulses — and your phone does not buzz. Notifications are for when you are not looking.

## spec

`/attention` made the foreground/background fork in the service worker: the push was always *sent*, and the worker chose whether to display it. On ash's phone the push rang while the app was focused (#p26a) — the named risk in attention.md, that a browser insists a push shows something, met a phone. So the fork moves to where the wire starts: **a user whose page is listening is not sent a push at all.**

The server can see who is listening. `/messaging`'s long-poll (`POST /msg/wait`) is a page that is running and waiting for events; a page re-waits the moment a wait returns, so a present user always has a wait open or is between two by milliseconds. This node marks a user *present* on every wait's entry and exit, keyed by their world key, and `attention_push_to_user` returns without sending when the owner was marked within the last 30 seconds (one wait cycle is ~25s). The screen update itself is `/to-owner`'s relay, unchanged. The service-worker fork stays as the second line, for the case where a wait died with the page still visible.

**Known edge, recorded:** a phone that has just been backgrounded may hold a wait open until it times out (up to 25s), so a stamp landing in that window updates the screen it can no longer see and sends no notification. A desktop tab hidden behind another keeps waiting and counts as present — desktop notifications were never the target. Neither is a rung this node owes; the honest signal for both is page visibility reported to the server, a later refinement.

## hostile cases

- No cookie (localhost tooling) → empty key → never marked present, never suppressed.
- A user with no live wait for 30s → "away", the base sends as before.
- Server restart → presence empty → first stamp after boot notifies even a present user, until their next wait lands (within seconds).

## glossary

- **present**: a user whose page has held a `/msg/wait` within the last 30 seconds.

## code description

`present.lib.rs` is the verbatim library: a process-global `Mutex<HashMap<String, u64>>` of world key → last-seen ms, with `presence_touch` and `presence_recent`.

`present.rs` extends `msg_wait` (touch on entry and exit; `presence_key` turns the cookie into the `phone:` world key exactly as `/per-user` does) and redefines `attention_push_to_user` to return early, with a log line, when the owner is present; otherwise it logs "away" and calls `existing`.
