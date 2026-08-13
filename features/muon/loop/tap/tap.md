# tap
*the first interactive feature: a tap counter proving the loop*

> (transcripts/2026-08-13-fm-spec.md#p97)
> OK let's do the event core next

## spec

The event core's proof, and the pattern every interactive feature copies: one node owning an `update` extension (react to its event, transform its state key), a `render` extension (draw from state), and a styling fragment. A "tap" pill sits under the logo; tapping it increments `tap_count` in state and the label becomes "taps: N". Untick the node and the pill, its behaviour, and its styling all leave the build.

## user

Tap the pill, watch it count. That's Rust handling your finger through the whole fm chain — and the template for every muon app to come.

## glossary

(no new terms)

## code description

`tap.rs`: the `update` /extension/ delegates via `existing.update`, then increments `tap_count` when the event is its own (`ev == "tap"`); the `render` /extension/ appends the pill (`data-ev="tap"`) with the count-aware label.

`tap.css` styles the pill.
