# own-notes
*the report writer is told what the posts are: the team's own notes, never recordings of the public*

> (transcripts/2026-09-03-housekeeping.md#p14a)
> somewhere in your memory there is a misapprehension that we're going to be capturing "doorstep content" - this isn't true, and in fact will get us in trouble, because recording the public is (currently) considered a no-no. So the intent of the app is just to let team members post anything they want (usually notes or impressions after the fact). I figured this out because the report makes mention of "doorstep content".

## user

A report describes what it read truthfully: notes and impressions the team wrote or dictated themselves, usually after the fact. It never calls them doorstep recordings, because they are not.

## spec

`/reports` told its writer the data was "doorstep posts written or dictated by canvassers" and headed the corpus "DOORSTEP POSTS" — a builder's misapprehension that surfaced in a report (#p14a). Ash's ruling: the app lets team members post anything they want, usually notes or impressions after the fact; recording the public is a no-no and would get the campaign in trouble. One reading, so it builds. `/reports` already made its writer's instructions a seam (`reports_system`); this node's first act opens the second — the corpus heading becomes `reports_corpus_heading`, the parent's behaviour unchanged — and then redefines both: the instructions say the posts are the team's own notes and impressions, written or dictated by team members, and that none of it is a recording of a member of the public; the heading reads "THE TEAM'S POSTS, NEWEST FIRST". Reports already written keep their wording until they are run again. Untick and the old words return.

## glossary

(no new terms)

## code description

`own-notes.rs` — redefines `reports_system` (the writer's instructions, with the data described truthfully) and `reports_corpus_heading`.
