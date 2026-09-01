# projects
*something we're trying to get done, and who is doing what in it*

> (transcripts/2026-08-25-accounts.md#p87)
> my hitlist before tara comes in: 1) projects; 2) posts; 3) map view.

> (transcripts/2026-08-25-accounts.md#p7, the design)
> a "project" is "something we're trying to get done" — it could be as simple as "book a trip to China" or as complex as "build a team to campaign for MP in 2029"; I want to have "person P has role R in project X" stored in a database, and interrogable. So on my page it would say "lead developer for [app]", and connect to Tara who would be "candidate for [sevenoaks 2029]".

> (transcripts/2026-08-25-accounts.md#p14, the seed)
> note that "miso" is a project (this project) and I'm "lead dev" of miso.

## user

**projects** is a tool with a flag on it. It shows the projects you are in —
your own and the ones you have been given a part in — as a grid or a list, with
the same pill top-left that 👤 has. **new** makes one: title it, say what you
are trying to get done.

Under a project you own there is a **people** section. **add** shows the people
whose cards you hold; pick one, type what they do — "canvasser" — and they are
in. Their phone gets the project the moment you add them, and every time you
change it. On anyone's card page, under their mission, is the line **canvasser
for miso** for every project you both hold — tap it and the project opens. Add
yourself and your own card reads **lead dev for miso**.

Taking somebody's role away removes them from the project you hold. The copy
already on their phone stays there — withdrawing a card is a thing miso cannot
do yet.

## spec

A **project is a card of type `project`** (#p10's "a type is a word"). The
object is untouched: title, picture, text. `/new`'s `CardNew {type:"project"}`
is the only way one is made.

**A role is a link on the project card**, not on the person: `links: [{kind:
"role", to: <the person's profile card id>, name: <their name>, role:
"canvasser", t}]`. The project's owner is the only one who writes them, because
a link is a block of their own card. "person P has role R in project X" is then
a query over the project cards you hold, and `/cards`' `links[]` — declared
empty on the first card ever written so this would not be a migration — is what
it is for.

`to` is a card id and `name` is the word to draw. Both, because they answer
different questions: the id is what a query joins on, the name is what renders
when the holder has never held that profile card (a role added by somebody whose
world is not yours). Nothing here looks a person up in order to draw a row.

**A profile card page grows a role line per project.** `card_page_html` is
extended the way `/tag` and `/location` extend it — splice into the page
`existing` returns — and for a card of type `profile` it appends one `.crow` per
project card **in the reader's own world** that links to that profile: the role
word where the number sits, "for miso" beside it. The query is over what you
hold, which is the whole shape of #p8: you see the roles you were told about.

**Members are handed the project.** `/exchange`'s door is `exchange_give`, and
this node knocks on it from a `route` link of its own — outside `/exchange`'s,
because provenance puts #p87 after #p71 — watching the same `POST /msg` cards
op. A project card of yours whose `edited` moved goes to every person named in
its role links, copied through `exchange_copy` so it carries `from` and is
read-only on arrival. Adding a role stamps `edited`, so the add and the edit are
one path and not two.

**And only to them.** `/exchange`'s rung one hands every card you own to
everybody your invite links reach; a project is not a profile and must not
travel that way. So this node redefines `exchange_give` and drops any card of
type `project` bound for a world with no role in it. Every other card is passed
through untouched, and the same filter covers the login seed. This is a chain
extension of `/exchange`, not an edit of it — and it is the reason a project you
make is not on your invitees' phones until you put them in it.

**The surface is `/browse`'s**, through the two seams it left. `browse_cards`
returns the project cards while the projects tool is open and `existing`'s
answer otherwise, so `/people`'s own filter is untouched. `browse_row_left` says
the member count on a card of type `project` — "3 people" where the number goes
(/taste 6) — and defers for everything else. The picker, the tile, the grid, the
list, the two device vars and the card page are all `/browse`'s, unchanged.

**The people sheet is the page half's**, built at load and living outside `#app`
like `/frame`'s and `#cardToast` — a repaint while it is open cannot take it
away. It reads the profile cards out of the bridged `s.cards`, so it needs no
fetch and no state of its own; the pick and the role word are JS-local until
**add** sends `RoleAdd`.

**Dependencies, stated rather than discovered.** This node extends `/browse`
(the seams, the picker, the set, `browse_open`), `/cards` (`card_page_html`,
`card_esc`, `cards_read`/`cards_write`), `/new` (`CardNew`) and `/exchange`
(`exchange_give`, `exchange_copy`, `exchange_cards_of`, `exchange_name_of`). It
composes only with those four ticked; unticking `/exchange` while this is on is
a link error, not a degraded surface, and that is the honest report.

**One node, no children.** `roles` and `members` were offered as a split; they
are one prompt's worth of one thing — a link, the people who are in it, and the
copy that follows from being in it — and splitting them would put the writer of
a link in one node and the reader of it in another.

## hostile cases

- **The same person added twice.** The new role **replaces** the old one, so
  re-adding with a different word is how you change what somebody does. Refusing
  would leave no way to fix a typo.
- **A role link to a profile id the holder does not have.** The row renders from
  the link's `name`; nothing is looked up, so nothing is missing.
- **An empty role word, or nobody picked.** `add` stays dim and sends nothing;
  the server refuses a `RoleAdd` with an empty `role` or `to` as well, because
  the wire is not the page's to trust.
- **A role written onto a card you do not own.** Refused: `RoleAdd` only touches
  a card with no `from` on it, which is `/exchange`'s own test for "you wrote
  this".
- **A member with no entry on the guest list** (a name that is not a user):
  there is no world to hand to, and the link still renders. A role is a claim
  about a person, not a subscription.
- **A project with no people yet.** The owner sees the section with `add` in it;
  everyone else sees nothing at all rather than a heading over an empty box.
- **The tool opened with no projects.** "no projects yet", one line, no ground.

## parked, and named

Each is a node extending a chain this one leaves, not a change to it:
`card_page_html` is where "the posts in this project" splices in, `projects_hand`
is where another type of card learns to travel to a project's members, and
`people_order` is where membership becomes a proximity cue.

Withdrawal — taking a copy back off a phone — is `/exchange` stage two and is
not here; removing a role removes them from the project *you* hold and stops the
updates, and their stale copy stays. Shared project membership as the second
visibility cue (#p71's "later") joins at `people_order`, which `/people` left as
a chain for exactly this. "Current project" as a per-user var feeding the
contexts machinery, and a post carrying a `project` link, are both extensions of
the link shape rather than changes to it.

## glossary

- **project**: a card of type `project` — something a group of people is trying
  to get done.
- **role**: a link of kind `role` on a project card, naming one person and what
  they do in it.
- **member**: a person a project card has a role link to; the set of people the
  card is handed to.

## code description

`projects.rs` extends `tools_list` with the tool — a drawn flag in
`currentColor` (/glyphs), tinted by `/ember`'s stable pick for the name — and
`tool_controls` with the **new** button, inserted before `/undo`'s, which stays
last in every row. The button wears the projects tool's own colour, which is
`/plus-tinted`'s rule and also avoids `/ember` handing "new" and "undo" the
same blue. It carries no `data-ev`: the page half makes the card.

`projects.rs` takes `/browse`'s two seams. `browse_cards` returns the project
cards while `open_tool` is `projects` and `existing`'s list otherwise;
`browse_row_left` returns the member count for a card of type `project` and
defers otherwise. `render` draws the surface the way `/browse` draws its own —
the picker, then the open card's page or the set — and says "no projects yet"
when there is nothing.

`projects.rs` extends `card_page_html` twice over. For a `project` card it
rewrites the two placeholders (a project is not a person) and appends the
**people** section: one `.crow` per role link, with `✕` on each and an **add**
control for the owner, read-only and header-less for everybody else. Both
controls carry `data-proj` rather than `data-ev` — they are sent by the page
half, because a role event carries a time and the wasm half of the loop has no
clock (`now_ms` is `SystemTime`, which panics in a browser). For a
`profile` card it appends one `.crow` per project in the reader's world that
links to it — "canvasser / for miso" — tapping which sends `proj_open:<id>`.
`projects_roles_from` is the seam those projects come from (every project
card held; refactored out 2026-08-28 so `/delete-project` can sift it).

`projects.rs` extends `update` with `RoleAdd {card, to, name, role, t}` and
`RoleDrop {card, to, t}` — both write the project card's `links` through
`cards_write` and stamp `edited`, so `/guard`, `/exchange` and this node's own
hand-over see an ordinary card write — and with two clicks: `proj_open:<id>`
(open the projects tool on that card, from wherever the tap was),
and the projects tool's own button while a card is open, which goes back to
the set instead of closing the tool (`/people`'s idiom, read before the chain
beneath runs).

`projects.rs` extends `route` with the hand-over. It is the outermost `route`
link, so like `/exchange`'s it runs outside `/edit`'s turn and outside
`/per-user`'s identity, where another world may be named and written. It watches
`POST /msg` for a cards op, diffs the writer's project cards across the inner
chain, and gives each changed one to every world its role links name.
`projects_key_for_name` is the only lookup: a name to a world key, off the guest
list.

`projects.rs` redefines `exchange_give` as a filter: a card of type `project`
reaches a world only if that world has a role in it. `projects_is_member` is the
test, matching a link by the owner prefix of its `to` id and falling back to its
`name`.

`projects.rs` carries two extension points a later node takes (cut for
`/audience`, 2026-09-01, and behaviour-neutral — each returns exactly the
expression it replaced): `projects_role_link(d, to, name, role, now)` builds the
role link a `RoleAdd` writes, and is handed the whole event data so a later node
may take a field of its own off it; `projects_people_role(l)` is the role cell of
a row on the project page — the page's own answer to "what does this person do",
extensible without touching the row. The role lines on a *person's* card are a
different question and read the link themselves.

`projects.js` is the people sheet: furniture made at load outside `#app`, a row
per profile card in the bridged `s.cards`, a role box, **add** and **cancel**.
It also owns the **new** button, which reads the owner's name off their own
profile card (falling back to `auth/whoami`) and sends `CardNew`, and the `✕`,
which sends `RoleDrop`. `roleData()` is the page-half twin of
`projects_role_link`: what the sheet says about the person it has picked, in its
own function so a later node may add a field without rewriting the send.

`projects.css` styles the section, the role lines and the sheet against
`/taste`: the `#161619` ground, the `.crow` grammar, 999px pills, and the one
accent — `#9db7d8`, *chosen* — on the picked person and on **add**.
