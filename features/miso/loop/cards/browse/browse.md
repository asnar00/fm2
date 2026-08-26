# browse
*a tool that shows every card you hold — as a grid of tiles or as a list*

> (asks#1787669564115)
> add grid and list views for multiple cards
> *(filed from the field on 2026-08-25 by ash)*

> (accounts #p59, 2026-08-25 15:29, revision — where the switch goes)
> ah, I had an idea: let's add the selector button for grid/list at the top
> left of the screen, same height as the noob button. also, later when we add
> a map view, we'll select it from there

*(The anchor is the ask, not the revision. The revision's prompt sits at 15:29
and the ask at 15:52; citing the earlier one would move this node in
provenance order, ahead of four nodes written between those two times, for a
change that only says where a button goes. It is quoted as a revision, in the
shorthand the tree already uses for a prompt it is not citing, and the ask
keeps the position.)*

## user

The toolbar has a **cards** button. Tap it and you see everything you hold:
a grid of tiles, picture and title. At the top left of the screen, level with
the nøøb button, a small pill holds the views — a grid and a list — and the
one you are in is lit. The list is one line per card: its type on the left,
its title, and when you last changed it. Tap a tile or a line to open that
card and write in it; tap the cards button again to come back to the set, and
once more to leave. Which view you chose is remembered on this device.

## spec

`/cards` gave the card two renderings — the tile and the page — and only ever
mounted the tile behind a dev flag (`?cardtiles=1`). This node is the surface
that consumes it: a **tool** called `cards` whose display surface shows the
whole set.

**Two views of one set, and the set is everything you hold.** The grid is the
existing `.card-tiles` layout of `card_tile_html`; the list is one `.crow` per
card in the house list grammar (`/taste` 6) — the type where the number sits,
the title, the edited-when dim at the right. Nothing is filtered and nothing is
sorted: the set is `cards_read()` in the order the world holds it. Filtering by
type or project, sorting, and other people's cards are named and parked — they
want links and exchange, which do not exist yet.

**Which view, and which card, are navigation and therefore live on the
device.** Two vars, both `(device, last-write, own)` like `/tools`' own
`open_tool`: `view` (`"grid"` or `"list"`) and `open` (a card id, empty for
the set). Device scope is what makes them free — a device-scoped write queues
no op, so switching view or opening a card puts nothing on the wire and syncs
to nobody. Where you are is not what you own.

**The view picker is a place, not a control.** The first build made it two
sub-tools in the toolbar's control row, which is where a tool's buttons
normally go. Ash moved it: top left of the screen, level with the nøøb
lozenge, and named the reason in the same breath — *"later when we add a map
view, we'll select it from there"*. That makes it a **view picker** rather
than a toggle, so it is built as one: a pill in the top strip, mirroring the
lozenge across the screen (`/raised`'s `inset + 2px` top, the same 16px
margin, the lozenge's own `#121215`-on-`#3a3a3f` round, and its 33px height,
which is the lozenge's 15px line plus its 8px padding plus its border), whose
children are the views. The chosen one wears `#9db7d8`, the accent that
already means *chosen* everywhere else. **A third view joins as one more
child**: `browse_views()` is a chain, so a map node redefines it, calls
`existing`, and appends its button — one link, no layout change, because the
pill is a flex row that grows to hold what it is given.

The picker is on screen the whole time the tool is open, a card page
included, and picking a view from a card page puts the set back in that view.
A mode switch that changed a mode you cannot see would be a control doing
nothing; this way the picker always means the same thing.

**The way back is the tool's own button, one level at a time.** `/tools`'
grammar is "tap the open tool's button to return" (#p88). With a card open,
that tap means *back to the set*; with the set on screen, it means *leave the
tool*, exactly as it does everywhere else. So the card page needs no second
glyph in the control row and the row never carries two of the same shape.
The mechanism is a re-write in the same turn: `/tools` handles `tool_cards`
first and closes the tool, and this node — provenance-newer, therefore
outermost — puts `open_tool` back and clears `open` when a card was showing.
Leaving by any other tool button clears `open` too, so the tool always opens
on the set.

**Remembering the view is the client's job, and it is done without wrapping
the loop.** A device-scoped var never reaches the server, so nothing writes it
down — `/remember` says so in as many words. `/restore` solves the identical
problem for `open_tool` by wrapping `feature_Loop.apply` from a timer, which is
the race this tree has ruled against, so the first build of this node carried
a `localStorage` fragment of its own instead. `/world-cache` landed on main
while this one was being built and caches **device** vars on the device
precisely because nothing else can — so the fragment was deleted rather than
kept. The view comes back after a reload, the open card comes back with it,
and the mechanism is the tree's own rather than this node's copy of it. This
node has no page half at all. (`/world-cache` unticked, the two vars fall back
to their declared defaults on each load — the tool opens on the grid, on the
set. Smaller, not broken.)

**Reads go to the context, not to the bridged state.** `/payload` republishes
vars into the loop state part-way down the update chain, and this node's links
are outside it — so `s.open_tool` in a render that follows this node's own
write is one turn stale. Every read here is `open_tool_read()` /
`browse_view_read()` / `browse_open_read()` against the live context. It is
the same one-turn lag `/cards` documents for `s.cards`, met from the other
side: a budget may read a stale value, a renderer may not.

**The glyphs.** The tool is a stack of two rounded cards; the views are four
squares (grid) and three lines (list), all drawn inline SVG in `currentColor`
per `/glyphs` — no character with an emoji presentation, and no filter working
to correct an asset. *The brief proposed four squares for the tool itself;
that shape is the grid view's, and one screen carrying it twice reads as a
mistake — so the tool wears the stack instead.* Nothing this node draws goes
in the control row at all, so `/undo`'s button stays last there by not being
disturbed.

**Editing is not this node's.** A card opened from either view is
`card_page_html`, the same rendering `/me` uses, so `/keep`, `/frame`,
`/newline` and `/undo` act on it unchanged — they delegate on
`[data-card][data-block]`, which is what the page carries wherever it is
drawn. This node adds no editing code at all.

**When a card was edited** is drawn from `edited` by arithmetic, not by a
clock: a wasm build has no local time zone and no `SystemTime`, so the date is
computed from the epoch milliseconds in UTC and shown as `25 Aug` (with the
year when it is not this one — and "this one" is itself taken from the newest
card in the set, since there is nothing else to ask). Near midnight in summer
that is a day out. A relative "3h ago" would need the current time, which only
an event carries; it is the honest later rung.

The `?cardtiles=1` dev mount in `/cards` is untouched: with both on, the page
carries two grids, which is what a dev flag is for.

## hostile cases

- **No cards.** One dim line, `no cards yet`, where the grid would be.
- **A card with no title block.** The tile draws an empty face and an empty
  caption; the list row draws its type and its date with nothing between. No
  placeholder text is invented — an untitled card should look untitled.
- **A broken picture data URL.** The tile's `<img>` fails to decode and the
  face is an empty box; nothing throws, because a tile is markup and not a
  fetch. The list row never touches the picture at all.
- **Forty cards.** The grid and the list each scroll inside the display
  surface, which is fixed between the safe area and the toolbar — the toolbar
  stays put and the page body never scrolls.
- **`open` names a card that is gone** (deleted on another device, or the
  world arrived without it): the set is drawn instead, silently.
- **`/cards` unticked.** It is this node's parent, so unticking it takes this
  node with it and the tool leaves the toolbar.
- **A reload while a card was open.** `/world-cache` brings both device vars
  back, so the tool reopens on that card — where you were, which is what
  `/restore` already promises for which tool was open.

## glossary

- **the set**: every card in your world — what the cards tool shows.
- **view**: grid or list, the two ways the set is drawn — a map is named as
  the third.
- **view picker**: the pill at the top left of the screen that chooses a
  view; the top strip's left half, as the nøøb lozenge is its right.

## code description

`browse.vars` declares the two navigation vars, `view` (`"grid"`) and `open`
(empty), both `(device, last-write, own)` and neither bridged to the page:
nothing on the page half reads them, and a `js:` column is a promise.

`browse.rs` extends `tools_list` with `{id: "cards", label: "cards", icon}`,
the icon being an inline SVG string — `render_toolbar` drops it into the
`.icon` span, which is where a drawn glyph belongs.

`browse_picker_html` draws the top-strip pill, and `browse_views()` is the
chain a later view joins at — it returns the buttons and nothing else, so a
map node redefines it, calls `existing`, and appends one more.
`browse_view_button(which, on)` draws one, lit or not.

`browse.rs` extends `update` with four clicks. `browse_grid` and `browse_list`
write `view` and clear `open`. `browse_open:<id>` writes `open`. `tool_cards`
is the way back, handled after `/tools` has already closed the tool, as the
spec describes; any other `tool_` click, and `tools_home`, clears `open`.

`browse_cards(state)` is the seam for WHICH cards the surface draws — the
default is `cards_read()`, everything you hold, and a node that re-aims the
surface at a subset redefines it. `browse_row_left(card)` is the seam for the
left cell of a list row, where `/taste` 6 puts the number: the default is the
card's type, and a surface whose cards are all one type says something less
redundant there. Both were added for `/people` with their defaults unchanged,
so this tool renders exactly as it did.

`browse_when_of(card)` is the seam for WHICH of a card's times its row shows —
the default is `edited`, which is what the set of everything you hold wants,
and a card type whose date means something else redefines it (`/post-time`
is the first). Added with its default unchanged, and read by this node's row
and by `/portrait`'s alike.

`browse.rs` extends `render`: with the cards tool open it appends the picker
and then either the card page for `open` or the whole set.
`browse_set_html` is the grid/list switch and the empty case;
`browse_grid_html` wraps each tile in the element that carries `data-ev`;
`browse_list_html` draws the `.crow` rows; `browse_title_of` and `browse_when`
pull the two strings a row needs out of a card, and the four `browse_civil_*`
functions are the epoch-to-date arithmetic underneath `browse_when` — split
into four so no signature carries a comma-bearing return type the chain parser
cannot read.

There is no `browse.js`: the whole tool is Rust, a stylesheet and two
declarations. Nothing on the page half needs to know this node exists.

`browse.css` places the picker in the top strip against the lozenge's own
measurements, and places the two surfaces where `.card-tiles` already sits — fixed
between the safe area and the toolbar, scrolling their own contents — and
styles the list against the `.crow` grammar it borrows: the type dim where the
number sits, the title in the prose weight, the date the dimmest thing on the
line.
