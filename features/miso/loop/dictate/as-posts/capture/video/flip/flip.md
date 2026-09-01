# flip

*which camera a video is filmed with — front or back — chosen once and
remembered on the device*

> (transcripts/2026-09-01-saturday.md#p19a)
> also, we should be able to flip to front camera for video posts - that should
> be a persistent mode, again so we don't have to keep doing it.

## user

Tap the mode control with video chosen and the strip of kinds now ends with a
camera. Tap it: it becomes a face. That is the camera your next video is filmed
with, and every one after it, until you tap it again. Nothing else changes —
the add button still makes a video, the recording still stops itself at a
minute.

The choice stays on this device, like the kind the add button makes. A new
phone starts on the back camera.

## spec

The camera is a device var, `facing`, holding `"back"` or `"front"` — the same
declaration `/one-add`'s `mode` carries, for the same reason: how you work is
not what you own, and a device-scoped write queues no op. It is bridged to the
page half (`js:facing`), because the page half is where the camera is actually
asked for.

`/capture/video` asked `getUserMedia` for `facingMode: 'environment'` in a
literal. That literal is now a named /extensible function/ on the page half —
`constraints()` — answering exactly what it always answered, and this node
redefines it. The refactor changes no behaviour: with this node unticked the
same object reaches the same call.

**Where the control lives.** In `/one-add`'s kind picker, beside the kinds,
and only while video is the kind the add button makes. The viewfinder was the
brief's suggestion and is the wrong place: it exists only *while recording*,
and a flip there either lies — takes effect next time, on a control that looks
like it is doing something now — or costs you the clip, because a
`MediaRecorder` cannot be handed a different camera mid-take. The ask's own
words were "a persistent mode, again", meaning the mode `/one-add` shipped the
day before; this is where that mode lives. The cost is one extra tap the first
time you switch the add button to video, and none after.

The control is untinted while the kinds are tinted, so the kinds stay the lit
set and this reads as the setting beside them (`/taste` 2 — hierarchy is
dimness, and `/one-add`'s own reasoning for its mode button).

It shows the camera **you will get**, not the act of flipping: the back
camera is `/capture/video`'s camera mark, the front one is the person it points
at. Two drawn glyphs in `currentColor` per `/glyphs`, readable without colour
and without a second tap.

Tapping holds the picker open. `/one-add` closes it on every tap that is not
one of its own; this node's update is composed outside `/one-add`'s, so the
close happens and is then undone — you see the glyph change where you tapped
it, which is the whole point of a two-faced control.

Flipping **mid-recording is not offered**, and that is the honest answer rather
than a missing one: the stream the recorder holds cannot be swapped without
ending the take.

**Placement.** A subfeature of `/capture/video`: it changes what that node asks
the camera for, and nothing else in the tree has an opinion about cameras.

## glossary

- **facing**: which camera video is filmed with on this device — `"back"` or
  `"front"`. `/capture/video`'s `constraints()` turns it into a `facingMode`.

## code description

`flip.rs` reads and writes the var through the live context (`flip_read`,
`flip_write`), exactly as `/one-add` does and for the reason `/one-add` gives:
a render following this node's own write would read a stale bridge.

`one_add_choices` is extended: with video the chosen kind, the strip gains
`flip_button`, which draws the camera-you-will-get with `video_camera_svg` or
`flip_face_svg`.

`update` answers one click, `vid_flip`: write the other camera, then set
`oneadd_picking` back on so the strip stays up.

`flip.js` redefines `feature_Video.constraints` to ask for `user` or
`environment` off the bridged `facing`. Redefined rather than wrapped: there is
one answer, not a chain of them.
