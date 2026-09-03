# hint
*the framing sheet says how*

> (transcripts/2026-09-03-invite-test.md#p74d)
> above the photo crop UI, add a line explaining that you can pinch-zoom /
> drag to crop the picture

## user

When a photo opens in the square window, a quiet line above it says what to
do: **pinch to zoom, drag to move**. The square is what you keep.

## spec

`/frame`'s window explains itself to someone who has met a crop before and
to nobody else; a canvasser meeting it for the first time sees a square and
two buttons (#p74d). One line above the window, in the "purpose" grey
(`/taste` 2), says the two gestures. Microcopy needing a second sentence
would mean the design was wrong (`/taste` 7); this needs six words.

**Furniture beside furniture.** `/frame` builds its sheet at load, outside
`#app`, and keeps it on `feature_Frame.sheet`. This node's fragment runs
after it and inserts the line before `feature_Frame.win`; the sheet's column
layout puts it above the window with the same gap. `typeof`-guarded: with
`/frame` absent nothing is inserted.

## hostile cases

- **`/frame` unticked.** No sheet, no hint.
- **This node unticked.** The window and the buttons, as before.

## code description

`hint.js` — inserts `#frameHint` before the window in the framing sheet.
`hint.css` — the line's colour and size.
