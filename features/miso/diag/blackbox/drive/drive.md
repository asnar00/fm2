# drive
*an agent can tap the screen: commanded interactions, scripted demos*

> (transcripts/2026-08-13-fm-spec.md#p109)
> I think we need one more feature as part of blackbox: the ability for you to reach out and actually tap the button (and by extension, any UI object) yourself, either at a whim from you, or by running a script. That would let us put demos together nicely: a demo script is just a set of UI interactions, followed by assertions on the readout.

## spec

The inverse of `/readout`: where readout lets an agent see the screen, drive lets it act on one. With `?drive=1` the page polls the server (4×/second) for queued commands and executes them — `send` (an event through the `/loop` Rust loop), `tap` (any CSS selector, a real click — chrome and login included), `type` (fill an input and fire its events). Commands are enqueued by `POST diag/drive` and popped one per poll; localhost is open for tooling, the tunnel requires a cookie both ways. Together with readout this is miso's native demo-and-test framework: **a demo script is interactions followed by assertions on the readout**, and `tools/drive.py run <script>` executes exactly that, failing loudly on a missed assertion.

## user

For agents: `python3 tools/drive.py tap '#build'` pokes the live page; `drive.py send '{"type":"click","ev":"tap"}'` speaks straight to the Rust loop; `drive.py run demos/<name>.json` performs a whole scripted demo and checks its assertions. Scripts are JSON step lists: `send` / `tap` / `type` / `wait` / `assert` (find-by-attributes, then check `text`, `text_starts`, `hidden`, or `exists`).

## glossary

- **drive**: executing commanded interactions in a live page, as if a finger had done them.
- **demo script**: a JSON sequence of interactions and readout assertions — simultaneously a demonstration and a regression test.

## code description

`drive.page.js`: `feature_Drive` suppresses `/gate`'s login redirect when active (a driven demo is not a user session, same stance as `/replay`) then polls `/diag/drive/next`, executing each command — `send` via `feature_Loop.send` (typeof-guarded), `tap` via `querySelector(...).click()`, `type` by setting the value and dispatching an `input` event.

`drive.rs`: a `route` /extension/ — `POST diag/drive` appends one command to a file-backed queue (size-capped, normalised to an array), `GET diag/drive/next` pops and returns the head or `{}`; both share `/readout`'s guard (localhost free, tunnel cookie-gated).

`tools/drive.py` (scaffolding) is the sender and script runner: subcommands for single pokes, `readout` to print the current screen, and `run` — post each step, settle, pull the readout for assertions, report pass/fail.
