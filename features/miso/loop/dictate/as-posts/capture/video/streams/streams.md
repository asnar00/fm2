# streams
*the clip goes up while it is being made, in two-second pieces*

> (transcripts/2026-09-04-field-walk.md#p7)
> 1) stream audio and video to the server as we're recording it (or to a local cache with an upload queue if we're on a slow/nonexistent connection)

## user

Record a note and it is already most of the way to the server before you take
your thumb off stop. On a bad line the pieces wait on the phone and go up when
the signal comes back, oldest first, and the note is transcribed the moment the
last one lands. You are never told about any of this.

## spec

`/video` recorded into one blob and `/mirror` sent it whole afterwards, so a
minute of video began its upload at second sixty. Here the recorder is asked
for a piece every **two seconds** and each piece is posted as it arrives
(`POST blob/<id>/part/<n>`); stop says how many there were, and the server
joins them in order. On a good connection the wait after stop is one piece
long instead of one clip long.

**The pieces are kept on the phone too, and that is the queue.** A piece is
written to the device's store before it is posted, so a piece the network
refused is a piece that can be sent later. `/mirror`'s existing catch-up pass
— on `online`, at boot, and after every new recording — asks this node for a
recording's bytes and gets "the pieces that are still missing, oldest first".
The queue is the one that was already there; only its grain is finer.

**Anything that goes wrong falls back to yesterday.** A piece that was never
stored, or a server that refuses one, and the whole clip goes up the old way
with `parts: 0` on it, which is how the server is told to stop waiting for
pieces. **iOS Safari may honour a timeslice only at stop** — then there is one
piece, it is the whole clip, and the join of one piece is a copy. Every one of
these is a slower upload and a correct note.

**The join is the server's, and it happens before anything else in the
chain.** `RecShared` carries the count; this node's link joins and only then
passes the message on, because further in is where the clip is queued for
transcription and a queued clip with no file is a job that gets dropped. A
piece that arrives *after* `RecShared` — the phone was offline for it — is
joined at the route instead, and queues the clip itself.

**A joined file that will not decode.** MediaRecorder's pieces are one file
cut at byte boundaries, so a plain concatenation is the file; nothing here
re-wraps. If a container ever does come back undecodable, ffmpeg `-c copy` on
the mini is the repair and a single transcode is the fallback — neither is
built, because neither has been needed.

**The companion audio recorder retires.** `/video` kept a second, audio-only
recording beside every clip for `/phone`'s on-device whisper to eat. There is
no model on the device any more (`/phone` is unticked for miso), so that
recording is a second encode of every note, stored and read by nobody. It goes
through the seam `/video` opened for it, not by editing the recorder.

**The pieces are swept when the exchange has them all.** A device that kept
both the pieces and the whole clip would keep every note twice.

## glossary

- **piece**: one `timeslice` of a recording, posted as it is made and stored
  on the device until the exchange has it.

## code description

`streams.js` fills three seams `/video` opened and one `/mirror` opened, and
edits neither file. `timeslice` becomes two seconds; `onChunk` writes the
piece to the device store and posts it; `metaFor` puts the count on the
recording's metadata, which is what rides to the server on `RecShared`;
`companionAudio` becomes nothing at all. `sendBytes` is `/mirror`'s new seam:
the missing pieces oldest first, or the whole clip if any of them cannot be
found, or false for "not now" so the pass stops and retries on the next
reconnect. `del` opens a transaction on the store `/dictate` publishes, for
this node's own `p:<id>:<n>` keys only, because `/dictate` has no delete.

`streams.rs` claims `POST blob/<id>/part/<n>` ahead of `/mirror`'s
`blob/<id>` — this link is outermost, so `/mirror` never sees a path with
slashes in the id. A piece is written under `parts/<id>/<n>`; the join
concatenates 0..n-1 into the clip and removes the directory, once, and never
rebuilds a clip that is already there. `handle_msg` joins on `RecShared`
before passing the message on, reading the count off the message because the
index it would otherwise read is written further in.
