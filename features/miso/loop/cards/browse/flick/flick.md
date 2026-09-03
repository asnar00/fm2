# flick
*swipe up or down on a card to reach the next or the previous one*

> (asks#1788463807285)
> in post or user view, swipe up and down to scroll to prev/next visible
> post/user without having to close

## user

Reading a post or a person, flick up and the next one in the list is
there; flick down and the previous one is. No closing, no going back to
the grid. A card that scrolls still scrolls: the flick counts only at the
end you are already at — at the bottom flicking up, at the top flicking
down.

## spec

A card page is left by ‹ or by the new ✕, and the next card is two taps
away. Tara's phone asked for the flick (the ask).

**The set is the surface's own.** Two events, `browse_next` and
`browse_prev`, handled in `update`: the list is what the open tool draws —
`posts_set()` under the posts tool, `browse_cards(state)` otherwise (the
seam `/people` and `/current-project` already re-aim, so the flick walks
exactly the cards the list shows, in its order) — the open card's index
is found and the neighbour written to `browse_open`. At either end the
event does nothing. The page repaints from a real turn, as a tap on a tile
would.

**The gesture is the page half's.** A vertical pointer sweep on
`.card-page` — at least 60px, under 40px sideways, within 600ms, not
starting in an editable block — sends `browse_next` when the page is
scrolled to its bottom (or does not scroll) and the sweep goes up, and
`browse_prev` at the top going down. In between, the sweep is a scroll and
nothing is sent. `/swipe-away`'s sideways flick on the map is untouched:
the two never overlap in axis.

## hostile cases

- **One card in the list.** No neighbour; nothing happens.
- **The open card is not in the list** (a project card opened from a
  person's page). No index; nothing happens.
- **A flick while editing.** The editable block has focus; ignored.
- **A long card, mid-scroll.** A scroll, never a flick.
- **This node unticked.** ‹ and ✕, as before.

## code description

`flick.rs` — `update` handles `browse_next` / `browse_prev` against the
surface's list. `flick.js` — the vertical sweep on the card page, gated by
the scroll position, sends the event.
