# announced
*what the builder is building is on everyone's sheet*

> (transcripts/2026-08-26-session.md#p150a)
> when we're building something that's been requested from claude code, we should add a "building" feature to the "upcoming features" list, so users can see what's being worked on

## user

Open the nøøb sheet: under "building" are the things being worked on right now — including the ones asked for in conversation with the builder, not through the app.

## spec

`/being-built` lists the user's own asks that are under way, from their `asks` var. An ask made in conversation with the builder had no record anywhere, so nothing on any sheet said it was being built (#p150a). One reading, so it builds: a `builds` var in the global scope — a list of `{t, text, status, build?}` — that the builder writes from the repo with `tools/stamp_ask.py --announce TEXT --status building|shipped [--build N]`, matching a shipping call to its building one by the words. Its `building` entries join `/being-built`'s rows in every world, newest first, and leave the list when shipped (the release list already carries shipped work). The agent instruction beside this node makes announcing part of the build flow. Untick and the sheet shows only the app's own asks again.

## hostile cases

- The same words announced twice: one entry, updated.
- A user's own app ask that the builder also announced: the own row wins; the announced copy is not repeated (by `t`).
- Shipped without a prior announcement: the entry is made shipped, and never shows.

## glossary

- `/announced` — a build the builder has put on everyone's sheet.

## code description

`announced.vars` — `builds`, global, last-write.

`announced.index.js` — `feature_BeingBuilt.building` extended with the global list's building entries.

`announced.agent.md` — the two calls, at build start and at ship.
