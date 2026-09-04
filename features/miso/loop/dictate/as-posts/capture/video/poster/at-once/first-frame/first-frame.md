# first-frame
*every clip gets a face, however short it is*

> (transcripts/2026-09-04-field-walk.md#p76)
> for the thumbnail, just use the first frame as a default - that should work regardless of length of clip. we *never* want a situation where theres no thumbnail - that degrades user confidence.

## user

However briefly you film — a second, half a second, a tap you did not mean — the post arrives with a picture on it. There is no such thing as a post with a blank square where its face should be.

## spec

`/at-once` keeps a frame off the viewfinder every 400 ms and uses one of them at the stop. A clip shorter than a tick had no frame at all, so it fell back to the slow road and stood without a picture for seconds — and a clip that never decoded a frame had nothing on either road. Ash's ruling is that this must never happen: the first frame is the default, and a post with no thumbnail costs confidence (#p76).

**The first frame, before the first tick.** The moment the camera is up, the viewfinder is asked every 30 ms whether it has decoded anything yet; the first time `videoWidth` is non-zero, `/at-once`'s own `grab` is called once. That frame goes into the same two slots the ticks use, so it *is* the poster for any clip that ends before the first tick, and a later tick replaces it on a clip long enough to have one — which is the ruling in its own words. The watch gives up after four seconds and never outlives the recording.

**And a face even then.** If the stop comes before the camera has decoded anything, the frame is taken from the canvas as it stands: the app's own dark ground with the viewfinder drawn over it, which on a camera with nothing yet is a dark square. That is a worse picture than a face and a better one than none, and it goes through `/poster`'s framing and `/pic-beside`'s mint like any other, so the card is minted with a picture block naming it exactly as it would be otherwise.

**A later frame may replace a dark one; nothing may remove one.** A dark face is the one face this node would rather see replaced, so it does not stand the slow road down — `/poster` reads the clip back, takes a real frame, and that frame is written over the dark one. `/poster`'s own op writes only into an *empty* picture block, so the replacement goes by `/cards`' `CardPic`, which sets a block's data and leaves everything else, the poster mark included, as it was. If the slow road fails or finds nothing, the dark face stays: there is no road here that ends in no picture.

Untick and a clip shorter than 400 ms has no face until the slow road brings one, and a clip that never decoded has none at all.

**What was measured**, on the rig with a real recorder, one clip per page:

| clip | face on the card and in the lozenge | pictures minted | slow road |
|---|---|---|---|
| 300 ms | 183 ms after the stop | 1 | did not run |
| 700 ms | 189 ms | 1 | did not run |
| 5 s | 204 ms | 1 | did not run |
| 700 ms, camera giving no frame at all | 212 ms (the dark square) | 2 | ran, and its frame replaced the dark one |

A clip of 100 ms makes **no post at all** — `/capture/video`'s recorder
produces no chunk that short and `save` returns before anything is minted — so
there is no post there to be missing a face.

## hostile cases

- **A clip of 300 ms.** The first frame was taken within a few tens of milliseconds of the camera coming up; it is the only slot, so it is the face.
- **A clip of five seconds.** The first frame has long since been pushed out by the ticks; a later one is the face, as `/at-once` intended.
- **A stop before the camera decoded anything.** The dark square, minted like any face, replaced later by the slow road's real frame if it finds one.
- **The slow road failing on a dark face** (the clip unreadable, the budget refusing). The dark face stays; a post never loses the picture it has.
- **The camera never arriving at all** (refused, no device). `/capture/video` never calls `viewfinder`, so nothing here is armed and there is no recording to give a face to.
- **A second recording started while the first's watch is still running.** `arm` stops the old watch before starting the new one, and the dark id is cleared with it.
- **`feature_Poster.draw` refusing the dark square** (a tainted canvas, a ladder that cannot fit it). No reference; the metadata carries none and the slow road runs as it did before this node.

## glossary

(no new terms)

## code description

`first-frame.js` — `feature_FirstFrame`.

`watch()` polls the viewfinder for its first decoded frame and calls
`/at-once`'s own `grab` once; `stop()` ends it, and `/at-once`'s `arm` and
`disarm` are wrapped to start and end it with the recording.

`dim()` is the last resort: the app's dark ground with whatever the viewfinder
holds drawn over it, put through `/poster`'s `draw` like any other frame.

`replace(id, ref)` writes a later frame over a dark one through `/cards`'
`CardPic`, by the card whose `rec` matches and its first picture block.

`feature_AtOnce.frameFor` is wrapped to fall back to `dim()`, `already` to
report a dark face as no face at all so the slow road still runs, and
`feature_Poster.make` to write that road's answer over the dark one.
