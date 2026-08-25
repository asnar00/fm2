# exchange
*people see each other by handing over cards*

> (transcripts/2026-08-25-accounts.md#p71)
> yeah, invite should automatically make things visible. the other visibility cue will be shared membership of the same project, we can get to that later.

> (transcripts/2026-08-25-accounts.md#p8, the rule this obeys)
> OK, so I quite like the idea that users own their data, and exchange it back and forth

> (transcripts/2026-08-25-accounts.md#p69, the ask)
> OK so that works. let's talk about making people able to see each other.

## user

Open your cards and the people who invited you — and the people you invited —
are there: their picture, their name, their mission, tagged *profile*. Tap one
and it opens to read. It is theirs, so you cannot write in it, and a quiet
line under the name says who it came from. Change your own mission and their
copy has the new words a moment later, wherever they are.

Nobody taps accept, and nobody appears who did not invite you or take your
invitation. That is what inviting means (#p71).

## spec

`/cards` said it from the first line: *a card lives in its owner's world;
nobody else has it until you hand one over* (#p8). This node is the handing
over, and the first thing it buys is people — you see exactly the people the
invite tree connects you to, and nobody else. Consensual by construction,
which is the campaign's privacy story stated as a mechanism rather than as a
promise (#p69, #p70).

**Visibility is the invite tree.** Two people are linked when one invited the
other: `/invite` already writes `invited_by` — the inviter's world key — on
every guest-list entry it mints. Rung one reads that field and nothing else.
Shared membership of a project is the second cue ash named, and is later
(#p71).

**A card travels by being written into the other person's world.** Nobody can
write into your world from a device, but the server can, and it already has a
door: a `CtxOp` on the cards var, handed to `handle_msg` while the thread is
acting as the recipient. That is the same door a device's own edit comes in
by, so all the machinery beneath applies unchanged — **`/guard`** merges the
arrival into what they already hold, so nothing of theirs can be displaced;
**`/converge`** applies it and relays a `CtxUpdate` to their open pages, so a
phone with the app in front of it updates within a beat; **`/remember`** logs
it, so a phone that was off finds the card in its world when it next joins.
No inbox, no polling, no new store.

**The fan-out watches `POST /msg`, and it does so from outside the turn.**
This is the one piece of mechanism worth reading twice. `/edit` freezes a view
of the context when a request opens; every `with_context` inside the turn
reads that frozen view, and `edit_context` replays its closure against it. So
switching this thread's identity *inside* the sender's turn would read the
wrong world and write the recipient's list into the sender's frozen view. This
node is the newest in the composition, so its `route` link is the **outermost**
one — outside `/per-user`'s identity link and outside `/edit`'s turn boundary.
Out here there is no frozen view and no ambient identity: a world can be
named, read and written honestly, and every read is of the live value. That is
why the link sits on `route` watching `POST /msg` rather than on `handle_msg`,
which is where a first sketch would put it. Four public seams are used —
`context_user_set`, `handle_msg`, `cards_read`, `card_of_type` — and nothing
inside `/context` is touched.

**Only what changed travels.** The writer's list is read on both sides of the
chain beneath; a card of theirs whose `edited` moved, **or that was not there
before**, is handed on. Newly-present counts as changed because the first
write of a card stamps `edited` equal to `created`, and that first write is
exactly the one that has to travel. An unchanged list sends nothing at all.

**Invite is an exchange, automatically** (#p71). A successful `POST
auth/verify` is the cheapest honest signal that a person has arrived — it is
what `/invite` already stamps `joined` from — so the same route link watches
it: on a successful login, the inviter's own cards are written into the new
arrival's world there and then. The invitee's card travels back the first time
they write one, through the fan-out above. Neither of them taps anything. The
invite tree is the first web of links, which is what #p71 asked for.

**A copy is marked, and the mark is what makes it read-only.** A card written
into somebody else's world carries three fields its owner's original does not:
`from` (the owner's name), `via` (the world key of the person the copy came
through — for an invite link, the owner themselves) and `received`. **`from`
is the whole test** — a card that carries it is one you did not write. That is
deliberately not a comparison of the owner's name against the logged-in name:
two people can share a name, the logged-in name is not in the world at all, and a page half
that had to fetch it would race the paint. Decided in Rust at render time, the
read-only-ness is structural rather than defensive: `card_page_html` draws a
foreign card with **no `contenteditable` anywhere in it**, so `/cards`'
focusout, `/keep`'s input timer and Enter rule, and `/frame`'s picture chooser
have nothing to fire on, and `/location`'s *dim* pill — which would ask the
phone for a fix and stamp somebody else's card with it — is taken away rather
than merely dimmed. A foreign card cannot send `CardEdit`, `CardPic` or
`CardPlace` because the DOM those listeners look for is not drawn. One CSS
rule finishes the job: `.foreign .card-pic { pointer-events: none }`.

`via` is read by nothing in this node. It is here because the surface that
comes next wants to order people by proximity, and proximity is *how did this
reach me*; putting the field on the copy now costs a line and saves a
migration. **It is a raw world key, which for a phone user is their phone
number** — so a person's number lands in the worlds of everyone they are
invite-linked to. Within one campaign's guest list that is defensible and it
is what the next node was told to expect; the cheap fix, if it is not, is an
opaque `hmac(secret, "exchange:" + key)` in its place, which orders just as
well and says nothing.

**The copy path has a name on purpose.** `exchange_copy` is the one place a
card becomes a copy and `exchange_give` the one door into another world. A
later way of handing a card over — a send-to sheet, a project — is meant to
call those two and write no marking or merging code of its own.

**`/me` had to be told about copies.** It asks `/cards` for "the profile card"
with no owner at all, and its comment said why it could: *"a world holds only
its owner's cards today; exchange is what earns it."* This node earns it, so
`card_of_type` is redefined here — an ownerless ask skips the cards that came
from somebody, and answers "you hold none of your own" rather than handing
back a neighbour's. Asked *with* an owner, which is what `CardEnsure` does, it
is `/cards`' own answer, untouched.

**Parked, with the seam each one joins at** (`/anticipation`'s test — the next
asks should be new nodes, not rewrites of this one):

- **the people surface** — the 👤 tool showing every profile card, self first,
  ordered by how near the person is: `via` on every copy is the raw material,
  and the cards are already in the world.
- **send to a number** — a route of its own that calls `exchange_copy` and
  `exchange_give` and adds no marking or merging code.
- **project membership as the second visibility cue** (#p71's "later") —
  `exchange_links` is the one function that answers "who can see me", and a
  second answer is appended to it.
- **withdrawing a card, an accept tap, links, rings.** Named, not built.

## hostile cases

- **The recipient has never logged in / their phone is off.** The card is
  written into their world on the server and logged by `/remember`; they find
  it when they next join. Nothing depends on them being reachable.
- **The recipient's world takes the write badly.** `handle_msg`'s reply is
  read: anything that is not a `CtxUpdate` is announced on the log with the
  reason. The sender's own write has already succeeded and is not undone —
  their card is theirs whether or not it reached anybody.
- **A copy tries to travel on.** Refused: only cards with no `from` and the
  writer's own `owner` are handed on, so a card cannot be relayed through a
  third party.
- **A card claims an id that is not its owner's.** Refused twice. A card is
  handed on only if its id begins with its owner's name, which is `/cards`' own
  `<owner>.<created>` contract (`exchange_owns_id`); and a card is never
  written onto an id the recipient already holds under a *different* owner
  (`exchange_not_theirs`), which is the case two guests sharing a name could
  otherwise reach. **This was rig-found and it was a real loss:** before the
  first check, one linked user could mint a card carrying another's id with a
  newer `edited`, and `/guard` — which sees only an id and a timestamp — merged
  the forgery over the owner's own card, on the owner's own page.
  The merge is a union, so a set can never delete a card.
- **The inviter has written no card yet** when their invitee first logs in:
  the seed says so on the log and does nothing; the card arrives on the
  inviter's next edit instead.
- **A blank card arrives.** `/guard` discards a blank card only when it would
  duplicate a card of the *same owner and type* the world already holds; a
  blank card of somebody else's is kept, because "I have only just made mine"
  is a legitimate state to see.
- **Both ends edit at once.** Each write is one op into the other's world,
  merged by id with the newer `edited` winning — the same rule `/guard`
  already applies to two devices of one person.
- **`/invite` unticked.** No entry carries `invited_by`, so no two people are
  linked and this node does nothing at all. It reads the guest list itself and
  has no code dependency on `/invite`.
- **`/location` unticked.** The dim-pill removal finds no mark and returns the
  page unchanged.
- **The message body was truncated** at `/messaging`'s cap: it parses as
  nothing, is not recognised as a cards write, and is refused by the chain
  beneath as it always was.

## glossary

- **copy**: a card in your world that somebody else owns — marked `from`,
  read-only, kept current by its owner.
- **via**: the world key a copy reached you through — the raw material for
  ordering people by proximity, later.
- **invite-linked**: the relation rung one makes visibility out of — the
  person who invited you, and the people you invited.
- **fan-out**: writing a changed card into the worlds of everyone linked to
  its owner.

## code description

`exchange.rs` extends `route`, and that link is the whole server half. Being
the newest node's, it is the outermost link on the chain — outside
`/per-user`'s identity link and outside `/edit`'s turn boundary — which is
what makes naming another world safe here and nowhere else.

`exchange_watch_msg` handles `POST /msg`: for a cards `CtxOp` from a
cookie-proven caller it reads their list before and after the chain beneath
and calls `exchange_share`. `exchange_watch_verify` handles `POST auth/verify`:
on a 200 it calls `exchange_seed` for the phone that just logged in.

`exchange_share` picks the writer's own cards whose `edited` moved or that are
new, marks each one a copy (`exchange_copy`) and gives them to every linked
world. `exchange_seed` gives a new arrival their inviter's cards. Both check
`exchange_owns_id` before handing anything over.

`exchange_copy` is the one place a card becomes a copy, and `exchange_give`
the one door into another world; a later way of handing a card over is
expected to call these two and add nothing of its own.

`exchange_not_theirs` is the last gate before a write: it drops any card that
would land on an id the recipient already holds under a different owner.

`exchange_give` is the door: a `CtxOp` on the cards var, signed with the
recipient's own audience so `/converge` relays it to their pages, handed to
`handle_msg` with `context_user_set` naming the recipient for the length of
the call and restoring what was there afterwards.

`exchange_links` reads the guest list for the invite relation in both
directions; `exchange_users`, `exchange_key_of`, `exchange_name_of` and
`exchange_audience_of` are its four small readers, and `exchange_cards_of` is
one world's cards.

`exchange.rs` extends `card_page_html`: `data-owner` on the page always; and
for a card carrying `from`, the `foreign` class, every `contenteditable`
removed, `/location`'s dim pill removed (`exchange_no_dim_place`) and the
*from* line spliced under the title (`exchange_with_from`).

`exchange.rs` redefines `card_of_type` so an ownerless ask never answers with
a copy.

`exchange.css` is three rules: the *from* line as the dimmest thing on the
page, the foreign picture made unreachable by the pointer, and a default
cursor over blocks that are not text fields. There is no `exchange.js`: the
page half of this node is nothing at all, because nothing on the page needs to
know it exists.

*(Review fix, same day: `via` is an HMAC tag of the world key, not the key itself — a phone number must not land in other people's worlds. `from` stays the person's name, which they chose to share.)*
