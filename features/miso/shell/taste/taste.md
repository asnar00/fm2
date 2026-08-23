# taste
*the aesthetic standard, encoded — so visual judgment stops being re-litigated*

> (transcripts/2026-08-23-plans.md#p25)
> before we continue, the other thing I'd like to encode is some kind of "aesthetic standard" so that visual stuff doesn't have to keep being re-litigated. Looking at what we have now, would you like to have a stab at extracting some principles we can use going forward?

## user

miso has a look — dark and calm, one gentle voice at a time, colour
only when it means something. This entry is that look written down, so
everything new arrives already matching, and so you can read exactly
what the standard is and change it by asking.

## spec

The first node in the tree whose implementation is written in the
tree's third language (#p29): agent instructions. It carries no `.rs`,
no page fragment, no assets — its whole behaviour is
`taste.agent.md`, nine principles extracted from the shipped surface
(#p25, the extraction reviewed live by ash), assembled by the linker
into the product's `skillset.md` and read by any agent building or
judging a visual surface (agents.md step 4a; the brief template's
taste-notes line).

The principles bind agents, not pixels: nothing in the composed app
changes when this node is ticked or unticked. What toggling changes is
the *builder* — unticked, the standard leaves the skillset and visual
judgment falls back to per-ask taste notes, which is exactly the
re-litigation this node exists to end. That asymmetry is the new
species, named at #p26: a feature that affects agents only.

Placement under `/shell`: the shell owns the app's visible chrome, and
every principle in the standard was extracted from surfaces the shell
family ships. Aesthetic asks from ash amend `taste.agent.md` — each
amendment a refinement prompt in this node's history — rather than
becoming one-off rulings scattered through notes.

## glossary

- **taste**: the encoded aesthetic standard — the set of principles an
  agent must satisfy before a visual surface ships.
- **agent instructions**: the tree's third language (`<name>.agent.md`
  beside `.rs` and `.js`): implementation whose runtime is an agent
  rather than a device, assembled per product into `skillset.md`,
  toggleable with its node.

## code description

`taste.agent.md` is the node's entire implementation: the nine
principles (ground, hierarchy, colour-as-meaning, shape, motion, the
list grammar, copy, one-channel, and 4a's two standing "no"s), each
stated with the shipped precedent it was extracted from, addressed to
the agent building or reviewing a visual surface.
