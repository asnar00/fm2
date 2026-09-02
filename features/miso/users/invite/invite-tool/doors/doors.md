# doors
*the invite page is two buttons — show QR code, invite by name — each giving a rank, into your current project*

> (transcripts/2026-09-02-self-check.md#p68)
> for the invite workflow, couple of things: by default, the person should be invited to the current active project (i.e. sevenoaks) - so the new person needs to automatically be added. Also, when inviting by name/phone, we should also be able to assign a role from a dropdown. The page should show two buttons: "show QR code" and "invite by name"; the former should let you choose the role (same for all invitees) and then display the QR code; the latter should pop up a single "name/phone/role" chooser and an invite/cancel buton.

> (transcripts/2026-09-02-self-check.md#p69)
> also, we can lose the "joined" list on the invite page

> (transcripts/2026-09-02-self-check.md#p70)
> in the "add person" toolset, don't show "edit" since edit has no meaning during add-person.

## user

Open 👤 and tap the plus. The page is two buttons: **show QR code** and
**invite by name**. Nothing else — the list of people you invited is gone.

**Invite by name** pops up one card: a name, a phone, a rank to give (six —
admin, candidate, team, volunteer, supporter, public; team is picked for you),
and **invite** / **cancel**. A quiet line says which project they are going
into — the one you have selected — or "no project selected". Cancel folds the
card with nothing sent.

**Show QR code** pops up the same card with just the rank: pick one for
everyone who will scan, tap **show**, and the code fills the screen as before.

Whoever comes in on either road joins your selected project at that rank the
moment they have a card of their own (`/invited-into`), and holds your card as
before (`/exchange`). While the invite page is open the control row carries no
pencil: there is nothing to edit here.

## spec

`/invite` is a name-and-number row, `/invite-someone` folded it behind a pill,
`/invite-tool` made it a page, and `/qr` put a "show a QR code" pill above it.
An invitee landed in nobody's project and with no rank, so a kick-off needed a
second pass per person. Ash asked for the page to be two buttons, each with a
rank to give, and for the person to land in the current project (#p68); then
for the invited list to go (#p69) and the pencil with it (#p70). One reading
of each, so it builds.

**The page is redrawn, not patched.** `invite_rows_html` is redefined with no
`existing` call: the send row, the list and `/qr`'s pill all leave with it,
and what it returns is the two buttons on `/invite-tool`'s card-shaped page,
carrying the selected project's id and title as data attributes so the sheet
can say where the person is going. Untick and the old page — pill, row, list —
is back exactly as it was.

**One sheet, two faces.** Furniture built at load and living outside `#app`
(`/projects`' add sheet is the precedent, for the same reason: the loop
repaints `#app` wholesale and a half-typed invite must survive it). In its
*name* face it shows name, phone, the rank and **invite**; in its *qr* face the
rank alone and **show**. Both faces carry the quiet project line and **cancel**.
The sheet swallows its own taps in the capture phase, as `/qr`'s does, because
`/backdrop` would otherwise read a tap on the card as a tap on bare ground and
close the invite page underneath.

**"Role from a dropdown" is the rank.** The six words of `/audience`'s ladder,
in its order, in a real `<select>` — the app's other rank picker (`/audience`'s
row on the project add sheet) is six pills, but ash asked for a dropdown by
name and a select is what the phone opens as one. The free role word on the
link defaults to the rank's word; changing it later is the project page's
business, as today.

**The name road carries the rank and the project.** The sheet POSTs `users/invite`
with `rank` and `project` beside `name` and `phone`. `invite_add` is wrapped:
before the inner add, `invite_into_ok` checks that the inviter *holds* that
project (own or copy, not deleted), stands in it (owner or role link), and is
not giving a rank above their own — a volunteer cannot invite an admin. After a
200 the guest-list entry is stamped `project` and `rank`, two fields beside
`authority`, never overloading it: rank is project standing, authority is app
standing. A member inviting into a project they do not own is allowed and the
owner's world is written by the server at join (`/invited-into`) — the sheet
does not refuse it. With no project selected nothing is sent and the person
comes in as before.

**The pencil.** `/editing/toolbar` draws its pencil for whatever
`feature_Editing.page()` returns, and that is any `.card-page` that is not
foreign — the invite page counts. This node wraps `page()` at load to answer
nothing for `.invite-page`; the toolbar's own `apply` then removes the button on
the next paint, and `/editing`'s locking has nothing to lock. Card pages are
untouched.

**No retrofit.** People already on the guest list were invited into no
project; nothing is written for them (`/retrofit`'s ruling, recorded here).

**Parked, and named.** Inviting into a project other than the selected one —
a project picker on the sheet, reading the same data attribute. A QR per rank
on one screen — the qr face grows a row. Removing from the project on take-back
— the ✕ is gone from this page; that is `/invite`'s uninvite's business.

## hostile cases

- **A rank above your own.** Refused with "you're a volunteer there — you
  can't invite someone in above that"; the sheet shows the sentence.
- **A project you do not hold**, or one deleted since the page was drawn:
  "you don't hold that project" / "you're not in that project". The page never
  sends one, but the route is public to anyone with a cookie.
- **A rank that is not one of the six.** Refused before the guest list is
  opened; the select cannot produce one.
- **No project selected.** The line says so; `project` is not sent; the entry
  carries neither field and `/invited-into` does nothing.
- **The invite fails** (duplicate, bad number): `/invite`'s own sentence in the
  sheet, the fields kept, nothing folded.
- **A repaint mid-typing.** The sheet is outside `#app`; nothing is lost.
- **`/qr` unticked.** The show-QR button is hidden at load (`feature_Qr`
  absent); the name road stands alone.
- **`/current-project` or `/audience` unticked.** This node's Rust reads their
  functions — the dependency is the ask's own ("the current active project",
  "a role from a dropdown").

## glossary

- **door**: one of the two ways onto the guest list from this page.
- **rank**: `/audience`'s grade, given at the invitation.

## code description

`doors.rs` redefines `invite_rows_html` to the two buttons, with the selected
project's id and title on the block; wraps `invite_add` with `invite_into_ok`
before and `invite_into_stamp` after; `invite_into_ok(who, project, rank)`
is the three checks (held, in it, not above your own) as one seam so `/ranked`
obeys exactly the same ones on the QR road.

`doors.js` builds the sheet at load, opens it in one of two faces from the
page's buttons, sends the invite or the mint, and wraps
`feature_Editing.page` to withhold the pencil.

`doors.css` — the two buttons, the sheet in `/projects`' sheet grammar, the
select in the field grammar with a drawn chevron.
