# last-row
*the invite list ends without a line under it*

> (asks#1787667686547)
> we don't need the horizontal line at the bottom of the card content
> *(filed from the field on 2026-08-25 by ash, birthplace `👤 @ miso/shell/panel/account`)*

## user

The last row of your invite list has no line under it; the list simply ends.

## spec

`/invite` borrows the `.crow` list grammar, whose every row carries a bottom rule — so the list ended with a line under nothing, at the bottom of the card and of the invite tool's page. Ash asked for it gone. One reading, so it builds: the last `.crow` inside `.invite` drops its rule. Untick and the line returns.

## glossary

(no new terms)

## code description

`last-row.css` — one rule on `.invite .crow:last-child`.
