# readout
*what's on screen, as data — a JSON readout of the DOM*

> (transcripts/2026-08-13-fm-spec.md#p108)
> This indicates we need a better feature than a screenshot - we need something that give us a json structure describing what's on the screen. A readout of the DOM, if you will. That would be a lot easier to process.

## user

For agents: after driving the app (replay or otherwise), `curl localhost:8095/diag/readout` and assert on structure — find the node with `"ev":"tap"` and check its text, confirm the panel's `hidden` flag, read an input's value. Screenshots remain for questions of *appearance*; readout answers questions of *content and structure*.

## spec

Machine-readable screen state, replacing pixel-reading for verification. With `?readout=1` (replay includes it automatically), the client serialises the DOM into a lean JSON tree — tag, id, classes, `data-ev`, leaf text (capped), input values, a `hidden` flag — and posts it to the server, debounced 250ms after any DOM mutation, so the stored snapshot always describes the settled screen. The server keeps the **latest** snapshot only (overwrite, not a log) and serves it back on GET: on localhost that's the agent's verification instrument (`curl /diag/readout`); through the tunnel both directions require a session cookie — screen contents are user data.

## glossary

- **readout**: a structural snapshot of the live DOM as JSON — what is on screen, not how it looks.

## code description

`readout.page.js`: `feature_Readout.capture()` walks `document.body` (skipping script/style), building `{tag, id?, cls?, ev?, hidden?, value?, text? | kids?}` per element; a MutationObserver schedules a debounced `post()` of `{t, url, body}` to `/diag/readout`; active only when the URL asks.

`readout.rs`: a `route` /extension/ — `POST diag/readout` stores the latest snapshot (size-capped, overwriting `/tmp/miso-readout.json`), `GET` returns it; both refuse tunnel traffic without a valid cookie, while localhost stays open for tooling.
