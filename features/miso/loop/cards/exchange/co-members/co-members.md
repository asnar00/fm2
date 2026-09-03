# co-members
*people in the same project see each other*

> (transcripts/2026-09-03-invite-test.md#p57)
> OK, I signed up as Tara - but I should see the other users in the sevenoaks
> project, I just see Tara.

> (transcripts/2026-09-03-invite-test.md#p59, the rule)
> because the invitation was made with sevenoaks active, tara should
> immediately see all users of sevenoaks project on her users page - it
> shouldn't wait for anyone else to do anything (that should be true of all
> new users, not just tara)

## user

Join a project and everyone in it is on your people page at once — their
profiles, ready to open — and you are on theirs. Nobody has to do anything
for that to happen. From then on, whatever any of you changes on your own
card reaches the others a moment later, exactly as it does between a person
and the one who invited them.

## spec

`/exchange` made visibility the invite tree and named project membership as
the second cue, later (#p71). This is later. Ash's rule (#p59): a newcomer
whose invitation carried a project sees every member of it the moment they
join, and waits on nobody.

**The links grow.** `exchange_links(key)` — who receives a person's cards on
every write, and whose cards seed them — is redefined: the base's inviter
and invitees, plus everyone who holds a role in any project card this person
holds, own or copy. The role links carry the member's name; the guest list
turns a name into a world key (`projects_key_for_name`). The person's own
world is read — their own copy of each project card names its members — so
one world read answers it. Inside a turn that is `cards_read()` on the
current context; outside, `exchange_cards_of`.

**The moment of joining is the moment of seeing.** `/invited-into` stamps
the newcomer when the role link is on the card and the project has been
handed to every member. This node redefines `invited_into_stamp`: after the
base, it seeds the newcomer from every link (`exchange_seed`, whose base
gives the inviter's cards and which this node widens to give every linked
person's own profile), and hands the newcomer's own profile to every link in
return. Nobody else's write is waited for. `exchange_seed` widened means the
PIN and scan logins seed from co-members too, for a person who already held
a project when they logged in.

**Profiles, not everything.** What travels by membership is each member's
profile card — "see each other". Posts filed in a project are `/audience`'s
business and a later ask.

**Parked, named:** the people page ranks by invite distance (`people_bfs`) and
a co-member with no invite link ranks as unknown, last, with no word — a
"same project" word is its own ask. A member leaving a project (a role link
removed) keeps the copies they were handed; taking them back is a later
rung and would go through `/delete`'s tombstone.

## hostile cases

- **A project card with no role links.** Nothing added.
- **A member whose name is not on the guest list** (removed by `reset_user`).
  `projects_key_for_name` is empty; skipped.
- **The newcomer is the owner.** The base stamps and returns before the
  hand-over; nothing to seed.
- **The same person in two projects with you.** Keys are deduplicated.
- **`/invited-into` unticked.** `invited_into_stamp` is not composed; this
  node's link on it is refused by the linker — the join is the seam, so the
  two travel together.
- **This node unticked.** The invite tree alone, as before.

## code description

`co-members.rs` — `exchange_links` widens the base's list with the keys of
every role link on every project card in the person's world. `exchange_seed`
runs the base then gives the newcomer each other link's own profile cards.
`invited_into_stamp` runs the base then seeds the newcomer and hands their
profile to every link. `co_members_of(key)` reads the project cards;
`co_members_profiles(key)` is a person's own profile cards as copies.
