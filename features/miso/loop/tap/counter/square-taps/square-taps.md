# square-taps
*a sub-tool that multiplies the count by itself*

> (asks#1787346132373)
> Square the tap count
> *(filed from the field on 2026-08-21, birthplace `taps @
> miso/loop/tap/counter`, proposal approved in the ask box — the second
> ask of the evening, and the first whose node cites the ask store
> directly: the `asks#` anchor was rebuilt for it)*

## user

Long-hold the taps tool and a **n²** button joins reset, ×2 and −1.
Press it and the count becomes its own square — 3 taps become 9,
everywhere you're signed in.

## spec

The exact shape of `/double-taps`, with squaring where doubling was: a
`tap_square` event reads the count the user can see and writes its
square as an epoch reset, so the fleet converges on the new number the
way it does on zero. `saturating_mul` keeps a huge count at the u64
ceiling instead of wrapping. The control registers only while the taps
tool is open, like its siblings.

## glossary

- **n²**: the control's face — the count, squared, once per press.

## code description

`square-taps.rs`: `feature_SquareTaps` extends `update` (claims
`tap_square`: `tap_count_read()` then `tap_count_reset` of the
saturating square) and `tool_controls` (appends the n² button while
`open_tool` is `taps`), both by the `/double-taps` idiom.
