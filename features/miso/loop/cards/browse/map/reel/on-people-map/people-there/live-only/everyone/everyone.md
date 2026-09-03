# everyone
*the people reel is everyone the map shows, live or not*

> (asks#1788464261004)
> reel in users map view should show all visible users

## user

On the people map the band along the bottom lists everyone whose pin is on
the map: the people sending their position now, and the people placed by
their card. Flick along it and the map follows, as before.

## spec

`/people-there` made the band the map's set; `/live-only` narrowed it to
the people sending positions (asks#1788450507238). Tara's phone asked for
all visible users (the ask): the pins are the band, whichever way a person
got there.

**The union, live first.** `feature_Reel.posts` is wrapped once more on the
people map: the rows `/live-only` keeps (`liveRows`, a person's live place
and beat) and the map's set as `/people-there` read it (`#mapData`'s ids
against the held cards, a card's own location block), joined by card id —
a person who is both live and placed appears once, at their live place and
time. Newest first, as both parents sorted. Nobody live and nobody placed:
no band, as before.

## hostile cases

- **A live person with no card.** `/live-only`'s row stands under its
  `live:<name>` id.
- **`/live` unticked.** No live rows; the band is the map's set alone.
- **This node unticked.** Live only, as `/live-only` drew it.

## code description

`everyone.js` — wraps `feature_Reel.posts` on the people map with the
union of `/live-only`'s rows and `/people-there`'s reading of the map's set.
