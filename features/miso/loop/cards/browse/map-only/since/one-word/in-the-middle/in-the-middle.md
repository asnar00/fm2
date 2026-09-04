# in-the-middle
*the project's name sits in the middle of the screen again*

> (transcripts/2026-09-04-field-walk.md#p89)
> the project title ("sevenoaks") should be horizontally centered

## user

The name of the project you are in sits in the middle of the top of the screen,
with the filter word at the left and the nøøb button at the right. A long name
gets an ellipsis rather than reaching either of them.

## spec

`/title` put the name in the middle of the screen and ash asked for it there
again, so this node is a return rather than a move. What took it away was
`/since`: four filter pills 175 points wide, and a centred name would have sat
on top of them, so it was pinned into the gap that was left — 200 points from
the left edge in `since.css`, re-tuned to 80 in `one-word.css` when the slot
became one word, with `strip-on-black.css` capping its width against the
lozenge. Three nodes carrying one tacit arithmetic, which was named as a risk
each time it grew.

**`/one-word` retired the reason and this retires the arithmetic.** One word
takes about 50 points, so the middle of the strip is free; the name is centred
on the **screen** — `left: 50%` and a half-width shift, `/title`'s own idiom,
restated because `/since` and `/one-word` had replaced it — rather than centred
in a gap whose edges two other nodes were describing.

**Why it hangs off `/one-word` and not off `/strip-on-black`.** The strip's
black lozenges were asked for at 16:53 and this at 16:44, so this node is the
older of the two and cannot be that one's child — causality bounds extension,
and the linker says so. `/one-word` is the truer parent anyway: it is the node
that made the middle free, and its stylesheet holds the `left: 80px` this
replaces. `/strip-on-black` composes after and sets `right: auto` and the same
cap, so the two agree wherever they meet.

**One number survives, and it is a clearance rather than an offset.** A centred
box may be at most the screen less twice the room its neighbours need: 210
points is the widest filter word on the left and the nøøb lozenge on the right,
with a gap either side. That is a fact about the strip's two ends and not about
where the name begins, so it does not move when the slot's word changes — which
is what made the old numbers fragile. A name longer than that ellipsises inside
its pill; the ellipsis is `/title`'s and is untouched.

**The pill still hugs.** `right: auto` is restated so `/strip-on-black`'s
shrink-to-fit survives — a centred box with both edges pinned would stretch —
so the black lozenge is still the width of the name, now centred.

## hostile cases

- **A long name.** Capped at `100vw - 210px` and ellipsised inside the pill;
  centred, so it grows equally towards both neighbours and reaches neither.
- **A very narrow screen.** Below about 260 points the cap goes to nothing and
  the pill collapses to its padding. Named, not guarded: the smallest phone the
  tree targets is 375 and the tree has no layout for narrower.
- **No project current.** `/title` draws nothing; there is nothing to centre.
- **`/since` or `/one-word` unticked.** The filter slot goes back to four pills
  175 points wide and a centred name overlaps them. This node's premise is
  `/one-word`; unticking that should untick this. Said here because the
  arithmetic this node removes was what used to absorb the difference.
- **This node unticked.** The name is back in `/one-word`'s gap at 80 points
  from the left, which is where it is today.

## glossary

(no new terms — **the strip** is `/strip-on-black`'s)

## code description

`in-the-middle.css` returns `.proj-title` to `left: 50%` with a half-width shift,
keeps `right: auto` so `/strip-on-black`'s pill still hugs its name, and caps
the width at the screen less the room the strip's two ends need.
