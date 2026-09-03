# face-first
*the face stays on the player until the clip plays*

> (transcripts/2026-09-03-housekeeping.md#p5a)
> the frame shouldn't be black until it plays - it should show the poster frame

## user

Tap a video post's poster and the picture stays where it is while the clip gets ready; the first thing that replaces it is the moving picture. A clip put back by a repaint, paused or finished, shows its face too — never a black square.

## spec

`/poster`'s tap takes the face out and puts `/capture/video`'s player in, and the player is a bare `<video>` with `preload="metadata"`: iOS paints nothing for that until playback starts, so the square went black between the tap and the first frame, and again whenever `restore` put an opened clip back paused. Ash saw the black square (#p5a). One reading, so it builds: the face the poster showed becomes the player's own `poster` attribute. `open` remembers the face's data URL under the clip's id before it removes the frame; `put` sets it on the element it makes. Both are wrapped, not edited — the node owns its act. The browser shows a `poster` until the first frame is painted, and again after a load with no playback, which is exactly the two moments that were black. A clip that never got a face (the frame could not be taken) has nothing to show and is as it was. Untick and the square is black again until it plays.

## hostile cases

- A poster tapped and playing: the face is under the first frame, never seen again.
- A repaint mid-play: `restore` re-opens the clip; the face is set on the new element and the clip resumes over it.
- A post whose poster was never made: no face, the player as before.
- A foreign copy: no player, nothing to do.

## glossary

(no new terms)

## code description

`face-first.js` — wraps `feature_Poster.open` (remember the face's `src` by clip id) and `feature_Video.put` (set `poster` on the new `<video>`).
