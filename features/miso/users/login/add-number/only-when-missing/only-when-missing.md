# only-when-missing
*add your number is offered to the people whose number we do not have, and to nobody else*

> (asks#1788558115121)
> on profiles, only show "add number" if you don't have the user's phone number

## user

The **add your number** row on your own card appears only if we do not already
have a number for you — which means the people who scanned a code and typed
just their name. If you joined with your number, the row is gone: there is
nothing to add, and the card says nothing about it. A number you added
yourself is still shown back to you on that row, as it always was.

## spec

`/add-number` draws its row from one answer: `GET users/number`, which reports
the account's **alias** — the number that account has *added*. That is empty
for everyone who never added one, and everyone whose real number has been on
the guest list since the day they were invited is in that set. So the row
offered to add a number to every person the campaign can already text. Ash,
from the field: *"on profiles, only show 'add number' if you don't have the
user's phone number."*

**Which half was the bug.** The row was never on anyone else's card — `/me`
draws its block only when the 👤 tool is open on your own card
(`me_landing()`), and `me_under` is called from nowhere else, which the rig
confirmed by opening another person's card and finding no row. The fault was
entirely on your own card: a real number and an empty alias look the same to
the slot.

**We have your number when your world key is one.** A key is `phone:<digits>`
and it is the number the guest list holds. The one case where it is not a
number is `/name-only`: a scan-in that typed no number is given a placeholder —
`+9` and sixteen digits, seventeen in all, two past E.164's cap of fifteen —
so that every consumer of the key keeps working. Length alone answers the
question, and that is `/instant`'s own `instant_is_synthetic`. This product
does not compose `/instant` (unticked in `products/miso`), so the rule is
restated here for the same reason `/name-only` restates the mint.

**The seam was already open.** `/add-number`'s `me_under` returns the chain
beneath untouched when the slot answers `ok: false` — the shape it uses for
"not logged in". This node redefines `addnum_state` to answer exactly that for
an account whose number we hold, with a `why` beside it. No client code
changes, the pull and the two POST routes are untouched, and unticking this
node puts the row back on every card.

**A number already added still shows.** When the alias is recorded the row is
no longer an offer — it is the number, displayed — so this node steps aside
and the base draws it. That is the only case where the row survives a real
number: an account that was name-only, added a number, and can now see it.

## hostile cases

- **A placeholder that has just been replaced.** Adding a number records an
  *alias*; the world key stays the placeholder, so the row remains and shows
  the number back. Nothing flickers and nothing is lost.
- **Logged out.** The base already answers 403 and this link returns it
  untouched; the card is not drawn for a logged-out device anyway.
- **Another person's card.** Never drew and still does not: `me_under` is
  reached only through `/me`'s own landing test.
- **A real number that begins with 9** (+90, +91, +98…). Fifteen digits at
  most, so the length test never mistakes one for a placeholder.
- **The slot answered something that is not JSON**, or a status other than
  200: returned untouched.
- **This node unticked.** Every account sees the row again, including the ones
  whose number we hold — the state ash reported.

## glossary

- **placeholder number**: the seventeen-digit key `/name-only` mints for a
  scan-in with no number — a world key that is not a phone number.

## code description

`only-when-missing.rs`, `addnum_state()` /extension/: reads the base's answer,
and when it is a live slot offering to add a number (`ok`, no alias recorded)
for an account whose key is a real number, replaces it with
`{"ok": false, "why": …}` — the answer `/add-number`'s own `me_under` already
reads as "draw nothing". Everything else passes through.

`onlywhen_no_number` is the test: more than fifteen digits in the world key
means the guest list is holding a placeholder rather than a number.

## risks

**The test is a length, not a flag.** Nothing on the guest-list row says "this
is a placeholder", so this reads it from the shape of the key, as `/instant`
and `/name-only` both do. The day a row carries an explicit mark, all three
should read it instead.
