# one-medium

*a card page carries one piece of media, so the empty picture slot yields to a
recording or a video*

> (transcripts/2026-09-01-saturday.md#p16a)
> on posts, there should just be one piece of media, either audio, video or
> still image. Currently a new video post shows "add a picture" as well as the
> video asset - it shouldn't.

## user

A post holds one piece of media: a recording, a video, or a picture. Open a
video post and you see the player and nothing else; open a recording and you
see the play row. The dashed *add a picture* invitation appears only on a post
that carries no media at all.

A post you made before this — a picture and a recording both — keeps showing
both. The rule governs the empty slot, not your history.

## spec

`/cards` draws a `picture` block with no data as a dashed invitation, and every
card is minted with one. `/as-posts` appends its `audio` block after the three
`/cards` mints, and `/capture/video` turns that block's kind into `video`, so a
post made by recording carries the invitation *and* a medium — two media slots
on one page, which is what ash saw and asked to lose (`#p16a`).

This node is a rendering rule, not a migration. Cards keep the blocks they
have: the `audio` and `video` blocks stay at index 3 and the `picture` block
stays at index 1, so every `data-block` `/keep` and `/frame` send still names
the block it always did. What changes is what the page draws — and, because the
dashed block *is* the add-a-picture road (`/cards`' delegated click opens the
file chooser on `.card-pic`), taking it out of the page takes the road out too.
The guard is where the affordance is, so there is nothing to add a second
medium *through*.

The test is one named function, `one_medium_carried` — it answers *which*
medium a card already carries, not merely whether it carries one, because the
next ask in this direction is "swap the video for a photo" (`/anticipation`)
and a swap has to know what it is replacing. Today three kinds answer: a
filled `picture`, an `audio` block, a `video` block. A fourth medium extends
this one function and the rule follows.

A filled picture block is never cut — only the empty one — so an old post with
a picture and a recording keeps both, and a profile card is untouched (it
carries no audio or video, so the invitation stands). A foreign card reaches
this node with the invitation already gone: `/exchange` cuts it for its own
reason, and this node's cut finds nothing and returns the page as it was.

**Placement.** The brief expected `kinds/posts`, which stands at its six
children. It is not needed: the rule is about what a *card page* draws — the
same rendering `/me`, `/browse` and `/posts` all open — so it belongs with the
other nodes about the card as a page, and `/page` had four children. No
regroup, and the node is honest about its subject. Nothing under `kinds/posts`
would have owned it: `/picture-first` orders the picture block, it does not
decide whether one is drawn.

## glossary

- **carried medium**: the piece of media a card already holds — a filled
  picture, a recording, or a video. `one_medium_carried` names it; the empty
  picture slot is not one.

## code description

`one-medium.rs` extends `card_page_html`. It reads the card, asks
`one_medium_carried` what medium is already there, and returns the page beneath
untouched when the answer is nothing. Composed last of the `card_page_html`
chain — its prompt is the newest in the tree — so it sees the finished page,
audio row and video mount included.

`one_medium_carried(card)` walks the blocks in order and returns the first
medium it finds: `"picture"` for a picture block with data, `"audio"` or
`"video"` for those kinds. An empty string means the card carries none. This is
the /extension point/ a later medium — or a swap — grows from.

`one_medium_no_empty_pic(html)` cuts the dashed block out of the drawn page:
find `<div class="card-pic empty"`, find the `</div>` that closes it, splice
the rest. A block's text is escaped on the way in (`/cards`' `card_esc`), so
that first `</div>` is its own. The same cut `/exchange` makes for foreign
cards, for a different reason.
