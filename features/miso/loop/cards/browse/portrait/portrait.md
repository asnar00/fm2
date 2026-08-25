# portrait
*a list row is a face, a name, and a line of what the card says*

> (transcripts/2026-08-25-accounts.md#p85)
> let's make the list view nicer. how about: the picture on the left, the name and contents (suitably exerpted) on the right.

## user

In the list view each row shows the card's picture on the left (or its initial, if it has none), and on the right the name in bold with the first line of what the card says beneath it; the row's word and the date sit small beside the name.

## spec

`/browse`'s list was the bare `.crow` grammar: a word, a bold title, a date. Ash asked for a picture-led row (#p85). One reading, so it builds. This node redefines `browse_list_html`: each row is the card's face — the first picture block, cropped square at 46px, or the title's initial dimmed — then a body with the name, `browse_row_left`'s word (the distance under `/people`, the type elsewhere) and the date on one line, and an **excerpt** beneath: the first text block with words, whitespace folded, cut at a word boundary before 80 characters with an ellipsis, one line. The row keeps `.crow`, `.browse-row` and its `browse_open:` event, so `/people` and the open path are untouched. Untick and the bare row returns.

## hostile cases

- No picture: the initial. No title and no picture: an empty face box.
- No text block, or an empty one: no excerpt line (the row is one line tall).
- A long single word: cut mid-word by the ellipsis of the line itself.

## glossary

- **excerpt**: the first line of a card's words, shortened.

## code description

`portrait.rs` — `browse_list_html` redefined with the same walk as `/browse`'s; `portrait_face` and `portrait_excerpt` are the two new cells.

`portrait.css` — the row layout and the type ramp.
