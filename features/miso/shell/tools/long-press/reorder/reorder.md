# reorder
*keep holding, and drag the button to where you want it*

> (asks#1787704230398)
> if we hold down a tool button to see its tooltip, we should then be able to drag it left and right to reorder the toolbar
> *(a field ask, filed 2026-08-26)*

## user

Hold a tool button until its card appears, then — without lifting your
finger — slide it left or right. The other buttons part to make room; let
go and the tool stays where you put it. The row you arrange is yours: it
follows you to every device you sign in on. A hold that doesn't move still
just shows the card, and a tap still opens the tool.

## spec

The hold already says *this button, and I mean it*. From there a sideways
move is the second half of the same gesture: the card goes away, the held
button follows the finger along the row, and the buttons it passes shift
aside to open the slot it will land in. On release the row as arranged is
the row, and it is remembered.

**The order is the user's, not the device's.** `tool_order` is a
`(user, last-write, own)` `/var` — a JSON list of tool ids — so arranging
the row on a phone arranges it on a laptop, unlike `open_tool` beside it,
which is device-scoped because navigation is a place, not a preference.

**A partial order, not a fixed one.** The list names the tools it names;
everything else keeps registration order behind them. A tool that ships
next week appears at the end of a row someone arranged last month, rather
than vanishing or displacing what they chose; a tool they turn off simply
falls out and its place closes up.

**Only the launcher reorders.** With a tool open the toolbar is that tool's
control surface — its own button and its controls, whose order means
something the user did not choose — so the drag only arms when `open_tool`
is empty. Vertical drift is still a scroll and still disarms the hold, as
it always did: a drag is horizontal, and only after the card has shown.

**A chosen order beats a default order, whichever way round provenance puts
them.** `/lead` — "projects, posts, users first" — was asked six minutes after
this was, so its link sits *outside* this one and its default would have quietly
undone every drag. Chain position cannot settle that argument, so a seam does:
`tools_order_chosen()` on `/tools` (base `false`) is redefined here to say yes
once this person has arranged the row, and a feature imposing a default order
asks before imposing it. Both nodes stay independently tickable and each is
unchanged alone.

Parked (asked for, not yet): reordering controls inside an open tool's row,
and hiding a tool from the row altogether — that is the chooser's tick.

## glossary

- **launcher**: the toolbar with no tool open — every tool's button in a
  row, which is the row this feature reorders.
- **slot**: the gap the other buttons open at the index a dragged button
  will land in.

## code description

`reorder.vars` declares `tool_order` (`"[]"`, user-scoped, last-write,
own): the order as a JSON list of tool ids.

`reorder.rs` redefines `tools_order_chosen` — the order seam this node added to
`/tools`, base `false`, so `/tools` alone is unchanged — to answer yes once the
var holds a non-empty list. `/lead` asks it before applying its default; a
garbage or empty var reads as "not chosen", so a broken value falls back to
whatever default the composition has rather than to nothing.

`reorder.rs` also redefines `tools_list`, outermost on the registry chain by
provenance, so every tool has registered by the time it sorts: ids the
order names come first in its order, everything else follows in
registration order. `render_toolbar` and the catalog both read the chain
through the composed global, so both see the sorted row without knowing
this node exists. `update` handles the `ToolOrder` event by writing the
var; an empty or malformed payload is ignored rather than written, because
a dropped drag must never cost someone the arrangement they had.
`tool_order_read`/`tool_order_write` are the address, written once.

`reorder.js` extends `/long-press` the way `/sub-tool-cards` does — its own
listeners reading the parent's public state, no edit to the parent's
handlers. It arms on `pointerdown` over a launcher tool button, and on
`pointermove` past the parent's own 12px threshold it starts a drag *only*
if the parent's card has already fired (`feature_LongPress.fired`) and the
move is more horizontal than vertical; anything else falls through to the
parent's disarm untouched. `begin` measures the row once — the DOM is not
swapped while a finger is down, so the button centres stay true for the
whole drag — `move` translates the held button and picks the nearest centre
as the target index, `slot` shifts the buttons between old and new index by
one pitch, and `drop` clears the transforms and sends
`{type:"ToolOrder", data:{order:[…]}}`, whose repaint renders the row from
the var. A capture-phase `click` listener swallows the click after a drag
and clears the parent's `fired` flag — the parent's own swallow only fires
when the click lands on a tool button, and a release can end anywhere.
Every reference to `feature_LongPress` and `feature_Loop` is typeof-guarded,
so the fragment is inert if either is unticked.

`reorder.css` gives launcher tool buttons `touch-action: none` (without it
the browser claims the sideways move as a scroll and no drag ever starts on
a phone), the held button no transition and a lift above its neighbours,
and the parting buttons the toolbar's own 0.18s ease-out.
