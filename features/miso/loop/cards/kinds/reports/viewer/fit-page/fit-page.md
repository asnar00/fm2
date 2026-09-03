# fit-page
*the report in the sheet is scaled to fit, with its margins*

> (asks#1788448235046)
> PDF report display should zoom out enough to show margins properly
> *(filed from the field on 2026-09-03 by ash)*

## user

Open a report and the whole page fits the phone's width, margins and all — the map inside it too — as it would on paper.

## spec

`/viewer` puts the printed page in the sheet as it was written for paper: an A4 sheet whose margins live in `@page` (which a screen ignores) and a map frame 1120 px wide, so on a phone the words ran to the edges and the map ran off them (asks#1788448235046). One reading, so it builds: when the page has loaded in the frame, the paper's own margins are given to the body as padding, the page's natural width is measured, and the whole document is zoomed down so that width fits the sheet — never up. A window that changes size (a turn of the phone) refits. Untick and the page shows at its natural size again.

## hostile cases

- A narrow page (no map): natural width at or under the sheet's — zoom 1, nothing shrinks.
- The phone turned: refit on resize.
- The frame not yet loaded: the fit waits for its load.

## glossary

(no new terms)

## code description

`fit-page.js` — wraps `feature_Viewer.open` to fit after the frame loads; `fit()` measures and zooms.
