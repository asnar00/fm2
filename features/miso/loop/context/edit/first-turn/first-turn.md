# first-turn
*starting up is a turn, so it freezes the context once instead of per gate*

> (transcripts/2026-08-21-hybrid.md#p56)
> let's fix all residuals next.

## user

For agents. Nothing to see: the app starts the same way and shows the same
thing. It stops cloning its whole world once per gate while it does it.

## spec

Rung 4 left this note: a gate calls `with_context`, and outside a turn that
reads *a copy of the live value* — a full `Context` clone, every time. During a
place's boot there is no turn, so every gate that runs while the app starts pays
one. Measured on the composed binary with 121 nodes: **15 clones per boot**,
each of a 121-var world, before the first event is even delivered.

This node opens a turn around the entry chain and closes it after: one freeze,
and every gate in the boot reads it. The count goes from 15 to 1.

**What that is worth, measured rather than assumed**: a boot takes 363µs with
the turn and 369µs without it (medians of 20, native debug build) — a 1.6%
difference, at the edge of this rig's noise. A `Context` clone is cheap because
most of its 121 vars are `Copy` scalars, so the win here is allocation pressure
rather than latency, and it grows with the tree: the count is one clone per gate
call, and the tree only gets bigger. No latency claim is made for it.

**Boot's own edits stay visible to boot.** `init` links write the context —
the open tool, the tool catalog — and `edit_context` mirrors a turn's own
writes into its frozen view, so a later link reads what an earlier one wrote,
exactly as it does inside an event. What the frozen view hides is a FOREIGN
edit landing mid-boot, which is the boundary law doing its job; on the client,
which is the place with a boot chain, there is no other writer at that moment.

**The server is deliberately not wrapped, and that is a measurement rather than
a preference.** Its entry is `serve`, which binds the port and then loops
forever accepting; a turn around it would hold one frozen world on the accept
thread for the life of the process. Startup on that side was measured at **one**
clone, because nothing state-carrying runs before the loop — so the wrap would
cost memory and buy nothing. Each request already opens its own turn.

**It depends on the depth counter.** Nothing inside a boot opens a turn today,
but if something ever does, `edit`'s counter makes the inner pair a no-op on
this freeze instead of a re-freeze from live.

## glossary

- **boot**: a place's entry chain — `boot()` in the wasm client, `serve()` in
  the native server.

## code description

`first-turn.rs` extends `boot` (line 12): open the turn, run the chain beneath,
close it, and hand back what the chain returned unchanged.
