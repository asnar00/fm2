# audience
*a post belongs to the project you are in, and reaches everyone in it at your rank or above — at once*

> (transcripts/2026-09-01-saturday.md#p15)
> upload/broadcast logic: when a person makes a new post, it should
> immediately appear on the map/list views of other team members at the same
> rank/role. For this to work, we'll need a graded option-list of roles for
> people in the project: admin, candidate, team, volunteer, supporter,
> public. By default, a post is visible to all at the same or higher rank,
> and there's a "promote" button that elevates visibilty of a post (that only
> the author can make). Oh, also, all posts are associated with the active
> project when they're made, and only appear when the project is selected.

## user

Put someone in a project and you now say two things about them: what they do,
in your own words — *canvasser*, *lead dev* — and **what rank they hold**,
picked from a list of six: admin, candidate, team, volunteer, supporter,
public. The words are still the words; the rank is what decides who sees
what. Everybody already in a project counts as **team** until you say
otherwise.

Select a project, write a post, and the post is **in** that project. It shows
on your posts list and your map while that project is selected, and nowhere
else. Unselect, and you see the posts that belong to no project — the ones
written before there was a project to write them in.

The moment you save it, that post lands on the phone of everyone in the
project at your rank or above. They do not pull down, tap, or reload: their
list gains a row and their map gains a pin. Somebody below your rank does not
get it at all.

Under a post of your own there is one quiet line — *visible to the team and
up* — and beside the bin there is an **up-arrow**. Tap it and the line reads
*visible to volunteers and up*; the post goes out to the volunteers there and
then. Tap again for supporters, again for everyone in the project. It only
goes one way, and only the person who wrote it has the arrow at all.

## spec

**Two ladders, and they are not the same ladder.** `/authority`'s
admin / support / member says what you may do to the *app* — invite, see the
diagnostics, hand out authority. This node's admin / candidate / team /
volunteer / supporter / public says where you stand *inside one project*, and
it says nothing about the app: a `member` may be a project's admin, and an app
`admin` may be a supporter in someone else's campaign. Nothing here reads
`/authority` and nothing there reads this. Named together because the two
words `admin` collide, and a reader who conflates them will build a privilege
escalation.

**The grade rides on the role link.** `/projects` writes
`links:[{kind:"role", to, name, role, t}]` on the project card, and only its
owner may (a copy carries `from`, and the event refuses one). This node adds
one field to that link, `grade`, taken from the six-word option list. A link
with no `grade` — every role written before today — is **team**, stated once
here and read nowhere else: `audience_grade_in` supplies the default, so no
card is rewritten and no migration exists. The project's **owner is admin** by
being the owner; there is no role link to themselves and none is invented.

**Rank is an order, not a set.** admin 0, candidate 1, team 2, volunteer 3,
supporter 4, public 5. "The same or higher rank" is the *lower or equal*
number. Everything in this node compares two numbers and nothing compares two
words.

**A post is filed where it was written.** `/cards`' `card_new` is the one door
every capture road goes through — typed, photo, video, audio, a recording
(`/as-posts` builds its card with it too) — so the stamp is put there once and
no road carries a copy of it. With a project selected and held, a new post is
minted with `links:[{kind:"in", to:<project card id>, t}]` — the shape
`/cards` reserved and `/current-project` already honours — and with
`floor: <the author's own grade in that project>`. With no project selected
the card is `card_new`'s own, untouched.

**Filed where it was written, shown where it was filed.** `/current-project`
narrows the posts to those *related* to the chosen project — filed in it, or
written by somebody in it. That was the right rule when nothing filed
anything; the ask is narrower. This node tightens `posts_set` to an equality:
a post appears exactly when its `in` link names the selected project, and a
post with no `in` link appears exactly when no project is selected. The map
follows for nothing — `/browse` hands `browse_set_html` the same set. **Posts
written before today have no `in` link**, so they now live under "no project
selected"; that is the ask ("only appear when the project is selected"), and
it is stated here because it moves cards that already exist.

**The floor is the audience.** A post carries `floor`, a grade; a world holds
the post if its grade in that project is at or above the floor. The floor is
stamped at creation as the author's own grade — "visible to all at the same or
higher rank" — and **promote** lowers it one rung, which is what raises the
audience. Only the author may: the event refuses a card carrying `from`, and
the control is not drawn on such a page (`/delete`'s rule, decided in Rust
where it is structural rather than by comparing names — the logged-in name is
not in the world).

**Delivery is `/exchange`'s road, widened by one lane.** Immediacy already
exists: a cards write is watched from outside the turn, the writer's changed
cards are copied and handed into each linked world through `handle_msg`, and
`/converge` relays a `CtxUpdate` to that world's open pages. Nothing new is
transported. Two links do the whole job:

- `exchange_share` gains a second pass, `audience_hand`: a post of the
  writer's whose `edited` moved and which carries an `in` link goes to every
  person named on that project — its owner and every role link — as well as
  down the invite tree the pass beneath already walked. This is the lane that
  reaches a project-mate who never invited you and whom you never invited.
- `exchange_give` — **the** door into another world, and the one both lanes
  and the join-time seeding all pass through — refuses a post whose project
  the recipient does not hold, or holds at too low a grade. One gate, so no
  road can go round it, and the same idiom `/projects` used to keep a project
  card to its members.

Qualification is read from the **recipient's own copy** of the project card,
which they hold because `/projects` hands a project to everyone in it. So
"can Carol have this" is answered by what Carol holds, not by what Alice
believes — and a person who is not in the project holds no such card and is
refused with no special case.

**Join-replay is safe because there is no arrival trigger.** Nothing here
watches for an arrival; the gate is on the way *in* to a world. A phone that
was off finds the qualifying posts already in its world when it joins
(`/remember` logged them) and none of the others, and it rings about nothing —
`/attention`'s ladder decides channels and is untouched (`/taste` 8).

**Two behaviour-neutral seams were cut in `/projects`** (agents.md's refactor
rule — the parent lacked an extension point): `projects_role_link` builds the
role link the `RoleAdd` event writes, and `projects_people_role` builds the
role cell of a row on the project page. Both were inline expressions; both are
now functions returning exactly what they returned. `projects.js` gained
`roleData()` for the same reason. Nothing else in `/projects` moved.

**The wire.** Every new byte rides one op — `CtxOp` on `miso/loop/cards`, the
op that already carries every card edit. A `grade` is ~18 bytes on a role
link, an `in` link ~60 and a `floor` ~20 on a post. `/wider`'s caps are 192KB
of body and a 160KB list; a hundred posts of overhead is under 1% of one.

**Parked, and named** (`/anticipation`): **demote** — the floor only falls,
and the day it should rise the event is `PostPromote`'s twin and the ladder
is already an order; **who saw this** — `audience_people_of` is the list, and
a surface would ask it; **posting to a project other than the selected one** —
the stamp is one function, `card_new`'s link, and a chooser would set what it
reads; **a public surface** — `public` as a floor is data today and means
"everyone in the project", because there is no outward surface for it to mean
more on; **per-post custom audiences** — the floor is a grade and would become
a list. None is built.

## hostile cases

- **A role link with no `grade`.** Read as team, everywhere, by one default in
  `audience_grade_in`. Every role written before today is this case.
- **An unknown word in `grade`.** The picker cannot send one; the event takes
  the field only when it is one of the six, so a hand-made op writes nothing.
- **Promote on a post you did not write.** The card carries `from`, the event
  refuses it, and the arrow was never drawn.
- **Promote on a post in no project.** No floor to lower and nobody the floor
  would reach; the arrow is not drawn.
- **Promote at `public`.** The rung is the last one; the arrow is not drawn
  and the event clamps rather than wrapping.
- **The recipient does not hold the project.** `exchange_give` refuses. This
  is also what makes a stranger's forged `in` link inert: naming a project you
  are not in gets your post nowhere.
- **The author's own grade is unknown** (a project of somebody else's they are
  not actually on). No floor is stamped and no `in` link is either — the card
  is `card_new`'s own and travels the invite tree as posts always did.
- **A post promoted while a recipient's page is open.** It is an ordinary card
  edit: `/guard` merges by id, `edited` is newer, the page repaints.
- **A recipient already holding the post is handed it again.** Both lanes may
  reach the same world; `/guard` merges the identical card and the page does
  not flicker. Two writes where one would do, named as the cost of not
  building a union of the two audiences.
- **`/current-project` unticked.** This node reads its var and its card;
  they travel together, and this node is its consumer, not the other way
  round.
- **`/projects`, `/posts`, `/exchange` or `/delete` unticked.** The same: this
  node extends all four and does not compose without them.
- **This node unticked.** No `grade` is written or read, no post is stamped or
  narrowed, `exchange_give` and `exchange_share` are `/exchange`'s own, and
  the invite-link copying that ships today is exactly what happens.

## glossary

- **grade**: where a person stands in one project — one of admin, candidate,
  team, volunteer, supporter, public. Not `/authority`'s app-wide ladder.
- **rank**: a grade's position, admin 0 to public 5. Higher rank is the
  smaller number.
- **floor**: the lowest rank a post reaches — a grade on the post card.
  Stamped as the author's own, lowered by promote, never raised.
- **`in` link**: `{kind:"in", to:<project card id>, t}` on a post — the
  project it was written in.

## code description

`audience.rs` extends `card_new`: a post minted while a held project is
selected gets its `in` link and its `floor`. Every capture road mints through
this one function, so no road carries the stamp itself.

`audience.rs` extends `posts_set` to the equality the ask asks for — a post's
`in` link names the selected project, or both are absent. It runs outside
`/current-project`'s own narrowing, which it tightens rather than replaces.

`audience.rs` extends `update` with `PostPromote {id, t}`: the floor of an
owned post in a project drops one rung, clamped at `public`, and the step is
recorded through `/undo`'s two library calls (`undo_var_before` off a snapshot
taken at the top of the link, `undo_push`) — this node is newer than
`/undo/late`, so its write lands after the scan, exactly as `/delete` found.

`audience.rs` extends `tool_controls` with the up-arrow, in front of `/undo`'s
button through `/posts`' `posts_before_undo` and wearing the posts tool's own
colour (`/glyphs`), drawn only on your own post that is in a project and not
already public. `audience_arrow_svg` is the drawn glyph. It extends
`card_page_html` with the one quiet line saying the rung, spliced inside the
page's scrolling box through `/projects`' `projects_inside`.

`audience.rs` extends `exchange_give` — the one door into another world — to
refuse a post whose project the recipient does not hold or holds below the
post's floor, and `exchange_share` with `audience_hand`, the second lane that
carries a project post to project-mates the invite tree does not reach.
`audience_may_hold` reads the recipient's own copy of the project card
through `exchange_cards_of`.

`audience.rs` extends `/projects`' two new seams: `projects_role_link` gains
the `grade` field when the event carries one of the six words, and
`projects_people_role` puts the grade under the role word on the project
page's rows — the project page only, never on a person's own card.

`audience.rs` holds the ladder: `audience_rank` and `audience_grade_at` are
the order, `audience_grades` the option list, `audience_words` the plural a
sentence needs, `audience_line` the whole sentence. `audience_in_of`,
`audience_floor_of`, `audience_grade_in`, `audience_project_in` and
`audience_people_of` are the card readings.

`audience.js` puts the six options into `/projects`' add sheet as a row of
pills (an option list, not a form) and wraps `feature_Projects.roleData` so
the chosen grade rides the `RoleAdd` event, and `feature_Projects.open` so the
row resets to team each time the sheet opens. Both are `typeof`-guarded. It
takes `posts_promote` in the capture phase and sends `PostPromote` with the
time on the event — there is no clock inside `update` (misses.md, the clock in
wasm).

`audience.css` gives the grade pills the chosen accent when picked, the grade
under a role word the ignorable step of `/taste` 2, and the audience line the
same.
