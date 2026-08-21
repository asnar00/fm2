# square
*the tap button has corners now*

> (transcripts/2026-08-21-hybrid.md#p66)
> I made an ask: can you fire up the monitor and work on it please? Thanks
> *(the ask arrived from the field on 2026-08-21, filed from inside the taps
> tool — text "Square", birthplace `taps @ miso/loop/tap/counter`, proposal
> approved in the ask box; ask record t=1787345636335)*

## user

The tap button is square-cornered — a plain rectangle on miso's graph
paper, instead of the pill it launched as.

## spec

The ask is one word, read literally: the `.tap` button's `border-radius:
999px` pill becomes a true square corner (`0`). The background grid and
the toolbar's squares are the surrounding language; a sharp rectangle
sits in it naturally. If a softened corner was wanted instead, that is a
thirty-second follow-up ask — the interpretation is surfaced here rather
than silently chosen (the request-box precedent, fm-spec-2 #p15).

Shape is a taste parameter: by the promotion rule (notes.md #p18) this
first ask ships the literal constant; a second shape-flavoured ask
promotes it to a declared var. Unticking this node restores the pill.

## glossary

- **pill**: a fully-rounded button (`border-radius: 999px`), the tap
  button's original shape.

## code description

`square.index.css`: one override — `.tap { border-radius: 0; }` —
composed after `/tap`'s own stylesheet, so unticking falls back to the
pill.
