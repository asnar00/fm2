# top-right
*the logo moves to the top-right corner; the toolbar owns the whole bottom edge*

> (transcripts/2026-08-14-fm-spec-3.md#p49)
> also let's move the logo icon to the top right of the window - that gives the toolbar full control of the button of the screen

## user

The little logo now lives at the top right of the screen — still glowing when an update is waiting. The toolbar stretches across the whole bottom.

## spec

The logo button leaves the bottom edge: it sits at the top-right of the window (safe-area relative, per `/corner`'s rule — chrome is placed from the insets, never the physical edge). The bottom edge belongs entirely to `/tools`: the toolbar's right-hand clearance for the old corner stamp is released, so tool buttons and controls can use the full width. Everything else about the button is unchanged — the update glow still pulses there, and its tap stays reserved for the agent interface (`/account`).

## glossary

(no new terms)

## code description

`top-right.index.css` is two cascade overrides, winning by composition order (this node is newest, so its rules come last at equal specificity): `#build` gets `top` from the safe-area top inset with `bottom: auto` cancelling `/corner`'s pin; `.toolbar`'s right edge drops from the 84px stamp-clearance to the standard 10px margin.
