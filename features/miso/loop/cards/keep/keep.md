# keep
*your words are kept as you type, and a repaint never takes them*

> (transcripts/2026-08-25-accounts.md#p20)
> ok, do all four - go ahead

> (transcripts/2026-08-25-accounts.md#p19, the ask this node serves)
> I'd like to look at the user card edit workflow next

## user

Write in a card and it is kept a moment after you stop typing — you do not have
to tap away, and nothing arriving from another device can take the sentence you
are in the middle of. Press Enter in the name and the keyboard goes; the name is
kept. Hold your finger on a picture and it offers to remove it; a normal tap
still opens your photos.

## spec

`/cards` edits in place, and the four weaknesses that showed up when the
workflow was read back (#p19) are this node's whole scope: a repaint can eat an
unsaved draft, saving happens only on tap-away, Enter in a one-line title
inserts a line break, and a picture can be replaced but never removed. #p20
approves all four, and nothing else — an edit mode with a Done button, a "kept"
indicator, per-card merge and the blob path are named and parked.

**A repaint keeps the block you are editing.** Every loop event redraws `#app`
wholesale, and the element the caret is in is destroyed. So `/loop` gains one
extension point, `feature_Loop.paint(html)`, which `apply` calls and which
defaults to exactly what `apply` used to do inline. This node redefines it: it
remembers the focused `[data-card][data-block]` element's live child nodes and
the caret's character offset, paints, then finds the same card and block in the
new DOM and puts the text, the caret and the focus back. The remembered content
travels as cloned nodes rather than as HTML, so nothing is re-parsed on the way
back in. `/me` is the precedent for taking a seam by replacing it at load
(`feature_Account.openTool`); this is a direct property replacement at load, not
a timer-installed wrapper, so it is not exposed to the apply-wrapper race
(notes.md, "the apply-wrapper race").

**What the repaint actually did, measured rather than assumed.** #p19 said the
draft dies silently because a destroyed contenteditable does not reliably fire
focusout. On the rig's engine it *does* fire it, synchronously, in the middle of
`innerHTML = html` — so today the words usually survive and the **caret never
does** (proven with `keep` unticked: after a repaint `document.activeElement` is
`BODY`). That firing is not a gift. `/cards` saves on it, and its save is itself
a repaint, so the old behaviour re-enters `apply` from inside an in-progress
`innerHTML` assignment, once per repaint, on every keystroke's worth of events.
With the caret restored it would do so endlessly. So this node **swallows the
focusout its own repaint causes**, in the capture phase, where it is reached
before `/cards`' listener whatever order the fragments loaded in. A repaint is
not a tap-away.

**And a block that does not come back is still saved.** If the repaint took the
card off the screen — the tool closed, another card opened — the restore has
nowhere to put the words and the swallowed focusout would have been the only
save. So the held text is sent as one `CardEdit` on the next tick, after the
paint has finished rather than re-entering it. The draft outlives the screen it
was on.

**Save as you type.** A card block sends `CardEdit` 600ms after the last
keystroke, as well as on tap-away. The event is the one `/cards` already has —
a debounced edit is just more of them — so nothing changes on the server. Two
keystrokes inside the window are one send, with the final text: the timer is
reset, not queued. Tap-away still saves at once, and cancels a pending timer so
the same text is never sent twice.

**Enter in the title is done.** A title is one line. Enter in a `.card-title`
blurs it rather than inserting a break, and the blur is the save that already
exists. A paragraph is prose and keeps its Enter.

**Long-press the picture offers remove.** Holding a filled picture block for
half a second raises a one-word pill — *remove* — centred **on** the picture;
tapping it empties the block, tapping anywhere else puts it away. On the
picture, not above it, because the title sits directly above and the first
version of the pill landed on top of it (4a); a pill over its own subject
cannot collide with a neighbour. The gesture is `/long-press`'s
idiom, reused and not depended on: 500ms, a 12px drift cancels it (a scroll is
not a question), and a capture-phase click listener swallows the one click the
press would otherwise have delivered, so the photo chooser does not open under
the reader. A plain tap is untouched and still opens the chooser; an empty
picture block has nothing to remove and ignores the hold.

**Removing reuses `CardPic`.** `/cards`' `CardPic {id, i, data}` writes whatever
`data` is, so an empty string is a removal: no new event, and no line of
`cards.rs`, is needed. That is the reuse the brief asked to be named — there is
no `keep.rs`.

## hostile cases

- **The repaint removes the card.** Open the tool away while a block is focused
  and the block is not in the new DOM: the restore finds nothing, writes into
  nothing, throws nothing — and the held words are sent as one `CardEdit` on the
  next tick, so closing the tool mid-sentence keeps the sentence.
- **A caret outside the block.** If the selection is not inside the remembered
  element the offset is `null`; the text is still restored, and focus lands at
  the end rather than at a guessed position.
- **Two keystrokes in one window.** One `CardEdit`, carrying the later text.
- **A pending edit and a tap-away.** The timer is cleared by the focusout, so
  `/cards`' own immediate save is the only send.
- **A long-press on an empty picture.** Nothing appears; the block is a
  placeholder with nothing in it to remove.
- **`/cards` unticked.** It is this node's parent, so this node goes with it and
  the `paint` seam is left at its default.
- **`/long-press` unticked.** Unrelated: this node reuses the idiom and has its
  own listeners, so the picture hold still works.

## glossary

- **paint**: `/loop`'s one-line extension point for putting html on the screen —
  the seam a feature takes when a repaint must not be blind to what is under it.

## code description

`keep.js` redefines `feature_Loop.paint` at load, capturing the default and
calling it in the middle: `hold()` reads the focused card block into
`{card, block, nodes, text, caret}`, the default paints, and `restore()` finds
`[data-card][data-block]` for the same pair and puts the nodes, the caret and
the focus back, reporting whether it found anywhere to put them. `caretOf`
measures the caret as a character offset with a range cloned to the element's
start, and `putCaret` walks the restored text nodes to the same offset — the two
agree because both count text-node characters only. `rescue` is the other
branch: no block to restore into, so the held text goes out as one `CardEdit` on
a zero timeout, outside the paint.

`painting` is true for exactly the span of the default paint and the restore,
and a capture-phase `focusout` listener stops propagation while it is — the
repaint's own focusout is not a tap-away, and `/cards` must not save (and
repaint) from inside a repaint.

`keep.js` delegates five more listeners on the document. `input` on a
`[contenteditable][data-block]` (re)starts the 600ms timer that sends
`CardEdit`; `focusout` clears it, leaving `/cards`' immediate save alone.
`keydown` turns Enter in a `.card-title` into a blur. `pointerdown` on a filled
`.card-pic` arms the 500ms hold that shows the remove pill, with `pointermove`
past 12px, `pointerup` and `pointercancel` disarming it, and a capture-phase
`click` swallowing the press's click so `/cards`' chooser stays shut.

`#cardRemove` is the pill, made at load and living outside `#app` — the
`#cardToast` idiom — so a repaint cannot take it away. It is centred on the
picture it belongs to, clamped to the viewport, and its tap sends `CardPic`
with empty `data`.

`keep.css` styles the pill: the dark button ground, a 999px round, one plain
word, and the 0.18s ease-out fade the rest of the app uses.
