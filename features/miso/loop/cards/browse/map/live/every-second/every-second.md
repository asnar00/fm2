# every-second
*the live pin updates every second*

> (asks#1788370789212)
> increase the frequency of location updates to once every second

## user

On the people map, a person who is moving moves — their pin follows their phone once a second, not every ten.

## spec

`/live` reports the phone's position every ten seconds and a map that is up asks every five. Ash, out with the app, asked for once a second. This node sets both of `/live`'s constants — `BEAT_MS` and `POLL_MS` — to a second at load. Nothing else changes: publishing is still bounded by visibility, the server still keeps one entry per person in memory and drops it sixty seconds after the last heartbeat, and the same audience rule applies. Cost, stated: one small POST a second per phone in front, one GET a second per open map; both answered from memory. On the phone, `getCurrentPosition` with `maximumAge` answers from the last fix when nothing moved, so a still phone costs little.

## hostile cases

- A phone in the background: nothing is sent (the predicate is `/live`'s, unchanged).
- Twenty phones in front: twenty small requests a second at the mini, still nothing beside a tile fetch.
- Node unticked: ten seconds and five, as before.

## parked

- An adaptive rate (faster while moving, slower when still) — extends the same two constants from a motion reading.

## code description

`every-second.js` — sets `feature_Live.BEAT_MS` and `feature_Live.POLL_MS` to 1000 at load.
