# plain-words
*"visible to candidates", not "visible to candidates and up" — one table for the line under a post and the level list alike*

> (asks#1788532427384)
> "Visible to candidates and up" is confusing: just say "visible to candidates" (or whatever)
> *(filed from the field on 2026-09-04 by ash, birthplace `posts @ miso/loop/cards/kinds/audience`)*

## user

Under your own post the line now reads **visible to candidates** — or *visible
to the team*, *visible to volunteers*, *visible to supporters*, *visible to
everyone in the project*, or *visible to the project's admins only*.

The publish-level list in the recording row says the same things, in the same
words: *candidates*, *the team*, *volunteers*, *supporters*, *everyone in the
project*, *the project's admins only*, and *your own rank* for **same as me**.

Nothing about who actually gets a post has changed. Only the words.

## spec

`/audience` wrote the line as "visible to *<them>* and up", and `/explained`
took the same phrasing into the level column because two surfaces saying one
fact should say it the same way. Ash read it in the field and said it is
confusing (`asks#1788532427384`): the reader has to reconstruct a ladder
before they can read a sentence. "and up" goes.

**One table, both surfaces.** This is the point of the node. `/audience` builds
the line in `audience_line`; `/explained` builds the column's sentence in
`armed_says`. Both are redefined here from one `plain_words_of`, so the words
cannot drift apart — and there is no new copy of them anywhere. (The *ranks*
are a separate thing and are still held in three places, which `/armed` named
as its own cost; this node adds none.)

**Placement.** A child of `/audience`. The line ash read is `/audience`'s, the
ladder the words describe is `/audience`'s, and the ask was filed from the
posts surface. `/explained`'s sentence is the same fact said on another
surface, and is reached from here because this node is newer than it —
causality lets a node redefine anything that existed when it was written.

**`admin` keeps its own sentence.** "visible to admins" would read as "any
admin anywhere", and `/audience`'s two ladders collide on exactly that word: an
app `admin` and a project's admin are different people. So the project's is
named — "visible to the project's admins only" — which is also what the column
already said and what ash asked to keep.

**`same as me` says what it resolves to**, "your own rank", rather than
restating that it is the default; the lit mark already says that.

**What did not change.** The ranks, the order, `audience_words`, the floor, who
receives a post, promote. `audience_words` is left exactly where it is and is
still what the base `audience_line` uses — this node does not touch it, so
unticking restores the old sentence with nothing else moved.

**Parked, and named** (`/anticipation`): naming the project in the line
("visible to the Sevenoaks team"), which needs `/current-project`'s title; and
a count of who that actually is, which `audience_people_of` could answer.

## hostile cases

- **A grade with no words** — a hand-made op, or a seventh rung added by a
  later ask before this table knows it. `plain_words_of` answers empty and
  `audience_line` falls through to `existing`, so the reader gets the old
  sentence rather than a broken one. `armed_says` answers empty and the column
  row is its name alone, which is `/armed`'s own fallback.
- **`/explained` unticked, or not composed at all.** `armed_says` is a
  function nobody calls — no `existing` call is made, so nothing is missing —
  and the line under a post still loses its "and up".
- **`/armed` unticked.** The same: only the byline is in play.
- **The promote button's own line.** `/audience` draws the *next* rung's
  sentence on the arrow through the same `audience_line`, so it loses "and up"
  with everything else and the two lines still agree.
- **This node unticked.** "visible to *<them>* and up" and the column's "and
  up" both come back, together, from their own nodes.

## glossary

(no new terms)

## code description

`plain-words.rs` — `plain_words_of` is the table: who a post at a grade
reaches, in one phrase.

`plain-words.rs` — `audience_line` frames that phrase as "visible to …" for
the line under a post, falling through to `existing` for a word the table does
not know.

`plain-words.rs` — `armed_says` answers `/explained`'s seam with the same
phrase for the publish-level column, and "your own rank" for *same as me*.
