# reset-taps
*the first tool built by asking for it: a reset button in the taps tool*

> (transcripts/2026-08-15-fm-spec.md#p27)
> fair warning: it's going to be a "reset" button in the taps tool. So I'd be in the taps tool, realise I want to reset the counter, and ask for "reset taps".
> *(and so it came: the ask arrived from the field on 2026-08-15, filed from inside the taps tool on muon build 144 — text "reset tap count", birthplace `taps @ muon/loop/tap/counter`, proposal approved in the ask box: "The taps tool gains a new ability: reset tap count. It appears in the tool and does exactly that.")*

## user

The taps tool gains a new ability: reset tap count. It appears in the
tool and does exactly that.

## spec

Reset is a **sub-tool of taps** (#p32's correction: "reset is a
sub-tool of taps, the way record is a sub-tool of dictate"): its
control rides the toolbar while the taps tool is open, via the
`tool_controls` chain — not a button floating in the tool's canvas.
Tapping it sets the shared count to zero with register semantics —
`Var::<u64>::global` `set`, written locally for the instant feel and
shipped as a last-write-wins `VarSet`, so every instance converges to
zero the way tap increments already converge upward. The honest CRDT
footnote (#p128's op-fold question, met in the wild for the first
time): a tap in flight at the moment of a reset lands before or after
it by arrival order — the register and the counter ops race, and
last-write-wins is the chosen answer, not a hidden accident.

## glossary

- **register semantics**: reset writes a value, not an op — concurrent
  increments race it and arrival order decides; the count converges
  either way.

## code description

`reset-taps.rs` extends two chains. `tool_controls` (the seam `/tools`
declares for exactly this, `/dictate`'s record button the precedent):
when `open_tool` is taps, it appends the ↺ control
(`tool-button ctrl`, `data-ev="tap_reset"`) after whatever controls
came before. `update`: a `tap_reset` click sets
`Var::<u64>::global("tap_count")` to zero — local write plus the
synced `VarSet`, `/scope` carrying it everywhere. The toolbar's
standing machinery does the rest: ember centres the control in the
free space, steady keeps it still while you tap.
