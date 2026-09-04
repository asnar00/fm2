# role-in-the-tag
*a person's tag says what they are on this project, not that they are a person*

> (asks#1788558038133)
> On user profiles, instead of saying "profile", show the role(rank) in the lozenge
> *(filed from the field on 2026-09-04 by ash)*

## user

Open someone's card and the little tag in its corner says **candidate**, or
**team**, or **admin** — where they stand in the project you are working in.
Your own card says yours. Someone who is not on this project, or a card you
open with no project chosen, still says **profile**, because there is no role
to show.

## spec

`/tag` puts a card's `type` in its corner. On a person that word was
**profile**, which told the reader something they could already see — they are
looking at a person — while the fact that actually matters, the one that
decides who sees which post, was nowhere on the card. Ash asked for the role
instead.

**The word is the grade itself**, not `/plain-words`' sentences. Those are
written to finish "visible to …" — *"the project's admins only"*, *"everyone
in the project"* — and read as prose; a tag is a label, and the labels are
`/audience`'s own six words: admin, candidate, team, volunteer, supporter,
public.

**Asked, not re-derived.** `audience_grade_in(proj, name)` is the one place
that answers "where does this person stand in this project", including the two
cases that are easy to get wrong on your own: the project's owner is `admin` by
being the owner, with no role link to themselves, and a role link carrying no
grade is the default, `team`, which is what makes every role written before
`/audience` existed a team role. This node asks that question and shows the
answer.

**The current project decides.** `current_project_card()` is the project you
are working in, so with two projects the tag reads the role in the one you
chose, and with none chosen there is nothing to ask about and the word is
`profile` again.

**`/tag` gained a seam for this.** It had none: the word was `c["type"]`
inline. `card_tag_word(card)` is now the /extension point/ for which word the
tag shows, defaulting to the type, so `/tag` alone draws exactly what it always
drew. The **colour** is deliberately left keyed to the type rather than the
word: the word varies and the kind does not, so every person's tag stays one
colour and reads as the same kind of card differently labelled, rather than six
new colours arriving unasked (`/taste` 3 — a colour is a word, and the word
here is still "person").

**Where the word appears.** One place: `/tag`'s tag on the card page. The reel
lozenge and the band show the author and the time (`/reel`'s `.reel-meta`) and
have never carried a kind word, and the grid and list that did are unreachable
under `/map-only`. So the ask's "wherever it says profile" is one function, and
this node changes it there.

## hostile cases

- **A role link with no grade.** `audience_grade_in` answers `team`, the
  default, which is `/audience`'s own reading and not this node's guess.
- **Two projects.** The current one decides; switching project changes the word
  on the next paint.
- **No project chosen.** `profile`, as before.
- **A person on another project but not this one.** `audience_grade_in` answers
  empty, so `profile` — they are still a person, and inventing a role they do
  not hold here would be a lie.
- **Your own card.** The same rule, which on a project you own reads `admin`.
- **A card that is not a profile.** Handed straight down the chain; a post
  still says `post`.
- **`/current-project` or `/audience` unticked.** This node calls their
  functions by name, so it links only with them — the three untick together.
  Named rather than guarded: Rust has no `typeof`, and a node may extend any
  chain that existed when it was written.
- **A grade word with markup in it** (a hand-made op): `/tag` escapes the word
  on the way out, as it always did.

## parked

- The role on the reel lozenge as well. Nothing there says what a card is
  today, so putting a role there is a new surface rather than a rewording, and
  the ask said "instead of saying profile".

## glossary

(no new terms — **grade** is `/audience`'s)

## code description

`role-in-the-tag.rs` redefines `/tag`'s `card_tag_word(card)`: a card that is
not a profile goes down the chain untouched; a profile with a current project
and a role in it wears that role; anything else — no project chosen, no role
there — goes down the chain and keeps `profile`.
