# name-only
*your name is enough at the scan; a number can come later*

> (transcripts/2026-09-03-invite-test.md#p30)
> it's still asking for a phone number - a phone number shouldn't be mandatory
> for QR code invitees. It should be optional to fill in in the profile page

## user

Scan the session's code, type your name, tap join, and you are in. The number
field is there if you want it, and says so — *number (optional)*. Leave it
empty and nothing is asked of you.

Later, on your own 👤 card, the row **add your number** is waiting: type it,
get the code by text, and from then on you can log in on a second device
too. Until then this device is your way in, and everything you make is yours
all the same.

## spec

`/scan-is-proof` made the scan the login, but the claim still went through
`/invite`'s shape check, which wants a number at least eight digits long — the
join page asked for it and refused without it. Ash (#p30): the number is not
mandatory for a person who scanned the code; the profile page is where they
may add one.

**A claim with no number gets a placeholder number.** This node redefines
`qr_claim`: when the body's phone normalises to nothing and the name is there,
it mints a **synthetic number** and hands the claim on as if that had been
typed. The scheme is `/instant`'s (unticked in this product, its reasoning
kept): `+9` followed by sixteen digits — seventeen in all, two past E.164's
cap of fifteen, so it can never collide with a real phone, and the last four
kept clear of every entry's last four. Every consumer of the world key
(`phone:<digits>`) keeps working because none of them ever read the digits as
a phone. `/scan-is-proof` then sees a fresh number and logs the device in.
Nothing about the account is half-made: it has a guest-list row, a world, a
cookie.

**The number comes later on the card.** `/add-number` already draws **add your
number** on the own 👤 card whenever the account has no alias, texts the code
through the ordinary PIN road, and records the real number as an alias the
login road resolves back to the account. A name-only account is exactly the
case it was built for. The key never changes.

**The page says the field is optional.** `join.html` (`/qr`'s asset): the
placeholder reads *number (optional)*. With this node unticked an empty
number still meets the base's refusal — *that doesn't look like a phone
number* — so the words are the only thing the page changes.

**What a synthetic number cannot do.** No text can reach it (`/instant`'s
`auth_request` guard is not composed here, so a PIN request for a synthetic
number would try to send; the base's Vonage path fails on a seventeen-digit
number and says "couldn't send the code" — bounded, not silent), and it is
not shown as the person's number anywhere: `/add-number`'s row reads the
alias, which is empty until they add one.

## hostile cases

- **A name and no number, twice on one code.** Two accounts, two synthetic
  numbers: the second scan is a second person as far as the list knows. That
  is the doorstep's own rule for a fresh number, and a name is not an
  identity.
- **A number typed after all.** The body's phone is not empty; this node
  steps aside and the claim is `/scan-is-proof`'s.
- **A `_` name, a dead code.** Refused by the base before or after the
  rewrite exactly as before; the placeholder number is never written.
- **This node unticked.** An empty number is refused by the shape check; the
  page's placeholder still says optional, which is then a lie — retick or
  reword.

## code description

`name-only.rs` — `qr_claim` reads the body; if the phone is empty and the name
is not, it replaces the phone with `name_only_number()` and calls
`existing.qr_claim` on the rewritten request. `name_only_number` mints the
seventeen-digit placeholder against the current guest list.
