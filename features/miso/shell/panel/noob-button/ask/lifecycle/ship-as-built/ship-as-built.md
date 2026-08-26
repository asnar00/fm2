# ship-as-built
*agent instruction: a proven node deploys at once*

> (transcripts/2026-08-25-accounts.md#p107)
> I'd suggest deploying features as soon as they're built - there's no way of knowing yet how urgent they are to the user

> (transcripts/2026-08-25-accounts.md#p108a)
> again, useful if this goes into an agent feature node

## user

For agents. When a node is proven and committed, read `/ship-as-built` in the composed skillset: deploy it now, one node at a time; never batch; never rig in the deploy's build directory while a deploy runs.

## spec

An agent-only node in the ask lifecycle, beside `/did-you-mean` and `/anticipation`: its whole implementation is `ship-as-built.agent.md`, composed into the product's skillset and toggling with the node. It exists because on Tara's morning seven proven asks sat committed behind one slow deploy — slowed by triage's own rigs relinking the shared build directory — and ash saw a queue on the phone (#p107), then ruled the lesson belongs in the tree (#p108a).

## glossary

(no new terms)

## code description

No runtime code. `ship-as-built.agent.md` is the instruction; the linker emits it into `products/miso/build/skillset.md` in provenance order.
