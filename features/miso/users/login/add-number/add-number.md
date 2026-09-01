# add-number
*a quiet slot on your own card: add your number, and it lets you in from anywhere*

> (transcripts/2026-09-01-saturday.md#p10)
> yeah - add your number later is good - there can be a slot in the user's
> profile to do that.

## user

On your own 👤 card there is one quiet row: **add your number**. Type it, tap
add, and we text you a code the way we always do. Type the code back and the
number is yours: from then on you can log in on any other device — a second
phone, an iPad, a replacement handset — with the ordinary code-by-text login.

Nothing else changes. Everything you have made stays exactly where it was, and
your other devices carry on without noticing. Adding a number is a door being
opened, not a move.

If the number is already someone else's on this campaign, we say so and nothing
happens. You can see the number you added on the same row afterwards.

## spec

`/instant` puts people in the app with no number at all. This is the road back
out of that state, for the two things a number is still worth: logging in on a
**second device**, and getting back in if the first one is lost.

**The world key never changes.** This is the whole design, and it came from the
ask itself: the number is recorded as a **login alias** — "this number may log
into this account" — and never becomes the account's identity. Worlds, blobs,
push subscriptions, cards and authority stay keyed exactly as they were born.
The migration this replaces would have had to rewrite a user's key in every
place the server had filed it, and would have been the one genuinely dangerous
piece of the feature.

The alias road reaches into the login machinery at exactly two points.
`find_user` learns to answer for an alias as well as a number, which is what
lets `auth/request` find the account and text a code to it. Then `auth/verify`,
having checked that code the ordinary way, has its cookie **swapped**: the
session is issued for the account's own key, never for the number that was
typed. So a person logging in by alias lands in the same world as always,
holding the same identity they have always held.

That swap is what makes the `find_user` widening safe. A token spelled with an
alias would name a second, empty world — so nothing is allowed to mint one, and
nothing does: `auth/verify` is the only place an SMS login issues a cookie, and
it swaps; `/passkey` mints from the key already in a valid session. As a second
line, `authority_of` is deliberately **not** widened, so even a hypothetical
alias-keyed session would hold no rung at all.

**Verification is the PIN machinery, unchanged.** The request sends a code
through `send_sms` and the same hourly cap; the confirm checks it through the
same pending file, expiry and attempt count. Nothing new was invented for a
problem that was already solved — the only new thing is what happens after the
code is right.

**A number belongs to one account.** Both the request and the confirm check the
number against every entry's own number and every entry's alias, and the confirm
re-checks inside the store lock, so two people confirming the same number at the
same moment cannot both take it. A number already on the list is refused with
the plain sentence rather than a hint about who holds it.

Adding is the only verb. Changing or removing an alias, and holding more than
one, are not built.

Unticking this node removes the slot and the two routes, and `find_user` and
`auth/verify` return to their previous answers — an alias already recorded
simply stops being a way in, and the entry it sits on is otherwise untouched.

## glossary

- **`/login alias`**: a phone number recorded on an account that may be used to
  log into it, without being the account's world key.

## code description

`add-number.rs` defines `feature_AddNumber`.

`route` answers `users/number` (the slot's state), `users/number/request` and
`users/number/confirm`, then defers. All three want a live session and nothing
else — the caller is read from the cookie alone.

`addnum_who` is the caller's account key off the cookie. `addnum_alias_of` and
`addnum_account_for_alias` are the two directions of the lookup: an account's
recorded alias, and the account a number is an alias for.

`addnum_taken` is the one-number-one-account rule, asked by both routes: it is
true if any entry carries the number as its own or as its alias.

`addnum_request` checks the caller, the shape and `addnum_taken`, then sends a
code with `make_pin`, `save_pending` and `send_sms` under the existing hourly
cap. `addnum_confirm` checks the code against the pending file exactly as
`auth_verify` does — expiry, three attempts, `constant_eq` — and on a match
writes the alias onto the caller's entry inside the store lock, re-checking
`addnum_taken` there.

`find_user` (redefined) falls through to an alias match when the base finds no
number. `auth_verify` (redefined) swaps the issued cookie to the account's own
key when the number that logged in was an alias.

`me_under` (redefined) draws the slot: the recorded number if there is one, else
the field and its button, else the code box once a code is out.

`add-number.js` keeps the two fields out of the DOM, pulls the slot's state when
the card page appears, and sends. `add-number.css` is the row in the card's own
`.crow` grammar.
