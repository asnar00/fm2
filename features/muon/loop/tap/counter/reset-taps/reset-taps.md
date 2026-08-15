# reset-taps
*the first tool built by asking for it: a reset button in the taps tool*

> (transcripts/2026-08-15-fm-spec.md#p27)
> fair warning: it's going to be a "reset" button in the taps tool. So I'd be in the taps tool, realise I want to reset the counter, and ask for "reset taps".
> *(and so it came: the ask arrived from the field on 2026-08-15, filed from inside the taps tool on muon build 144 — text "reset tap count", birthplace `taps @ muon/loop/tap/counter`, proposal approved in the ask box: "The taps tool gains a new ability: reset tap count. It appears in the tool and does exactly that.")*

## spec

A **reset** button joins the tap pill in the taps tool. Tapping it
sets the shared count to zero with register semantics —
`Var::<u64>::global` `set`, written locally for the instant feel and
shipped as a last-write-wins `VarSet`, so every instance converges to
zero the way tap increments already converge upward. The honest CRDT
footnote (#p128's op-fold question, met in the wild for the first
time): a tap in flight at the moment of a reset lands before or after
it by arrival order — the register and the counter ops race, and
last-write-wins is the chosen answer, not a hidden accident.

The button renders under the pill, only where the pill itself renders
(the launcher-aware condition `/tap` established), and only when there
is something to reset — a zero count keeps the tool clean.

## user

The taps tool gains a new ability: reset tap count. It appears in the
tool and does exactly that.

## glossary

- **register semantics**: reset writes a value, not an op — concurrent
  increments race it and arrival order decides; the count converges
  either way.

## code description

`reset-taps.rs` extends both chains. `render`: after the existing
output, under `/tap`'s own visibility condition (launcher-aware, taps
open) and only when the count is nonzero, it appends the reset button
(`data-ev="tap_reset"`, reusing the pill's look with a `reset`
modifier). `update`: a `tap_reset` click sets
`Var::<u64>::global("tap_count")` to zero — local write plus the
synced `VarSet`, `/scope` carrying it everywhere.

`reset-taps.css` sizes the button down and dims it — an accessory to
the pill, not a rival.
