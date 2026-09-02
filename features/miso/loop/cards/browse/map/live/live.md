# live
*while you are in the app, the people who hold your card see where you are*

> (asks#1788346282800)
> on the users page, the map view should show the current location of the user's active device, but only if they are actively focused on the app
> *(filed from the field on 2026-09-02 by ash, birthplace `-`)*

## user

You are on the map when the person you are looking for has the app open,
and only then. Open 👤, tap the map, and a pin wearing each person's face
stands where their phone is right now — everyone whose card you hold who
has the app in front of them at this moment, and you. It moves as they move.
Put the app away and your pin is gone from everyone's map within the minute;
open it again and you are back. Nobody is shown who is not in the app, and
nothing about where you were is kept anywhere.

## spec

The people surface already has a map (`/map`), and the pins on it are cards
that carry a place — where a card was *made*. Ash asked for a different
thing beside them (#asks 1788346282800): where each person's phone *is*,
bounded hard to the moments they are actually in the app. This node is that,
in one piece with two halves: the phone that says where it is, and the map
that asks.

**Reading the ask.** "The user's active device" was read at triage as each
person the 👤 page shows, not only the viewer — a map of one pin is the
withdrawn position readout again. The stamp hedges the reading: everyone
you hold, while they are in the app, tap if you meant only yourself.

**Who sees whom is `/people`'s audience, unchanged.** The 👤 page shows the
profile cards you hold: your own and the copies `/exchange` handed you. The
server answers `GET live/near` with the live positions of exactly those
people — anyone whose profile copy is in your world, matched by the copy's
**id**, which `/exchange` keeps as the owner's own card id (review,
2026-09-02: a match by `from` name would have shown one Bob's phone to the
holder of another Bob's card) — plus your own. Nothing new decides who is visible to whom; if you cannot see
somebody's card, you cannot see their pin, and a later visibility cue (a
project) joins here by handing cards, as it does today.

**Location is never written anywhere.** Not the op log, not any var, not any
world, not a file. A position lives in the server's memory alone — one entry
per user, `{lat, lon, t}` in a `Mutex<HashMap>` inside a function body, the
idiom `/one-way` and `/adopt` use — and an entry is dropped sixty seconds
after its last heartbeat, swept on every read and write. A restart forgets
everyone, which is right. The server prints nothing that carries a
coordinate. This is the whole reason the node is an endpoint and not a var:
the world machinery remembers, relays and logs by design, and a location
trail is exactly what this feature must not produce.

**Publishing is bounded by one predicate, `feature_Live.may()`:**
`document.visibilityState === 'visible'`, and nothing else. On a phone one
app is in front at a time, so visible is focused. Window focus is not a
signal there: an installed app on iOS answers `hasFocus()` false for its
whole life, never fires `focus`, and fires `blur` at odd moments (the
Spotlight overlay closing at launch) with no `focus` to balance it — proven
on the iPhone 17 Pro simulator, 2026-09-02, when two earlier cuts that read
focus never published. While it
holds, the page asks `navigator.geolocation.getCurrentPosition` every ten
seconds — `/location`'s options, high accuracy off, the same permission the
phone already granted for posts, so nobody is asked twice — and `POST`s the
answer to `live/here`. The moment it stops holding — `visibilitychange` to
hidden, `blur`, `pagehide` — the page sends `live/gone` at once (a beacon,
so it leaves even as the page is torn down) and the timer stops. There is no
`watchPosition` and no background timer: a phone with the app behind another
app, or locked, publishes nothing. A "share for the next hour" is a second
clause in `may()`, and not built.

**Drawing is `/map`'s own seam.** `feature_Map.sync` runs after every paint
and is the moment the map is known to exist; this node takes it as
`/boundaries` did, by replacing the property at load. While the map view is
up (and the page visible) it fetches `live/near` every five seconds and
keeps one marker per live person, moving an existing marker rather than
remaking it so a pin slides instead of blinking. The face is the one the
grid tile draws — the server reads it from the card *you* hold, through
`/map`'s own `map_face_of` — in `/map`'s own pin markup with one addition: a
`map-live` class, and a quiet breathing ring (`/taste` 5: 1.6s ease-in-out
opacity, nothing faster). No colour is added: the ring is the app's own
light grey. A live pin stands above the placed pins. Leaving the map view,
or the page going hidden, stops the polling and clears the markers.

**Your own pin.** The requester's own entry is included in the answer and
drawn like the others: you see yourself as your team sees you, which is the
honest way to know what you are sharing.

**Nothing shows anywhere else.** The grid and the list do not change; no
coordinate text appears on any surface. The endpoint's answer carries a
name, a card id, a face and a position — never a world key, never a phone
number (`/exchange`'s rule).

**Parked, by design.** A trail ("where have they been today") is refused
here: the ephemeral store is the privacy promise, and a trail would need
its own explicit opt-in node with a different store. Sharing while not in
the app: a second clause in `may()`, its own node. Tapping a live pin to
message: the marker's click seam, its own node.

## hostile cases

- A real tap on a live pin (found by the one-pin review, 2026-09-02): the
  open is sent a beat after the click, not inside it. Sent inside, the page
  repainted under the finger, `clear()` removed the marker, and `/backdrop`'s
  document listener saw a card page and a tap on nothing it owned — the bare
  ground — and closed the card at once.

- An installed app on iOS: `hasFocus()` false, no `focus`, a stray `blur` —
  none of it matters; the page publishes while visible, stops on
  `visibilitychange` hidden, resumes on visible (the simulator, 2026-09-02).
- A desktop window behind another window is still visible, and publishes;
  the phone is the device this is for.

- Two people with the same display name, one card held: the holder sees the
  pin of the one whose card they hold and never the other — the match is by
  card id, and a person with no profile card yet matches nobody's copy.

- **Permission refused, or no geolocation** (a desktop, a rig with none):
  nothing is published, nothing is said. The person is on nobody's map and
  their own map still shows everyone else.
- **The phone goes dark mid-heartbeat**: the fix that lands after `gone` is
  not sent — `may()` is re-checked at the moment of posting, not only when
  the timer fires.
- **The app is killed without `pagehide`** (iOS swiping it away): the last
  heartbeat expires sixty seconds later and the pin goes with it.
- **The server restarts**: the store is empty; every phone's next heartbeat
  refills it within ten seconds.
- **A stranger's or expired cookie**: 403 on all three routes, `who are
  you?`, and nothing is stored.
- **A garbage position** (out of range, not a number, a body over 256
  bytes): dropped before it reaches the store.
- **Somebody whose card you do not hold is in the app**: their entry is not
  in your answer; the list is filtered by your held copies on the server,
  never on the page.
- **You hold nobody**: your answer is your own pin, or an empty list if you
  are not publishing.
- **Two guests with the same name**: their entries collapse onto one face —
  `/people`'s known limit, for the same reason.
- **`/map` unticked**: this node is its child and goes with it; nothing
  publishes, the routes are gone.
- **`/exchange` unticked**: you hold only your own card, so the answer is
  only you.

## glossary

- **live pin**: a person's face standing where their phone is now, drawn
  only while they are in the app.
- **heartbeat**: one position posted every ten seconds while the app is
  visible and focused; the entry it refreshes dies sixty seconds after the
  last one.

## code description

`live.rs` extends `route` with three paths, outermost on the chain (this
node is newest) and so gated here: `POST live/here` (a position in the
body), `POST live/gone` (forget me), `GET live/near` (who I may see). All
three take the caller from the cookie the way `/exchange` does
(`live_who`); no cookie or an invalid one is 403.

`live_store()` is the ephemeral store: a `Mutex<HashMap<String, Value>>` in
a `OnceLock` inside the function body — one `{lat, lon, t}` per world key.
`live_sweep(map, now)` drops entries older than `live_ttl_ms()` (60 000);
every route runs it before touching the map. `live_put`, `live_drop`,
`live_read` are the three verbs.

`live_near(me)` builds the answer: the caller's cards read outside any turn
through `exchange_cards_of`, the profile cards kept, and each live entry
matched — the caller's own key directly, everyone else by `exchange_name_of`
against the copies' `from`. Each row is `{name, id, face, initial, lat, lon,
t, me}`, the face and initial through `/map`'s `map_face_of` and
`map_initial_of`.

`live_sound(lat, lon)` is the bounds test a position must pass before it is
kept, written here so the route stands with `/location` unticked.

`live.js` is `feature_Live`. `may()` is the publish predicate. `beat()` runs
every ten seconds while `may()` holds: one `getCurrentPosition`, re-checked
against `may()` when the fix lands, then `POST live/here`. `leave()` sends
`live/gone` by `sendBeacon` and clears the timer; `arrive()` restarts it.
The listeners for `visibilitychange`, `focus`, `blur` and `pagehide` are
attached at load.

`feature_Live.sync()` wraps `feature_Map.sync` at load: with `#mapData`
present and the page visible it starts a five-second poll of `live/near`
(and fetches at once); otherwise it stops the poll and removes the markers.
`draw(rows)` moves, adds and removes markers by name, in `/map`'s pin markup
plus `map-live`, `zIndexOffset` 1000 so a live pin stands above placed ones.
If the map has never fitted anything, the first live rows fit it once.

`live.css` draws the ring: a `map-live` face carries a light-grey ring that
breathes at 1.6s.
