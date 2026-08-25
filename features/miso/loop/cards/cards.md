# cards
*one object for everything: a card is owned, typed, and made of blocks*

> (transcripts/2026-08-25-accounts.md#p9)
> OK, so there's a basic object here which is a "card", right? It's the same for a post, for a profile, for a group, etc. We have a bunch of cards, and links between them; and cards are 1-5 phone screens long.

> (transcripts/2026-08-25-accounts.md#p10, revision)
> instead of subclassing profile as a type of card, let's give card a "type" field - one of those could be "profile".

> (transcripts/2026-08-25-accounts.md#p11, revision)
> also, what's interesting is: our dictaphone tool draws a grid of thumbnails for recordings (audio + transcript) - I'd argue that those recordings are cards, in prototype.

## user

A card is a page you own: a title, a picture, and paragraphs, in the order you
put them. Tap a paragraph to write in it, tap away to keep it. Tap the picture
to choose one from your phone — it is shrunk on the device before it is stored,
so a card stays small. Your cards live in your world: they are on all your
devices, and nobody else has them until you hand one over.

## spec

The **card** is miso's one content object. Everything the app will hold — a
profile, a post, a project, a group, a recording — is a card, told apart by a
`type` field rather than by a class of its own (#p10). A card is
`{ id, owner, type, created, edited, blocks: [..], links: [..] }`.

`id` is `<owner>.<created ms>`, so a copy of a card is recognisable as the same
card in whatever world it lands in. `owner` is the user's name as `auth/whoami`
gives it, stamped by the page half at creation because the name lives behind the
cookie and not in the world. `type` is a free string; `profile` is its first
value. `created` and `edited` are epoch milliseconds.

`blocks` is the body, in order. A block is `{kind, ..}`: `title` and `text`
carry `text`, `picture` carries `data` — a data URL, inline in the card for now.
Blobs stored beside the card (#p11's audio) are the later rung; a picture small
enough to inline does not need that machinery yet.

`links` is declared and empty. Typed card→card links are the next node's, and
the field is here so the object's shape is honest from the first card written
rather than migrated later.

A card has **two renderings**, both this node's: the **tile** — picture and
title, the thumbnail that sits in a grid or a list — and the **page**, the one
to five phone screens you scroll and edit. The dictaphone's grid of recordings
is the tile's named future consumer (#p11); nothing in `/dictate` is touched
here, and that migration is its own rung.

**Editing is in place.** A text block is `contenteditable`; what you type
reaches the store when focus leaves it, which is "tap away to keep". No editable
block carries a `data-ev`, because a click that runs the loop would repaint
`#app` under the caret.

**The store is the owner's world**: one `cards` /var, a JSON list, user-scoped,
last-write, bridged to the page at `s.cards`. `/ask`'s `asks` list is the exact
precedent. **Known limit, accepted:** last-write on the whole list means two
devices editing two different cards at the same moment lose one of the edits.
Per-card merge — a var per card, or a merge kind that folds lists — is a later
rung, not a surprise.

**The picture cap is hard, visible, and set by the wire.** A chosen image is
drawn to a canvas at 256px on its longest edge and encoded as JPEG, quality
stepping 0.8 → 0.65 → 0.5 → 0.4 → 0.3 → 0.2 until it fits. If the smallest
attempt is still over, the picture is refused out loud — "that picture is too
big to keep" — rather than stored.

The cap is **8KB of data URL for one picture and 14KB for the whole list**, and
the numbers are `/messaging`'s, not a guess: the entire cards list travels as
one op in one `POST /msg`, whose body is truncated at 16384 bytes. A truncated
message is invalid JSON, is answered `400 untyped message`, and is retried by
the outbox **forever** — one oversized picture jams every op the instance will
ever send, the join included. This was found on the rig with a 40KB cap, which
is what the first version of this node shipped with; it is written down here
because the number is a load-bearing consequence of another node's limit and
will silently rot if that limit moves. The honest fix is a var per card and a
blob path for pictures, which is the same later rung the merge limit names.

**Placement, recorded rather than assumed.** #p9 talked about a root-level
`cards` and a regroup to make room. The node is a child of `/loop` instead: the
card is loop state with a renderer and an event, `loop` had a free child slot
(this is its sixth, the cap), and a root regroup is a prompted event of its own
rather than a side effect of this build.

## hostile cases

- **No card yet.** The first open creates one: the page half sends `CardEnsure`
  with the logged-in name, and a card of that type and owner is made if none is
  there. A second `CardEnsure` finds it and does nothing.
- **The var arrives malformed.** Anything that is not a JSON array reads as the
  empty list; nothing throws, and the next write starts from `[]`.
- **A 5MB photograph.** Downscaled before it touches state, or refused with a
  message. The full-size file never enters the world.
- **A picture that fits but the list will not.** Refused too — "no room for
  that picture" — because a list over the message budget jams the outbox.
- **An unknown block kind** renders as nothing rather than as an error.

## glossary

- **card**: an owned, typed page of blocks with an id and a time — the unit
  everything in miso is made of.
- **block**: one item in a card's body — `title`, `text` or `picture`.
- **tile**: a card's thumbnail rendering, for grids and lists.
- **page**: a card's full rendering, one to five phone screens of blocks.

## code description

`cards.vars` declares the store: `cards`, a JSON list in a string,
`(user, last-write, own)`, bridged to the page key `cards`.

`cards.rs` extends `update` with the three card events. `CardEnsure {owner,
type, t}` creates a card if the owner has none of that type — title seeded with
the owner's name, an empty picture, an empty paragraph. `CardEdit {id, i, text}`
writes one text block; `CardPic {id, i, data}` writes one picture block. Both
stamp `edited`. All three read and write through `cards_read` / `cards_write`,
the pair that keeps the var's address in one place, exactly as `/ask` does.

`cards.rs` also holds the two renderers. `card_page_html(card)` draws the
scrolling page — title, picture (or its empty placeholder), paragraphs — each
block tagged `data-card` and `data-block` so the page half knows what it is
editing. `card_tile_html(card)` draws the thumbnail: the picture as the tile's
face with the title beneath, and a dimmed initial where there is no picture yet.
`card_of_type(owner, type)` is the lookup a consumer asks with, and
`card_esc` is the one HTML escape both renderers pass every stored string
through.

`cards.rs` extends `render` with a **dev mount for the tile**, and nothing else:
with `cards_tiles` set in the loop state — which the page half sets only when
the URL carries `?cardtiles=1` — a grid of every held card's tile is appended.
It is the `?readout=` convention: an affordance that costs nothing until it is
asked for, and the seam the dictaphone grid will grow into. The page half
writes the var on every load, from the URL, so the mount turns itself off
again — a device that asked for tiles once is not stuck with them.

`cards.js` is the page half of editing. It delegates on `focusout` for
`[contenteditable][data-block]` and sends `CardEdit`; it delegates on `click`
for `.card-pic` and opens a file input it made at load; the chosen file goes
through `shrink` and then the list budget (`held`), and is sent as `CardPic` or
refused into `#cardToast`, a note it owns that lives outside `#app` so a repaint
cannot take it away. It sets the `tiles` var once, from the URL.

`held(id, at)` is the budget read, and it discounts the block being replaced:
charging a new picture for the one it overwrites would let you set a picture
once and never change it. It reads the bridged `s.cards`, which lags the store by exactly one turn —
the payload bridge republishes from the view frozen at the top of the turn, so
what a fragment reads is the value before this event's edit. That is fine for a
budget and would not be fine for a renderer, which is why both renderers are in
Rust and read the context directly.

`cards.css` styles the page and the tile against `/taste`: the `#161619` page
ground, 14px card radii, hierarchy by dimness, and one accent — the dusty blue
that already means *chosen* — on nothing but the focused block's border.
