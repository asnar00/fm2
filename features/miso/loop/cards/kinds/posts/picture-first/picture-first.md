# picture-first
*on a post, the picture sits above the words*

> (asks#1787702564278)
> New post: picture should be above text box
> *(filed from the field on 2026-08-26 by ash, birthplace `posts @ miso/loop/cards/kinds/posts`)*

## user

A post's page shows its picture first, with the words beneath.

## spec

`/posts` drew a post words-first, moving the picture block below the text. Ash asked for the picture above (`asks#1787702564278`). One reading, so it builds: the card page is a flex column, so this node gives the post's picture block `order: -1` and it leads; the blocks' indices, and so every edit, are untouched. Untick and the words lead again.

## glossary

(no new terms)

## code description

`picture-first.css` — one rule on `.card-page.post .card-pic`.
