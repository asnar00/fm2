# invite
*support adds a person to the guest list from the 👤 page*

> (transcripts/2026-08-25-accounts.md#p13)
> we'll do that using the invite process

> (transcripts/2026-08-25-accounts.md#p4, the triage reading of the day's plan)
> invite — support/admin adds a person (name + phone) from the profile page; server writes `users.json` under the store lock; a "people you've invited" list with pending/joined; remove.

## user

At the bottom of your own 👤 card, if you are support or admin, there is a
name box, a phone box and **send**. Type someone in and they are on the guest
list: they install miso, ask for a code, and log in the ordinary way. The row
under the card says **invited** until they do and **joined** afterwards. A ✕
takes back an invite nobody has used yet. Nothing is texted to them — their
first code arrives when they ask for it.

## spec

The guest list has always been a JSON file on the mini, and adding a person
has always meant ash over SSH. This node is the first act the app performs on
*another person*: it lets an authorised user append to `users.json` from the
👤 page, and it is the first real consumer of `/authority`'s ladder.

**Who may invite is `support` and above** — `authority_rank ≥ 2`, the same rung
`may_write_shared` uses, because inviting is a shared-reach act: the guest list
is the one list everybody is on. A **member sees no invite row at all**, and
the seam is server-side rather than a hidden div: the page asks
`GET users/invited`, the server answers `{may:false, list:[]}`, and the
renderer draws nothing. Widening this to members is ash's to ask for; the
default recorded here is the least-privilege one.

**Three routes, all on the `route` chain, all requiring a valid cookie of
their own** — they sit outside `/gate`'s wall because `/invite` is newer than
`/gate` and therefore outermost, so each checks `token_valid` itself before
looking at authority. `POST users/invite {name, phone}` normalises the phone
exactly as `/users` does, refuses a number already on the list, and appends
`{name, phone, invited_by, invited}` under `with_store_lock`. A new entry
carries **no** `authority` field, so an invitee is a member. `POST
users/uninvite {phone}` removes an entry, and `GET users/invited` lists the
caller's own invitees with a `joined` flag.

**Joined is a stamp, not a guess.** There was no existing signal that honestly
means "this phone has logged in since the invite": `passkeys.txt` only fills
if a device enrols Face ID, and `sends.txt` records codes *sent*, not codes
accepted. So `auth_verify` gained one line at its outermost end — a successful
verify stamps `joined: <ms>` on that phone's entry, under the lock, after the
inner chain has released it. It is an optional field on an existing store, no
new file, and `joined ≥ invited` is what the page reports. A passkey login does
not stamp; an invitee's *first* login is always a code, which is the case that
matters, and the flag never goes backwards once set.

**Uninvite is narrow on purpose.** An entry can be removed only if it carries
an `invited` field (so only invites can be uninvited — ash's own hand-written
entry can never be deleted through the app), only if it has **never joined**,
and only by the person who sent it, unless the caller is `admin`. There is no
route that changes anybody's authority: elevation stays a thing done on the
mini.

**The guest list is never written from a failed read.** `invite_list` returns
a JSON null — not an empty list — if the file is missing, unreadable, not
JSON, or not an array, and every writer refuses on null and says so in the log.
The write itself is a temp file, `chmod 0600`, then a rename, so a crash
mid-write leaves the old list intact and a fresh list is never born
world-readable.

**The page half renders nothing and fetches everything.** The rows are drawn
in Rust from an `invite` key in the loop state, which the page half fills by
sending an `InviteList` event after each fetch — a transient state key, not a
/var, because the guest list is the server's and has no business syncing to
devices as world state. The 👤 page notices itself through a `MutationObserver`
on `#app`, never by wrapping `feature_Loop.apply` (notes.md, "the
apply-wrapper race").

**The seam it renders into is new and defaults to nothing.** `/me` grew one
function, `me_under(state)`, returning the empty string, whose result is
placed inside the card page's own scrolling box. With `/invite` unticked the
seam returns nothing and 👤 renders exactly as it did.

**The button says "invite", and the fields are always there.** The in-hand
story had a tap on *invite* that reveals fields and then a tap on *send*; the
copy rule for this node is "invite", "invited", "joined" and nothing else, and
the fields cost one row whether or not anyone is being invited. So the two
readings are collapsed into the shorter one: two boxes and the word **invite**,
one tap to send.

**No SMS is sent.** An invite text is a later node, as is sending the inviter's
card along with it (the `exchange` rung).

## hostile cases

- **A member calls the route directly.** 403, from the same check the page
  obeys; a stranger with no cookie gets the same 403.
- **A malformed phone.** Anything normalising to fewer than seven digits is
  refused — "that doesn't look like a phone number" — before the lock is taken.
- **A number without its country code.** `07700 900003` normalises to
  `+07700900003`, which is a *different* number to every other part of the
  tree, so an invitee entered that way could never log in. No country code
  begins with a zero, so a leading `+0` is refused — "that number needs its
  country code" — rather than stored as an entry nobody can use. Found on the
  rig, where the duplicate test spelled the same number two ways and got two
  entries.
- **An empty name.** Refused: "that invite needs a name". The name is what the
  row says and what the login says, so it cannot be blank.
- **The same number twice.** Refused with "they're already on the list",
  whether they were invited or hand-added; the page shows the sentence.
- **Two sends at once.** Both take `with_store_lock` in turn, so the second
  reads the first's write and refuses as a duplicate — one entry, not two. The
  page also holds a `busy` flag, which is comfort, not the guarantee.
- **`users.json` unreadable or malformed.** Every route answers 500 "the guest
  list can't be read" and logs the reason; nothing is written. A wipe is the
  one failure mode this file cannot have. The store's health is asked *before*
  the cookie, because with the list unreadable nobody is authed at all
  (`token_valid` re-checks the guest list), so an authority-first order
  answered a broken box with "you can't invite people" and logged nothing
  true — found on the rig, and the reason the check sits where it does.
- **Uninviting someone who has joined**, someone else's invitee, or a
  hand-added entry: all refused, each with its own sentence.
- **A repaint mid-typing.** `#app` is redrawn wholesale by the loop, so a
  half-typed invite would be lost; the page half keeps the two field values in
  `draft` and restores them after every repaint.

## glossary

- **invite**: a guest-list entry created from the app, carrying who made it and
  when.
- **joined**: an invitee has completed a login since being invited — the
  `joined` stamp written by the verify path.

## code description

`invite.rs` extends `route` with the three endpoints. `invite_add` checks
authority, validates name and phone, then appends under the store lock;
`invite_remove` applies the four uninvite conditions; `invite_invited` answers
the page with `{ok, may, list}`, where `may` is the whole of the member seam.

`invite.rs` extends `auth_verify`: on a 200 it stamps `joined` on the verified
phone's entry, outside the inner chain's lock.

`invite.rs` extends `me_under`, `/me`'s new under-the-card seam, drawing the
send row and one `.crow` per invitee — status where the number sits, ✕ on the
unjoined — from the `invite` key of the loop state, and nothing at all until
the server has said `may`.

`invite.rs` extends `update` with `InviteList`, whose data becomes that state
key verbatim.

`invite_list` / `invite_save` are the store pair: a JSON null means "do not
write", and the save is temp-write, own-only, rename. `invite_caller` names the
requester from the cookie as `phone:+44…`, the identity `/authority` reads, and
`invite_may` is the `≥ support` test.

`invite.js` is fetch and events: `pull` asks `users/invited` and sends the
result into the loop, `send` and `remove` POST and then pull again, `look` is
the `MutationObserver` callback that pulls once per appearance of the card page
and restores the draft after every repaint.

`invite.css` styles the two fields, the send word and the ✕ against `/taste` —
the `.crow` grammar borrowed whole, the one accent (`#9db7d8`) on the focused
field and on nothing else.
