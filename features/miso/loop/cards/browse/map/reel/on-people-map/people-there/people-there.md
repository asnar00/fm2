# people-there
*on the people map the reel is people: the ones the map draws*

> (asks#1788449864303)
> The users reel should show users not posts
> *(filed from the field on 2026-09-03 by ash)*

## user

Under 👤's map the band holds the people on the map, newest place first: each lozenge a face, a name and their line, and when they were last placed. Flick it and the map glides to them; tap one and their card opens.

## spec

`/on-people-map` put posts in the band under the people map, reading the earlier ask that way; ash's ruling is users there (asks#1788449864303). One reading, so it builds: on the people map the band lists the set the map drew — the ids `/reel` already writes on `#mapData` — whatever their type, so a person's lozenge is their picture, their name and line, and the time of their place, ordered newest place first; each is `browse_open:<id>` so a tap opens the card, and the pin ringed is theirs. Elsewhere the band is as `/on-people-map` left it. Untick and the people map's band is posts again.

## glossary

(no new terms)

## code description

`people-there.js` — wraps `feature_Reel.posts`: on the people map, the map's own set as lozenges.
