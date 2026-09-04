# hugs-its-words
*the dropped column, and every lozenge in it, is as wide as its word and no wider*

> (transcripts/2026-09-04-field-walk.md#p88)
> the filter option switch looks good. small bugfix: in the dropdown, the background lozenge is too wide, as is the rounded rect backing - both should be wide enough to hold the option text, no more.

## user

Tap the filter word and the four drop under it in a box that is exactly as wide
as the longest of them. The one you are in is lit, and its lozenge is exactly as
wide as the word inside it — **all** wears a short lozenge, **month** a longer
one. Nothing is padded out to a width it did not ask for.

## spec

`/one-word` gave the column a 96px floor and stretched the pills across it,
which is the flex default. Both were arbitrary: 96 was a number that looked
about right, and the stretch meant a three-letter word wore a five-letter
lozenge. Ash saw it as soon as it shipped (#p88). One reading, so it builds.

**`max-content` is the whole change.** The column's width becomes the width of
its widest row; the floor that was holding it open goes to 0. Each pill's width
becomes its own text plus `/since`'s padding, and `align-self: flex-start`
stops the column's default stretch from overriding it. Nothing else moves: the
column's ground, its position under the slot, its animation and its rows are
`/one-word`'s and are untouched.

**Why the rows are still left-aligned and not centred on each other.** They are
a list of words, and a list reads down its left edge (`/taste` 6). Ragged right
is what a list of different-length words looks like; centring them would make
four lozenges of four widths float in a box, which is busier, not tidier.

**Why the lit pill is the one that mattered.** The report names "the background
lozenge" first — that is the accent behind the chosen word, and it was the
visible fault: on a column showing **all**, a short word sat in a lozenge sized
for **month**. The backing was the same fault one level out.

## hostile cases

- **A longer word later** (a custom range, say): the column grows to it, because
  nothing here names a width. That is the point of `max-content` over a number.
- **A word wider than the screen.** `max-content` would push the column past the
  right edge; nothing caps it. Named rather than guarded, because the four words
  are `/since`'s own and the longest is five characters — a cap would be code
  for a case the tree does not have. The cap belongs with the fifth option.
- **`/one-word` unticked.** This node is its child and goes with it; there is no
  column to size.
- **A different font or a larger text size.** The column follows the text,
  which is what it could not do before.

## glossary

(no new terms)

## code description

`hugs-its-words.css` gives `.since-drop` `width: max-content` and drops its
96px floor, and gives `.since-drop .since-pill` `width: max-content` with
`align-self: flex-start` so the column's stretch does not override it.
