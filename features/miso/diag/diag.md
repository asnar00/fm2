# diag
*remote eyes on installed clients*

> (transcripts/2026-08-13-fm-spec.md#p44)
> Maybe we should build enough diagnostic features to let you reach out and debug the app on my phone?

## spec

Lets a developer see what an installed PWA is doing on a device they cannot touch. The shell posts a report to `diag/report` on every launch (running version, server version, authed, service-worker control, user agent) and on every JS error (`window.onerror` + loader try/catch). The server appends reports as timestamped lines to `/tmp/miso-diag.log` (2KB per report, log rotated at 1MB). The endpoint is public by design: the broken or logged-out client is exactly the one that needs to report in. Read with `ssh mini tail -f /tmp/miso-diag.log`. First step towards the always-on /blackbox/.

## user

Nothing to do — every launch of the app phones home a one-line status. To watch a device live: `ssh microserver@microservers-Mac-mini.local tail -f /tmp/miso-diag.log`. The tiny version stamp in the app's bottom-right corner shows which build a device is running at a glance.

## glossary

- **launch report**: the one-line JSON status a client posts on startup.

## code description

`diag.rs`'s `route` /extension/ intercepts `POST diag/report` and delegates everything else via `existing.route(r)`.

`diag_report` caps the body at 2KB and appends a `<ms> <json>` line via `append_diag`; `rotate_diag_log` renames the log to `.old` once it passes 1MB.

The client half lives in `/shell`'s loader: the `diag()` helper, the `window.onerror` hook, and the launch report posted after paint.
