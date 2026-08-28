# delete-project
*a project you own can be deleted, and the people in it lose it too*

> (asks#1787903851453)
> add "delete project" tool to project toolset. Only project owner sees it.
> *(filed from the field on 2026-08-28 by ash, from the projects tool)*

## user

Open a project of your own and the control row has a **bin**, the same one a
post of yours has. Tap it once and it asks — the button becomes the word
*sure?* for three seconds; tap it again and the project is gone: from your
grid and your list, and from the phones of everyone you had given a part in
it, the next time they look. The undo button beside it is lit: tap that and
the project comes back, people and all, and they get it back too.

A project somebody else made has no bin on it. Only its owner can delete it.

## spec

**The op is `/delete`'s.** `CardDelete {id, t}` names no type — *"the same
event deletes a project or a profile the day one is asked for, and only the
button is posts-only"*. This node is that day for projects: it draws the
button and sends the same event, and the tombstone, the guard's merge, the
sifting from every surface and the undo step all happen in `/delete`,
unchanged. What a project needs that a post did not is one thing: its
audience.

**A project tombstone keeps its role links.** `/delete` empties a tombstone's
`links` along with its words. For a post that is right — nothing hangs off a
post. For a project the role links are not content but the list of people
the card was handed to (`/projects`' hand-over gives a changed project to
everyone its links name, and its `exchange_give` filter lets a project into a
world only if that world has a role in it). Emptied, the tombstone would
reach nobody, and the deleted project would live on, editable-looking, on
every member's phone. So this node redefines `delete_tombstone` for cards of
type `project`: the words and the picture go, `deleted` is stamped, and the
role links stay. The tombstone then travels by the ordinary path — `edited`
moved, so `projects_hand` copies it to each member, the filter admits it, and
`/guard` takes it over their copy because it is newer. Nothing under
`/projects` or `/exchange` learns what a tombstone is.

**And a deleted project's roles are not roles.** With links on the tombstone,
`/projects`' role lines on a profile page — "canvasser for miso" — would keep
listing it, with an empty title. `projects_roles_from` is the seam (the
project cards a role line may come from; refactored out of
`projects_roles_html` for this node, behaviour unchanged), and this node
sifts tombstones from it. Everything else already looks the other way: the
member count is `/browse`'s row and the row is sifted; the people section is
the page and a tombstone's page is `/delete`'s one dim line.

**Only the owner.** A copy carries `from` (`/exchange`) and that is the whole
test, the same one `/delete` uses and `/projects` uses for the ✕: the button is
drawn only on an own project that is not already gone, and the event refuses
a card with `from` on it in `/delete` before this node is reached.

**The two-tap is `/delete`'s, made once more.** `feature_Delete` became a
maker for this node (`make(ev)` — the same three seconds, the same word, the
same capture-phase listener, bound to another `data-ev`); the posts bin is
the instance it was already. The projects bin is `feature_Delete.make
('projects_delete')`, and this node's page half is that one line.

**Undo brings the people back.** `/guard/revert` restamps the restored list,
so the project returns newer than its tombstone; `edited` moved, so the
hand-over gives it to every member again and their tombstone yields.

## hostile cases

- **Deleting somebody else's project.** No bin; the event, if forged, finds
  `from` and does nothing (`/delete`).
- **A member who is not a user.** No world to hand the tombstone to; the link
  stays on the tombstone and nothing is shown for it (`/projects`' rule).
- **A member's phone was off.** The tombstone waits in their world like any
  other handed card (`/exchange`); on join it is merged and sifted.
- **A member opens the project as the delete lands.** `/delete`'s page rule:
  one dim line, *deleted*.
- **A role added to a tombstone.** Unreachable — the people section is on the
  page, and the page is not drawn.
- **`/delete` unticked.** This node calls `delete_tombstone` and
  `delete_gone`; it composes only with `/delete` ticked, and a link error is
  the honest report (the `/projects` ↔ `/exchange` precedent).

## code description

`delete-project.rs` redefines `delete_tombstone`: for a card of type
`project` it puts the role links back on the tombstone `existing` returns,
so the delete reaches the project's members through `/projects`' own
hand-over. `delete_project_roles(card)` is the list of `role` links.

`delete-project.rs` redefines `projects_roles_from` to drop tombstones, so a
deleted project's roles leave every profile page.

`delete-project.rs` extends `tool_controls`: with the projects tool open on a
project of your own, the bin goes in front of `/undo`'s button through
`/projects`' `projects_before_undo`, in the projects tool's own colour;
`delete_project_own(id)` is the gate and the glyph is `/delete`'s
`delete_bin_svg`.

`delete-project.js` binds `/delete`'s two-tap to `projects_delete`.

`delete-project.css` gives the button the same ink and the same *sure?*
sizing as the posts bin.
