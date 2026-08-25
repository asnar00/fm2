# anticipation
*agent instruction: ship the ask, shaped for its successors*

> (transcripts/2026-08-25-accounts.md#p74)
> yeah I think the "gift of anticipation" is our friend here. The more we know about the user's task, the more we can anticipate what their asks are going to be. We can use that anticipation to build the ask in the right way, so it's easy to extend later when they inevitably ask for next steps

## user

For agents. When you brief or build an ask, read `/anticipation` in the composed skillset: build the literal ask, choose its shape from the asks you can foresee, and name those under "parked" rather than building them.

## spec

An agent-only node, beside `/did-you-mean` in the ask lifecycle: its whole implementation is `anticipation.agent.md`, assembled into the product's skillset and toggling with the node. It exists because the day produced both failure modes in one afternoon — an exchange brief that built the foundation instead of the feature (#p72), and a cards store built with no foundation in its shape — and ash named the principle between them (#p74). It governs the triage seat (the brief) and the worker (the shape of the delivery).

## glossary

- **anticipation**: knowing the user's task well enough to shape today's ask for the asks to come, without building them.

## code description

No runtime code. `anticipation.agent.md` is the instruction; the linker emits it into `products/miso/build/skillset.md` in provenance order.
