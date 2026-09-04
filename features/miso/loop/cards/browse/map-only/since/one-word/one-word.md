# one-word
*the slot shows the filter you are in; tap it and the four drop under it*

> (asks#1788534899566)
> Change the today/week/etc selector so it only shows the selected filter, tap to drop down a selectable list
> *(filed from the field on 2026-09-04 by ash)*

## user

Top left there is one word — **today**, **week**, **month** or **all**,
whichever you are in. Tap it and the four drop down under it, one to a line,
the one you are in lit. Tap one and the column puts itself away and the word at
the top is the one you picked. Tap the word again, or anything else on the
screen, and it puts itself away without changing anything. ‹ closes it too, and
leaves you where you were.

## spec

`/since` put four pills where the view picker used to be. Four words take 175
of the strip's 402 points where three glyphs took 96 — enough that `/title`'s
project name had to be pinned into what was left of the middle. Ash asked for
the slot to show only the chosen filter and to drop the list on a tap
(asks#1788534899566). One reading, so it builds.

**One word in the slot.** `browse_slot_html` — `/map-only`'s seam for that
place — draws a single pill carrying the chosen word, always lit, because
there is nothing beside it for it to be brighter than. It keeps `.since-pill`'s
class and look, so `/since`'s stylesheet, its long-press arming and its swallow
of the click after a read all still apply: this node adds a shape, not a second
grammar. The slot is about 50 points wide now, and `/title`'s name gets the
middle back.

**The column is a popover, not a level.** `/in-place` settled the house shape
for this a few hours ago — a picker that pops where you already are, because a
*setting* should not cost a level of the tree of tools — and this is that
shape, one screen edge up: the slot is at the top, so the list drops instead of
rising. Nothing here writes `open_tool` and nothing descends.

**Open is a flag on the turn's state, not a var.** `/in-place`'s idiom exactly,
and its consequences are that idiom's: no op on the wire, nothing stored,
and a column cannot outlive a relaunch. The flag is read off the state coming
in and written onto the state going out, so a tap that both closes the column
and does something else does both.

**Four rules, three of which are one rule.** The word toggles it. A pick closes
it — which is what puts it away once you have chosen, and the pick itself is
`/since`'s own `since_today` … `since_all`, unchanged, so the marks still ride
the tap and the filter still narrows in the same turn. Anything else on the
screen closes it. And ‹ is caught *before* the chain, so it closes the column
rather than also climbing a level; a second ‹ climbs as it always did.

**No guard on which tool is open.** The flag can only be set by a tap on the
word, and the word is only drawn where the slot is; every other click —
`tool_reports` included — clears the flag on the same turn, before `render`
runs. So a frame at another level cannot find a column in it.

## hostile cases

- **A pick that changes nothing** (tapping the word you are already in): the
  period is rewritten to the same value, `/since` skips the write, and the
  column closes. No op, no repaint beyond the close.
- **The column open when a message arrives.** A non-click event does not close
  it — the rule is about taps — so the map repaints under an open column and
  the column stays, which is what a popover should do.
- **The column open and the app backgrounded.** The flag is on the state, not
  in the world, so a relaunch has no column.
- **A long press on the word.** `/since` arms `.since-pill[data-ev]` for
  `/long-press` and swallows the click that follows a read, so the card shows
  and the column does not open.
- **`/since` unticked.** This node is its child and goes with it; there is no
  slot to hold a word.
- **`/in-place` unticked.** Nothing shared: the two nodes use the same idiom,
  not the same code.

## parked

- A sentence under each word. Ash's brief allowed one "if a sentence helps";
  four one-word options do not need explaining, and `/taste` 7 says a
  microcopy line beside a thing that shows what it does is the design being
  wrong. Named here because the next filter — a custom range — would need one.
- The word saying *which* today when the marks are a day out; `/marks-with-the-tap`
  makes that a turn long at most.

## glossary

- **the column**: the four periods dropped under the slot's word.

## code description

`one-word.rs` redefines `browse_slot_html()` with the single pill, and
`one_word_now()` is the chosen word — `all` for anything the var has never held,
the same reading `/since`'s own row makes.

`one_word_open(state)` reads the popover flag off the turn's state.
`one-word.rs` extends `update` with the four rules, catching `tools_home`
before the chain so ‹ closes the column instead of climbing.

`one-word.rs` extends `render` with the column when the flag is set, drawn from
`/since`'s own `since_pill` so the rows are the same control the strip used to
hold.

`one-word.css` sizes the single pill to its word, drops the column under the
slot on `/long-press`' card ground, makes the pills fill the column's width so
it reads as a list, and gives `/title`'s name back the width the four pills
were taking.
