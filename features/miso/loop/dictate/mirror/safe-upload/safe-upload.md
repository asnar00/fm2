# safe-upload
*a full device cannot crash the catch-up upload*

> (transcripts/2026-09-01-saturday.md#p2)
> 2] clean up the "make posts" interface to include video, audio, photo+type, transcription

*(found while building that ask: the capture worker's rig, stubbing
`feature_Dictate.put`, surfaced the failure below in `/mirror`'s
pre-existing path; filed as its named risk and fixed here in the same run.)*

## user

Nothing visible. On a device with no room left, recording keeps working as
well as it can: what has reached the server is safe, and the app does not
throw.

## spec

`/mirror`'s `upload()` guards its network calls but not its local writes:
after a successful `POST blob/<id>` it stamps the meta `uploaded` through
`feature_Dictate.put`, and on a genuinely full device that put raises out of
the async function as an unhandled rejection. No data is lost — the exchange
already holds the blob, and an unstamped meta is simply retried on the next
catch-up pass — but an unhandled rejection is a crash report waiting in the
console and a promise chain nobody owns.

This node replaces `feature_Mirror.upload` at load — property replacement,
the house idiom — with a wrapper that calls the original inside a catch and
swallows the failure quietly: the retry-later behaviour the function already
has for network failures becomes its answer to storage failures too.

## hostile cases

- **The device stays full forever.** Every pass re-POSTs the unstamped blob;
  the server answers ok each time; the meta stays unstamped. Wasteful but
  bounded by the pass cadence, and it heals the moment space returns.
- **This node unticked.** The unhandled rejection returns — the state before
  today, no worse.
- **`/mirror` unticked.** The typeof guard finds nothing to wrap.

## glossary

(no new terms)

## code description

`safe-upload.js` wraps `feature_Mirror.upload` in a try/catch at load,
typeof-guarded, preserving the original's `this`.
