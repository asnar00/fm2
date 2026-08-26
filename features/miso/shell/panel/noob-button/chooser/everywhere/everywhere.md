# everywhere
*a chooser row carries its tick wherever it is drawn*

> (transcripts/2026-08-25-accounts.md#p102)
> when the user makes an ask in the "do something" box, the resulting feature list should show the enable/disable tick so the user can turn it off/on - and eventually even change the settings

## user

Ask for something in the nøøb panel's box; the features it finds come back as rows with their tick, and you can switch one off or on right there.

## spec

`/ask` draws its results with `/chooser`'s rows and then strips the tick — "result rows introduce, they don't configure". Ash asked for the tick (#p102): a found feature is exactly the one you want to switch. One reading, so it builds. `/ask` grew a seam, `feature_Ask.stripTicks(box)`, which this node replaces with a sync: the ticks stay, and each is set from the chooser's own `ticks()` — `on` for the row's path, `shaded` when an ancestor is off — the moment the rows are drawn; `feature_Chooser.reflect` is wrapped so every change of the ticks reaches the result rows too. The tap itself was always the loop's (`ftick_<path>`, the same event the chooser's list sends); `/ask`'s row handler now ignores a tap on the tick, as the chooser's does. Settings on a row are the anticipated next ask — a row's tunables are `.vars` the chooser can already name; parked. Untick and the ticks leave the results again.

## hostile cases

- No results: nothing to sync. `/chooser` unticked: no rows at all (the ask only draws feature rows when the chooser is composed).
- A tick tapped on a shaded row (ancestor off): the tick flips, the row stays shaded — the same truth the chooser shows.

## glossary

(no new terms)

## code description

`everywhere.index.js` — `sync(box)` sets `.on`/`.shaded` on result rows from `feature_Chooser.ticks()`; replaces `feature_Ask.stripTicks` with it at load and wraps `feature_Chooser.reflect` to re-sync `#askResults`.
