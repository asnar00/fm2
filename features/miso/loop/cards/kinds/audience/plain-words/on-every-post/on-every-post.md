# on-every-post
*the "visible to …" line is under every post you open, not only your own*

> (transcripts/2026-09-04-field-walk.md#p120)
> let's show post visibility on all posts (in expanded view), even for posts not authored by this user

## user

Open somebody else's post and it says who it reaches, in the same words and the
same place your own posts say it: *visible to candidates*, *visible to the
team*, *visible to volunteers*.

There is no button on it. Changing who a post reaches is still the author's,
on their own phone.

## spec

`/audience` drew the line only under your own post, because it arrived beside
the arrow that changes it and the arrow is the author's alone. Ash asked for
the fact on every post (#p120): who a post reaches is worth knowing whoever
wrote it.

**Nothing is fetched and nothing is guessed.** `/exchange` copies the whole
card, so a copy carries its own `floor` — the same field, read the same way,
said in the same words by `/plain-words`. The line is put in the same place,
through `/projects`' `projects_inside`, so a post reads the same whoever opened
it.

**The button is untouched.** `/visibility`'s eye and `/audience`'s arrow before
it both gate on the card having no `from`. A copy has one, so it gets the line
and no control — which is the ask, and also the only safe shape: the floor on
your copy is a fact about what you were handed, not a thing you may set.

**A post filed before `/audience` draws nothing.** Its card carries no `floor`,
and the card's own field is read here rather than `audience_floor_of` — that
reader answers `team` for a card carrying nothing, which is the right default
for deciding who may *hold* a post and a level invented out of nothing if it
were put on the screen as a fact. So an old copy says nothing, which is true.

**Whole redefinition, not a wrapper.** The base's answer for a copy is "no
line" — the thing being replaced — and calling `existing` first would draw the
author's own line twice on the author's own post. The test for whose post it is
is inverted here and the two links never both draw.

**Parked, and named** (`/anticipation`): the line saying who put it in front of
you as well as who it reaches ("from Tara, visible to volunteers"), which
`/exchange` records in `from` and `/byline` already draws separately.

## hostile cases

- **Your own post.** Untouched: this link returns immediately for a card with
  no `from`, and `/audience`'s own line is the one drawn.
- **A copy with no floor** — a post filed before `/audience` existed. Nothing
  drawn, because there is no floor to report and `team` would be an invention.
- **A copy in no project.** Nothing drawn, the same test `/audience` makes: a
  post outside a project has no audience to name.
- **A copy of something that is not a post.** `posts_is` is false and the link
  returns; a profile copy under 👤 is not touched.
- **A floor word that is not one of the six** — a hand-made card. Not a grade,
  so nothing is drawn rather than a sentence built around a word nobody knows.
- **`/plain-words` unticked.** The line is drawn with `/audience`'s own older
  wording, on copies as well as your own; the words are that node's, the reach
  is this one's.
- **This node unticked.** The line is the author's own again, and a copy shows
  nothing — `/audience` exactly as it reads today.

## glossary

(no new terms)

## code description

`on-every-post.rs` — `card_page_html` draws the same `.card-audience` line for
a post that carries `from`, out of the copy's own `floor` field, and leaves
every other card to the chain beneath.
