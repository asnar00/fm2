# tile-words
*a post's tile shows a bit of what it says*

> (asks#1787702640027)
> a post in grid view should show a bit of the text content, not who made it
> *(filed from the field on 2026-08-26 by ash, birthplace `posts @ miso/loop/cards/kinds/posts`)*

## user

In the grid, a post's tile is captioned with the start of its words rather than the name of whoever wrote it.

## spec

`/posts` captions a post's tile with its author, as the row's bold cell does. Ash asked for the words instead (`asks#1787702640027`). One reading, so it builds: this node extends `card_tile_html` and, for a post with words, replaces the tile's caption with `/portrait`'s excerpt — the one rule for "a bit of the words". A post with no words keeps the author. Untick and the author returns.

## glossary

(no new terms)

## code description

`tile-words.rs` — `card_tile_html` calls `existing`, then swaps the `.card-tile-title` text for `portrait_excerpt` when the card is a post with words.
