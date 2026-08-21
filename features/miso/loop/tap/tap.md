# tap
*the first interactive feature: a tap counter proving the loop*

> (transcripts/2026-08-13-fm-spec.md#p97)
> OK let's do the event core next

## user

Tap the pill, watch it count. That's Rust handling your finger through the whole fm chain — and the template for every miso app to come.

## spec

The event core's proof, and the pattern every interactive feature copies: one node owning an `update` extension (react to its event, transform its state key), a `render` extension (draw from state), and a styling fragment. A "tap" pill sits under the logo; tapping it increments `tap_count` in state and the label becomes "taps: N". Untick the node and the pill, its behaviour, and its styling all leave the build.

## glossary

(no new terms)

## the counter moved into the context (rung 7)

The tap count is a declared `/var` now — `tap_count: Counter = Counter::zero()
(device, counter, own)` — and not a key in the loop's JSON state. Nothing a user
sees changed; what changed is where the number lives and what keeps it true.

**The seam, and why it has to exist.** SyncVar chose a var's scope at the CALL
SITE: this node said `local`, `/sync` said `global`, on one key, and unticking
`/sync` was what turned a shared counter back into a device one. A declaration
fixes scope at link time, so no single declaration can be both. The choice
therefore moves into three functions — `tap_count_read`, `tap_count_bump`,
`tap_count_reset` — which this node defines against its own device-scoped var
and which `/sync` redefines against a global one it declares itself. Unticking
`/sync` leaves these definitions standing, and the count is per-device again,
which is exactly what it always meant.

Two declarations rather than one is the honest cost of that. `tap`'s var is
unused while `/sync` is composed and `/sync`'s is undeclared while it is not,
so at any moment exactly one of them exists in a running world.

**Reads say `.sum`.** A `counter` var holds an epoch beside its number, because
that is what lets a reset drop the taps still in flight from before it. The
three tap tools are resets: `tap_reset` to zero, `×2` and `-1` to computed
numbers, each opening a new epoch. Under SyncVar those were `.set` on a summed
counter, which silently lost concurrent adds; now the loss has a rule and a
line in the log (converge.md argues the direction — reset wins).

## code description

`tap.rs` counts through the lattice's bottom rung: `Var::<u64>::local("tap_count")` — `add` on a tap event, `get` in the render extension that draws the pill (`data-ev="tap"`). Local scope never leaves the device; `/sync` escalates the same key to shared.
