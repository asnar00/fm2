# open-chip
*every path to a tool ends at its door: found features grow an open button*

> (transcripts/2026-08-15-fm-spec.md#p19)
> If I type "record a voice note" I get three hits - dictate (the right one), ask, and phone. But it's only showing me features - I can read the instructions, and follow them, which is fine, but we're not quite at the stage of it doing it for me. Whereas if I type "dictate", I get an "open dictate" button. I think it would be better if both those paths led to an "open dictate" button somehow.

## spec

Finding a feature and opening its tool were two different endings —
label matches got a door, meaning matches got reading material. Now
every found feature that belongs to a tool leads to the same **open**
chip. The ground truth is the tree itself: the export stamps each node
that registers a toolbar tool with its tool id (`tool_of` in
export_features — the #p54 feature→tool mapping, built at last), and a
found node resolves to the nearest tool **in its lineage, looking both
ways** — itself, then ancestors, then descendants — because the
relationship runs in both directions in the wild (`dictate/phone`'s
tool is on its ancestor; `tap`'s is on its child `counter`).

Resolved tools appear as the standard open chips above the result
rows, deduplicated against any the label match already made, and only
for tools actually present in this composition's toolbar — a stamped
tool whose feature is unticked simply resolves to nothing.

## user

Ask for a thing in any words — "record a voice note" — and along with
the features it found, miso offers the button that just does it:
**open dictate**. Reading about it stays one tap away; doing it is now
zero.

## glossary

- **lineage**: a node, its ancestors, and its descendants — the family
  line an open chip is resolved along.

## code description

`open-chip.index.js` wraps `feature_Ask.go` (composing after `/ask` and
`/semantic-find`, so it sees whatever rows any finder produced): after
the original renders, it reads the result rows' paths, resolves each
through `feature_Chooser.byPath` — self and ancestors via the `parent`
links, then descendants breadth-first — to a stamped `tool` id, keeps
those present in `/ask`'s tool catalog (state-derived: the toolbar's
DOM shows only the open tool in open mode, so it is view, not truth —
the lesson this node's first test taught), drops duplicates of chips
already rendered, and prepends the new chips into the results' chip
strip (creating it when the label match made none). The chips carry
the standard `data-open` attribute, so `/ask`'s own delegated click
handling — land in the tool, close the panel — needs nothing new.
