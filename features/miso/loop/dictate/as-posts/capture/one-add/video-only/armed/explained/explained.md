# explained
*the publish levels stand in a column, each with a line saying who sees a post at it*

> (transcripts/2026-09-04-field-walk.md#p30)
> for the publish level, let's make the list of options vertical, and explain each publish-level in a short sentence.

## user

Open the publish level and the seven entries are a column now, one to a line,
each with the level's name and a quiet line under it saying who a post at that
level reaches:

- **same as me** — your own rank, and up
- **admin** — the project's admins only
- **candidate** — candidates and up
- **team** — the team and up
- **volunteer** — volunteers and up
- **supporter** — supporters and up
- **public** — everyone in the project

The one you have chosen is marked as it was.

## spec

`/armed` drew the levels as pills that wrap: seven words over three lines, and
nothing to say what any of them costs. Ash asked for a column with a sentence
each (#p30). One reading, so it builds.

**The column is CSS and the sentence is code.** `.armed-list` becomes a
column stretched to one left edge, and `.armed-pill` stops being a full-round
lozenge and becomes a 12px row — `/taste` 4's radius for a card-shaped thing,
which is what each entry now is, and `/taste` 6's list grammar: the thing
itself bold, the description one line down, no headers.

**The sentence is spliced into the element `/armed` drew**, not drawn instead
of it. `armed_pill` is redefined to call `existing` and put one more span
inside what comes back, so the lit class, the `data-ev` and anything a later
sibling adds to that element all survive. A node redefines the narrowest thing
that will do (misses.md, "siblings at one anchor").

**The words are `/audience`'s own words for the same fact.** A floor is the
lowest rank that holds a post, so every level but the last reads
"*<them>* and up" — which is the sentence `/audience` already writes under a
post ("visible to the team and up"). Two surfaces saying one fact must say it
the same way (learned 9). *public* is the exception `/audience` also makes:
"everyone in the project". And *same as me* says what it resolves to — "your
own rank, and up" — rather than restating that it is the default, which the
lit mark already says.

**The table is held here**, not asked of `/audience`, for the reason `/armed`
holds its own list of the six: this half must not fall over when `/audience`
is not composed. The cost is the one `/armed` already named — the tables have
to agree — and it is one more copy of a ruling that changes by ask.

**The level is read off the event name.** `/armed` put it there:
`armed_lvl_<word>`, and `armed_lvl_` with nothing after it is *same as me*. No
new argument is threaded through, and a word this node has no sentence for
gets none rather than an empty line.

**Parked, and named** (`/anticipation`): the sentence naming the project by
name ("the Sevenoaks team and up") — it needs the selected project's title,
which `/current-project` has and this node does not ask for; a count ("reaches
11 people"), which `/audience`'s `audience_people_of` could answer; and the
same column shape for `/audience`'s own grade pills in the invite sheet, which
would be that node's ask, not this one's.

## hostile cases

- **A level with no sentence.** `armed_says` returns empty and the row is the
  name alone — what `/armed` drew. A word from a future ask lands here, not in
  a broken line.
- **An event that is not a level.** Anything not beginning `armed_lvl_` gets no
  sentence: this redefinition is on `armed_pill`, which only that list uses,
  and the test is on the prefix rather than on the caller.
- **`/audience` unticked.** Nothing changes here: the words are this node's
  own, and the list is `/armed`'s.
- **The list drawn somewhere other than the level page.** The CSS is written
  against `.armed-list` and `.armed-pill` with no page in front of them, so a
  sibling that moves the list into a popover gets the column for nothing.
- **This node unticked.** The pills wrap as they do today, with no sentences.

## glossary

(no new terms)

## code description

`explained.rs` — `armed_pill` calls `existing` and splices one `.armed-says`
span inside the element that comes back; `armed_says` is the table, keyed by
the level word read off the event name.

`explained.css` — `.armed-list` becomes a stretched column, `.armed-pill` a
12px row with its name in bold, and `.armed-says` the quiet line under it.
