# members
*ordinary members can invite other members*

> (transcripts/2026-09-02-self-check.md#p54a)
> Also, we should allow ordinary members to invite other ordinary members.

## user

You were invited yesterday; today, open 👤 and the invite tool — the person
with a plus — is in its control row, the same as for the person who invited
you. Tap it, tap **show a QR code**, hold the phone up: whoever scans it
types their own name and number and is in. They are members, like you; you
have their card and they have yours. You can type an invite instead, and take
back one nobody has used.

## spec

`/invite` gated inviting at `support` — `authority_rank ≥ 2`, the rung
`may_write_shared` takes — and its spec said in so many words that widening
it to members was ash's to ask for. This is the ask (#p54a): a canvassing
team grows itself, so the people it grows must be able to grow it.

**One redefinition.** `invite_may(who)` becomes `authority_rank(who) ≥ 1` —
*on the guest list* — the way `/light-basemap` redefines
`tiles_default_url`. Not "anyone with a cookie": a token whose entry has been
removed from the list ranks 0 and is refused as before, so the ladder keeps
its shape and a struck-off member cannot go on inviting.

**Every door reads the same answer, so every door opens.** `invite_may` is
asked in seven places and each now says yes to a member: `POST users/invite`
(a typed invite), `POST users/uninvite` (a take-back), `GET users/invited`
(the list — and `may:true` on it is what `/under-account` reads to put the
plus in 👤's control row, so the tool appears with no page-half change), `/qr`'s mint
and revoke (the session code), `/qr`'s look-up (a scanned code is live only
while its owner may still invite — for a member, while they are still on the
list), and `/instant`'s mint (a name-only account; it takes the same `may`,
so it comes along). Nothing in the page half reads authority: `invite.js`
and `invite-tool.js` follow the server's `may` alone, checked by grep.

**What an invitee becomes is not widened.** No route has ever set an
`authority` field; a typed invite, a scanned session code and an instant
code all write an entry without one, and `/authority` reads a missing field
as `member`. So a member's invitee is a member, and support is still made
only on the mini — *only support can invite support* was already so, and
stays so. `invite_admin` (rank 3) is untouched: taking back somebody
**else's** unused invite stays an admin act, and `/pretend`'s `_` test users
stay admin-only, since `invite_name_ok` is a separate chain this node does
not touch and `/qr`'s claim refuses a `_` name outright.

**The session code's bounds are per inviter and unchanged** — cap, gap and
expiry are `/qr`'s. A member's code has exactly the bounds a support's had;
the only thing that changed is who may hold one.

**Parked, by name.** Limiting how deep a member's invitations may go; support
revoking a member's invitees. Anticipated: a member's join page already shows
the inviter's name (`/qr`'s check); counts per inviter would extend the
invite list.

## hostile cases

- **This node unticked.** `invite_may` is `/invite`'s own again: a member gets
  `may:false`, no tool, 403 on every route — today's behaviour.
- **A stranger with no cookie.** `who` is empty; refused before the rank is
  read, as before.
- **A member struck off the list.** Rank 0: refused, and a session code they
  minted answers "this invite has expired" to anyone who scans it.
- **A member takes back someone else's invite.** "that isn't your invite" —
  `invite_admin` still decides that.
- **A member types a `_` name.** "a name can't start with _" — the admin-only
  exception in `/pretend` is rank 3, unchanged.
- **A member invites a number already on the list.** Refused, as for anyone
  — "that number belongs to someone already" since `/one-claim`.
- **A member's invitee invites too.** Yes: carol, invited by bob, has the
  plus in her 👤 row. How deep this may go is the parked question above.

Proven on a headless rig (2026-09-02): `_bob` (member, invited by `_ash`)
gets `may:true`, has the plus, opens the sheet, mints a code; `carol` joins
through it by typing name and number, lands on the list with no authority
field, invited by bob; on her login bob's profile copy is seeded to her, and
her first card write hands hers to him. Bob takes back his own unused invite
and is refused ash's; `_zed` from bob is refused, from ash accepted. On the
build without this node bob gets `may:false` and no plus.
`tests/sim/invite-members.json` is the member's walk, runnable by `simrig`
or the headless stand-in.

## glossary

(no new terms)

## code description

`members.rs` redefines `invite_may(who)` as `!who.is_empty() &&
authority_rank(who) >= 1`. Nothing else: every route that asks it, and the
tool that follows its answer, are `/invite`'s and its children's own.
