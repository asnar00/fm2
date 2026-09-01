# poster

*a video post gets a face: a frame off the clip, stored as the card's picture,
shown on the post with a play mark over it*

> (transcripts/2026-09-01-saturday.md#p18)
> video posts should choose a poster frame (random is fine) and use that as
> thumbnail, plus show that in the post view with a play button over it.

## user

Record a video and the post it makes has a face. In the grid it is a frame
from the clip rather than a letter; on the map its pin carries the same frame.
Open the post and the frame fills the page with a play mark over it — one tap
and the clip plays where the frame was.

Which frame is luck: somewhere in the middle of the clip. The first moments of
a recording are a lens waking up, so the pick avoids both ends.

A video post made before this has no face and opens as it always did: the
player, ready to play. Nothing is regenerated.

A video someone hands you carries its face — the poster travels on the card —
but not its bytes, so the post shows the frame and says the video stays with
its owner, which is what a shared recording already says.

## spec

The face is the card's **picture block**. Every card is minted with one
(`/cards`), `/as-posts` leaves it empty on a post made by recording, and this
node fills it. That one decision is what makes the tile, the map pin and
`/picture-first` need no changes at all: each reads the first picture block
and now finds one.

The block is marked `poster: true`. The mark is the difference between a face
the app took and a picture the user chose, and two things turn on it: the page
merges a poster with its player and never merges a chosen picture, and
`one_medium_carried` — `/one-medium`'s named /extensible function/ — answers
`"video"` for a card carrying both, because the medium is the clip and the
picture is its face. A picture already in the slot is never overwritten: the
user's own outranks the app's.

The poster is **taken once, on the device that recorded**, and reaches the card
through an op that names the recording rather than the card. `/as-posts` mints
the card id out of the owner and the recording's moment, and the page half
would have to guess both; `rec` is already on the card and survives a delete
(`/delete`'s tombstone clones it), so a poster for a deleted video lands
nowhere. A device that mints the same post from an announcement alone
(`/mirror`) shows no face until the card itself arrives.

The poster rides the cards list on the wire, so it is charged the same budget a
picture is: `/cards`' `CAP` for one data URL and `LIST_CAP` for the whole list,
as `/roomier` raised them. It is a thumbnail, not a photograph — `/cards`' own
`EDGE` and its own quality ladder, stepping down until it fits. A three-second
clip's face measures a few thousand bytes.

**Nothing is ever said out loud when this fails.** A canvas that will not
decode, a seek that never lands, a clip whose bytes are not there, a list with
no room: each ends in a post with no face, which is a post. The clip is written
before any of this runs, so a failure cannot cost a recording. A clip shorter
than the moment picked clamps to just inside its end.

**Playback.** The tap turns the poster into `/capture/video`'s own player, in
place: the holder puts on that node's class and its mount finds it on the next
pass, with `playing` already set so the clip starts on one tap rather than two.
Which posts are open is remembered in the page half, exactly as `/capture/video`
remembers where a clip had got to — so a repaint mid-play does not slide the
poster back over a playing video. A post you have played once stays a player
for the rest of the session; the frame is what it opens with the first time.

**Placement.** A subfeature of `/capture/video`: it is a refinement of what a
video post is, and both halves of it — taking the frame at the end of a
recording, drawing the post — are that node's own ground. `/capture/video` had
no children; this is its first.

## glossary

- **poster**: the still frame that stands for a video post — stored as the
  card's picture block with `poster: true`, drawn as the tile face and as the
  post's own face.

## code description

`poster.rs` extends `card_page_html`. Composed last of that chain, it receives
the finished page — `/cards`' filled picture and `/capture/video`'s player row,
two media presences for one medium — cuts both, and puts one back where the
picture was, so `/picture-first` and `/titled/above` order the poster exactly as
they ordered the picture.

`poster_block(card)` is the test everything turns on: the first picture block
with data and the `poster` mark, or null.

`one_medium_carried(card)` answers `"video"` for a card holding a poster and a
video, and defers to the chain beneath for everything else.

`poster_cut(html, mark)` takes one element out of the drawn page: find the
opening tag, find the `</div>` that closes it, splice the rest. Neither element
holds a nested div and every stored string is escaped on the way in
(`card_esc`), so the first `</div>` is the right one. A mark that is not there
leaves the page alone, which is how the own and foreign rows share the cut.

`poster_row` draws the frame with `poster_play_svg` over it, carrying `data-vid`
for the page half and `data-rec` for `/as-posts`' "transcribing…" hint — the two
handles the player row carried, so nothing downstream notices the swap.
`poster_foreign` draws the frame with the note instead of the glyph.

`update` answers one event, `CardPoster {rec, data, t}`: find the card whose
`rec` matches, write the first *empty* picture block's data and mark, stamp
`edited`.

`poster.js` wraps `feature_Video.save` — after the clip is stored and the
recording announced, which is what mints the card. `make(id, est)` reads the
clip back out of IndexedDB, takes a frame, checks `/cards`' budget and sends the
op.

`grab(blob, est)` is the frame: a detached `<video>` on a blob URL, seeked and
drawn. A MediaRecorder webm reports `Infinity` for its duration until it has
been seeked once, so a seek past the end comes first and the recording's own
measured length is the fallback. Every wait is bounded — a decode that never
finishes leaves the post alone rather than hanging inside `save`.

`pick(dur)` is the choice of moment, and the /extension point/ the next ask
grows from: a random point in the middle half of the clip today, a chooser's
answer tomorrow, and nothing else changes.

`draw(v)` puts the frame on a canvas through `frame(w, h)` — `/cards`'
`frameOf` when it is there, the whole frame at `EDGE` otherwise — and steps the
JPEG quality down until the data URL is inside the budget. Going through
`frameOf` is what makes a poster obey `/square-crop` with nothing of its own.

`open(h)` swaps a poster holder into `/capture/video`'s player and starts it
inside the tap, because a browser gives sound to a `play()` still in the
gesture and refuses one that is not. `warm(id)` is what makes that possible: it
makes the clip's blob URL in `/capture/video`'s own caches while the poster is
still showing, so the mount is synchronous when the tap comes. `restore()`
re-opens the posts that were open, and warms the ones that are not, after every
render.

`poster.css` gives the frame the picture's width and ground, and the play mark
a thin ring in `currentColor` with the only ground it lays over the picture
inside the ring itself.
