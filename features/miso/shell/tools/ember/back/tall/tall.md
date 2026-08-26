# tall
*‹ is the row's height, and half its width*

> (transcripts/2026-08-26-session.md#p152a)
> we could also make the < button the same height as the other ones to make hitting it easier

> (transcripts/2026-08-26-session.md#p153)
> seems to work now. let's halve the width of the "back" button but keep it the same height.

> (transcripts/2026-08-26-session.md#p152)
> the reason is that the map / background temporarily captures the press events - I tried something where I pressed down on the back button and then dragged, and the map moved.

## user

The ‹ at the left of an open tool's row is as tall as the buttons beside it and half as wide, and a thumb hits it first time.

## spec

‹ was 24 px wide (`/tools`), then 30 (`/bigger-buttons`), and only as tall as its glyph — a 32 × 32 target in the bottom corner, while every other button in the row is 50 × 50. Ash found presses on it going to the map instead (#p152): in map view the ground under the row is a Leaflet surface, so a press that misses the small target by a few pixels starts a map drag rather than a tap. The rig confirmed the sizes (‹ 32 × 32; the others 50 × 50) and no repaint churn that could explain it otherwise. Ash's reading (#p152a) is the fix: one rule gives ‹ the row's own height, at full opacity like the rest. At 50 × 50 it worked (#p153), and the width came back down to half — 25 × 50: a door beside the tools, not another tool. Untick and ‹ is the small glyph again.

## hostile cases

- `/bigger-buttons` unticked: this rule still says 50 × 50 — ‹ would then be larger than a 40 px row; acceptable, and stated.
- The glyph: `/arrow`'s 22 px `.icon-svg` centres in the larger box unchanged.

## glossary

(no new terms)

## code description

`tall.css` — one rule on `.toolbar .tool-button.back`.
