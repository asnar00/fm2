# photo

*a picture makes a post, and the caret is already in the words beneath it*

> (transcripts/2026-09-01-saturday.md#p2)
> clean up the "make posts" interface to include video, audio, photo+type,
> transcription

## user

The camera in the posts row takes or picks a photo and makes a post out of
it, with the caret already in the words so you can type what was said while
the picture is still on the screen.

## spec

A post could always carry a picture — but only by making an empty post first,
scrolling to its picture block and tapping that. Four acts at a door. This is
one: tap the camera, take the photo, type. It is the "photo+type" of the ask
(`#p2`).

The photo goes in through exactly the doors that already exist. `/cards`'
`shrink` is what makes a phone photograph small enough to keep — 384px, JPEG,
quality stepping down until it fits — and its `held` is the budget check; both
are read off `feature_Cards` at use, so `/roomier` and `/wider`'s numbers are
the ones that apply. Then `/new`'s `CardNew` and `/cards`' `CardPic`, in that
order, in one turn: `CardNew` mints the card with `<owner>.<t>` as its id, so
the picture's op can name the card before the world has answered.

**The wire.** Nothing new travels. The picture is a data URL in block 1 of
the card, and the cards list is one `/msg` op — 24KB per picture and 160KB
for the whole list, `/wider`'s numbers, under its 192KB body cap. Over
either, `/cards`' own words say so out loud and no post is made: the failure
is one toast and nothing written, not a post with a hole in it.

The file chooser carries no `capture` attribute, so the phone offers its own
menu — take a photo, or pick one from the library — which is the platform
default and the whole of what was asked for. A gallery of our own is parked.

Untick and the camera leaves the row; every other way of putting a picture on
a post is untouched.

## glossary

(no new terms)

## code description

`photo.rs` redefines `/capture`'s `capture_extra` to append the camera
button, and draws the glyph: a body, a lens and the little hump of the
viewfinder, in `currentColor` (`/glyphs` — never an emoji presentation).

`photo.js` owns a hidden file input, made at load and living outside `#app`
so a repaint cannot take it away (`/cards`' pattern). The tap is taken in the
CAPTURE phase and stopped there, so `/loop`'s delegated click never sends it
on — the same reason `/posts` takes the plus that way: the owner's name is
behind the cookie, so the post has to be made here.

`make(file)` shrinks, checks the budget, arms `/editing`'s `openNext`, sends
`CardNew` then `CardPic`, and puts the caret in the words through `/posts`'
own `caret` and `settle` — so a photo post lands exactly where a typed one
does. `openNext` is the flag `/editing` arms from a click on the buttons that
make a card; it lists those buttons by event, and the camera is not one it
knows, so the half that makes the card is the half that says so. Without it
the post would open locked and the words would be a pencil-tap away, which is
not "photo+type". Every cross-feature reference is typeof-guarded; with
`/posts`, `/cards` or `/editing` absent it does nothing rather than throwing.
