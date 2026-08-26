# build-below
*the build row sits below what is pending*

> (asks#1787704394017)
> Move the "build xxx up to date [features] to below the pending / update section
> *(filed from the field on 2026-08-26 by ash)*

## user

In the system panel, your requests and the update row come first; the "build … up to date" line with the features button sits beneath them.

## spec

`/less-busy` ordered the panel ask, build line, requests, update, policy…; `/build-row` folded the features button into the build line. Ash asked for the build row below the pending and update section (`asks#1787704394017`). One reading, so it builds: at load, after `/build-row` has made `#buildRow`, this node moves it after the update row — or after the requests list when the update row is tucked away. Untick and the row is back under the ask box.

## glossary

(no new terms)

## code description

`build-below.index.js` — one DOM move at load.
