# decrement-taps
*a −1 in the taps toolbar, floored at zero*

> (transcripts/2026-08-15-fm-spec.md#p40)
> NEW PROPOSED … [in taps @ muon/loop/tap/counter] PROPOSAL: decrement tap count if >0
> *(a field ask whose arrival is its own anchor: filed from inside the taps tool on 2026-08-15, muon build 150 — the first node whose founding quote is the wish itself, verbatim, as it reached the builder)*

## user

decrement tap count if >0

## spec

A **−1** control joins reset and ×2 in the taps toolbar while the taps
tool is open (`tool_controls`, the sub-tool idiom). Tapping it lowers
the shared count by one, **only when the count is above zero** — the
asked-for guard, which is also the u64 floor. Register semantics like
its siblings: the decremented value computes from the local replica
and writes with a last-write-wins `VarSet`; increments in flight race
it by arrival order.

## glossary

- **−1**: the decrement control — one off the count you can see,
  everywhere, never below zero.

## code description

`decrement-taps.rs` extends two chains. `tool_controls`: when
`open_tool` is taps, appends the −1 button (`tool-button ctrl`,
`data-ev="tap_dec"`). `update`: a `tap_dec` click reads the local
`tap_count`; when zero it returns unchanged, otherwise it sets
`Var::<u64>::global("tap_count")` to one less.
