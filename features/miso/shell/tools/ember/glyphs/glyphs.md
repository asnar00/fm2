# glyphs
*agent instruction: a toolbar icon is ink, never a picture*

> (transcripts/2026-08-25-accounts.md#p44a)
> this should be another agent-instruction feature

> (transcripts/2026-08-25-accounts.md#p44, the ruling it carries)
> the trouble is that you used what looks like a standard emoji character for undo - the intent was to change the *shape* of the icon, not use a coloured emoji at low brightness. Build a proper icon that matches the aesthetic of the other tool icons, rather than using a colour bitmap.

## user

For agents. When you make a toolbar icon, read `/glyphs` in the composed skillset: a filtered emoji or a drawn SVG in `currentColor`, never a character with an emoji presentation; control buttons black on a palette colour; undo stays last in every row.

## spec

An agent-only node, the third of its kind after `/taste`, `/did-you-mean` and `/attention`: its whole implementation is `glyphs.agent.md`, assembled by fmlink into the product's skillset and toggling with the node. It exists because the undo arrow shipped as ↩ (U+21A9), which iOS draws as a colour bitmap, and the fix that followed (`/arrow`) had to learn the rule the hard way (#p44). Ash ruled the lesson an agent-instruction feature rather than an amendment to `/taste` (#p44a): the rule is about *making a glyph*, a builder's act, and lives under `/ember`, which owns the toolbar's ink.

## glossary

- **emoji presentation**: a Unicode character the platform renders as a colour emoji rather than text.

## code description

No runtime code. `glyphs.agent.md` is the instruction; the linker emits it into `products/miso/build/skillset.md` in provenance order with a provenance comment.
