# title
*the project you are in, named at the top of every screen*

> (transcripts/2026-09-03-invite-test.md#p63)
> one change: let's show the active project name as a title between the
> grid/list/map selector and the noob button, at all times - not just when
> we're in a tool-tab that supports the selector. i.e. detach active project
> from the selector and make it visible at all times.

## user

The name of the project you are working in sits at the top of the screen,
between the view picker on the left and the nøøb button on the right, on
every tool — the launcher, a card, the map, 👤. Tap it and you leave the
project, as the chip did. When you are in no project the top is quiet.

## spec

`/current-project` signed the picker with a chip — *"every list's picker
grows a chip saying which project you are in"* — so the sign was only there
where a picker was. Ash (#p63): detach it and show it always.

**A title on the render root.** This node extends `render` — the whole
page's chain — with one element after everything else: `.proj-title`, the
current project's name, fixed at the top centre, drawn whenever a project is
current. It carries the chip's own event, `proj_select:<id>`, so a tap
leaves the project exactly as before; `/current-project`'s update handles
it unchanged. No project current: nothing drawn.

**The chip retires.** Its element is still composed by `/current-project`
and hidden by this node's stylesheet — the same information twice at the
top of one screen would be noise (`/taste` 8), and the title is the chip's
successor, not its sibling. Untick this node and the chip is back.

**Placement.** Fixed, `top` level with the picker's row and the build
lozenge's, centred; at most 44vw wide with an ellipsis, so it never reaches
either neighbour on a phone; z-index with the picker. The name in the one
accent that means chosen (`/taste` 3), weight 600, 13px; no ground of its
own, so it reads as a title, not a button — the long-press card (`/tool-words`)
does not cover it, so `title` says what a tap does.

## hostile cases

- **No project current.** Nothing drawn; the chip is hidden but was not drawn
  either.
- **A project with an empty title.** "a project", as the chip said.
- **A very long name.** Clipped with an ellipsis at 44vw; the full name is
  the element's `title`.
- **`/browse` unticked.** No picker; the title stands alone, centred.
- **This node unticked.** The chip in the picker, as before.

## code description

`title.rs` — `render` appends `current_title_html()`: the `.proj-title`
element for `current_project_card()`, or nothing.
`title.css` — the fixed placement and the accent; `.proj-chip` hidden.
