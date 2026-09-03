# with-close
*every card page has a close at the top right; the type tag moves left*

> (asks#1788463654477)
> All cards (posts, profiles, etc etc) should have a close button at top
> right. Move the type lozenge left to make room

## user

Every card — a post, a profile, a project, a report — has a small ✕ in its
top-right corner. Tap it and the card closes, back to the list it came
from, exactly as ‹ does. The type tag sits just left of it.

## spec

A card page closes by ‹ in the toolbar, one level up (`/one-level`). From
the second iPhone, filling a card for the first time, ash asked for a close
where a phone user's thumb looks for one: the top-right corner. One
reading, so it builds. `/tools`' rule — never buttons on a page to choose
between actions — is about choosing; a close chooses nothing, it is the
card's own dismissal, and the toolbar's ‹ keeps doing the same thing.

**The same event as ‹.** The ✕ carries `data-ev="tools_home"`, so the
loop's delegated click sends what a tap on ‹ sends and `/one-level`,
`/browse` and `/posts` do exactly what they do for ‹: a card page goes back
to its set. Nothing new is navigated.

**Placed where the tag was; the tag steps left.** This node extends
`card_page_html` after `/tag` and inserts `.card-close` beside the tag, an
ink ✕ (`/glyphs`: drawn SVG in currentColor) in a 30px ring on the card's
ground, on the title's centreline as `/centred` put the tag. The tag's
`right` moves in by the ring and a gap.

## hostile cases

- **The first profile page** (`/profile-first`, ‹ withheld). The ✕ is on
  the page but sends ‹'s event, which that gate's page half turns into
  staying on the card, as it does for ‹; the card is not left before it is
  filled.
- **A tombstone's page** (`/delete`'s one dim line). It is a card page; the
  ✕ is there and closes it.
- **`/tag` unticked.** This node is its child and goes with it.
- **This node unticked.** ‹ alone, the tag at the edge.

## code description

`with-close.rs` — `card_page_html` inserts `.card-close` just inside the
page's opening div, after the tag. `with-close.css` — the ring, the glyph,
and the tag's new `right`.
