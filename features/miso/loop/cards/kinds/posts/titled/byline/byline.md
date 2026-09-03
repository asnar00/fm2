# byline
*a post's author and date, just below the title*

> (asks#1788463692230)
> author name and date on a post should be just below the title

## user

Under a post's title, one quiet line: who wrote it and when — *asnaroo ·
3 sep*. On a post handed to you it replaces the *from* line, which said
the same name.

## spec

A post page had no date at all, and its author only as `/exchange`'s *from*
caption on a copy. Tara's phone asked for both under the title (the ask).

**One line under the title.** `card_page_html` is extended for posts
(`posts_is`): after the title block, `.post-byline` with the owner's name
and `browse_when(post_time_of(card))` — `/post-time`'s reading of when a
post was made, `/browse`'s words for a date, without a year (a wasm
render has no clock to compare against; the year is the parked rung
`/browse` named). The page's own `post` class (`/picture-first` and `/above` scope by it)
is what this node's styles and the child `/plain` scope on; the byline takes
the title's `order` so it follows the title in the page's ordered column.

**The from line retires on posts.** It named the author already; two names
under one title is noise (`/taste` 8). Hidden by this node's stylesheet on
post pages only; profiles and projects keep it.

## hostile cases

- **A post with no time.** `post_time_of` falls back to `created`; a card
  with neither prints the name alone.
- **A post with no title block.** The byline goes after the page's first
  child instead — still at the top.
- **This node unticked.** No byline, the from line as before.

## code description

`byline.rs` — `card_page_html` inserts `.post-byline` after the title. `byline.css` — the line's style; `.card-from` hidden on
post pages.
