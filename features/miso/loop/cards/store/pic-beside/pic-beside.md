# pic-beside
*a picture's bytes live beside the card; the card keeps a reference*

> (transcripts/2026-09-03-invite-test.md#p159)
> do everything now

> (transcripts/2026-09-03-invite-test.md#p158, the diagnosis it answers)
> `feature_Cards.held('', 1)` on ash's real list = **176,020**, i.e. room = **−16,020**. Verdict: **REFUSED**, for the measured poster and for both observed phone posters

> (features/miso/loop/cards/cards.md, the rung named on the day cards arrived)
> The honest fix is a var per card and a blob path for pictures, which is the same later rung the merge limit names.

## user

Pictures no longer weigh down your cards. A photograph you take is kept on its
own beside the card and the card just points at it, so the list your phone
sends when you change a word is a few kilobytes of text instead of a hundred
and seventy. Your cards still look exactly the same. A picture you take with
no signal shows straight away and goes up when you are back.

## spec

Measured before anything was built, on the four real worlds on this machine:

| world | cards | list | picture bytes |
|---|---|---|---|
| ash | 25 | 180,541 | 170,591 (94%) |
| Tara | 19 | 133,193 | 125,270 (94%) |
| (third phone) | 15 | 88,019 | 81,972 (93%) |
| (fourth phone) | 14 | 108,323 | 101,751 (93%) |

Every edit of any word on any card resends that whole string to the server,
`/exchange` reads it twice more per linked person, `/remember` appends it to
the op log, `/world-cache` writes it to IndexedDB — and `/cards`' list budget
compares a new poster against it and refuses (#p158).

**A picture block's `data` becomes a reference instead of bytes.** It was
`data:image/jpeg;base64,…`; it becomes `pic/<id>`, twenty-eight characters,
where `<id>` is twenty-four hex characters naming bytes the server holds.
Nothing else about the card changes.

Measured on the rig, on the road each thing actually takes: a photograph
through `/frame` cost the list **28 bytes instead of 12,663** (a 9,478-byte
JPEG at `EDGE`); a whole new photo post through `/photo` cost **236 bytes**
including its title, its text and its reference; and `feature_Poster.draw`,
the function whose 22 KB answer #p158 caught being refused, now returns
**28 characters**. A planted world in the old shape went from a 13,158-byte
list to **523** and back to 13,158, byte for byte.

**This is why the change is nearly free at every consumer.** Every reader of a
picture in the tree — `card_page_html`, `card_tile_html`, `/map`'s
`map_face_of`, `/portrait`'s face, `/reel` and its four descendants, `/live`'s
pin, `/one-medium`, `/profile-first`'s gate, `/sound`, `/guard`'s blank test —
does one of exactly two things with `data`: asks whether it is empty, or drops
it into an `<img src>`. A relative URL answers both questions the way a data
URL did, so **not one consumer is edited**. The two shapes coexist for good: a
card written last week still carries its bytes inline and still draws.

**The clip road could not be reused; this is its sibling.** `/mirror` serves
`blob/<id>` out of the *caller's own* directory (`~/.miso-blobs/<key>/`), which
is exactly why a copy's clip "stays with its owner" and will not play in the
recipient's world. A picture must show — the whole point of handing a card
over is that you see the face — so this store is addressed by id alone,
`~/.miso-blobs/pics/<id>`, with authority decided per request rather than by
which directory the bytes sit in.

**The authority rule, stated: the reference in your world is the capability.**
`GET pic/<id>` is answered when the caller is logged in and the caller's own
`cards` list contains the string `pic/<id>`. Nothing else. That rule needs no
new concept — it is exactly the visibility `/exchange` already grants, so it
follows `/co-members` and every later way of handing a card over with no code
here; a `/delete` tombstone that drops the reference takes the access with it;
and it fails closed for anyone the card was never given to. Ids are unguessable
(96 bits), so enumeration is not a road either.

**Writing is open to any logged-in caller, and write-once.** `POST pic/<id>`
stores the body if that id holds nothing yet, and answers `ok` without writing
if it does. Bytes at an id therefore never change, which is what makes the
long cache header honest and what stops one user overwriting another's
picture: to overwrite you would have to guess an id already in use, and to
guess it you would already have to hold the card.

**The bytes become a reference where the picture is made, not where it is
sent.** `feature_Cards.shrink` is `/cards`' own named seam — *the seam every
road that stores a picture goes through* — and `feature_Poster.draw` is the
video half's twin. Both are taken by redefinition (`/frame`'s precedent, which
takes `chose` the same way), so the *budget gates in `/cards`, `/photo` and
`/poster` measure the reference*, all three become true again, and no other
feature's file is edited. A third redefinition of `feature_Loop.send` is a net
under both: any road that reaches an event with an inline `data:image/…` in it
— one written after this node, say — has it converted there instead.

**Offline is the local copy, not a fetch.** Every picture this device makes is
kept in its own IndexedDB store (`miso-pics`), and a `MutationObserver` swaps
`src="pic/<id>"` for an object URL wherever the bytes are held here. So the
picture is on screen in the same frame it was taken, before any upload, with
no network at all — and it stays there offline, forever, whatever the server
knows. The upload rides **its own queue, not `/messaging`'s outbox**: the
outbox carries JSON ops through `POST /msg` and bytes do not fit that road.
The queue drains at load, straight after a capture, and on `online`, and a
picture leaves it only when the server has answered `ok`.

**A picture the device does not hold and cannot fetch is hidden, not broken.**
The one new failure this shape can produce is a reference whose bytes have not
arrived: a recipient looking at a copy during the seconds between the owner's
op landing and the owner's upload finishing. The image's `error` fires, the
element is hidden (a broken-image icon is worse than an absence), and one
retry is scheduled four seconds later — one, not a loop, because a picture
that is genuinely gone must stop asking. What a genuinely-gone picture does
keep costing is one request each time the surface it is on is repainted, since
each repaint is a new element with a fresh `src`: measured at three requests
over eleven seconds on the rig, against a timer loop's dozens. That is the
price of the other half of the behaviour, which is worth more — a picture that
turns up late appears by itself, with no reload, the moment a repaint asks for
it again.

**Retrofit, both ways, through the op door** (`/retrofit` is doctrine).
`POST pic/retrofit` walks every world, or one named world: `out` moves each
inline picture into the store and writes the reference in its place, `back`
reads the bytes out of the store and writes the data URL back. Neither
direction touches `edited` — moving bytes is not an edit — which is what makes
the pair symmetric under `/guard`, whose merge resolves a tie to the incoming
list. The write goes in the same door `/exchange` uses, `handle_msg` with
`context_user_set`, so `/guard` merges it, `/converge` relays it to open
pages and `/remember` logs it: **the op log holds every prior value and one op
restores it**, and `back` is the tested inverse besides. `dry` reports what
would move and writes nothing. The id in `out` is the SHA-256 of the bytes,
truncated, so running the retrofit twice is a no-op and the same photograph in
two worlds converges on one file.

**The retrofit deliberately does not fan out.** It writes through `handle_msg`
rather than `POST /msg`, which is the route `/exchange` watches, so a
retrofitted card does not travel to the people who hold copies of it. Their
copies keep their inline bytes and keep drawing; they take the reference the
next time the owner really edits the card. This is the quiet path, and it is
the right one: a backfill must not put a hundred writes on the wire.

**One list, not one var per card — measured, and this is the argument.** The
brief left the choice open. Removing the pictures takes ash's list from
180,541 bytes to under 10,000, an eighteenfold cut, and the costs that made a
per-card var attractive are all proportional to the list: four world reads per
write are now four reads of ten kilobytes, an op-log line is ten kilobytes, the
wire is ten kilobytes against a cap of sixteen thousand. A var per card would
buy two more things — a same-card concurrent edit resolved per *field* rather
than per card, and a write proportional to one card rather than to the list —
and it would cost a new merge kind, a migration of the var address every
consumer knows, and `/guard`, `/exchange`, `/revert` and `/world-cache` all
rewritten around it. That trade is worth making when the list is large again
for a reason pictures are not; it is not worth making today. Named as the next
rung, not built (`/anticipation`).

**Parked, with the seam each one joins at:**

- **a picture as big as a photograph.** `/cards`' `CAP` (8 KB per picture) and
  `EDGE` (256 px) exist because the picture travelled in the list. It does not
  any more. Raising them is now a change to two numbers and a bigger upload —
  the seam is `shrink`, which this node already wraps.
- **the clip on the same road.** `/mirror`'s `blob/<id>` could be redefined to
  answer from this store under the same capability rule, and a copy's video
  would play. Named in `/exchange`'s own parked list as "withdrawing a card";
  this is the mechanism it was waiting for.
- **pruning the local store.** The device keeps every picture it ever made.
  Twenty-five pictures is 200 KB, so nothing is pruned here and nothing is
  evicted — an eviction that dropped the one picture the device had not yet
  uploaded would destroy the thing the store exists to protect. When the store
  is pruned it must prune only ids the server has confirmed.
- **a var per card**, above.

## hostile cases

- **A card written before this node.** Its picture is still `data:…` inline
  and still draws, in the page, the tile, the pin and the reel. Nothing
  converts it until the retrofit is run, and a world that is never retrofitted
  never breaks.
- **The device is offline when the photograph is taken.** The bytes go to the
  local store and the object URL is on screen in the same frame; the reference
  goes into the card; the upload sits in the queue. Back online, the queue
  drains. Nothing was waiting on the network at any point.
- **The upload fails, repeatedly.** The id stays in the queue and is retried at
  every load and every `online`. The owner keeps seeing the picture from the
  local store throughout, so a failure that never resolves looks to its owner
  like nothing at all — and to a recipient like a card with no picture, which
  is the truth.
- **A recipient opens the copy before the owner's upload lands.** `GET` is 404,
  the image is hidden, one retry four seconds later finds it. If it does not,
  the card reads as a card with no picture until the next paint.
- **A recipient who was never given the card asks for the picture.** Their own
  cards list does not carry the reference, so 403. The bytes are not readable
  by holding the id alone.
- **Two devices of one person.** The second holds the card (via `/converge`)
  and so holds the reference, and fetches the bytes from the server exactly as
  a recipient does. It does not need the local store.
- **IndexedDB is unavailable** (a private window, a quota refusal). The
  in-memory copy still serves this session and the upload still runs; after a
  reload the device falls back to fetching its own picture from the server like
  anyone else. Never fatal, and never silent about it in the log.
- **A picture the server holds and the card no longer names** (a `/delete`
  tombstone, a picture replaced). The blob stays. Nobody can read it — no world
  carries the reference — and reclaiming it is a sweep, not this node's.
- **`POST pic/<id>` for an id already stored.** Answered `ok`, nothing written.
  That is what makes the queue's retry safe.
- **The retrofit run twice.** The id is the content hash, so the second run
  finds every reference already in place and writes nothing.
- **`back` run on a world whose blobs were deleted.** The reference is left
  exactly as it is and the world is reported unchanged rather than emptied: a
  revert that cannot restore the bytes must not destroy the reference to them.
- **This node unticked.** `shrink`, `draw` and `send` are handed back at
  runtime by the linker's fragment gate; new pictures inline again as they did
  before. Cards already carrying references stop drawing their pictures — which
  is the honest reading of "turn the picture store off" and the reason `back`
  exists.

## glossary

- **reference**: what a picture block holds instead of bytes — `pic/<id>`, the
  address the server answers with the picture.
- **the picture store**: `~/.miso-blobs/pics/<id>`, addressed by id alone, one
  file per picture, never rewritten.
- **the local copy**: the device's own IndexedDB store of every picture it
  made, which is what makes a picture show before and without an upload.
- **the picture queue**: the upload road, separate from `/messaging`'s outbox
  because bytes do not travel as JSON ops.

## code description

`pic-beside.rs` extends `route`, and being the newest node's, that link is the
outermost on the chain — outside `/edit`'s turn boundary, which is what lets
`pic_holder` name another world and read its live value. `/exchange` documents
the same reasoning for the same reason.

`pic_route` claims three paths and hands everything else to `existing`.
`GET pic/<id>` reads `pic_file(id)` and answers it as `image/jpeg` marked
private and immutable for a year, after `pic_may_read` has said yes.
`POST pic/<id>` writes `r.raw` if the file is absent, refuses a body over
`PIC_MAX`, and answers `ok` either way. `POST pic/retrofit` runs the backfill.

`pic_may_read(cookie, id)` is the authority rule in one function: the caller's
key from `sender_of`, then `pic_holds(key, id)` — `context_user_set` to that
world, `cards_read`, restore, and a substring test for the reference.

`pic_retrofit(world, mode, dry)` walks `pic_worlds()` (the op-log directory,
percent-decoded back to keys) or the one world named, and calls
`pic_move_list` on each, which rewrites the list and reports what changed.
`pic_out_block` decodes a data URL, hashes it to an id, writes the file and
returns the reference; `pic_back_block` reads the file, sniffs its type from
the first bytes and returns the data URL. `pic_write_list` is the op door:
a `CtxOp` on the cards var handed to `handle_msg` with `context_user_set`
naming the world, the same door `/exchange` writes a copy through.

`pic-beside.js` is the page half. `mint(dataUrl)` is the one conversion: a
random id, the blob decoded synchronously so an object URL exists before the
function returns, the record put in the local store, the id queued. It is
called from three redefinitions — `feature_Cards.shrink` and
`feature_Poster.draw`, the two roads a picture is made on, and
`feature_Loop.send` as the net beneath them.

`resolve(root)` swaps the reference for a local object URL on every
`img[src^="pic/"]` it finds, and `watch()` runs it from a `MutationObserver`
so it covers the loop's repaints, `/map`'s own DOM writes and any surface
written later. The capture-phase `error` listener is the other half: an image
that could not be fetched is hidden and retried once.

`open`, `put`, `all` and `drain` are the local store and the queue: IndexedDB
`miso-pics`, every record read into memory at boot so `resolve` can answer
synchronously, and a sequential POST of everything not yet confirmed, run at
boot, after each mint and on `online`.

`pic-beside.css` is one rule: a picture that could not be fetched takes no
space rather than drawing a broken icon.
