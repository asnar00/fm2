# visibility
*promote becomes a visibility button that pops the same level list the recording row uses, and one tap sets it*

> (transcripts/2026-09-04-field-walk.md#p114)
> for the "promote" workflow, let's change it to a "visibility" toolbar that pops up the same option panel as in the settings in 'add post'.

## user

Open a post of your own and the row has an **eye** where the up-arrow used to
be. Tap it and the same list you set the level with before recording rises out
of the row — the six roles, each with its line, and the one this post is at
already marked.

Tap a role and that is who the post reaches, in one tap, up or down. The list
puts itself away. Under the post the line follows: *visible to volunteers*.

Tap the eye again, tap anywhere else, or press ‹ once, and the list closes and
leaves you on the post; ‹ again goes back to the list of posts. Somebody
else's post has no eye, as it had no arrow.

## spec

`/audience` gave a post an up-arrow: one rung per tap, one direction, and no
way back — three taps to get a candidate's post to the volunteers. Ash asked
for a visibility toolbar popping the panel the recording row already has
(#p114).

**The arrow is cut, not gated.** `/audience`'s own link still decides
everything it decided; this node removes the drawn button from the row
(`/plus-at-home`'s cut) and puts its own beside it. Untick and the arrow is
back with all of `/audience`'s tests intact, because none of them moved.

**One test is dropped, deliberately.** `/audience` hid the arrow at `public`,
because a promote there could not move. A picker can go back down, so
"already the widest" is no longer a reason to have no button.

**The panel is the recording row's, through a seam, not a copy.** `/armed`'s
`armed_level_row` is split into `armed_level_entries(prefix, lit)` and
`armed_level_box(what, entries)` — both answering exactly what the expression
they came out of answered — and this node asks for those entries with its own
event prefix and the post's own floor lit. So whatever `/own-role`,
`/explained` and `/plain-words` have done to that list (six roles, a sentence
each, the plain words) this surface has for nothing, and the two can never
drift. The box is `/in-place`'s own `.armed-pop`: one popover shape in the app,
not two.

**Open is a flag on the turn's state**, `/in-place`'s idiom, with
`/in-place`'s rules: the eye toggles it, anything else closes it, and ‹ is
caught before the chain so the first press closes the panel and leaves the card
open. A tap on bare ground is `/in-place`'s own listener, which fires for any
`.armed-pop` on screen and finds this one too.

**`PostSetFloor {id, floor, t}`, a new event.** Not a widened `PostPromote`:
promote means one rung, one way, and `/undo`, the black box and anything
reading the log are entitled to keep that meaning. This one says where the
floor is to be and says it once. It is sent from the page half because the
write needs a clock and `update` has none (misses.md, the clock in wasm) —
`/audience`'s promote is sent the same way, in the same capture phase.

**It travels the road promote travelled.** The same `cards_write`, the same
`edited` bump — so `exchange_share` hands the post out to everyone on the
project exactly as it did for a promote, and a node watching for a floor that
moved (the withdrawal landing beside this one) sees this write like any other.
Nothing new is transported and no second road exists.

**The clamp is the recording row's**, for its reason: a floor above the
author's own role would hide the post from the person who wrote it. An author
with no role in the project the post is filed in is not clamped — there is
nothing to clamp to, and `card_new` would not have stamped a floor there at
all.

**Undo.** The step is filed the way promote's is, through `/audience`'s own
`audience_record`, so the arrow's undo behaviour is this button's too.

**Parked, and named** (`/anticipation`): the level shown on the eye itself, so
the row says who holds the post without opening anything; a "who has it" list,
which `audience_people_of` would answer; and the same picker on the posts list
for a post you have not opened, which would need a card id the row does not
have.

## hostile cases

- **Somebody else's post.** No eye — the card carries `from`, which is the
  structural test `/audience` and `/delete` both use. The panel's own render
  makes the same test, so a stale frame cannot show a picker over a copy.
- **A post in no project.** No eye and no panel: there is no floor to set, and
  `card_new` never stamped one.
- **The panel open when the card closes.** ‹ closes the panel first and leaves
  the card; the second ‹ closes the card. Any other tap — including the tap
  that closes the card — closes the panel in the same turn, and the panel's
  render also requires a post to be open, so it cannot outlive the card by a
  frame.
- **A pick on the level the post is already at.** Not a write: no `edited`, no
  hand-out, nothing on `/undo`'s stack. The panel still closes.
- **A pick above the author's own role.** Clamped to that role, as the
  recording row clamps. If the post is already at that role the clamp lands on
  "no change" and nothing is written.
- **A hand-made `PostSetFloor`.** The word must be one of the six
  (`audience_is_grade`), the card must be a post of yours in a project, and the
  clamp still applies — the same three gates promote has.
- **`/armed`, `/own-role`, `/explained` or `/plain-words` unticked.** The panel
  draws whatever list is composed: seven rows with "same as me" if `/own-role`
  is off (its row sends an empty floor, which the page half declines to send at
  all), no sentences if `/explained` is off. This node does not compose without
  `/armed` — it is that node's list it is showing, and a copy of it is the one
  thing the ask asked not to make.
- **`/in-place` unticked.** The popover's ground and place come from its CSS,
  so the panel would draw unstyled — inside the page rather than over the row.
  Named: the shared shape is a shared dependency, and it is the same trade the
  shared list makes.
- **This node unticked.** The up-arrow is back, one rung per tap, hidden at
  public — `/audience` exactly as it reads today.

## glossary

- **visibility**: the level a post is at, set in one tap from its own page.
  The same six roles the recording row offers, for a post that already exists.

## code description

`visibility.rs` — `tool_controls` cuts `/audience`'s promote arrow out of the
row and puts the eye in its place, on your own post that is in a project;
`vis_button` builds it, `vis_strip` is the cut, `vis_eye_svg` the glyph.

`visibility.rs` — `render` draws `/in-place`'s popover holding `/armed`'s own
level list, asked for with this node's event prefix and the post's floor lit.

`visibility.rs` — `update` toggles the panel on the eye, closes it on anything
else, catches ‹ before the chain, and hands a `PostSetFloor` to `vis_set`.

`visibility.rs` — `vis_set` writes the floor through `/cards`' own
`cards_write` with `edited` moved, so `/exchange` hands the post out as it does
for a promote; `vis_clamped` holds it to the author's own role, and `vis_shut`
closes the panel whatever the pick did.

`visibility.js` — the capture-phase listener that turns a tap on a row into
`PostSetFloor` with the time on it, and stops the click so the generic close
rule cannot shut the panel before the pick lands.
