# tick-right
*on the first profile page the tick is on the right and undo on the left*

> (transcripts/2026-09-03-invite-test.md#p117)
> first profile page: 1) move the "tick" button to the RHS of the screen,
> and the undo to the left.

## user

While you are filling in your card for the first time, the row at the
bottom has undo at the far left and the tick at the far right, where a
"done" belongs. Everywhere else the row is as it was.

## spec

`/profile-first` withholds every tool button while the gate stands, leaving
the tick and undo alone in the row, side by side on the left (`/glyphs`:
undo last). With only two controls, ash wants them at the two ends (#p117):
the tick reads as *done*, at the thumb. This node's stylesheet reorders the
row under `body.fm-profile-first` only: undo first with the row's slack
after it, the tick last. Nothing moves once the gate lifts.

## hostile cases

- **The gate down.** The class is off the body; the row is `/tools`' own.
- **This node unticked.** Both at the left, as before.

## code description

`tick-right.css` — under `body.fm-profile-first`, `[data-ev="ctx_undo"]`
takes `order: -1; margin-right: auto`, `[data-ctl="card_edit"]` `order: 99`.
