# with-live
*your own live pin joins the fan, so it never sits on top of a post you made there*

> (transcripts/2026-09-03-housekeeping.md#p19a)
> the "fan" display should include post and displayed user - right now the user (me) is overlapping the posts I made here.

## user

Standing where you made your posts, your own live pin fans out with them instead of covering them.

## spec

`/fan-out` laid out `/map`'s pins and said the live pins were another hand's and not in the fan; ash found his own live pin sitting on the posts he had just made (#p19a). One reading, so it builds: the layout gathers every marker on the map that has a pin face — `/map`'s and `/live`'s alike — and lays them out together; and because `/live` moves its pins every second, the layout runs again after each of its draws, so a pin that walks into a group joins it and one that walks away leaves it. Untick and the live pin stands on the place alone again, over whatever is there.

## hostile cases

- A live pin alone at its place: a group of one, no turn.
- The live pin moving each second: re-laid each second; a group it stays in keeps its angles (the same marker keeps the same slot order).
- Live off: only `/map`'s pins, as `/fan-out` had it.

## glossary

(no new terms)

## code description

`with-live.js` — redefines `feature_FanOut.layout` to gather every marker layer on the map; wraps `feature_Live.draw` to lay out after it.
