# post-time
*a post is dated by its photograph, or by the moment it was made*

> (transcripts/2026-08-25-accounts.md#p103)
> posts also need a "post time" - so we can order them most recent first in grid and list view. post time should come from the photo time, or entry time if no photo.

## user

A post takes its time from the photograph on it. Post a picture you took
yesterday afternoon and the post sits among yesterday's posts, and its row
says yesterday's date — not *just now*. A post with no picture takes the
moment you made it. Grid and list are both newest first by that time.

## spec

**The datum.** A post card grows a top-level `when` — epoch milliseconds,
beside `created` and `edited` — and `when_from`, the word for where that time
came from. `/cards`' `card_new` gives neither, so **absent is the ordinary
case and it means "the making was the moment"**: a post with no `when` orders
by `created`, and every post written before this node existed reads correctly
without being touched or migrated. There is no event for "made now", because
the card already carries that time.

**From the photograph.** At `chose` time — `/cards`' seam for a file the user
picked, taken by redefinition with the previous one kept in a closure, which
is `/from-picture`'s idiom and composes with it — the ORIGINAL file's EXIF is
read for `DateTimeOriginal` (tag `0x9003` in the Exif sub-IFD, whose pointer
is tag `0x8769` in IFD0), falling back to IFD0's own `DateTime` (`0x0132`).
Reading the original matters: `/frame`'s canvas re-encodes the picture and
throws every EXIF tag away, and provenance puts this node outside `/frame`'s
link, so the file this node sees is the one off the phone. The time rides on
the picture — `CardWhen` is sent after `/cards` sends `CardPic`, at the end of
both the framed and the unframed path — so a cancelled framing or a photo
refused for being too big leaves the post's time exactly as it was.

Only a card of type `post` is given a time. A profile's date is when you last
edited it, which is what `/browse` already says, and a photograph on a profile
should not move the profile into last year.

**The date is believed as it is written.** EXIF carries no time zone: the
camera wrote the wall clock where the picture was taken, and that reading is
turned into the epoch millisecond it names in the device's own zone. A camera
whose clock is wrong dates its post wrong — including into the future — and
that is the honest answer: the alternative is the app silently disagreeing
with the timestamp the user can read on their own photograph. Time zones are
parked.

**The order.** `/posts`' `posts_set` is resorted: newest first by
`when || created`, with the id still breaking the tie so every device agrees.
The chain beneath keeps deciding *which* cards are in the set; only their
order is this node's, and the grid and the list both read that one set, so
neither view had to be told about the change.

**The date on the row.** `/browse` decided a row's date at the call site —
`browse_when(c["edited"], …)`, written out twice, once in `browse.rs` and once
in `/portrait`'s row. There was no seam to extend, so one was made: **
`browse_when_of(card)`**, whose default is `edited` — exactly what both call
sites passed — and which this node redefines to `when || created` for a post
and delegates for everything else. The refactor is behaviour-preserving by
construction (`agents.md` step 3); with this node unticked every row reads as
it did. Keying on the card's type rather than on which tool is open means a
post carries its own date onto any surface that asks the seam.

## anticipation

Shapes reserved, not built (`/anticipation`). **Setting the time by hand** is
the same `CardWhen` with `source: "hand"` — the event already carries the
word, so the hand-set time is a control, not a field and not a migration.
**Showing where the time came from** is `when_from`, already stored and drawn
nowhere. **A post inside a time range** reads `when || created` through
`post_time_of`, the one place that fallback is written down.

## hostile cases

- **A photograph with no date.** No `CardWhen`; the post keeps the time it
  had, which for a new post is the moment it was made.
- **A malformed Exif sub-IFD offset** — a pointer past the end of the slice,
  an entry count out of range, a truncated ASCII value. Every offset is bounds
  checked against the slice before it is followed, and the whole walk sits
  inside one `try`: the answer is "no date", never a throw.
- **A file that is not a JPEG**, or one whose APP1 is not EXIF. No date. The
  picture itself is `/cards`' business and is unaffected.
- **A camera clock set to next year.** Stored as it is, so the post sorts to
  the top. Said out loud above; correcting a user's own timestamp is worse
  than believing it.
- **Changing the picture for one with a different date.** The new `CardWhen`
  replaces `when` and the post moves. Changing it for one with *no* date
  leaves the old time standing rather than reverting to the making — the time
  that was read is still the truest one we have.
- **`/frame` or `/from-picture` unticked.** Both take the same `chose` seam
  and all three links compose in provenance order; this one is outermost
  either way, and the EXIF walk here is self-contained, so nothing is lost.
- **`/portrait` unticked.** `/browse`'s bare `.crow` row asks the same seam
  and reads the same date.
- **A foreign post.** `/exchange` clones the whole card, so `when` travels
  with it and a copy sorts and dates exactly as the original does.
- **Two devices, one post.** `/guard` merges by the newer `edited`, and
  `CardWhen` stamps `edited`, so a time set on one device is not merged away
  by another holding the older card.

## glossary

- **`/post time`**: a post's `when` — the photograph's own moment if it had
  one, and otherwise the moment the post was made.

## code description

`post-time.rs` — `update` /extension/ takes `CardWhen {id, when, source, t}`:
one post's time, read and written through `/cards`' `cards_read` /
`cards_write` so the var's address stays in one place and `cards.rs` is never
edited. A zero time, an empty id, or a card that is not a post is ignored;
`edited` is stamped so `/guard`'s merge keeps the change.

`post_time_of(card)` is the fallback written once — `when` if it is there,
else `created`. `posts_set` /extension/ resorts `/posts`' set by it, id as the
tie-break. `browse_when_of` /extension/ answers it for a post and delegates
for every other type.

`browse.rs` gained `browse_when_of(card)`, the seam for which of a card's
times its row shows, defaulting to `edited`; `browse.rs`' own list row and
`/portrait`'s row both call it instead of reading `edited` at the call site.
Behaviour with nothing redefining it is identical.

`post-time.js` — `taken(file)` is the EXIF date read: the first 256KB, the
JPEG segment walk to the first EXIF APP1 (`parse`), then `date` — TIFF header,
IFD0, the Exif sub-IFD pointer `0x8769`, `DateTimeOriginal` `0x9003`, falling
back to IFD0's `DateTime` `0x0132`. `each` is the bounds-checked IFD walk that
both readers go through; `find` reads a LONG, `ascii` a string, `ms` turns
`YYYY:MM:DD HH:MM:SS` into local epoch milliseconds. The walk is written here
rather than borrowed from `/from-picture`, which exposes its IFD readers but
not the TIFF offset they need — and a post's time must still be read with
`/location` unticked.

`isPost(id)` reads the bridged `s.cards` for the card's type, and falls back
to the open page's `post` class if the world has not caught up.

The two links: `feature_Cards.chose` redefined at load with the previous one
in a closure — it reads the original file and hands it on untouched — and
`feature_Loop.send` redefined the same way, sending `CardWhen` on the back of
the matching `CardPic`. Both are load-time redefinitions of named functions,
never a timer wrapping `apply`.
