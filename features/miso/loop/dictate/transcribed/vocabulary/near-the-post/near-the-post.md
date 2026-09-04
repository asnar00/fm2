# near-the-post
*seed a clip with the streets around where it was made, not the streets around
the campaign*

> (transcripts/2026-09-04-field-walk.md#p154)
> the "bourke and bloor" was taken at a specific GPS location, so we could (if we were being clever) look up the map to figure out what those words were actually likely to be :-) maybe there's a post-transcribe process where we can look over the text and identify words that are "interesting" or seem to be names, and see if we can match them to where we actually are. i'd have thought the seeding with names would have helped?

> (transcripts/2026-09-04-field-walk.md#p155)
> build both now, I think. We'll probably need a manual transcription error fix UI later - we'll build that once we have some real field data.

## user

Record a note anywhere and the street names come back right — in the patch,
where they always did, and outside it, where they did not. A note made in Soho
is seeded with Soho's streets.

## spec

`/vocabulary` took the **thirty nearest** entries of the constituency's list
with no distance test at all, so a post outside the patch was seeded with the
nearest Sevenoaks streets however far away they were. A clip made in Soho was
seeded with streets thirty kilometres off and came back **"Bourke Street …
and Bloor"**; the corner was almost certainly Berwick Street and Broadwick
Street, and nothing nearby had been offered to the recogniser to hear them
with.

**Four hundred metres, from wherever the post is.** Far enough to hold the
streets a person can see from where they are standing, near enough that
nothing across town gets in. The stocked list answers first — it is free and
offline, and inside the patch it is the whole answer. Outside it, one live
Overpass radius pull for that point.

**One pull per cell, not per post.** A walk down one street makes a dozen
posts. The cache key is a cell about five hundred metres square and the pull
is eight hundred metres around its centre, so every point in the cell has its
own four hundred covered by one query.

**An empty seed beats a wrong one.** When Overpass cannot be reached — down,
rate-limited, a dead spot — this answers with nothing at all, and the clip is
seeded with the geocoded address alone, which `/vocabulary` already supplies.
It never falls through to the parent's answer: "the thirty nearest anywhere"
is precisely the wrong list for a post out of area. A missing seed costs a
little accuracy; a wrong seed invents street names, which is the bug being
fixed.

**A miss is remembered for an hour.** Overpass rate-limits, and asking again
for every post of a walk is both rude and useless — the answer will be the
same refusal. An empty patch of countryside is remembered the same way and
re-asked on the same clock.

**No location on the post, no streets.** `/vocabulary` returns the
constituency and the geocoded address before it ever asks for streets, and a
post with no fix stops there, exactly as it did.

## glossary

- **cell**: the rounded coordinate a live pull is cached under, so the posts of
  one walk share a single query.

## code description

`near-the-post.rs` redefines one function of `/vocabulary`'s, `vocab_streets`,
and adds nothing to any other chain.

`near_metres` is a flat-earth distance with a real cosine — exact to well under
a metre at this scale, where the parent's fixed 0.62 factor was only right at
one latitude.

`near_from_list` filters the stocked `streets.json` to the radius;
`near_pick` is the shared sort-and-cap, nearest first, de-duplicated by name,
cut at `/vocabulary`'s own thirty.

`near_from_overpass` is the cache and the back-off: a cached hit is re-picked
against the post's own point, a cached miss under an hour returns nothing, and
anything else asks. `near_ask_overpass` runs the query `tools/streets.py` runs,
as a radius instead of a boundary, and reads a way's `center` or a node's own
coordinates.
