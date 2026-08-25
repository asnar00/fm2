# unchanged
*a tap-away that changed nothing sends nothing*

> (transcripts/2026-08-25-accounts.md#p76)
> ok let's do this. the "user" tool should show all users …
> *(the residual `/people` named on delivery: after merely looking at a card block, the first tap on a toolbar button did nothing; fixed in the run under the residuals rule, #p50)*

## user

Tap into your card, tap a toolbar button: the button works first time, whether or not you typed anything.

## spec

`/cards` saves on every focusout, a save is a repaint, and a repaint between a tap's mousedown and mouseup replaces the button under the finger — so the first tap after merely looking at a block was lost (rig-proved by `/people` to predate it). This node listens for focusout in the capture phase, ahead of `/cards`' listener, and when the block's text (by `/cards`' own `textOf` rule) equals what the store holds for that block, stops the event there: no `CardEdit`, no repaint, the tap lands. A real change still saves exactly as before. `/keep`'s own swallow (its repaint's focusout) is unaffected. Untick and the empty save returns.

## hostile cases

- The bridged `cards` lags one turn (`cards.md`): the comparison may see the previous text and let one redundant save through — a wasted repaint, never a lost edit.
- A block with no stored counterpart (malformed var): the event passes through as before.

## glossary

(no new terms)

## code description

`unchanged.js` — one capture-phase focusout listener; `fm_storedText` reads the block's text from the bridged `s.cards`.
