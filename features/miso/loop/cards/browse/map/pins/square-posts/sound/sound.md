# sound
*an audio-only post pins with a sound glyph, not the author's initial*

> (asks#1788463866276)
> for audio only posts, show a "sound" icon rather than the author initial
> in map

## user

On the map, a post that is only a recording — no picture, no clip — wears a
small speaker on its pin instead of the letter of whoever made it.

## spec

`/map` gives a pin a face when the card has one and the owner's initial
otherwise; a post that is a recording alone has neither and showed a
letter that said nothing about it (the ask).

**The row says so.** `map_surface_html` is extended after `/square-posts`:
its rows, matched to cards by id as that node does, gain `sound: true`
when the card has an `audio` block and no picture with data and no video
— `sound_only(card)`. **The page half draws it.** `feature_Map.pinHtml` is
wrapped once more (the idiom `/square-posts` uses): a row with `sound`
gets its initial replaced by a drawn speaker (`/glyphs`: ink, currentColor)
inside the same face.

## hostile cases

- **A recording with a poster** (a video post). Has a video block; not
  sound-only; the poster face as before.
- **A post with a picture and a recording.** The picture face wins.
- **A live pin.** `/live` writes its own markup; untouched.
- **This node unticked.** The initial, as before.

## code description

`sound.rs` — `map_surface_html` marks sound-only rows; `sound_only(card)`
is the test. `sound.js` — the speaker in the face. `sound.css` — its size.
