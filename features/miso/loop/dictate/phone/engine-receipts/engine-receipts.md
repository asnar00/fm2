# engine-receipts
*every transcription says what it did: which device, how long, on which build — in the diag log, live*

> (transcripts/2026-08-16-fm-spec.md#p13)
> we shouldn't be having to guess from timing - we should have proper monitorable logging, right? "ort ran xyz at time t". Let's do that and try again.

## user

Nothing to operate. When a note transcribes, your phone tells the server
what happened — which engine ran it, how many milliseconds it took, and
which build you're on. Watch any device live with
`ssh <mini> tail -f /tmp/miso-diag.log`. Nothing you said is in the
report: it carries the length of the text, never the text.

## spec

The `/tamed-request` field test could not answer its own question. The
words arrived fast, but "fast" was inferred from file timestamps on the
server, and the one fact that mattered — *which device actually ran the
model* — was never reported by anyone. Worse, the device's build was
unknowable too: `/diag` learns a client's version only in the launch
report, so a client that does not relaunch goes dark for hours.

A rung that cannot say what it did is not observable, so this node makes
the speech engine issue **receipts**. Each transcription reports twice:
`start` when the job is picked up (so a hang is visible as a start with
no finish), and `done` with the engine actually used (`webgpu` / `wasm`),
the elapsed milliseconds, the audio's duration, the transcript's length
in characters, and any error. **Every receipt carries the running build**,
which closes the blind spot the field test hit: from now on, any device
that transcribes announces its version, no relaunch required.

Privacy is a design constraint, not an afterthought: the receipt carries
`chars`, never the transcript. The words are the user's; the shape of
the work is ours. Errors ride verbatim (they name failures, not
contents).

Reports flow through `/diag`'s existing chain, so they land in the same
log as launches and JS errors — one place to watch. Absence degrades:
without `/diag` in the composition nothing is reported and transcription
is untouched. Replay is inherited — receipts wrap a function only ever
called from `/phone`'s replay-guarded watcher, so re-enactment issues
none.

Named future, deliberately not built: the general form — any feature
reporting timed events, and a periodic build heartbeat independent of
transcription. This node reports the rung that prompted it.

## glossary

- **receipt**: a device's own account of one unit of work — what ran,
  how long it took, on which build.

## code description

`engine-receipts.js` wraps `feature_Phone.run` (the transcription job,
typeof-guarded, installed once `/phone` exists). Before the original it
reports `{stt: "start", id, grade}`; after it (the original catches its
own errors, so this always runs) it reports `{stt: "done", …}`.

The elapsed time comes from `performance.now()` either side, and measures
the whole job — blob fetch, decode, engine import and build, inference —
so the first receipt of a session legitimately dwarfs the rest (measured
on desktop Chrome: 1716ms cold, 272ms warm, same audio, same device).
Reading a run's speed means reading the warm ones. The device
comes from `/stt/engine.js`'s `lastDevice()` — the seam refactored into
`/phone`'s engine for this node, reporting the device of the engine
actually built (the wasm fallback rebuilds, so a rescued run reports
`wasm`, which is the truth). The outcome comes from loop state, which
`send` has already settled synchronously by the time `run` resolves:
the file's `transcript` gives `chars`, its `dur` the audio length, and
`t_err` (stamped by `/honest-panel`) the failure reason.

`running` is read from `feature_Update.running`, typeof-guarded — the
build stamp that makes every receipt self-locating.
