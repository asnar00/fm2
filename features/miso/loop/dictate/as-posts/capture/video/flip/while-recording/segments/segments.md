# segments
*a flip mid-take cuts a new recording, and the mini joins the pieces into one file*

> (transcripts/2026-09-04-field-walk.md#p139)
> could you do something sneaky like this: if the user hits the flip button while recording, start recording a new clip and then after the fact, join the two clips together?

## user

Film, turn the camera round, turn it round again, as many times as you like,
and stop. You get one post with one clip that plays straight through: the part
on the back camera, then the part on the front, then the back again, in order.

There is a small gap at each turn — the moment the phone spends letting go of
one camera and picking up the other. The picture cuts; it does not stall.

## spec

`/while-recording` drew the camera onto a canvas and recorded the canvas, so
the recorder never saw a track change. That works, and it costs a copy of every
frame for the whole take and resamples the camera onto a second clock: when the
phone is busy the animation tick slips, the same frame is captured twice, and
that reads as a stutter. Ash asked for the other road (#p139) and, on the word
that followed, for **any number of segments** rather than two.

**The recorder is native again.** `/while-recording`'s `begin` seam is
answered with `/video`'s own stream: no canvas, no draw loop, no source
element, and its `drawing` flag stays false so its own flip watcher does
nothing. The canvas road is stood down from inside this node, through the seam
its parent opened, so unticking this one brings it straight back.

**A flip cuts.** The recorder is stopped, the new camera is opened, and a new
`MediaRecorder` is started on the new camera **with the same microphone
track** — the same microphone is in the room whichever way the phone points,
and a second one would be a second voice. The old camera is released; the
microphone is `/video`'s own and is released with the take.

**The part numbers keep counting.** The new recorder's `ondataavailable` is
`/video`'s own, rebuilt: pieces are pushed onto the same `chunks` and handed to
`/streams` with the index they land at. So `/streams`' `parts` count and the
server's join loop see one unbroken run of pieces, whatever happened in the
middle, and nothing in `/streams` changes.

**What changes is one number per segment.** `marks` is the part index each
container starts at — `[0]` for a plain take, `[0, 7, 15]` for two flips — and
it rides on the clip's own metadata beside `parts`, so it reaches the exchange
on the `RecShared` that announces the recording and lands on its index with
everything else. One segment sends no marks at all and the server behaves
exactly as it does today.

**The join is a real re-mux.** Two `MediaRecorder` outputs are two containers,
and gluing them end to end gives a file whose first header describes only the
first take: a player reads that and stops. So the pieces of each segment are
run together into a file of its own — within a segment, `/streams`' own byte
concatenation is exactly right — and ffmpeg joins those.

**Through MPEG-TS, and this is why the join is not one command.** The obvious
road is ffmpeg's concat *demuxer* over the segment mp4s with `-c copy`. It
offsets each input by what the one before it says its duration is — and a
`MediaRecorder` mp4 is written incrementally, so what its header says is not
always what it holds. Three takes joined that way came out right and the fourth
claimed **991 seconds for ten seconds of video**: 434 frames with packet stamps
running to 947 s, the audio beside it a sane 10.1 (rig-found, 2026-09-04). A
one-in-four wrong file is not a join.

MPEG-TS carries no global duration to be wrong about. So each segment is copied
into TS (`-bsf:v h264_mp4toannexb`), the TS streams are concatenated as bytes
(the concat *protocol*, which is what TS is for), and the result is copied back
into mp4 (`-bsf:a aac_adtstoasc`, `+faststart`). Two copy passes and a
bitstream filter each way — **still no re-encode** — and it is the standard
road for h264+aac precisely because of the header problem above. Four
consecutive four-segment takes since: container 13.19/13.19/13.19/13.21,
video and audio agreeing to a tenth in every one.

**No transcode, and when one would be needed.** Every segment came off the same
phone through the same `MediaRecorder`: iOS hands back
`video/mp4; codecs=avc1.42000a,mp4a.40.2` for all of them (measured on the
simulator, 2026-09-04), so the streams match. A phone that changed codec
mid-take would need a transcode, and that is named here rather than built:
guessing at a case that does not happen is worse than saying it is not handled.
If ffmpeg refuses at any step the clip stays in pieces and says so in the log —
it is never written half-joined.

**The marks reach the parts directory before the join.** `/streams` joins on
the announcement and joins BEFORE the rest of the chain, which is where
`/mirror` writes the index — so the marks cannot be read off the index at that
moment, because they are not there yet. This node's own `handle_msg` is
outermost and writes them beside the pieces first, so the join finds them
however it is reached: by the announcement, or by a piece arriving late
afterwards.

**A mark that is not trustworthy is not used.** The list must start at 0,
ascend, and stay inside the piece count; anything else and the clip is joined
the old way. A file that plays its first segment is a better failure than one
that plays nothing.

**What this phone plays.** The local copy is every piece run together, which is
several containers in one file: a player reads the first and stops. That is the
first segment, and it is honest, but it is not the take. So the next play of a
multi-segment clip asks the exchange for `blob/<id>` — the joined file — and
keeps it, marking its own metadata `joined`. If the exchange has not joined it
yet, or is not reachable, the first segment plays and the next play tries
again. **Nothing is ever deleted**: a take always has something to show.

**The gap.** Ash's word: it will not hurt us. It is the time between stopping
one recorder and the first frame of the next — one `getUserMedia` and one
`MediaRecorder.start`. The new camera is opened FIRST, before anything is
closed, so a camera that refuses costs nothing at all: the take carries on. The
gap as measured on the simulator is in the risks below.

**Parked, and named** (`/anticipation`): a cross-fade over the cut, which
needs the canvas road back for a few frames and could use it only there; a
transcode when the codecs differ, which is one branch here; and the mini
telling the phone the join is done rather than the phone asking on the next
play.

## hostile cases

- **A flip inside the first 300 ms.** The segment is dropped and the take
  starts clean on the new camera — a fraction of a second of mp4 is a header
  and little else, and ffmpeg's concat demuxer is entitled to dislike it.
  Nothing has been posted at that point (the timeslice is two seconds), so
  there is nothing on the exchange to contradict.
- **Two flips a second apart.** Two real segments: a second is well past the
  300 ms rule, and `MediaRecorder` hands over its piece on stop whether or not
  a timeslice has elapsed. Three flips make four segments, and the join is a
  loop.
- **The camera refuses the flip.** Opened before anything is closed, so the
  take carries on untouched on the camera it has, and the next paint tries
  again.
- **The recorder cannot be re-started.** The take is ended (`vid_stop` is sent)
  rather than left running with nothing recording — the state must not lie.
- **The app backgrounded between segments.** The awaited stop has a four-second
  ceiling, so a recorder that never reports back does not hang the flip; the
  segment is closed with what it has and the next one starts.
- **The mini offline at stop.** The pieces wait in the parts store and go up on
  the next pass, exactly as today; the join happens when the last piece lands,
  which is the road `/streams` already has for a piece arriving late.
- **A piece that never arrives.** The join waits — it checks every piece is
  present before it writes anything — and the post shows the first segment
  until it can be joined.
- **No ffmpeg on the machine.** Said in the log, and the clip stays in pieces
  rather than being written wrong. The lookup tries `MISO_FFMPEG`, the two brew
  paths and `/usr/bin` before it trusts `PATH`, because the server is started
  by launchd without brew in it (deploy.md).
- **A single-segment take.** No marks are sent, `segments_marks` finds none,
  and `existing.streams_join` does what it always did. ffmpeg is never run.
- **`/streams` unticked.** No pieces, no parts, no join — the whole clip goes
  up in one body and a flip mid-take still cuts a new recorder, so the local
  file is several containers and plays its first. Named: this node is
  `/streams`' partner and says so.
- **This node unticked.** `/while-recording`'s canvas road is back, whole: one
  container, no marks, no join.

## glossary

- **segment**: one continuous recording within a take. A take is one segment
  per camera it was filmed on, joined afterwards into one clip.
- **mark**: the part index at which a segment's container begins.

## code description

`segments.js` — `begin` answers `/while-recording`'s seam with `/video`'s own
stream, so nothing is drawn; `onApply` takes the flip in its place; `cut` opens
the new camera, closes the recorder, starts another on the new camera with the
same microphone, and records the mark.

`segments.js` — `settled` waits for a recorder's last piece with a ceiling on
it; `metaFor` puts the marks on the clip's metadata beside `/streams`' parts;
`installFetch` makes the next play of a multi-segment clip ask the exchange for
the joined file and keep it.

`segments.rs` — `handle_msg` writes the marks beside the pieces before
`/streams` joins, because the index is written further in than the join is.

`segments.rs` — `streams_join` runs each segment's pieces together into a file
of its own, copies each into MPEG-TS, concatenates the TS streams and copies
the result back into mp4; `segments_ff` is one ffmpeg run,
`segments_write_one` is one segment, `segments_marks` reads and checks the
marks, and `segments_ffmpeg` finds ffmpeg without trusting `PATH`.
