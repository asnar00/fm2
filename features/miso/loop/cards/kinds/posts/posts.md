# posts
*say something, from where you are*

> (transcripts/2026-08-25-accounts.md#p87)
> my hitlist before tara comes in: 1) projects; 2) posts; 3) map view.

## user

A **posts** tool in the toolbar, drawn as a speech bubble. It shows the posts you hold — yours and the ones invited people wrote — newest first, as the picture-led list or the grid; the picker at the top left chooses which.

**new** in the control row makes a post. The page opens with the caret already in the words; write, add a picture if you have one, and the post carries where it was made.

A post you did not write is read-only, like any other card that arrived.

## spec

A post is a `/card` of type `post` (#p9 — a card is the same for a post, for a profile, for a group), made through `/new`'s one door, so it is written, guarded, undone, copied and located by the machinery that was already there. This node is a surface for that type and three small differences in how a card of it is drawn. Nothing under `/cards` changes.

**The object.** `CardNew {type:"post", title:"", owner, t}` — the same three-block body as a profile: a title, a picture, a paragraph. The title stays empty and is not drawn: a post is not named. The body is deliberately unchanged so `/location` finds a card to put a place on, `/frame` finds a picture to crop, `/keep` finds a block to save and `/guard` finds an id to merge. The name comes from the page half (`auth/whoami`, `/me`'s own lookup order), because it lives behind the cookie and not in the world.

**The page.** `card_page_html` is extended for cards of type `post`: the title block is taken out of the page the chain beneath drew, the words are moved above the picture (a post is words), the empty paragraph says *say something*, and the page carries a `post` class so the words can be brighter and a size up — the weight the name carries on a profile. Moving the words is a rendering move only: the blocks keep their order and their indices, so every `data-block` the page half sends still names the block it always did.

**The place.** Nothing new. `/location`'s page half reacts to a dim place pill appearing inside any `.card-page` and asks the phone once; a new post's page appears the moment it is made, so the fix lands on it. A picture with a GPS tag beats the device fix through `/from-picture`, unchanged. No seam was added to `location.js`.

**The surface.** `/browse`'s, whole: the picker, the grid, the list, `/map`'s map, the open-a-card path, the way back. Drawing the set through `browse_set_html` rather than choosing a view here is what puts the posts on the map for nothing — `/map` took that seam, and a fourth view will be free the same way. The tool registers as `posts` in `tools_list`; `render` draws the surface while it is open, on **`posts_set()`** — the cards of type `post`, newest first by `created`, with the id as the tie-break so every device agrees. That set is read from `/cards`' store rather than through `browse_cards`: that chain has already been narrowed to profiles for 👤, and a second surface asking it a different question would have to undo that. `browse.rs` is untouched.

**The row.** A post has no title, so `/browse`'s two row seams are re-aimed, keyed on the card's type rather than on which tool is open — a post says who wrote it wherever it is drawn. `browse_title_of` returns the author's name, which `/portrait` puts in the bold cell that is the row's identity; `browse_row_left` returns nothing, because "post" on a surface of nothing but posts is the redundancy `/people` already ruled on. The date is `/browse`'s right-hand cell and the excerpt is `/portrait`'s, both untouched. In the grid, `card_tile_html`'s caption and its empty face come out blank for a post (they are drawn from the title), and the author goes into them.

**new.** One control in the row while the tool is open, drawn as a plus, wearing the posts tool's own colour — `/ember`'s pick for the name `new` is byte-summed to the same blue it gives `undo`, and two controls side by side in one colour read as one pair (`/taste` 3) — and inserted **in front of `/undo`** — undo is last in every row and a newer node's links land after undo's by provenance, so keeping the invariant is this node's job. Its tap is taken in the capture phase by `posts.js`, which sends the `CardNew` and then puts the caret in the words; `/loop`'s delegated click never sees it, so one tap makes one post.

**A second post is not a duplicate of the first.** `/guard` discards a card that arrives **blank** for an owner who already holds one of its type — that shape is `/me`'s ensure racing an empty world, and dropping it is what keeps a profile from being doubled. The premise, *you hold exactly one of these*, is true of a profile and false of a post: every post is blank at the instant `new` makes it, so with one post already written the rule threw the next one away before it could be typed into. Rig-found: the second `new` tap simply did nothing. This node answers the question the discard rule is really asking — *is this a copy of a card that should be unique* — with **no** for a post (`cards_guard_has_type`), and leaves every other type to `/guard`'s own answer. Nothing is dropped that was not dropped before; a card that would have been lost survives. `guard.rs` is untouched, and the general form — a type declaring itself a singleton — is named as the foundation the next card type will want.

**The caret, twice.** The world answers a moment after the post is made — the op comes back, the place arrives — and a repaint landing between the caret and the first keystroke can leave the words with nobody in them (seen in one rig run in three). `posts.js` puts the caret back once, 400ms later, and only into a post that is still empty with nothing else focused. After that the caret is the writer's, and `/keep` carries it through every later repaint.

**Copies.** Free: `/exchange` copies every card a writer owns to the people their invites link them to, with no test on type. Alice's posts tool shows ash's posts, read-only, because they carry `from`.

## anticipation

Shapes reserved, not built (`/anticipation`). A post's **project**: `links: [{kind:"in", to:<project card id>}]` on the card — `/cards`' `card_new` already gives every card an empty `links` array, so a post in a project is a link, not a new field and not a new type. **Rings**: a `ring` field on the card, absent meaning `"invited"` — today's audience is exactly the invite links, which is what `"invited"` will mean, so the default is what already happens and no post has to be rewritten. Comments and withdrawal are named and parked.

Named and NOT this node's to fix: on `/map`'s view a post with no picture draws a blank pin, because `map_initial_of` reads the card's own title block and a post has none. `/map` is provenance-newer than this node, so its chain cannot be extended from here; the one-line answer — fall back to the owner's name, as `browse_title_of` does — belongs in `map.rs`.

## hostile cases

- **A post with no words and no picture.** It renders: an empty paragraph saying *say something*, an empty picture, a dim place pill; its row is the author, the date and no excerpt.
- **Nothing posted yet.** One quiet line where the set would be — *say something* — not a box round nothing. Except in the map view, where `/map`'s own ruling stands: an empty map is still a map.
- **Forty posts.** The list and the grid scroll inside their own ground, `/browse`'s measurements unchanged.
- **A foreign post.** No contenteditable anywhere, no empty picture block and no dim pill (`/exchange` takes all three away), so the words-above-picture move finds no picture and leaves the page as it is.
- **The post that is open is deleted, or is not a post.** The set is drawn instead, silently.
- **Two posts in the same millisecond.** The id breaks the tie, so both devices show the same order.
- **`/ember` unticked.** `tool_colour` is empty and the new button is a plain control.
- **`/guard` unticked.** This node needs it: `cards_guard_has_type` is `/guard`'s chain and the linker refuses the composition outright — the same hard dependency `/under-account` has on `/tools`' `tool_controls`. Checked, and the error names both nodes.
- **`/portrait` unticked.** The bare `.crow` row: an empty left cell, the author in bold, the date.

## glossary

- **`/post`**: a `/card` of type `post` — words, a picture and a place, with no name.

## code description

`posts.rs` — `render` /extension/ draws the surface while the `posts` tool is open: `/browse`'s picker, then `card_page_html` for the open post or `browse_set_html` for the set. `tools_list` /extension/ registers the tool; `tool_controls` /extension/ adds the new button through `posts_before_undo`; `update` /extension/ turns a tap on the open tool's own button into "back to the posts", reading both vars before the chain beneath clears them.

`posts_set` is the subset and the order: type `post`, newest `created` first, id as tie-break. `posts_author` is the name off `owner`; `posts_is` the type test.

`browse_title_of` and `browse_row_left` /extensions/ re-aim the row cells for a post and delegate for everything else. `card_tile_html` /extension/ fills the grid tile's two blank cells with the author. `card_page_html` /extension/ marks the page, then `posts_no_title` cuts the title block out and `posts_text_first` swaps the picture block and the words in the drawn page.

`posts_bubble_svg` and `posts_plus_svg` are the two drawn glyphs.

`cards_guard_has_type` /extension/ answers `false` for a post, so `/guard`'s blank-duplicate discard cannot reach a type you may hold many of.

`posts.js` — `make()` sends `CardNew` and `caret()` puts the caret in the words; `settle()` puts it back once after 400ms if a repaint took it; `name()` is the guarded lookup of the author's name. The listener takes `posts_new` in the capture phase so `/loop` does not send it on.

`posts.css` — the post page's words, and the drawn plus in black on a tint.
