# from-picture
*the card's place comes from the photograph's own GPS tag, when it has one*

> (asks#1787667818434)
> cards should have a GPS coordinate field showing where we were when the post was made (or the GPS tag of the picture) - show as a "map location" button we can tap (for now pop up gps location view, placeholder)
> *(filed from the field on 2026-08-25 by ash, birthplace `👤 @ miso/shell/panel/account`)*

> `/location` built the first half of this ask — where **you** were — and
> named the parenthetical second source as parked. This node is that half:
> the ask's own words, finished. (Residuals are fixed in the run, not left
> for signature — the accounts transcript, prompt 50.)

## user

Put a photo on your card and the card takes its place from the photograph.
Most phone photos remember where they were taken; when yours does, **map
location** shows *that* spot — where the picture was made — rather than where
you happen to be standing now, and the place says **from the picture** when
you open it. A photo with no tag changes nothing: the place stays the one
your phone gave, and it says **from this phone**.

## spec

Ash asked for "the GPS coordinate field showing where we were when the post
was made (or the GPS tag of the picture)". A post is its picture, and a
picture usually knows where it was taken better than the phone holding it an
hour later. So a tag out of the photo is not a second opinion, it is the
better one: **it replaces the device fix**, and the fix can never come back
over it.

**The tag is read from the ORIGINAL file, at the moment it is chosen.** The
seam is `/cards`' `chose(file)`, taken by redefinition and kept in a closure
— the same seam `/frame` takes, and this node's link sits *outside* `/frame`'s
because provenance puts this node later. So the file read here is the one the
user picked, before `/frame`'s canvas throws the EXIF away (a canvas keeps
pixels and nothing else). It reads, then hands the file on untouched. Nothing
here knows anything about `/frame`: with `/frame` unticked the kept `chose` is
`/cards`' own and the read is identical.

**The place is sent when the picture lands, not when the file is picked.** A
framing that is cancelled, or a photo refused for being too big, must leave
the card exactly as it was — a card claiming a place with no picture on it is
a lie. `/cards` sends `CardPic` itself at the end of both paths, framed and
unframed, so the tag waits in `pending` and travels on the back of that event:
`feature_Loop.send` is taken by redefinition at load, the `CardPic` goes
through untouched, and the `CardPlace` follows it. This is a load-time
redefinition of a named function, not a timer wrapping `apply` — the race that
idiom causes (notes.md, "the apply-wrapper race") needs a timer, and there is
none here.

**The event is the one `/location` already has**, `CardPlace {id, lat, lon,
acc, t}`, with `source: "picture"` added and `acc: 0` — EXIF records a place,
never how sure of it the camera was.

**Precedence needs the server, which is why this node carries Rust.**
`/location`'s `update` stamps every place `"device"`, because when it was
written that was the only source there was, and the `source` field is the
datum the sheet has to read. Rather than edit `location.rs`, this node's
`update` link sits outside it: it notes the block the card held *before* the
chain below runs, lets that chain do its write, and then has the last word.
A place the event said came from a picture is re-stamped `"picture"`; a device
fix that has just displaced a picture's tag is undone, the picture's own block
put straight back — coordinates and all, not just the label. That second case
is the real collision, because `/location` asks the phone once per card per
page load on a ten second timeout, so a fix asked for before the photo was
chosen can easily answer after it. The other direction needs no work at all:
`/location` only asks when the pill is dim, and a picture-sourced card's pill
is lit, so a reload never re-takes a fix over one.

**The sheet says where the place came from.** `card_page_html` is extended
once more to splice `data-source` into the pill `/location` drew, and
`feature_Location.show` is redefined to write one dim line under the accuracy
— *from the picture* / *from this phone*. Three words, no sentence: `/taste` 7
is a close call here, and the line earns its place because it is a fact about
the datum rather than a caption explaining a control, and because the ask
named two sources the user has to be able to tell apart. `location.js` and
`location.rs` are both untouched.

**The parser is ours, about a hundred lines, no dependency.** JPEG only: SOI,
then the segment walk to the first APP1 whose data begins `Exif\0\0`, then the
TIFF header with its byte order honoured, IFD0, the GPS IFD at tag `0x8825`,
and `GPSLatitudeRef`/`GPSLatitude` (`0x0001`/`0x0002`) and
`GPSLongitudeRef`/`GPSLongitude` (`0x0003`/`0x0004`) as three rationals →
degrees + minutes/60 + seconds/3600, negated for S and W. Only the first 256KB
of the file is read: EXIF is at the front, an APP1 cannot exceed 64KB, and a
photo is megabytes. Every offset is checked against the end of what was read
before it is followed, every entry count is capped, and the whole walk is
inside one `try` — **a malformed file yields "no tag", never a throw.**

**HEIC is not parsed, and that it does not matter is a hypothesis, not a
finding.** iOS file inputs are understood to hand over a JPEG conversion of a
HEIC library photo, which would mean iPhone photos arrive parseable. That is
untested on a device and is written here as the expectation. If a HEIC ever
does arrive it reads as no tag and the device fix stands, which is exactly the
old behaviour.

**Parked, named, not built:** HEIC parsing, a real map under the coordinates,
and editing or removing a place by hand.

## hostile cases

- **A photo with no GPS tag** (a screenshot, a download, a phone with location
  off): no tag, no `CardPlace`, the device fix stands untouched.
- **A malformed JPEG** — a truncated APP1, a length running off the end, an
  IFD offset pointing into space, a rational with a zero denominator: every
  one returns null. Nothing is sent, nothing is stored, the page is not
  disturbed.
- **A tag of exactly 0, 0.** Cameras write zeros for "I had no fix"; the Gulf
  of Guinea loses, and it reads as no tag.
- **Coordinates out of range.** Refused here, and refused again by
  `/location`'s own `card_place_sound`.
- **The framing cancelled, or "that picture is too big to keep".** No
  `CardPic`, so no `CardPlace`: the card is exactly as it was.
- **A device fix answering after the picture's tag landed.** Undone by the
  `update` link — the picture's own block is put back.
- **A second photo chosen.** `pending` is cleared at every `chose`, so the
  place that lands belongs to the picture that landed.
- **A `CardPlace` for a card that carries no place.** `card_pic_place_put`
  only replaces an existing location block; this node never creates one.
- **`/location` unticked.** This node is its child and goes with it: no place,
  no pill, no sheet to speak of.
- **`/cards` or `/loop` somehow absent.** Both seams are `typeof`-guarded; the
  fragment does nothing and throws nothing.

## glossary

- **the picture's tag**: the EXIF GPS coordinate a camera writes into a
  photograph, saying where the photograph was taken.
- **source**: which of the two a card's place came from — `"picture"` or
  `"device"`.

## code description

`from-picture.js` is the whole of the reading. `tag(file)` slices the first
256KB and `parse(view)` walks the JPEG's segments to the first EXIF APP1,
handing off to `gps(...)`, which reads the TIFF header, follows IFD0 to the
GPS IFD and returns `{lat, lon}` or null. `each(...)` is the one IFD walk that
both `find(...)` (a LONG-valued tag) and `deg(...)` (a ref plus three
rationals) are built from, and it is where the bounds checks live; `rat(...)`
is one rational, null on a zero denominator.

`from-picture.js` takes three seams by redefinition, each keeping what it
replaced in a closure. `feature_Cards.chose` reads the original file and holds
the result in `pending`, awaiting the read before handing the file on so the
tag can never lose a race with the picture. `feature_Loop.send` follows a
matching `CardPic` with the `CardPlace`. `feature_Location.show` calls
`said(pill)` after the sheet is filled, which writes the source line;
`#placeSource` itself is made at load and inserted into `/location`'s
`#placeBox` above its close button.

`from-picture.rs` extends `update`: for a sound `CardPlace` it captures the
card's current location block, runs the chain, and then stamps the new block
`"picture"` with its accuracy dropped if the event said so, or restores the
captured block if a device fix has just displaced a picture's one.
`card_pic_place_of` and `card_pic_place_put` are the read and the write it
does that with, both through `/cards`' own `cards_read` / `cards_write`.

`from-picture.rs` extends `card_page_html` to splice `data-source` into the
lit pill's opening tag, taken from the card's own location block. A dim pill
has no place and so has no source.

`from-picture.css` styles `#placeSource` as the quietest step of the sheet
(`#5c5c63`, 12px), hidden entirely while it is empty.
