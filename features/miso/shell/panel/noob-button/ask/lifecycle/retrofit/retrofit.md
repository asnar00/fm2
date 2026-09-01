# retrofit
*a feature that changes what posts hold or render carries its own backfill*

> (transcripts/2026-09-01-saturday.md#p23a)
> just a note: in future, when we add a feature that requires posts to render or hold different data, we should always include a process that retrofits the new feature onto the existing posts, and take care to avoid destructive changes (i.e. if we change things, we should be able to revert if we get it wrong)

## user

Nothing on screen. New features that reshape posts arrive already knowing
about the posts you made before them, and a change that goes wrong can be
taken back.

## spec

An agent-instruction node, the `/anticipation` pattern: the instruction in
`retrofit.agent.md` composes into the product's skillset in provenance
order and toggles with this node. It was ruled the day two features left
old posts behind in one afternoon — video posts made before `/poster`
stayed faceless, and posts made before `/audience` fell out of view the
moment a project was selected. The rule it states: a brief that changes
what posts hold or render names its retrofit, and every retrofit is
additive and revertible.

## glossary

- **retrofit**: the process that brings existing posts up to a new
  feature's expectations — a backfill through the op door, or an explicit
  recorded ruling that old posts stay as they are.

## code description

`retrofit.agent.md` is this node's whole behaviour — an instruction to
agents, assembled by fmlink into `products/<product>/build/skillset.md`.
