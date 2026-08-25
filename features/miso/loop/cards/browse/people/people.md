# people
*👤 shows everyone whose card you hold — you first, then by how near they are*

> (transcripts/2026-08-25-accounts.md#p76)
> ok let's do this. the "user" tool should show all users, with the switchable grid/list view, just as the fourth "cards" view does - and we can lose the separate "cards" view. "self" should always come first in the list, and the other cards are ordered by invite proximity.

## user

Tap 👤 and you get the people, not a page: your own tile first, then everyone
whose card you hold — picture and name, in the same grid the cards tool had,
with the same pill at the top left to swap it for a list. Tap your own tile
and your card opens, editable, exactly as it was. Tap somebody else's and
theirs opens, to read. Tap 👤 again to come back to the people, and once more
to leave. The toolbar has no cards button any more: everything you hold is a
person today, and 👤 is where people live.

The order is you, then the people nearest you in the invite tree — whoever
invited you and whoever you invited, then theirs, and so on.

## spec

`/browse` built a surface — a grid, a list, a pill that swaps them, and a
card page one tap in — and hung it off a tool of its own. `/exchange` then put
other people's cards in your world. This node points that surface at 👤 and
retires the tool it was built on (#p76). Nothing is copied: `/browse` gained
two seams for it, both defaulted to what they already did, and this node
redefines them.

**The 👤 surface is the people set.** With `open_tool == "account"` and no
card open, the render chain appends `/browse`'s own picker and `/browse`'s own
grid-or-list, drawn over the **profile cards only**. The two device vars are
`/browse`'s unchanged — `view` and `open` — so which view you chose and which
card you had open still live on the device, still queue no op, and still come
back after a reload through `/world-cache`.

**Which cards, and which words, are the two seams.** `browse_cards(state)`
answers *which cards this surface draws* (default: everything you hold); this
node returns the profile cards, in people order. `browse_row_left(card)`
answers *what goes where the number goes* in a list row (default: the card's
type); on a surface where every card is a profile, "profile" on every line is
noise, so the distance word goes there instead. Both defaults are untouched,
so the cards tool — unticked, but still composable — renders exactly as
before.

**Losing the cards tool is one filter.** `tools_list` is a chain; this node
takes `cards` back out of it, the idiom `/under-account` already uses for the
invite tool. `/browse`'s code all stays: its picker, its renderers, its click
handling and its two vars are what this surface is made of. Untick this node
and the button is back, with its own tool behind it.

**Your own card is what your own tile opens.** `/me` still owns that page —
its `CardEnsure` and its patience are untouched, so the first tap on 👤 still
makes your card if you have none, and `/me`'s under-the-card seam still holds
whatever fills it. What changes is that `/me`'s page is no longer the
*landing* surface: it is one tap in. The mechanism is a **muted state**. `/me`
decides whether to draw by reading `open_tool` out of the loop state string it
is handed; this node is provenance-newer, so its render link is outermost and
hands the chain beneath a copy of the state with `open_tool` cleared whenever
the people set (or somebody else's card) is what should be on screen. Every
other render link reads the open tool from the live context
(`open_tool_read()`), and the two that do read the string test it for a
different tool (`/invite-tool` asks whether the *invite* tool is open, which a
cleared value answers correctly) — with one exception, repaired here:
`/under-account` reads the same key to decide whether to draw the invite plus
in 👤's control row, so this node redefines `tool_controls` to put the live
value back before the chain beneath sees it. The plus behaves exactly as it
did.

*(A seam in `/me` — "am I the landing surface?" — would be the honest form of
this and is a two-line change; it was out of this brief's bounds. Until then
the mute is a documented one-node contract: a later node that decides what to
render by reading `open_tool` from the state string rather than from
`open_tool_read()` must be told about it.)*

**Order is a chain, because proximity is going to grow.**
`people_order(cards, state)` takes the profile cards and the loop state and
returns them sorted: yourself first — the card with no `from` on it, which is
`/exchange`'s own test for "you wrote this" — then by *(distance, owner
name)*, and cards whose owner has no known distance last. Each card is
annotated with its distance under `near` on the way out, which is how the list
row gets its word without the row renderer needing the state. Shared
membership of a project is the second proximity cue ash named (#p71); it joins
by redefining this one function and mixing its own answer in, not by rewriting
the sort.

**Distance comes from the server, because the graph does.** The invite tree
lives in the guest list, which no device has: `/invite` writes `invited_by` on
every entry it mints, and that is one edge, undirected — being invited is as
near as inviting. `GET users/near` walks it breadth-first from the caller and
answers `{"ok":true,"near":{"<name>": distance}}` for everyone reachable, the
caller included at 0. The page half fetches it once each time the people
surface appears and hands it to the loop as a `PeopleNear` event, which this
node's `update` puts under the state key `near` — the `/invite-tool` idiom, and
for `/invite`'s reason: the guest list is the server's, and syncing it to
devices as world state would be a lie.

Distances are keyed by **name**, because a card's `owner` is a name and that
is the only thing the two sides share — the copy's `via` is an opaque tag now
(`/exchange`), and a world key is a phone number nobody should be handed. Two
guests with the same name collapse onto one distance; the guest list is one
campaign's and the failure is a wrong sort, not a wrong card.

**The word where the number goes.** `you` at distance 0, `n away` above it,
nothing at all for a card whose owner is not on the guest list. "invited" and
"invited by" were the alternative and were not taken: a distance is a number
and does not know which way round the edge was, so half the rows would be
guessing. Uniform and true beats warm and sometimes wrong (`/taste` 7).

**Only profiles, on purpose.** Projects, posts and recordings are cards too,
and they are not people. They get their own surfaces, which will re-aim
`/browse` exactly as this node does — a different `browse_cards`, a different
`browse_row_left`, the same picker and the same renderers. That is the shape
this node is built in rather than a promise it makes.

**Parked, with the seam each joins at**: project membership as a second
proximity cue (`people_order`); a projects surface (`browse_cards`); search
(the set is a function now, so a filter is a wrapper); telling you which way an
edge ran (a richer `users/near`).

## hostile cases

- **You hold nobody.** The set is your own tile, alone. Nothing says so — one
  tile is not an empty state.
- **No card of your own yet** (the first tap on 👤, before `CardEnsure`
  lands): the state is not muted at all, so `/me`'s own "making your card…"
  line is what you see, unchanged.
- **`near` has not arrived** (the first paint after 👤 opens): every distance
  is unknown, so you are first and the rest keep the world's order; the fetch
  lands a beat later and the set re-sorts.
- **`users/near` for a member who has invited nobody and was hand-added**:
  `{"<their name>": 0}` — themselves, and no one else.
- **A stranger's cookie**: 403 with `who are you?`. `/harden` makes a token
  invalid the moment its holder leaves the guest list, so an ex-member's
  year-long cookie cannot read the tree.
- **users.json unreadable or malformed**: the store's health is asked before
  the cookie — with the list broken nobody is authed, so asking authority
  first would answer a broken box with the wrong sentence (`/invite` learned
  this on a rig). The answer is `/invite`'s own: 500, `the guest list can't be
  read`, logged, nothing thrown.
- **`open` names a card that is gone**, or one that is not a profile: it is
  not in the set, so the set is drawn instead, silently — `/browse`'s own
  rule.
- **`/exchange` unticked**: you hold only your own cards, so 👤 is a grid of
  one tile that opens your card. Smaller, not broken.
- **`/invite` unticked**: no entry carries `invited_by`, so every distance but
  your own is unknown and the set is you first, then the world's order.
- **`/browse` unticked**: it is this node's parent, so unticking it takes this
  node with it and 👤 goes back to being your own card.

## glossary

- **the people set**: every profile card you hold — yours and the copies
  `/exchange` put in your world.
- **invite proximity**: the number of invite edges between you and someone,
  walked in either direction. You are 0; whoever invited you and whoever you
  invited are 1.

## code description

`people.rs` redefines `browse_cards`: it takes `/browse`'s answer, keeps the
cards of type `profile`, and hands them to `people_order`.

`people_order(cards, state)` is the ordering chain — self first, then by
`(distance, owner)`, unknown last — and annotates each card with its distance
under `near` on the way out. `people_rank` is the sort key that puts an unknown
distance last, and `people_word` turns a distance into the word a list row
shows.

`people.rs` redefines `browse_row_left` to that word, so the list's left cell
carries proximity instead of the type every row would repeat.

`people.rs` redefines `tools_list` to drop the `cards` entry, and `render` to
put `/browse`'s picker and set under 👤. `people_muted(state)` is the copy of
the loop state with `open_tool` cleared that the chain beneath is handed when
`/me`'s own-card page should not be the landing surface; `tool_controls` is
redefined to put the live value back for `/under-account`, the one other link
that decides on that key being `account`.

`people.rs` extends `update` with two things: the `PeopleNear` event, whose
data lands under the state key `near`; and the `tool_account` tap with a card
open, which means *back to the people* — `/tools` has already closed the tool
and `/browse` has already cleared `open`, so this link only puts the tool back,
which is why it reads both values BEFORE calling the chain beneath.

`people.rs` extends `route` with `GET users/near`. `people_users` reads the
guest list the three-way way `/invite` established (a list, or null meaning "do
not trust this"); `people_bfs` walks the `invited_by` edges in both directions
from the caller's world key and returns `{name: distance}`; `people_say` is the
error shape `/invite` uses. The walk is linear-scan rather than indexed: a
guest list is tens of rows, and a map would cost a type the chain parser cannot
read for nothing.

`people.js` fetches `users/near` once each time the people surface appears and
sends it into the loop as `PeopleNear`. It watches for the surface with a
`MutationObserver` on `#app` rather than by wrapping `feature_Loop.apply` —
that idiom races and orphans other fragments' wrappers (notes.md, "the
apply-wrapper race"), and `/invite`'s page half already reacts this way.
