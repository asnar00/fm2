# as-posts
*a recording is a post; the dictaphone retires as a tool*

> (transcripts/2026-08-26-session.md#p155, the origin)
> let's upgrade the "dictaphone recordings" to full cards, so they get the same grid/list/map, display format (but extended to handle audio+transcript).

> (transcripts/2026-08-26-session.md#p156, the ask as it was made)
> *actually* - I'd like to integrate the dictaphone's functionality into the "add post" toolbar, and then retire the dictaphone as a separate tool. The two recordings would become older posts visible in the posts view, and there'd be a "record" button added to the toolbar in the add-post tool's sub-palette.

## user

Open the posts tool. Beside **+** there is a **record** button. Tap it and it
becomes a stop button with a pulsing dot; talk; tap stop. A new post appears
at the top of your posts, and its words fill themselves in with what you said
as the phone finishes listening to it.

Open that post: a play control with the length of the recording, the words,
the place — a post like any other. Tap play and it plays; tap it again and it
stops. Give it a title, edit the words, add a picture, delete it: all the
things a post does.

The notes you recorded weeks ago are there too, as older posts, dated when
you recorded them. The 🎤 tool is gone; recording lives in posts now.

A recording someone handed you plays only for them: the words travel, the
audio stays with its owner.

## spec

A recording becomes a `/post` — a `/card` of type `post` — so it gets the
grid, the list, the map, the page, the title, the delete, the undo and the
copy that every post already has (#p155). The recording machinery is
`/dictate`'s, unchanged: this node moves its two controls onto the posts
toolbar, turns each recording into a card, and takes 🎤 out of the toolbar
(#p156). Nothing under `/dictate` is edited.

**The audio block.** A post made from a recording carries a fourth block,
`{kind:"audio", id:"rec-<t>", dur, mime}`, after the text block — index 3, so
title, picture and text keep the indices `/keep` and `/frame` send. `/cards`'
page renderer draws an unknown block kind as nothing, so the block is inert
everywhere except here.

**The key is on the card, not in the block.** The card also carries a
top-level `rec` — the same recording id — and that is what "does this
recording have a post yet" asks. `/delete`'s tombstone empties `blocks` but
clones the rest of the card, so `rec` survives a delete and a deleted
recording is never resurrected. The audio block is what the page draws from;
`rec` is what the bookkeeping matches on.

**The times.** `created` = the recording's `t`, so the id `/cards` mints is
`<owner>.<t>` — the same id on every device of the same person, which is what
lets two devices agree without a protocol: `/guard` merges by id. `when` =
`t` as well (`when_from: "recording"`), so `/post-time` orders the old notes
among the posts of the days they were made rather than at the top of today.

**The record control.** `tool_controls`, outermost, while the posts tool is
open and no card is open — the same rule `/plus-at-home` made for **+**:
these controls belong to the set of posts, not to a post you are reading.
Inserted through `/posts`' own `posts_before_undo`, so undo stays last
(`/glyphs`), wearing the posts tool's colour like **+** beside it. The glyphs
are drawn: a filled dot for record, a rounded square for stop. The events are
`/dictate`'s own — `dict_rec` and `dict_stop` — so its `update` and its page
half do the recording with nothing added and nothing wrapped, and the pulsing
`.rec-dot` is its own too.

**A recording becomes a post.** One pass over `dict_files` after every event,
extending `update` outermost: a file with no card carrying its `rec` gets one,
built with `/cards`' own `card_new` and written with `cards_write` — the pair
`/kinds/new` uses — so `/guard`, `/exchange` and `/converge` see one ordinary
card write. It is a pass rather than a handler on `RecSaved` because the same
answer is owed to four different arrivals: a recording made here, a recording
`/mirror` announces from the same person's other device (`RecShared`), the
index that comes back at boot (`RecIndexed`), and the notes already in
IndexedDB when this node ships (`RecList`). One rule, asked at every turn,
gets all four right and cannot be raced.

**The owner comes from your own card.** The logged-in name is behind the
cookie and not in the world, and `update` runs in the page's wasm where there
is no cookie to read. So the name is taken from the profile card you already
hold — `card_of_type(cards_read(), "", "profile")`, `/me`'s own lookup, which
`/exchange` narrowed to cards of your own. **Until there is one, no post is
made**: a card minted under the wrong owner could not be handed on
(`/exchange` checks that an id was minted by its owner) and could not be
corrected afterwards (`/guard/owner` refuses a write that changes an owner).
Waiting costs nothing, because the pass runs again on the next turn.

**The transcript lands in the words.** When a file's `transcript` appears or
improves, the post's text block takes it — but only while the words are still
the transcript's. `/keep` writes a block's `text` and nothing else, so the
test cannot be a flag that an edit clears: the text block carries `auto`, a
hash of the words this node last wrote there, and the words are replaced only
when they are empty or still hash to `auto`. One keystroke of the user's and
the hash stops matching, for good. **No edit of the user's own words, ever.**
`edited` is stepped by one millisecond rather than stamped with a clock —
there is no clock inside `update`, and `/guard` needs only that the change
looks newer than what it replaces.

**The page.** `card_page_html`, outermost: a play row before the words —
the drawn ▶ in the posts colour, the duration as m:ss, dim. Its `data-ev` is
`/dictate`'s own `dict_play_<id>`, so tapping it plays and tapping it again
stops, through the page half that already fetches the blob (and, with
`/mirror`, fetches it from the exchange the first time). Which recording is
playing lives in the loop state, not on the card, so the glyph is swapped by
`render` — one class on the row, both glyphs drawn, CSS choosing — and the
same link turns the empty words' placeholder into *transcribing…* while
`dict_transcribe` is aimed at this recording.

**The tile.** A small ▶ over the face of a tile whose post has audio, so a
recording is recognisable in the grid without opening it. The list row is not
marked — `/portrait`'s cells are the author, the words and the date, and a
fourth mark in a row is more than the ask asked for.

**A copy plays for nobody.** `/exchange` copies the whole card, so the audio
block and the words travel; the blob does not, because `/mirror`'s
`blob/<id>` route is per-user. A foreign post's play row is drawn dim, with no
`data-ev` and the words *audio stays with its owner* — never a control that
pretends (`/taste` 7).

**The tool retires.** `tools_list`, outermost, drops the `dictate` entry —
`/people`'s idiom. The launcher loses 🎤, `tools_catalog` (written at `init`
from the same chain) loses it, so the chooser's catalog and `/long-press`'
sub-tool cards lose it too. `/dictate`'s `render` and `/transcript`'s panel
only draw while `dictate` is the open tool, which can no longer happen, so
they go quiet without being touched; its `update` and its page half keep
working, which is the whole point.

## anticipation

Shapes reserved, not built (`/anticipation`). **A blob a copy can play**: the
audio block carries the owner and the id, so the fetch a holder of the card
would make is `blob/<owner>/<id>` — a route beside `/mirror`'s, not a change
to the card. **A second kind of blob on a card** (a video, a file) is another
block kind with the same three fields and the same store. **Trimming a
recording** is a new block, not an edit of this one: a recording is immutable
(`/dictate`'s own word).

Named and parked: deleting a recording's blob when its post is deleted — the
tombstone keeps `rec`, so the blob is findable and nothing has been leaked,
but nothing removes it yet; transcription rungs beyond `/phone`'s; and the
`cards` list's 14KB budget, which transcripts now spend (`/cards`' own known
limit, and the same var-per-card rung answers it).

## hostile cases

- **No profile card yet.** No post is made, and no card is minted under a
  guessed name. The pass runs again next turn; the recording is not lost —
  it is in IndexedDB and in `dict_files`.
- **The same recording on two devices.** Both mint `<owner>.<t>`, so it is one
  id and `/guard` merges them; the newer `edited` wins, which is the device
  that has the transcript.
- **The post was deleted.** The tombstone keeps `rec` and has no text block,
  so nothing is recreated and no transcript lands.
- **The user wrote their own words first.** `auto` does not match, so the
  transcript never touches them — the recording still plays, and its words
  are the user's.
- **A transcript that improves.** The words still hash to `auto`, so the
  better rung replaces them.
- **A foreign post with audio.** A dim row saying so, and the words. Nothing
  fetches, because there is no `data-ev` to fire.
- **`/mirror` unticked.** Recordings are local only; the pass is the same and
  a copy's dim row is still honest.
- **`/phone` unticked.** No rung is reachable, `dict_transcribe` is never set,
  the words stay empty and the post is a recording with a place and a title.
- **`/dictate` unticked.** The linker refuses the composition — this node
  calls `dict_files`' events and `/dictate`'s glyphless state by name. The
  same hard dependency `/posts` has on `/guard`, and it names both nodes.
- **`/posts` unticked.** Likewise: `posts_before_undo` and `posts_is` are its
  chain.
- **This node unticked.** 🎤 comes back with its grid, the record buttons
  return to its own toolbar, and the posts already made stay — they are
  cards, and nothing here owns them.

## glossary

- **audio block**: a card block `{kind:"audio", id, dur, mime}` — a reference
  to a recording in the device's blob store, never the bytes.
- **`rec`**: a card's recording id, the key that says which recording this
  card is the post of.

## code description

`as-posts.rs` — `tools_list` /extension/ filters the `dictate` entry out of
the registry chain, which is also what empties it from `tools_catalog`.

`tool_controls` /extension/ adds the record control while the posts tool is
open with no card open, through `/posts`' `posts_before_undo`;
`as_posts_rec_button` draws it — `dict_rec` with the dot glyph, or `dict_stop`
with the square glyph, the `.recording` class and `/dictate`'s `.rec-dot`.

`update` /extension/ runs `as_posts_sync` after the chain beneath: for every
`dict_files` entry, find the card whose `rec` matches (`as_posts_card_for`),
make one if there is none (`as_posts_card`), and land the transcript if it is
still ours to write (`as_posts_land`). One `cards_write` at the end, and only
if something changed. `as_posts_owner` is the name off your own profile card;
an empty answer stops the pass.

`as_posts_hash` is the fingerprint stored as a text block's `auto` — a
64-bit FNV-1a, never zero, so an absent `auto` can never match written words.

`card_page_html` /extension/ inserts the play row before the first
`.card-text` when the card has an audio block: `as_posts_play_row` for a card
of your own, and the dim, eventless form with *audio stays with its owner*
for a copy. `as_posts_audio` is the block lookup, `as_posts_mmss` the
duration.

`render` /extension/ does the two things that need the loop state: it marks
the play row `playing` when `dict_playing` names its recording (the glyphs
are both drawn and CSS picks), and it turns the words' placeholder into
*transcribing…* while `dict_transcribe` is aimed at the open card's
recording.

`card_tile_html` /extension/ puts a small ▶ over the face of an audio post's
tile.

`as-posts.css` styles the play row (a pill on the page ground, the posts
colour on the glyph, the duration dim), the dim foreign form, the tile mark,
and the black-on-tint rule the two toolbar glyphs need.
