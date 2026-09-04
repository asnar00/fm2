# own-role
*the publish level lists the six roles and lights the one you hold — no "same as me", and nothing anywhere calls it a rank*

> (transcripts/2026-09-04-field-walk.md#p109)
> on the "publish level" option, remove "same as me" from the option - but set the default to the user's own level. Don't refer to "rank" anywhere, instead refer to "role"

## user

Open **publish level** and there are six rows, not seven. The one you hold in
the project you are working in is already lit — a team member sees **team**
marked without having chosen anything. Record and stop, and the post is filed
at that level, exactly as it was before.

Pick another row and that is what your posts are filed at from then on, as
before. There is no *same as me* entry: the row that would have meant it is
the one already lit.

With no project selected, no row is lit and a post is filed with no level, as
it always was.

And nowhere does the app say **rank** any more. It says **role**.

## spec

**"same as me" was a name for something already in the list.** It resolved to
the author's own grade, so the list carried a seventh entry meaning one of its
own six. Ash asked for it gone and for the default to *be* the user's own level
(#p109).

**The floor logic does not move.** An unset `post_level` still stamps the
author's own grade — `/armed`'s `audience_new_floor` is untouched, and that is
what "same as me" always did. What changes is only that the list now shows
which of the six that is. A device left holding the old empty value therefore
needs no migration: it reads as "own role" because it always meant that.

**Which role, read the way the floor is read.** `own_role_mine` asks
`audience_grade_in` off the selected project's card, which is the same function
`card_new` uses to decide the floor — so the row that is lit is exactly the
floor an unset choice would stamp. It cannot drift from it, because it is the
same answer.

**"My card" is `/exchange`'s question, not this node's.** The owner name comes
from `card_of_type(cards, "", "profile")`, which `/exchange` redefined to skip
copies. Re-testing `from` here would be a fourth copy of a rule that has
already been got wrong once (misses.md, "the first profile card": a member
holding copies was sent to somebody else's).

**Nothing lit is a real answer.** No project, no project card, no role on it,
no profile card — every one answers empty, and an empty answer lights no row.
That is honest: with no project `card_new` returns before the floor line, so
there is no level, and a list claiming one would be lying.

**`armed_level_row` is redefined whole**, not wrapped, because every line of it
changes: one entry leaves and the lit test stops asking "is nothing chosen" and
starts asking "is this the one you would get". It still builds its rows through
`armed_pill`, so `/explained`'s sentences and `/plain-words`' wording are all
still on them.

**The word.** "Rank" is gone from everything a person can read. This is a
sweep, and it is listed rather than described:

| where | was | is |
| --- | --- | --- |
| `/armed`'s tool-word for publish level | "Who your next posts reach. Same as me, or any rank at or below your own." | "Who your next posts reach. Your own role, or a wider one." |
| `/tool-words`, `invite_qr` | "…pick the rank, show it…" | "…pick the role, show it…" |
| `/tool-words`, `invite_name` | "Type a name and number and pick a rank;…" | "Type a name and number and pick a role;…" |
| `/doors`, the invite refusal | "that isn't a rank" | "that isn't a role" |
| `/plain-words`, the *same as me* sentence | "your own rank" | "your own role" |
| `/explained`, its own *same as me* sentence | "your own rank, and up" | "your own role" |

The last two are for a row this node removes and are unreachable while it is
composed; they are changed anyway, because the ask is about the word and a
string nobody can reach today is one somebody reaches when a node is unticked.

Code identifiers keep their names — `audience_rank`, `asked_rank`, the `rank`
field on the wire, `.door-rank`, `.qr-rank`. Renaming those would touch the
invite protocol and the guest list for a word nobody reads. `/audience`'s own
glossary still calls the *number* a rank, which is what it is: role is what a
person holds, rank is where that role sits in the order.

**Parked, and named** (`/anticipation`): the lit row shown on the sliders
button itself, so the row says where the next post is going without opening
anything; and "your role in *<project>*" as the row's own note, which needs
`/current-project`'s title.

## hostile cases

- **A device holding the old `""`** (what "same as me" wrote). Read as the
  role you hold and lit accordingly; no migration, because the stored value
  always meant this.
- **A device holding a real grade.** Lit as chosen, exactly as before. The
  clamp in `/armed` is untouched, so a choice above your own role still files
  at your own.
- **No project selected.** No row lit, no floor stamped — `card_new`'s own
  behaviour, unchanged.
- **In a project you hold but are not on** (no role link for you). Empty, so
  no row is lit — and `card_new` also stamps nothing in that case, so the list
  and the floor agree.
- **Your own role changes** (somebody re-grades you). The lit row follows on
  the next paint, because it is read and not remembered.
- **A copy of somebody else's profile heading your card list.**
  `card_of_type` skips copies, so the name is yours.
- **`/audience` or `/current-project` unticked.** This node does not compose
  without them: it asks both for the role. That is a dependency it takes on
  knowingly and `/audience` already cannot be unticked (`/invited-into`,
  `/doors` and `/ranked` call its ladder), so it adds no new edge in practice.
- **`/explained` or `/plain-words` unticked.** The six rows draw with whatever
  wording is composed; this node changes which rows exist and which is lit,
  not what they say.
- **This node unticked.** Seven rows again with *same as me* at the head, lit
  when nothing is chosen — `/armed` and `/explained` exactly as they read
  today. The wording sweep does not come back with it: those are six other
  nodes' own strings, changed permanently in the same commit.

## glossary

- **role**: what a person holds in a project — admin, candidate, team,
  volunteer, supporter, public. The word every surface uses now. Its position
  in the order is still a *rank*, which is a number and is never shown.

## code description

`own-role.rs` — `armed_level_row` draws the six roles and lights the one an
unset choice would file at; `own_role_mine` reads that role off the selected
project's card, through `/exchange`'s own-card lookup and `/audience`'s
`audience_grade_in`.
