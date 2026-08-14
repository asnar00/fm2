# dictate
*record voice notes to the device — the transcribe tool's foundation*

> (transcripts/2026-08-14-fm-spec-2.md#p46)
> shall we implement record-to-local first? I'm thinking something like: tap dictate icon, we get a new tool panel with record, stop. recording creates a new file, which we visualise in a grid of file icons in the main area

> (transcripts/2026-08-14-fm-spec-2.md#p49, draft-phase revision)
> should we now add playback when we tap the file icon? also, let's change the icon we use for the file; something connoting "sound".

## spec

The 🎤 tool. Open it: the display surface shows your recordings as a grid of
file icons, with a record button; recording swaps it for a stop button and a
pulsing dot. Each recording becomes an immutable local file — muon's first
**blob store**: audio lives in IndexedDB (blobs can't ride in loop state);
state carries only metadata `{id, t, dur, size, mime}`. Effects are
state-driven, Elm-style: Rust owns intent (`dict_recording`), the page half
watches state and drives the microphone, then reports results as events
(`RecSaved`), so the blackbox records the whole story and replay needs no
microphone. Metadata reseeds from IndexedDB on boot (`RecList`) — recordings
survive restarts. Named next steps (the graded-derivation plan, #p36–39):
streaming/catch-up upload to the exchange, then the three transcribe rungs
as subfeatures.

## user

Tap 🎤 in the toolbar. Tap the record button and talk (first use asks for
microphone permission); tap stop when done. Each note appears as a file icon
in the grid. Notes are stored on this device — nothing leaves it yet.

## glossary

- **recording**: an immutable audio file captured by this device, identified
  by its capture timestamp.
- **blob store**: device-local storage for binary data (IndexedDB),
  referenced from loop state by id — state holds facts about blobs, never
  blobs.

## code description

`dictate.rs` owns the tool's state machine. `tools_list` registers
`{dictate, 🎤}`. `update` claims: `dict_rec` (set `dict_recording`),
`dict_stop` (clear it), `RecSaved` (append one metadata entry to
`dict_files`), `RecList` (replace `dict_files` wholesale — the boot reseed),
and `Transcribed` (stamp a file with `transcript`/`t_rung`/`t_grade`).

The transcription machinery (the #p36–39 plan, built): three slot chains
`transcribe_local/server/api` — base implementations return "" (unreachable);
each rung subfeature redefines its slot to "ready". The `transcribe()`
scheduler runs after every `RecSaved`/`RecList`/`Transcribed`: it picks the
highest-grade reachable rung (local 1, server 2, api 3), queues the first
`here` file whose stamp is below that grade as `dict_transcribe` intent, and
clears the intent when nothing needs work — with every rung unticked,
behaviour is exactly as before. `dict_file_extra(file)` is the per-tile seam
every grid routes through (this node's and `/mirror`'s replacement); its base
renders the transcript, clipped and HTML-escaped, titled with its rung.
`render`, when dictate is the open tool: the grid (one 🔊 icon + time label
per file, newest last; the playing note inverts and pulses). `tool_controls`
(the `/tools` chain, revised at #p48) puts the record/stop buttons in the
toolbar right of the mic: ● when idle, ■ with the pulsing dot while
recording. Tapping a note plays it (`dict_play_<id>` → `dict_playing`);
tapping the playing note — or `PlayEnded` from the page half — stops it.
Playback follows state edges like recording does: the page half fetches the
blob, plays it via an object URL, and reports the end; effects are skipped
during `/replay` (re-enactment must touch no hardware).

`dictate.js` is the hardware half: it opens IndexedDB (`muon-blobs`,
store `audio`); wraps `feature_Loop.apply` to watch `dict_recording` —
rising edge: `getUserMedia` + `MediaRecorder` start; falling edge: stop, and
on the recorder's stop event assemble the blob, write it keyed by id, and
send `RecSaved` with the metadata. On startup it reads all stored metadata
and sends `RecList`. Mic failure (denied, no device) sends `dict_stop` so
state never claims a recording that isn't happening.

`dictate.css` styles the grid, the file icons (monochrome, matching the
toolbar discipline), and the record/stop control.
