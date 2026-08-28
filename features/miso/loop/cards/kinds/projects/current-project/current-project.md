# current-project
*pick a project, and people and posts narrow to it*

> (asks#1787917567788)
> add "select this project" button to project toolbar - when pressed, only people and posts related to the project are displayed.
> *(filed from the field on 2026-08-28 by ash, from the projects tool)*

## user

Open a project and the control row has a **select** button — a ring. Tap it
and the ring fills: this is now the project you are working in. The people
list shows only the people in it (and you); the posts — grid, list and map —
show only posts by those people, or posts filed in the project. Every list's
picker grows a chip saying which project you are in; tap the chip, or the
filled ring, and you are back to everything.

The choice is yours, not the phone's: pick a project on your phone and your
laptop is in it too.

## spec

**One per-user var, `current`** — the id of the chosen project card, empty
for none. User-scoped (`/per-user`): it travels with you and is undoable
like every other var write. It is the "current project as a per-user var
feeding the contexts machinery" that `/projects` parked.

**Related means: by a member, or filed in it.** `current_project_related`
is the one test. A card is related to the current project if its owner is
the project's owner or holds a role in it (`projects_members` +
`projects_link_name`, the same reading `/projects` uses everywhere), or if
the card carries a link `{kind:"in", to:<project id>}` — the shape `/cards`
reserved for a post in a project. Nothing writes an `in` link yet; honouring
it now is `/anticipation`'s move, so the day a post is filed in a project
the filter already knows.

**Two sifts, no new surface.** `posts_set` is redefined so the posts tool,
its list and the map (which draws whatever set it is handed) narrow
together; `browse_cards` is redefined for the people list only — with the
account tool open and a project chosen, profiles not related to it are
dropped, **except your own**, which stays because a list of people that does
not contain you is a list of somebody else's people. Every other tool's
`browse_cards` — projects included — passes through untouched: you must be
able to see the other projects to switch.

**A project you were merely given still counts.** The select button is drawn
for any project on screen, own or copy: being in a project is what makes it
yours to work in, and a member is exactly who wants to narrow to it.

**The chip is the honest sign.** A filter with no sign is a trap ("where did
everyone go?"). `browse_picker_html` — the pill every card surface draws —
grows one chip naming the project, in `#9db7d8`, *chosen* (/taste 3); it is
the same word the filled ring says. Tapping either sends `proj_select`, which
toggles: choosing what is chosen unchooses it. A current project that is no
longer held, or is a tombstone (`/delete-project`), counts as none — the
chip is not drawn and nothing is filtered, so a deleted project cannot hide
the world.

## hostile cases

- **The current project is deleted, or its copy withdrawn.** Treated as
  none: no chip, no filter. The var keeps the id; choosing another replaces it.
- **A project with nobody in it.** People narrows to you; posts to yours.
  The chip says why.
- **Two devices, two choices.** Last write wins, as every user var does; the
  chip on each shows the outcome.
- **`/delete-project` unticked.** `delete_gone` is `/delete`'s, which is
  `/posts`', and posts are what this node filters — the dependency is the
  ask's own.
- **The chip on the projects surface itself.** Drawn there too, unfiltered
  set beneath it: a consistent sign beats a special case.

## parked, and named

Filing a post in the current project (writing the `in` link when a post is
made with a project chosen) is one node on `/kinds/new`'s write; the filter
is already waiting for it. A project's page listing its posts is
`/projects`' own parked `card_page_html` splice, and would read the same link.

## code description

`current-project.vars` declares `current` (user, last-write).

`current-project.rs` reads and writes it (`current_project_read`/`_write`),
resolves it to a held, undeleted project card (`current_project_card`), and
answers `current_project_related(card)` — owner among the members, or an `in`
link to it.

`current-project.rs` redefines `posts_set` and, for the account tool,
`browse_cards`, sifting by `current_project_related` while a project is
chosen; extends `tool_controls` with the ring on an open project's page
(before `/undo`, through `projects_before_undo`; filled and tinted when that
project is the chosen one); extends `browse_picker_html` with the chip; and
extends `update` with the `proj_select` click, a toggle.

`current-project.css` is the chip and the ring's two states.
