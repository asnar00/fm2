# current
*the post the map moved to is marked: a light outline and an arrow up to the map*

> (transcripts/2026-09-03-housekeeping.md#p19)
> highlight the post we moved the map to (a light-grey outline/arrow)

## user

The lozenge the map is showing wears a light grey outline, with a small arrow on its top edge pointing up at the map.

## spec

`/reel` settles on the lozenge at the left edge and pans to it, but nothing said which one that was. Ash asked for a light-grey outline and an arrow (#p19). One reading, so it builds: after every settle the current lozenge is marked `reel-current` and the others unmarked; the mark is a 1.5 px outline in the app's light grey and a small triangle drawn on the top edge in the same grey, pointing up. At a fresh render the first lozenge is current. Untick and no lozenge is marked.

## glossary

(no new terms)

## code description

`current.js` — wraps `feature_Reel.follow` and `feature_Reel.render` to mark the current lozenge.

`current.css` — the outline and the arrow.
