# plus-at-home
*the + is for the set of posts, not for a post you are reading*

> (asks#1787702682845)
> When viewing a post that's already been made, don't show the post button
> *(filed from the field on 2026-08-26 by ash, birthplace `posts @ miso/loop/cards/kinds/posts`)*

## user

Open a post and the control row shows just the way back and undo; the **+** is there when you are looking at the set.

## spec

`/posts` puts **+** in the control row whenever the posts tool is open, including with a post on screen. Ash asked for it gone there (`asks#1787702682845`). One reading, so it builds: this node extends `tool_controls` and, when the posts tool has a card open (`browse_open` non-empty), strips the `posts_new` button from the row `existing` returns. Undo stays last. Untick and the + is always there.

## glossary

(no new terms)

## code description

`plus-at-home.rs` — `tool_controls` calls `existing` and, with a post open, `plus_at_home_strip` removes the button element by its `data-ev`.
