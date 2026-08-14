# mirror
*your recordings appear on all your instances*

> (transcripts/2026-08-14-fm-spec-2.md#p50)
> for our next milestone, let's figure out distribution - the outcome should be that the first recorded file appears in the other muon instance on this laptop browser.

## spec

Recordings are user-scoped facts: record on one instance and the note appears
on your others. Replication follows the doctrine's two speeds — **metadata
travels eagerly** (a `RecShared` message through the outbox; the server keeps
a per-user index, broadcasts to the user's audience, and answers `RecIndex`
with the whole index for boot catch-up), **audio travels lazily** (the blob
uploads to the exchange when connectivity allows, marked per-file; another
instance fetches it on first play). Remote notes render dimmed until their
audio is local — never lie about what's actually on the device. Offline
recording queues both the upload and the announcement; reconnect delivers
them. The server stores blobs and index under `~/.muon-blobs/<user>/`,
outside the deploy tree.

## user

Record a note on your phone: within moments it appears in dictate's grid on
your laptop (dimmed until first played, while the audio fetches). Everything
you record reaches all your logged-in instances; nobody else's instances ever
see it.

## glossary

- **mirror**: user-scoped replication of recordings — eager metadata, lazy
  audio.
- **exchange**: the server's role for immutable facts: it stores and forwards
  copies; the birth device remains the origin of truth.

## code description

`mirror.rs` server half: `route` claims `blob/<id>` — POST stores the raw
body (binary-safe via `/serve`'s `raw` bytes, revised for this node) in the
sender's blob dir, GET returns it; both are cookie-gated on the tunnel and
the id is sanitised. `handle_msg` claims `RecShared` (append to the sender's
`index.json`, dedupe by id, publish to the `user.<sender>` audience) and
`RecIndex` (reply `RecIndexed` with the index — the boot catch-up).

`mirror.rs` client half: `update` merges `RecShared` and `RecIndexed`
entries into `dict_files` (skipping ids already present, so the origin
instance ignores its own echo; merged entries are `here: false`), and marks
an entry `here: true` on `RecFetched`. `render_files` is redefined (replacing
`/dictate`'s) to dim `here: false` tiles.

`mirror.js` owns transport: it wraps `feature_Dictate.getBlob` so a missing
blob fetches from `blob/<id>`, stores locally, and reports `RecFetched`;
`upload()` scans stored metadata for not-yet-uploaded blobs, POSTs each, and
on success marks it uploaded and queues `RecShared` through the persistent
outbox (offline-safe); runs at startup, after each save (`RecSaved` seen via
an apply-wrap), and on the `online` event. Startup also queues one
`RecIndex`.

`mirror.css` dims remote tiles until their audio lands.
