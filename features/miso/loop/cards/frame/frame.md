# frame
*choose the bit of the photo you want: pinch to zoom, drag to pan, keep*

> (asks#1787664806535)
> add controls to zoom/scale a user profile picture
> *(filed from the field on 2026-08-25 by ash, birthplace `👤 @
> miso/shell/panel/account`)*

> (the answered did-you-mean, the reading this node builds)
> **frame** — "frame it when choosing: zoom and pan to the bit you want"
> *(the other reading offered, "tap the picture to see it big", was not
> chosen and is not built here — it is a later ask if it is still wanted.
> The question travelled and was answered by tap, which is
> `/did-you-mean`'s whole point: which thing they meant is theirs.)*

## user

Tap your card's picture and pick a photo, and the photo does not just
appear shrunk-to-fit any more: it opens in a square window you can pinch
to zoom and drag to pan until the bit you want — your face, usually —
fills it. Tap **keep** and that square is your picture. Tap **cancel**
and the picture you had is still the picture you have.

## spec

`/cards` shrinks a chosen photo to `EDGE` px on its **longest** edge and
stores the whole frame, which a profile picture is then displayed inside
a square tile through — `object-fit: cover` crops it, and the crop is the
middle of the photo whether or not the person is in the middle. The
asker has no say in which bit is kept. That is the problem, and the
answered did-you-mean confirms it is the one being reported.

**The seam is `chose`, taken by redefinition.** `/cards`'s file input
already calls `feature_Cards.chose(file)` the moment a file is picked,
and everything after it — the budget read, the quality ladder, the
"too big to keep" toast, the `CardPic` send — is `/cards`'s and stays
`/cards`'s. This node redefines `chose` at load, keeping the original in
a closure: the new one opens the framing view instead, and on **keep**
hands the original the square it produced, as a Blob. `me.js` takes
`/account`'s `openTool` the same way; `roomier.js` sets `/cards`'s caps
the same way. **No file of another node was edited.** The linker's
fragment gate wraps the redefinition on its own, so unticking `frame`
restores `/cards`'s own `chose` at runtime as well as at link time.

**The framing view is furniture, made at load, outside `#app`** — the
`#cardToast` and file-input precedent in `cards.js`. The loop repaints
`#app` on every event, and a sheet that lived inside it would vanish the
moment anything else happened while it was open.

**The geometry is one square region of the source photo**, held as a
centre point in image pixels and a scale. The zoom floor is the scale at
which the photo's shorter side exactly fills the window, so the square is
never allowed to show anything that is not photograph; the centre is
clamped so it cannot be dragged past an edge. Zooming is about the
window's centre, not the pointer, because the centre is what the person
is aiming at.

**What is kept is drawn straight from the source image**, not from the
preview: the visible region is redrawn into a canvas of
`feature_Cards.EDGE` × `feature_Cards.EDGE` and handed over as a PNG
Blob. The preview canvas is display resolution and would throw away
detail; the source is not. `/cards`'s `shrink` then sees a square already
at `EDGE`, so its longest-edge scale is 1 and only its JPEG quality
ladder does any work — which is the point of handing it a Blob rather
than encoding here. The budget stays the budget.

**Cancel does nothing at all** — no send, no toast, no state. The card is
untouched because nothing was ever written.

**Copy is two words.** `keep` is the one primary action (`/taste` 7,
`/taste` 3: the dusty blue that already means *chosen*); `cancel` is
quiet.

## hostile cases

- **A photo smaller than the window** (a 100×100 avatar): the zoom floor
  scales it *up* to fill the square, so there is still nothing but
  photograph in the window, and `keep` still produces an `EDGE` square.
  It is upscaled and looks it — that is what a small photo is.
- **A file that is not a picture**: it never reaches the framing view.
  The image fails to decode, and the file is handed straight to
  `/cards`'s original `chose`, which raises `/cards`'s own "that file is
  not a picture" toast. One voice, and it is the one that was there
  before.
- **Choosing again while the sheet is open**: one sheet, ever — it is a
  single piece of furniture. The new photo replaces the old one in it and
  the framing resets.
- **A repaint while the sheet is open**: survived, because the sheet is
  not in `#app`.
- **`/cards` absent from the composition**: the load block does nothing
  (`typeof feature_Cards === 'undefined'`), and no furniture is made.
- **A picture that will not fit the budget even framed**: unchanged —
  `/cards` refuses it out loud, after the framing, in its own words.

## glossary

- **frame**: the square region of a chosen photo that is kept as the
  picture — chosen by zooming and panning, not by the middle of the file.

## code description

`frame.js` redefines `feature_Cards.chose` at load, holding the previous
one in `fm_frameChose`. The new `chose` decodes the file into an image;
on failure it hands the file to the original untouched, and on success it
opens the framing view. Nothing else in the chain moves.

`feature_Frame.open(img)` fits the view to the image (`min` is the scale
at which the shorter side fills the window; `scale` starts there, the
centre starts at the middle of the photo) and shows the sheet.
`feature_Frame.keep()` draws the visible region into an `EDGE` square,
encodes it as a PNG Blob and passes it to the held original `chose`;
`feature_Frame.close()` hides the sheet and drops the image.

`feature_Frame.draw()` is the preview: it paints the visible source
region into the window canvas at device pixel ratio.
`feature_Frame.region()` is the one place the geometry lives — visible
size is `win / scale`, the centre is clamped to keep the square inside
the photo, and both `draw` and `keep` ask it rather than repeating it.
`feature_Frame.zoom(f)` and `feature_Frame.pan(dx, dy)` are the two
gestures reduced to numbers, so touch, mouse and wheel all arrive at the
same two functions.

The load block makes the sheet — a full-screen ground, the square window
with its canvas, and the two buttons — appends it to `document.body`, and
binds the gestures: `touchstart/move/end` for one-finger pan and
two-finger pinch, `mousedown/move/up` and `wheel` for a desktop rig.
Touch handlers are non-passive and call `preventDefault`, so framing does
not scroll the page underneath.

`frame.css` styles the sheet against `/taste`: the near-black ground at
92% opacity, a 14px-radius window with the `#202026` border, `keep` in
the dusty blue that means *chosen* and `cancel` quiet in `#9a9aa2`.
