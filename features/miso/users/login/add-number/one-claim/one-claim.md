# one-claim
*a number that belongs to an account cannot be invited as a second one*

> (transcripts/2026-09-01-saturday.md#p10)
> yeah - add your number later is good - there can be a slot in the user's profile to do that.

*(a named risk from the add-number build, closed in the same run: the
typed invite still checked only the `phone` column, so a number already
recorded as somebody's alias could be invited again — two accounts with a
claim on one number, and the SMS code road answering for whichever it
found first.)*

## user

Typing an invite for a number that already belongs to someone — as their
own number or as the number they added to their account — answers
"that number belongs to someone already". Nothing else changes.

## spec

`/add-number` established one-number-one-account and owns the test
(`addnum_taken`: the `phone` column and every `alias`, both normalised).
`/invite`'s `invite_add` predates aliases and checks only the column. This
node, a subfeature of `/add-number` so it toggles with the machinery it
defends, extends the `invite_add` chain: it reads the same `phone` field
from the request, and when `addnum_taken` says the number is claimed it
refuses before the inner chain runs; otherwise it hands the request down
unchanged. The inner chain's own duplicate check stays — with this node
unticked the old column-only behaviour is exactly what remains.

The QR claim road (`/qr`) appends through the same chain, so one guard
covers the typed invite and the doorstep scan alike.

## hostile cases

- **This node unticked.** The alias gap returns — the pre-add-number
  state, no worse; `/add-number`'s own slot still refuses taken numbers.
- **An empty or garbage phone.** `addnum_taken` answers false for empty;
  the inner chain's shape rules refuse it as they always did.
- **The number is the inviter's own alias.** Refused the same as anyone
  else's — one number, one account has no exceptions.

## glossary

(no new terms)

## code description

`one-claim.rs` redefines `invite_add(r)`: parse the body's `phone`,
normalise it, refuse with 400 "that number belongs to someone already"
when `addnum_taken` holds, else `existing.invite_add(r)`.
