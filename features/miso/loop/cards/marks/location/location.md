# location
*where the card was made, kept on the card and shown behind one quiet pill*

> (asks#1787667818434)
> cards should have a GPS coordinate field showing where we were when the post was made (or the GPS tag of the picture) - show as a "map location" button we can tap (for now pop up gps location view, placeholder)
> *(filed from the field on 2026-08-25 by ash, birthplace `👤 @ miso/shell/panel/account`)*

## user

A card remembers where you were when you made it. Near the bottom of the card
there is a small **map location** pill: tap it and the place comes up — the
coordinates, how accurate they are, and when they were taken. If the card has
no place yet the pill is dim; tapping it asks your phone where you are and
keeps the answer. Saying no to the phone's question costs nothing — the pill
stays dim and you can tap it again whenever you like.

## spec

Posts in this app will be geotagged — that is the campaign's substrate — and
today a card has no place at all. Ash asked for the coordinate on the card and
a "map location" button to see it, with a placeholder view for now. That is
what this node is: **the datum, and one way to look at it**.

**The datum is a block.** A card's place is `{kind: "location", lat, lon, acc,
t, source: "device"}` in the card's own `blocks` list — not a new var, not a
field beside them. A card carries at most one: a second fix replaces the first
rather than piling up a history, because "where the card was made" is one fact.
`/cards`' page renderer draws nothing for a block kind it does not know, so the
block is invisible to everything except this node, and `keep`'s block indices
are undisturbed because the block is appended after the ones already there.

**The event is `CardPlace {id, lat, lon, acc, t}`**, added to `update` here and
read and written through `cards_read` / `cards_write` — `/cards`' own pair, so
the var's address stays in one place and `cards.rs` is untouched. A latitude
outside ±90, a longitude outside ±180, or either one missing is dropped on the
floor: a place that is not a place never enters the world.

**It costs the wire almost nothing, and the wire is the thing to watch.** The
whole cards list travels as ONE op through `/msg` (misses.md, "the picture
cap"): a location block is ~110 bytes of JSON against `/roomier`'s 64KB body,
so it is noise beside a picture — but it is charged against the same budget
`cards.js` counts in `held()`, and if the list ceiling is ever reached again
this block is part of why.

**When the fix is taken.** The page half watches `#app` with a MutationObserver
— `/invite`'s `look()` idiom — and when a card page appears whose pill is dim,
it asks `navigator.geolocation.getCurrentPosition` once, high accuracy off,
10s timeout, and sends `CardPlace`. Once per card per page load, so a repaint
(and there are many: every keystroke's save repaints) cannot turn into a
stream of position requests. **It is not an `apply` wrapper**: wrapping
`feature_Loop.apply` from a timer is the race that orphaned `/account`'s watch
(notes.md, "the apply-wrapper race"), and this node reads the DOM instead.

**Refusal is free.** Permission denied, position unavailable, timed out, or no
`navigator.geolocation` at all: nothing is stored, nothing is said, no error
reaches the screen. The pill stays dim and a tap on it asks again — the one
place where the user's own action is worth a second prompt.

**The pill.** `card_page_html` is extended to insert a `.card-place` span just
before the page's closing tag, so it lands after the mission and before
anything `me_under` puts under the card. Its words are `map location` and
nothing else — the state is carried by dimness, not by a second sentence
(`/taste` 7). It carries no `data-ev`, so the loop's delegated click never
fires for it and tapping it cannot repaint `#app` mid-tap.

**The view is explicitly a placeholder for a map.** It is a sheet on the dark
ground, furniture outside `#app` like `#cardToast` and `#frameSheet`, showing
the coordinates, the accuracy, when the fix was taken in plain words, and
**close**. It does not say "placeholder" on the surface — that sentence belongs
here, not in front of the user. A real map, with tiles, is the later rung.

**Parked, named, not built:** the ask's parenthetical second source — *the GPS
tag of the picture* — needs EXIF parsing of a file that is downscaled through a
canvas (which strips EXIF) before it is ever stored, so it is a node of its own
and not a line of this one. Also parked: map tiles, reverse geocoding to a
place name, editing or removing a location by hand, and a location on the tile
rendering.

## hostile cases

- **Permission denied.** No block, no toast, no console error; the pill stays
  dim and a tap asks again.
- **No `navigator.geolocation`.** Same, and nothing throws: the API is tested
  before it is called.
- **A location block full of garbage** — a string latitude, a null longitude, a
  latitude of 999 — reads as no location: the pill is dim and the page renders
  exactly as it would have.
- **A repaint storm.** The observer fires on every paint; the ask is guarded per
  card per page load, so one position is taken however many times the page is
  redrawn.
- **A second `CardEnsure`, or a reload.** The card already carries its block, so
  the pill is not dim and no position is asked for.
- **`CardPlace` for a card nobody holds.** No card matches the id, nothing is
  written, the list is not touched.

## glossary

- **location block**: the `{kind: "location", …}` entry in a card's blocks that
  says where the card was made.
- **map location**: the pill on the card that opens the place.

## code description

`location.rs` extends `update` with `CardPlace {id, lat, lon, acc, t}`: it
reads the list with `cards_read`, finds the card by id, replaces its location
block or appends one, stamps `edited`, and writes back with `cards_write`.
Coordinates out of range or absent make the whole event a no-op.

`location.rs` extends `card_page_html`: `card_place_of(card)` returns the
card's first sound location block (or null, which is what a garbage one also
returns), and the pill is spliced in before the page's last `</div>` — inside
the card's own scrolling box, after the blocks, before `me_under`'s fillers.
The block's numbers ride out as `data-lat` / `data-lon` / `data-acc` /
`data-t` so the page half needs no second read of the store.

`location.js` is the page half. `look()` runs from a MutationObserver on
`#app` and calls `ask()` when it sees a dim pill; `ask(id, again)` is the
single geolocation call, guarded once per card per load and forced by a tap;
`show(pill)` fills the sheet from the pill's data attributes and reveals it;
`since(t)` turns a timestamp into "just now" / "3 min ago" / "2 hours ago".
The sheet and its close button are made at load and appended to `document.body`
so a repaint cannot take them away while they are open.

`location.css` styles the pill as the quiet pill family (`#1a1a1d` on a 1px
`#3a3a3f` border, 999px, `#c9c9d2`; `#77777e` when dim) and the sheet as the
`#frameSheet` family — the dark ground, a 14px card, the coordinates in the
brightest step and everything else dim beneath them.
