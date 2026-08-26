# titled
*a post has a title, like a person and a project*

> (transcripts/2026-08-25-accounts.md#p127)
> ah ok: the "post" thing isn't a button, my mistake - it's the post type tag. let's give posts a title (like users and projects have names) so their format matches.

## user

A post has a title at the top of its page, like a person's name or a project's; give it one and the tile and the row lead with it. Without one, the post still says who wrote it.

## spec

`/posts` removed the title block from a post's page and captioned tiles and rows with the author. Ash asked for posts to match the other kinds (#p127). One reading, so it builds: `posts_no_title` is redefined to keep the block (its placeholder reads "a title"); `browse_title_of` and `card_tile_html` prefer the title when a post has one and fall back to what `/posts` and `/tile-words` drew. Untick and posts are title-less again.

## glossary

(no new terms)

## code description

`titled.rs` — `posts_no_title` keeps the block and sets its placeholder; `browse_title_of` and `card_tile_html` prefer a present title; `titled_title` reads it.
