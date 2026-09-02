# invited-into
*whoever comes in on an invite joins the project it was made in, at the rank it gave*

> (transcripts/2026-09-02-self-check.md#p68)
> for the invite workflow, couple of things: by default, the person should be invited to the current active project (i.e. sevenoaks) - so the new person needs to automatically be added. Also, when inviting by name/phone, we should also be able to assign a role from a dropdown. The page should show two buttons: "show QR code" and "invite by name"; the former should let you choose the role (same for all invitees) and then display the QR code; the latter should pop up a single "name/phone/role" chooser and an invite/cancel buton.

## user

Someone you invited into sevenoaks as a volunteer logs in and opens 👤. From
that moment they are in sevenoaks: your project's page has a row for them,
"volunteer", with the rank under it; their own card says "volunteer for
sevenoaks"; they hold the project, it is selected for them, and the first post
they write lands in it. Nobody taps anything. If somebody in sevenoaks who does
not own it invited them, the same happens — the project's owner sees the new
row on their next look.

## spec

`/doors` and `/ranked` record, on the guest-list entry, the project and rank an
invitation was made with. This node is the join: it turns those two fields
into a role link on the project card — the shape `/projects` writes and
`/audience` grades — and hands the project to the newcomer.

**The link is written when the person has a card, not when they log in.** A
role link's `to` is a profile card id (`/projects`; `projects_members` skips a
link without one), and a profile card is minted by the phone on the first
paint of 👤 (`/me`, `/patient`), not by the server at login. So the moment is
the invitee's first cards write that carries their own profile card: this
node's `route` link watches `POST /msg` from outside the turn — `/exchange`'s
position and reason — and after a 200 asks whether the writer's entry says
`project` and no `added`, and whether their world now holds their profile. Any
later write of theirs asks again until it is done, so a write that fails to
land is retried by the next one; a person who never opens 👤 is never added
(the onboarding rung in flight makes the profile the first thing everyone
does).

**Only the owner's world holds the original, and the server writes it.** The
inviter's held copy names the project's owner; the owner's world is read for
the original; the link — `{kind:"role", to, name, role:<rank>, grade:<rank>,
t}`, through `projects_role_link` so `/audience`'s grade lands as it does from
the add sheet — replaces any earlier link to the same person, exactly as a
`RoleAdd` does, and the card's `edited` moves past whatever it was so `/guard`'s
merge takes it. It goes in through the door `/exchange` opened — a `CtxOp` on
the cards var, handed to `handle_msg` as the owner — so `/guard` merges,
`/converge` repaints the owner's open page and `/remember` logs it. That is the
answer to the brief's second case: a member inviting into a project they do
not own needs no refusal and no wait for the owner's phone; the server writes
the owner's world and the owner's instance shows the row on its next paint,
which `/converge` makes now if the page is open.

**The changed project goes to everyone in it, the newcomer first.** A server
write is not a `POST /msg`, so `/projects`' own hand-over does not see it; the
copy is given here to every member's world through `exchange_copy` and
`exchange_give`, whose project filter admits the newcomer because the link is
already on the card. **Their current project is set** to it if they had none —
"joins your current project" means it is theirs to work in, and `/audience`
files their first post there. Then the entry is stamped `added`.

**Nothing is written twice and nothing false stays.** `added` is stamped only
after the owner's world took the write; a refused write logs and waits for the
next. A project that is gone by the time they join — deleted, or the inviter no
longer holds it, or its owner is not on the guest list — drops the two fields
from the entry, with a log line, rather than leaving a promise nobody can keep.

**No retrofit.** People already on the guest list carry no `project`; nothing
is written for them (`/retrofit`, recorded in `/doors`).

## hostile cases

- **The owner's phone is ahead of the server's clock.** `/guard` keeps the
  newer `edited`; the link's stamp is the later of now and the card's own
  `edited` plus one, so the server's write is never the older one.
- **The owner edits the project at the same moment.** Their device's next
  write of the card carries its own `edited` and, if it was read before the
  link landed, no link: last write wins and the row is lost. Named: a window
  of one paint, closed by the owner adding the person from the project page.
- **Two devices of the newcomer write at once.** Both see the entry pending;
  both write the same link (a replace, by `to`); the stamp is under the lock.
- **The invitee is the project's owner** (cannot happen — an owner is not on
  their own guest list as an invitee): stamped without a link; an owner is
  admin by being the owner.
- **The inviter was taken off the guest list.** Their world is still read;
  if their copy of the project is gone the fields drop.
- **`/invite` unticked.** No entry carries `project`; this node does nothing.
- **A member inviting in.** The owner may not hold the newcomer's profile card
  (`/exchange` follows invite links); the row still draws, from the link's
  name. Their card reaches the owner when the owner invites them or holds
  them some other way — the project-membership visibility rung, parked
  since #p71.

## glossary

- **added**: the guest-list stamp saying the join was done.

## code description

`invited-into.rs` extends `route`: on a cards write from a cookie-proven
caller, after the chain beneath, `invited_into_try(who)`.

`invited_into_try` is the whole join: the pending entry, the newcomer's own
profile (`invited_into_profile`), the inviter's held copy naming the owner,
the owner's original, the link through `projects_role_link`, the write through
`invited_into_put`, the copies to every member through `exchange_copy` and
`exchange_give`, `invited_into_select` for the newcomer's current project,
then `invited_into_stamp`. `invited_into_clear` drops the two fields when the
project cannot be joined.

`invited_into_put(to, card)` is the door: one card, a `set`, signed with the
recipient's audience, handed to `handle_msg` as them — `/exchange`'s own door
restated for a card that is not a copy.
