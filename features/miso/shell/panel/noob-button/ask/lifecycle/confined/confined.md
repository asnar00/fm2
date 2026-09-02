# confined
*agent instruction: a change confined to one node needs no toggle proof*

> (transcripts/2026-09-02-settings.md#p4)
> the next thing is that I'd like to find ways to speed up the workflow that leads to a release. I wonder if the enable/disable check can be skipped by enforcing certain patterns in the work.

> (transcripts/2026-09-02-settings.md#p5)
> yup that sounds good, let's go for it

## user

For agents. Before proving a node's toggle (agents.md step 4), read `/confined` in the composed skillset: if every change under the feature tree lies inside one node plus a tick added to its parent's order.md, the unticked build is the last release and the proof is implied — skip the untick. Otherwise prove it and record it in the commit.

## spec

An agent-only node in the ask lifecycle, beside `/ship-as-built` and `/retrofit`: its implementation is `confined.agent.md`, composed into the product's skillset and toggling with the node. It carries the argument (the linker never reads an unticked node's files, so a confined change cannot alter the composition without its node — the untick cannot observe it, and for a new node that composition is the last release itself), the pattern's exact boundary, and the two premises the gate checks. The pattern is enforced by scaffolding, not memory: `tools/toggle_proof.py` classifies a change, `fmlink.py <product> --prove` runs it from the working tree, and deploy.sh refuses a non-confined commit that carries no `Toggle-proof:` trailer.

## glossary

`/confined` — a change whose feature-tree footprint is one node (its descendants and its own order.md included) plus additions to its parent's order.md; its toggle proof is implied.

## code description

No runtime code. `confined.agent.md` is the instruction; the linker emits it into `products/miso/build/skillset.md` in provenance order. The classifier and the gate are scaffolding in tools/ (toggle_proof.py; the `--prove` flag in fmlink.py; the proof gate in deploy.sh, before the build), documented in deploy.md.
