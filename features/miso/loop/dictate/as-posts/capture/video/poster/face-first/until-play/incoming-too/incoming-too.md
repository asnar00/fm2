# incoming-too
*a post that arrives under your finger shows a still, not a clip loading*

> (transcripts/2026-09-04-field-walk.md#p34)
> the incoming post still flickers its media - it's as if the media preview is scanning forward to the thumbnail frame rather than just displaying it

## user

Sweep to the next post and its media is simply there: one still square with a play mark on it, from the first frame the card is drawn. Nothing loads, nothing scans, nothing appears late and pushes the words down. Tap it and the clip plays.

## spec

`/until-play` stopped the clip playing itself and stopped the picture blinking, on the road where a post is *repainted*. Ash swept to the next post and saw the media still flicker — "as if the media preview is scanning forward to the thumbnail frame rather than just displaying it" (#p34). It is the road where a post *arrives* that was left.

**What was measured.** On the rig, sweeping onto a video post whose poster has not arrived: **a `<video src=… preload="metadata">` is created 2.25 seconds after the card**, then `loadstart`, `loadedmetadata`, `canplay` — the browser fetches the clip, decodes it and paints a frame, with no finger anywhere near it. A clip that had been played before is seeked to where it had got to as well (`/capture/video`'s `at[id]`), which is the scanning ash described exactly.

**Why that post has no picture.** `/poster` replaces `/capture/video`'s player row with a frame only when the card carries a poster block *and* a video block. A post whose face has not arrived yet — the frame is taken after the recording is saved and travels as its own op — keeps the bare player row, and `mount()` gives it a live element. On a walk, the post you have just made is exactly that post.

**The rule, on the arrival road.** A player row whose clip the reader has not opened is not touched: its handle is lifted for the length of `/capture/video`'s `mount()`, so no element is made, nothing is fetched, nothing is seeked. In its place stands a still — the same square `/clips-too` gives the player, the same ground, the same corner, with `/poster`'s own play mark on it — drawn in the same turn as the card, so nothing arrives late and nothing moves when the clip does go in. The clip's blob URL is warmed while the still shows, `/poster`'s trick for `/poster`'s reason, so the tap mounts in one turn and the play lands inside its own gesture. A tap on the still marks the clip opened, mounts the player and starts it.

A post with a face is untouched: `/poster` draws its picture and its tap already sets the clip opened before mounting, so that road passes straight through.

Untick and an arriving post loads its clip on its own again.

## hostile cases

- **A post with a poster** (the usual). `/poster`'s frame is the row; there is no bare player to dress, and its own tap opens the clip as before.
- **A clip already playing when a repaint arrives.** The reader opened it, so it is not waiting; `/until-play` carries the element across as it did.
- **A foreign copy** (`post-video dim`). No `data-vid`; not dressed, not mounted, unchanged.
- **A tap on the still with the clip not yet in IndexedDB** (it is still uploading, or came from another device). `mount()` finds no blob and puts nothing there; the still is gone and the row is bare until the next paint redraws it.
- **`/clips-too` unticked.** The player is its own shape rather than a square, so the still is a square where the clip may not be — the page moves a little when it goes in.
- **A second tap while the first is mounting.** The clip is already marked opened, so the still is gone and the holder is no longer waiting; nothing happens twice.

## glossary

(no new terms)

## code description

`incoming-too.js` — `feature_IncomingToo`.

`waiting(h)` is the test: a player row with a handle whose clip the reader has
not opened.

`dress()` puts the still in every waiting row and takes it out of every row
that is no longer waiting, and warms the clip's blob URL while it shows. It
runs after every paint, wrapped around `feature_Loop.apply` so it is later than
`/poster`'s own restore — a holder about to become a player is never dressed.

`open(h)` is the tap: mark the clip opened, drop the still, mount, and start it
through `/poster`'s own `start` so the play is inside the gesture.

The wrapper on `feature_Video.mount` is the gate: every waiting row's
`data-vid` is moved aside for the length of the call, so `mount`'s own selector
does not find it and no element, fetch or seek happens.

`incoming-too.css` — the still: a square of the page's width on the player's
own ground, with the play mark centred in it.
