# while-recording
*the camera can be changed in the middle of a take, and the clip stays one clip*

> (asks#1788532375916)
> video record: need to be able to switch camera while recording
> *(filed from the field on 2026-09-04 by ash, birthplace `posts @ miso/loop/dictate/as-posts/capture/video`)*

## user

Tap **rec** and start filming. Tap the **camera** button and the picture turns
around then and there — the viewfinder shows the other camera, and the
recording keeps running. Turn it back as often as you like.

Tap **stop** and you get one post with one clip, which plays straight through:
the part filmed on the first camera, then the part filmed on the second, in
one piece.

## spec

`/flip` made the camera a setting and said plainly that it could not be
changed mid-take: *"the stream the recorder holds cannot be swapped without
ending the take."* That was true, and it is the sentence this ask overturns
(`asks#1788532375916`).

**Placement.** A child of `/flip`, not of `/armed`, and the code is what says
so. The button is `/armed`'s and needs no change at all: `armed_row` draws the
camera control whether or not a recording is running, and `/armed`'s `update`
answers `armed_flip` the same way either way — the var is already written
mid-take. What was missing is that nothing *acted* on it once `start` had its
stream, and what that promise belongs to is `/flip`, whose spec is the one
that said it could not be done.

**iOS decides the mechanism.** A `MediaRecorder` ends with an error the moment
its stream's track set changes, so swapping the video track under it is not
available. What is available is `canvas.captureStream()`, whose track is the
canvas: it never changes, whatever is drawn into it. So the recorder is handed
a canvas, the current camera is drawn onto that canvas every frame, and
changing cameras changes only which camera is being drawn. The recorder never
notices.

**This was measured before it was built.** On the iPhone 17 Pro simulator, iOS
26 (2026-09-04): `canvas.captureStream(30)` plus the microphone's track,
recorded with the same `opts()` the app already picks, produced
`video/mp4; codecs=avc1.42000a,mp4a.40.2` — the same container `/video` gets
today. A 5.1 s take with the camera swapped at 2.6 s came back as 343 KB and
played back whole, `duration 5.22`, `640×480`. The draw loop ran at 58.7 fps
uncapped, which is why it is throttled to 30 here: the capture is asked for 30
and drawing twice per captured frame is heat for nothing.

**The other road, and why not.** Stopping the recorder on each flip and
starting a new one would file the take as consecutive parts —
`/streams` already uploads a clip in parts and the server joins them. But two
`MediaRecorder` outputs are two containers, not one file's chunks: joining
them needs an `ffmpeg concat` on the mini and a `/streams` seam for it, and
the phone's own playback before the upload lands would have to play two
pieces. Since the canvas road carries on iOS, none of that is needed and the
clip is one file everywhere, from the moment it is made.

**One seam in `/video`.** `new MediaRecorder(this.media, …)` becomes
`new MediaRecorder(this.recordStream(), …)`, with `recordStream()` returning
`this.media`. That is the whole refactor and it answers exactly what the
expression answered. Everything else in `/video` is untouched: the chunks, the
minute cap, `onstop`, `save`, the metadata, `/streams`' three seams, and the
companion audio recording, which holds its own `MediaStream` over the original
microphone track and is not part of what changes.

**The microphone never moves.** The new stream is asked for with
`audio: false`, and the recorder keeps `/video`'s original audio track for the
whole take. That is not a nicety: taking a track out of the recorder's set is
the very thing that would end the recording, and it is the same microphone
whichever way the camera points.

**The source is its own element.** The canvas is drawn from a `<video>` of
this node's own, not from `/video`'s viewfinder: the viewfinder is `/video`'s
and is dressed by `/square-crop`, and a recording must not depend on how a
preview is styled. Two pixels at 1% opacity rather than `display: none`,
because iOS stops decoding a video it is not drawing anywhere — the probe
confirmed frames at that size.

**Cover, not fit.** The canvas is fixed at the size the take started on, and
each frame is scaled to fill it and centre-cropped. On a phone the front and
back cameras hand back different shapes; letterboxing one of them would put
black bars inside the file. `/square-crop` centre-crops at display anyway, so
this is the crop that surface already expects.

**The flip is noticed at the paint.** No new event and no new button: the tap
writes the var, the var reaches the page as a repaint, and this node reads the
answer the whole chain gives — `feature_Video.constraints()`, whoever composed
it — at each `apply`. So it works through `/armed`'s button, through `/flip`'s
own control if a composition still draws one, and through anything else that
ever writes the camera.

**The viewfinder follows at once**, because a flip you cannot see is a flip
that lies about which way the phone is pointing.

**Parked, and named** (`/anticipation`): a torch or a zoom, which are
`applyConstraints` on the live track and would sit beside `swap`; a
cross-fade over the swap, which is two `drawImage` calls and an alpha; and
recording at a lower frame rate to save battery, which is one number here.

## risks

**The simulator is not a phone.** Every measurement above is a mock camera at
640×480 drawn by a Mac's GPU. On a real iPhone the camera is larger, the draw
is a real per-frame copy, and the encoder is fed by the canvas rather than by
the capture pipeline directly — which on iOS usually means the hardware
encoder is still used but the zero-copy path from camera to encoder is not.
The cost in battery and heat on a phone filming for a minute is **not
measured**, and it is the thing to watch on the walk. If it bites, the fallback
is not the second road but a lower `FPS` here, which is one number.

**The first frames.** The canvas is filled black at the start and drawn as
soon as the source decodes — a few tens of milliseconds. A take is never
empty, but its first frame may be black rather than the room.

## hostile cases

- **No `captureStream`.** `begin` hands back `/video`'s own stream and the
  recording is exactly what it is today: one camera, no flip mid-take. The
  test is on the prototype, so an old iOS degrades rather than throws.
- **An audio-only stream** (no video track): handed back unchanged, for the
  same reason.
- **The camera refuses the flip** (busy, or a phone with one camera). The take
  stays on the camera it has and `facing` is put back to what the chain says,
  so the next tap tries again. A half-swapped take is the one outcome worth
  avoiding.
- **Two flips in quick succession.** `switching` holds the second off until
  the first has landed; the paint after it notices any difference that is left
  and swaps again.
- **Flip with no recording running.** `drawing` is false, so nothing here
  happens; the var is written and the next take opens on the new camera, which
  is `/flip`'s own behaviour.
- **Stop mid-swap.** `end` runs first in the wrapped `stop`, so the draw loop
  is down before the recorder is asked to stop; a swap still in flight finds
  `switching` cleared into a torn-down node and stops the stream it opened
  through `extra`.
- **The minute cap.** `/video`'s own timer sends `vid_stop`; nothing here
  changes it, and the cap counts the take, not the camera.
- **The tab is backgrounded mid-take.** `requestAnimationFrame` stops, so the
  canvas stops receiving frames and the recorded video freezes on its last
  one while the audio continues. That is the same freeze the viewfinder shows
  and is what iOS does to a backgrounded recording anyway; it is named, not
  fixed.
- **`/armed` unticked.** No camera button in a recording row, because there is
  no recording row; `/flip`'s own control in `/one-add`'s picker writes the
  same var and this node answers it if a take is running.
- **This node unticked.** `recordStream` returns `/video`'s own stream, the
  recorder holds the camera directly, and a flip mid-take does nothing until
  the next recording — `/flip` exactly as it reads today.

## glossary

- **the canvas road**: recording a canvas that the current camera is drawn
  onto, rather than the camera itself, so the recorder's track set never
  changes.

## code description

`while-recording.js` — `begin` builds the canvas at the first camera's size,
starts the draw loop and returns `canvas.captureStream(30)` with `/video`'s own
microphone track added; it is what `/video`'s `recordStream` seam answers.

`while-recording.js` — `draw` is the throttled loop, cover-fitting each frame
into the fixed canvas; `wanted` reads the camera the composed
`constraints()` asks for, `onApply` notices it changing at each paint, and
`swap` opens the new camera video-only, points the source and the viewfinder
at it, and releases the one before.

`while-recording.js` — `end` takes the loop down and stops every stream this
node opened; it is wrapped in front of `/video`'s own `stop`.

`while-recording.js` — `install` puts `recordStream`, the wrapped `stop` and
the wrapped `feature_Loop.apply` in place, off the same 100 ms wait `/flip`
uses, giving up after ten seconds.
