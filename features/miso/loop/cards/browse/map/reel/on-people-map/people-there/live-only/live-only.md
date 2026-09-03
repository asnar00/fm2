# live-only
*the people reel holds only those sending their position right now*

> (asks#1788450507238)
> users reel should only show users who are sending live positions right now
> *(filed from the field on 2026-09-03 by ash)*

## user

Under 👤's map the band holds the people who are live right now — the ones with a moving pin — and nobody else. As people arrive and leave, the band follows; as they move, the map follows the one you are on.

## spec

`/people-there` listed the map's whole set. Ash's ruling: only those sending live positions (asks#1788450507238). One reading, so it builds: `/live` polls the server every second and draws its rows — name, card id, face, place, time; this node keeps the last rows it drew, and on the people map the band is those rows, each lozenge the person's face and name (their card's line when they have a card), the time of their last beat, and their live place. When the set of live people changes the band is drawn afresh; when only places move, the lozenges' places are updated in step and the current one's pin re-marked, so a flick lands where the person is now. A live pin carries its card's id too, so `/on-the-pin`'s ring finds it. `/live` off, or nobody live: no band on the people map. Untick and the band is the map's whole set again.

## hostile cases

- Nobody live: no band.
- A person live with no card: a lozenge with their initial and name, no line.
- Someone leaves: their lozenge goes on the next beat.
- The posts map: untouched.

## glossary

(no new terms)

## code description

`live-only.js` — wraps `feature_Live.draw`/`clear` (keep the rows, redraw or update the band), `feature_Live.pinHtml` (the id on the live pin) and `feature_Reel.posts` (the live rows on the people map).
