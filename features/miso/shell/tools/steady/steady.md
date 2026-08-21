# steady
*the toolbar holds still: the slide plays on mode changes, not on every render*

> (transcripts/2026-08-15-fm-spec.md#p2a)
> I notice in the "taps" tool, whenever I tap the "taps" button, the tool icon at lower left jumps around - it should stay stable.

## user

The tool icon at the lower left sits still while you use the tool —
tapping away at the taps counter no longer makes it twitch. Icons still
slide in when you open a tool or come back out.

## spec

`/tools`'s slide-in animation was written for mode changes, but renders
are whole-DOM swaps, so the toolbar re-mounts — and the slide replays —
on **every** state change: each tap in the taps tool sends its icon
lurching in from the left. The rule this node enforces is the
animation's original intent: the slide belongs to a change of mode
(which tool is open, or none), and a re-render within the same mode
mounts the toolbar buttons exactly where they were, motionless.

## code description

`steady.index.js` wraps `feature_Loop.apply` (the page's one seam for
"the DOM was just swapped"): after the original runs, it compares the
state's `open_tool` with the value it saw last apply; when unchanged, it
stills the freshly-mounted toolbar buttons by setting their inline
`animation: none` — synchronously, before the frame paints, so the
replay never shows. A changed mode leaves the stylesheet's slide to
play. Boot counts as a mode change (nothing seen yet), so the first
appearance still slides.
