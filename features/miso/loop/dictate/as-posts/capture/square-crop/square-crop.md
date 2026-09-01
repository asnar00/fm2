# square-crop

*every picture a card stores is the central square of what was shot — a
landscape loses its ends, a portrait its top and bottom*

> (transcripts/2026-09-01-saturday.md#p19b)
> let's make all pictures and video frames square as well - just take the
> central square from the video/photo whether it's landscape or portrait.

## user

Take a photo and the picture the post keeps is the middle square of it. Record
a video and its poster frame is the middle square too. Pick a picture from the
roll for your profile and the same. Held sideways or upright, the picture is
square and centred on what you pointed at.

The video itself is untouched: it plays as it was shot. It is the still
pictures that are square.

Pictures stored before this keep the shape they were stored in. In the grid
they always looked square — a tile crops what it shows — and on a page they
still show at their own shape.

## spec

Every road that stores a picture ends in one place: `/cards`' `shrink`, which
draws the source onto a canvas and steps the JPEG quality down until the data
URL fits the wire's budget. `/capture/photo` calls it, `/cards`' own chooser
calls it, and `/capture/video/poster` draws its frame the same way. So the crop
belongs in that draw, once, and not in any of the roads.

`shrink` did not name the question it was answering, so this node's first act
is a refactor of the parent that keeps its behaviour exactly: **`frameOf(w, h)`
— which pixels of a source become the stored picture, and how big.** `/cards`'
answer is what it always did: the whole frame, longest edge to `EDGE`
(256, and 384 since `/roomier` raised it). Every
caller now goes through it, including the poster's frame-off-a-video.

This node redefines that one function to the central square: the shorter edge
is the side, the crop is centred on both axes, and the side is also the stored
size, capped at 256px and never blown up. Untick it and `/cards`' answer is
back with nothing else to undo — the shape of the crop is one function, which
is what makes "crop differently" (a face, a chosen crop, a rectangle again) a
change in one place.

**The video plays as it was shot.** The ask is about pictures and video
*frames*; letterboxing or cropping playback would be a different thing, and a
clip that plays differently from how it was filmed is a lie about what was
recorded.

A square picture is not a new idea here: `/frame`'s re-framing window already
keeps an `EDGE` square, so a picture you have re-framed was always square. This
makes the first capture agree with the re-frame.

**Old pictures are not migrated.** The data stays as stored; a tile has always
centre-cropped what it shows (`object-fit: cover` on a square face), so the
grid was already square and nothing there changes. On a card page an old
picture keeps its own shape and a new one is square — the difference is
visible only if you hold both, and rewriting stored pictures to make a page
uniform would cost quality for a cosmetic tidy.

**Placement.** A child of `/capture`, whose subject is exactly this — what
making a post out of a camera produces. The picture chooser is not under
`/capture`, and it inherits the crop anyway, because the seam is the one every
road shares; the alternative placements (`/cards` is at its six children,
`/look/layout` is about how a drawn card is laid out rather than what is
stored) were worse on both counts.

## glossary

- **frame of a source**: which pixels of a photo or a video frame become the
  stored picture, and at what size. `/cards`' `frameOf` answers it; this node
  makes the answer the central square.

## code description

`cards.js` gains `frameOf(w, h)` — the whole frame, longest edge to `EDGE` —
and `shrink` draws through it with the nine-argument `drawImage`. Identical
output to the five-argument call it replaces.

`square-crop.js` redefines `feature_Cards.frameOf` to the centred square: side
`min(w, h)`, offsets `(w - side) / 2` and `(h - side) / 2`, destination the
side capped at `EDGE`. Redefined rather than wrapped — there is one answer to
"which pixels", not a chain of them — and guarded, so the fragment survives
`/cards` being toggled off.

`/capture/video/poster` already draws its frame through `feature_Cards.frameOf`
when `/cards` is there, so the poster follows with no change of its own.
