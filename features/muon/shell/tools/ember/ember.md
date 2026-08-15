# ember
*an aesthetic experiment: each tool wears a colour from the ember 3400K Dark palette*

> (transcripts/2026-08-14-fm-spec-3.md#p56)
> OK: let's try this: let's assign each tool a colour from the first palette in that readme - and let's center the toolbar buttons horizontally. The appearance should be black icon on coloured background; light up the selected one (brighten the colour). I just want to see what aesthetic effect it has.

> (transcripts/2026-08-14-fm-spec-3.md#p86, draft-phase revision)
> What would actually be better is that the "<" and dictaford button would sit off to the left of the toolbar, kind of "owning" the toolbar space; and the rec button would then sit in the center of that area.

## spec

A deliberate look-and-see (the ideas.md ember note, cashed in): every tool button takes a colour from the first palette in the ember readme — 3400K Dark's six categorical colours, redshift-safe by construction — as **black icon on coloured background**, the selected tool lit by brightening its own colour rather than swapping to grey. The toolbar's buttons centre horizontally; in open-tool mode (#p86), the `‹` and the tool's own button sit at the left edge — the tool owns the bar — and its controls centre in the remaining space. Known tools get stable assignments (taps → blue, dictate → amber, account → teal); a tool this feature has never met picks deterministically from the remaining palette by name, so new tools arrive coloured without touching this node. The back chevron and tool-contributed controls (record/stop) keep the base monochrome look — colour marks *tools*, not controls. Untick to restore the white-on-grey discipline exactly.

## user

Your tool buttons are coloured now — each tool keeps its own colour, and the open one glows brighter. The row sits centred along the bottom. If you preferred the grey look, this feature can simply be turned off.

## glossary

- **ember**: the redshift-safe palette family this experiment draws from (github.com/carpdiem/ember); "first palette" = 3400K Dark, categorical set.

## code description

`ember.rs` redefines the `tool_colour` seam (created on `/tools` for this node, base: no colour): a fixed map for the three known tools, and for unknown ids a byte-sum pick from the six categorical colours — deterministic per name, so a tool's colour never flickers between builds.

`ember.css` does the look by cascade: `justify-content: center` on the toolbar; `.tool-button.tinted` backgrounds from the `--tool-colour` custom property the seam emits; icons filtered to black in both states; `.sel.tinted` brightened via `color-mix` with white (the "lit" state — same hue, more light). Controls and the chevron match no rule and keep the base discipline.
