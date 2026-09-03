# learned
*what the asker's tweaks have taught: the defaults to build with before being asked*

> (transcripts/2026-09-03-housekeeping.md#p31)
> I'd like to set up a "self improvement" / "learning" process that looks at how the user asked for modifications to initial asks, so we can get better at anticipating the tweaks a given user will ask for, and build things the way the user likes by default

> (transcripts/2026-09-03-housekeeping.md#p32)
> it could use data from previous asks as well

## user

Things arrive the way you would have asked for them: the button in the toolbar, the map still visible behind the card, the mark on the pin, the share glyph, the thumbnail already big enough. Fewer second asks.

## spec

An agent-instruction node (`/skillset`): `learned.agent.md` composes into the skillset every builder reads at session start, beside `/taste`. Its content is distilled from the tweak digest — `tools/tweaks.py` walks the tree, dates every node by its first commit, and prints each ask with the refinements its children asked for within two days, all of history (#p32) — 169 refinements of 76 asks at first run, one asker. The distillation is a builder's act, like the misses ledger: at each session end, run the digest since the last run, read what changed after shipping, and write the pattern in as a default, with the node names that are its precedent. A rule here is worth keeping only while the precedent holds; an ask that contradicts it amends it. Untick and the skillset is without the learned defaults; the digest tool stays (tools are scaffolding, not features).

## glossary

- **tweak digest**: `tools/tweaks.py`'s output — asks paired with the refinements that followed them.

## code description

`learned.agent.md` — the instructions; the tree's third language, assembled by fmlink into `products/miso/build/skillset.md`.
