# engineer — where engineer-level UI is allowed to live

*You are adding something a developer needs to see — versions, cache
state, timings, a diagnostic readout. This instruction toggles with the
`/engineer` node.*

The user surface is for the user. Anything engineer-level appears in
exactly one place: the engineer section on the nøøb sheet, which opens
only when the gear is tapped and starts folded on every visit. Never on
the home screen, never in a tool, never on the sheet's own face, never
in a toast or a corner stamp (ash, 2026-09-02, with the self-check ask).

To put something there, extend `feature_Engineer.fill(box)` at load —
capture the current function, replace the property, call the captured
one first, then append your block. Plain text; monospace is fine. If
your content changes while the section is open, call
`feature_Engineer.refresh()`. Do not add a second gear, a second
section, or a control outside the section that reveals it.
