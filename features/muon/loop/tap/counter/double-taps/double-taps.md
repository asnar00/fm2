# double-taps
*asked for as its own example: a ×2 sub-tool in the taps toolbar*

> (transcripts/2026-08-15-fm-spec.md#p33)
> It should concisely describe what the tool does : "double the tap count"
> *(and then it was asked for in earnest: filed from inside the taps tool on 2026-08-15, muon build 148 — text "double the tap count", birthplace `taps @ muon/loop/tap/counter`, approved proposal: "double the tap count" — the first ask drafted in the concise style it itself defined)*

## spec

A **×2** control joins reset in the taps toolbar while the taps tool
is open (`tool_controls`, the sub-tool idiom `/reset-taps` settled at
#p32). Tapping it doubles the shared count: the doubled value is
computed from the local replica and written with register semantics —
`Var::<u64>::global` `set`, local for the instant feel, a
last-write-wins `VarSet` sweeping the fleet. Same honest race as
reset: increments in flight land before or after the doubling by
arrival order.

## user

double the tap count

## glossary

- **×2**: the doubling control — reads the count you can see, writes
  twice that, everywhere.

## code description

`double-taps.rs` extends two chains. `tool_controls`: when `open_tool`
is taps, appends the ×2 button (`tool-button ctrl`,
`data-ev="tap_double"`) after whatever controls came before — reset
included, both centring in the toolbar's free space. `update`: a
`tap_double` click reads the local `tap_count` and sets
`Var::<u64>::global("tap_count")` to twice it.
