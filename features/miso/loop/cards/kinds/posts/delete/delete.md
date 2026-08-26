# delete
*a post can be deleted, and stays deleted everywhere*

> (asks#1787703278747)
> need a delete button on posts
> *(filed from the field on 2026-08-26 by ash, from the posts tool)*

## user

Open one of your own posts and the control row has a **bin**. Tap it once and it
asks — the button turns into the word *sure?* for three seconds; tap it again and
the post is gone. It leaves your grid, your list and the map, and it leaves the
worlds of the people you are linked to the next time they look. The undo button
beside it is lit: tap that and the post comes back, words and picture and all.

A post somebody else wrote has no bin on it. It is not yours to delete.

## spec

**Deletion is its own act.** `/guard` promised that a cards write can never drop
a card — *"a set cannot delete a card; deletion, when it comes, is its own op
with its own intent"*. This node is that op. `CardDelete {id, t}` does not
shorten the list: it turns one card into a **tombstone** — `deleted: t`,
`edited: t`, its blocks reduced to a single empty `title` and its `links`
emptied. The words, the picture and the location block are gone from the world
at the moment of the tap; what is left is the smallest thing that can still say
"this card was here and is finished".

**A tombstone is why nothing has to change underneath.** `/guard` merges by id
and takes the newer `edited`, so a tombstone beats every earlier copy of that
card, and a stale device that resends its old list without `deleted` loses to
it — an absence would have been silently repaired by the guard, which is exactly
the loss the guard exists to prevent. `/exchange` hands on an owned card whose
`edited` moved, so the delete travels on the writer's own tap and the copy
arrives tombstoned; no line of `/exchange` knows what a tombstone is.
`/guard/revert` restamps a restored list, so undo puts the card back *newer*
than its own tombstone.

**Only the owner's own cards.** A copy carries `from` (`/exchange`), and that is
the whole test, decided in Rust where it is structural: the event refuses a card
that carries `from`, and the button is not drawn on such a page at all. There is
no comparison against the logged-in name — the name is not in the world.

**Hiding is by seam, in three places.** `browse_cards` (the cards tool,
`/people`, `/projects`) and `posts_set` (the posts surface) both drop
tombstones, and the map follows for nothing because `/map` renders whatever set
`browse_set_html` is handed. `card_of_type` skips them too, so a deleted profile
is not the profile — unreachable today, since the button is on posts only, and
stated here because the op is type-agnostic and the next type will meet it. A
tombstone that is nevertheless opened — a page held on screen while the delete
arrives from another device — draws one dim line, *deleted*, rather than an
empty card.

**Two taps, and the second one means it.** The first tap arms: the button
becomes the word *sure?* for three seconds and then goes back to being a bin.
The armed state is the page half's and is held in no var — it is a question
being asked, not a thing the world knows — and it is re-applied after every
repaint, because a repaint that landed mid-question would otherwise answer it.
`/keep`'s remove pill is the house idiom for a destructive control that asks;
this one asks in the button itself, because a control row button has no picture
to sit on and a pill over the toolbar would cover its neighbours.

**Undo, and the general lesson it cost.** `/undo` records a turn's var writes by
scanning the outbox, and `/undo/late` moved that scan to the end of the update
chain — *"the pattern holds only while this is the outermost `update` link"*.
This node is newer than `/late`, so its write lands after the scan and would
have been invisible to undo, exactly as `/late` predicted in its own hostile
cases. Rather than reorder another node, this one records its own step through
`/undo`'s two library calls (`undo_var_before` for the prior value out of the
pre-event snapshot, `undo_push` for the step) — the snapshot is taken at the top
of this link, before anything is written, which is the same instant `/undo`
takes its own. One step, one turn. **The residual, named:** every node newer
than `/late` that writes a var has this problem, `/kinds/new` included — making
a post is not undoable today for the same reason. The general fix is `/late`'s
own named rung (move the scan to `/turn-end`, or make the record a turn-end
act), and it is a foundation change to `/undo`, not to this ask.

**Anticipation.** `CardDelete` names no type and reads no surface: the same
event deletes a project or a profile the day one is asked for, and only the
button is posts-only. What a later type has to add is a button, not an op.

**The one trap a later type must step over, written down before it is met.**
`/guard/singleton` asks "does this owner already hold a card of this type?" of
the raw list, tombstones included — so a deleted profile would still count as
the profile you hold, and `/me`'s `CardEnsure` (which this node teaches to
skip tombstones through `card_of_type`) would mint a replacement that the
guard then discards as a duplicate: no profile on the screen and no way back
except undo. Nothing reaches that today, because the button is on posts and a
post is not a singleton. Whoever makes a singleton type deletable must first
teach `cards_type_is_singleton`'s neighbour to ignore tombstones. Named here
rather than pre-built: it is a seam somebody else's ask will take
(`/anticipation` — ship the ask, not the foundation).

## hostile cases

- **Deleting a card you do not own.** The event finds `from` on it and does
  nothing; the button was never drawn.
- **Deleting a card that is already a tombstone.** Nothing changes and no write
  is queued, so undo is not offered a step that undoes nothing.
- **One tap and then nothing.** After three seconds the button is a bin again
  and no event was sent. A repaint inside those three seconds does not answer
  the question either way.
- **The open card is deleted from another device.** The set no longer holds it,
  so `/browse`'s existing rule draws the set instead — and where the page is
  drawn anyway, it is one dim line.
- **A stale device resends the pre-delete list.** `/guard` compares `edited`;
  the tombstone is newer and survives.
- **`/undo` unticked.** This node calls `undo_push` and `undo_var_before`, as
  `/guard/revert` already calls `undo_apply` — the two travel together; deleting
  without undo is not a shape this tree composes.
- **`/posts` unticked.** It is this node's parent, so this node goes with it.

## glossary

- **tombstone**: a card kept in the list to say it is deleted — `deleted`
  stamped, body emptied. What deletion leaves behind, so that no merge can
  resurrect it.

## code description

`delete.rs` extends `update` with `CardDelete {id, t}`. It takes the pre-event
snapshot at the top of the link, lets the chain beneath run, turns the named
card into a tombstone through `/cards`' own `cards_read` / `cards_write` pair,
clears `browse_open` if that card was the one on screen, and records the step
with `delete_record`. `delete_tombstone(card, now)` is the emptying, and
`delete_gone(card)` is the one-line test everything else asks.

`delete.rs` extends three seams so a tombstone is nowhere: `browse_cards` (every
`/browse` surface, the map included), `posts_set` (the posts surface), and
`card_of_type` (the lookup a consumer asks with). It extends `card_page_html` to
draw the dim *deleted* line for a card that is opened anyway.

`delete.rs` extends `tool_controls`: with the posts tool open on a post of your
own, the bin goes in front of `/undo`'s button through `/posts`'
`posts_before_undo`, wearing the posts tool's own colour (`/glyphs` — a control
is not undo's blue). `delete_bin_svg` is the drawn glyph; `delete_open_card` is
the "is there an own post on screen" test the button is gated on.

`delete.js` is the two-tap. A capture-phase click on `[data-ev="posts_delete"]`
never reaches `/loop`'s delegated send: the first tap arms and repaints the
button into the word *sure?*, a 100ms ticker keeps that word on the button
through any repaint, three seconds disarms, and the second tap sends
`CardDelete` with the id off the open `.card-page`.

`delete.css` sizes the armed button for a word rather than a glyph and dims the
*deleted* line to the ignorable step of `/taste` 2.
