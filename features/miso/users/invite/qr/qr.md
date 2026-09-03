# qr
*a code on the canvasser's screen; the person at the door types their own name and number*

> (transcripts/2026-09-01-saturday.md#p2)
> make a quicker QR-code based invite workflow that we can use instantly during canvassing

## user

*Context, corrected 2026-09-03 (transcripts/2026-09-03-invite-test.md#p15):
the code is shown to the canvassing team at the start of a session so they
all sign themselves up from one picture — never to a member of the public at
a door. The paragraphs below were written with the doorstep in mind; read
"the person at the door" as "a canvasser", and see `/scan-is-proof`.*

On the invite tool there is a row that says **show a QR code**. Tap it and the
screen fills with a code. The person at the door points their camera at it,
lands on a page that says who invited them, types their **own** name and number,
gets the text, types the code, and they are in miso. The canvasser types
nothing at all.

The same code works for the whole session — the count under it says how many
people have signed up on it. **new code** throws the old one away and makes
another; **done** puts the sheet away. A code stops working a day after it was
made, or the moment the canvasser makes a new one.

## spec

`/invite` is the guest list from the app: support types a name and a phone and
the person is on it. At a doorstep that is the wrong way round — it is the
canvasser's thumbs against a stranger's memory of their own number. This node
turns the invite inside out: the canvasser shows a **token**, and the person
who owns the number is the one who types it.

**The token is a row in a file, not a signed string.** `~/.miso-auth/invite-qr.json`
holds one row per inviter — `{token, by, made, expires, uses, cap, last}` —
written under `with_store_lock` beside `users.json`. A signed stateless token
would have been fewer lines and could not be revoked, counted, or capped; all
three are the security of this feature, so the token is a lookup. **One live
token per inviter**: minting again returns the same row, and **new code**
replaces it, which is what makes the old code dead the instant a canvasser
decides it is. The file therefore cannot grow past one row per support user,
and every mint prunes expired rows, so it does not fill.

**The token is 16 bytes of `/dev/urandom`, hex.** 128 bits, never logged whole
(the log says the first six characters), and the store file is written
own-only through the same temp-write-and-rename `/invite` uses.

**Five routes, all on the `route` chain, each checking its own gate.** This
node is the newest in the tree, so it is outermost and sees a request before
`/gate` does — which is how the two public ones work at all. `POST
users/invite/qr/mint` and `POST users/invite/qr/revoke` require
`invite_may` — the same `authority_rank ≥ 2` `/invite` requires, read from the
caller's cookie. `GET join`, `GET users/invite/qr/check?t=…` and `POST
users/invite/qr/claim` require **only a live token**: they are the one road
that legitimately crosses `/gate`'s wall for a stranger, because a stranger is
exactly who they are for.

**A claim is an invite with the authority moved to the token.** `qr_claim`
checks the token (known, unexpired, under its cap, not too fast), re-checks
that its owner may still invite *now*, validates the name and phone through
`/invite`'s own `invite_shape_ok`, and appends
`{name, phone, invited_by: <the token's owner>, invited}` to `users.json`
under the store lock — no `authority` field, so an invitee is a member, exactly
as a typed invite is. Then the page asks for a code on the ordinary
`/auth/request` road and the person logs in like anybody else.

**A number already on the list is answered as a success.** The typed invite
says "they're already on the list"; the claim page must not, because that turns
a leaked code into a membership oracle — anyone holding it could ask "is this
number in the campaign?" of every number they own. So a duplicate and a fresh
append return the same `{ok:true}`, and the page moves on to the code step,
which is also the right thing for a person re-scanning a code they already
used. A duplicate spends no use of the token and writes nothing.

**The claim page is an asset, not a composed page.** `assets/join.html` is
served by `qr_page` at the path `join`, so the code encodes
`https://…/join?t=<token>` — no query-mangling redirect, no new composition
target in the linker. It carries its own copy of the phone-then-code login
because the whole point is that the person types their number **once**; sending
them to `login.html` afterwards would ask for it twice. What it does not carry
is `/enrol` — Face ID and notifications are login-page furniture and a person
who has just scanned a code is in mobile Safari, not an installed app, where
`/enrol` would no-op anyway. Their first login inside the installed app enrols
them.

**The code is drawn on the device, from a vendored encoder.** `assets/qrlib.js`
is Kazuhiko Arase's `qrcode-generator` 1.4.4, MIT, committed whole and loaded
by a `<script defer>` in the head — not a CDN (the app is offline-first and
serves its own bytes) and not a server-rendered image, because an image is a
round trip at the exact moment a canvasser is holding the phone out and the
signal is bad. `qr.js` asks the encoder for a module matrix and writes its own
SVG, so the code is crisp at any size and takes its colours from this file
rather than a library's idea of them.

**The sheet is the one bright surface.** `/taste` 1 says nothing arrives white;
a QR code needs contrast or a camera cannot read it, so the module field is
white — treated as *content* on the house's dark ground, the way a photograph
is, and framed in `#161619` with the page family's border and radius. Nothing
else on the sheet is bright.

## hostile cases

- **A garbage token.** Anything that is not 32 hex characters is rejected
  before the store is opened; an unknown one gets "this invite link isn't
  valid". No stack trace, no blank page — the claim page shows the sentence on
  the ordinary dark ground with the logo above it.
- **An expired token.** "this invite has expired", the same way. Expiry is
  checked at `check` *and* at `claim`, server-side, against `now_ms` — the
  page's opinion is never trusted.
- **A revoked token.** Identical to unknown: the row is gone.
- **The inviter loses their rank, or is taken off the guest list.** `qr_claim`
  re-reads `invite_may` for the token's owner on every claim, so the code dies
  with the account rather than outliving it.
- **A leaked or screenshotted code.** This is the real one. Anyone holding the
  image can put name/number pairs on the guest list, and those people can then
  ask for a text and log in as members. That is precisely the power the
  canvasser has at the door — they are showing the code to strangers on
  purpose — so it is bounded rather than prevented: **25 claims**, **24 hours**,
  **2 seconds between claims**, revocable in one tap, and every entry stamped
  `invited_by` the canvasser, so the damage is attributable and removable
  through `/invite`'s own ✕. What a leaked code cannot do: gain any authority
  (an invitee is a member), read anything (no route answers with data), or
  send a text to a stranger (the code goes to the number *they* typed, and
  `/pin` still allows five an hour).
  The residual, named: within its day a circulating code can spend its 25 uses
  on junk numbers. The canvasser sees the count on the sheet and taps **new
  code**. Nothing bounds it by network address, because the request struct does
  not carry one — `cf-connecting-ip` is read for `tunnel` and thrown away.
- **A name beginning with `_`.** Refused on this road always, for anyone —
  `/pretend` lets an *admin* mint a test user by typing it, but nobody should
  be able to mint one by scanning something.
- **Two people claiming at once.** Both take `with_store_lock`; the second
  reads the first's write, so the duplicate check and the use count are both
  honest, and two claims can never spend the same last use.
- **`users.json` or the token file unreadable.** Both stores answer a JSON
  null rather than an empty list, every writer refuses on null, and the routes
  answer 500 with a sentence. Nothing is written from a failed read. Rig note:
  with `users.json` broken a claim is refused as "this invite has expired"
  rather than "the guest list can't be read" — nobody is authorised when the
  list cannot be read, so the token's owner fails `invite_may` first. Both
  refuse; neither writes.
- **A tap on the sheet closing the page underneath.** `/backdrop` closes the
  open tool on any tap that lands on nobody's ground, and its list of owned
  surfaces is its own file rather than a seam — so the sheet claims its own
  taps in the capture phase instead of editing that list. Without it, **done**
  put the invite page away too and dropped the canvasser on the dot grid; found
  on the rig.
- **The encoder fails to load.** The sheet says "couldn't draw the code" rather
  than showing an empty white square that a camera would sit on forever.
- **The sheet open when the page repaints.** `#app` is redrawn wholesale by the
  loop, so the SVG is redrawn from the token by the same `MutationObserver`
  idiom `/invite` uses — never by wrapping `feature_Loop.apply`.

## glossary

- **invite token**: a random string bound to one inviter, standing for their
  permission to invite, for a day and 25 people.
- **claim**: a stranger putting themselves on the guest list with a token.

## code description

`qr.rs` extends `route` with five endpoints: `qr_mint` and `qr_revoke` behind
`invite_may`, and `qr_check`, `qr_claim` and the `join` page behind a live
token only.

`qr_claim` is the whole of the crossing: token, cap, gap, the owner's current
right to invite, `/invite`'s shape rules, then the append under
`with_store_lock`. A number already listed returns the same answer as a fresh
one and spends nothing.

`qr_list` / `qr_save` are the token store's pair, in `/invite`'s shape — a JSON
null means "do not write", the save is temp-write, own-only, rename — with
`qr_prune` dropping expired rows on every mint so the file stays one row per
inviter.

`qr.rs` extends `invite_rows_html` with the **show a QR code** row at the top
of the invite rows, and `render` with the full-screen sheet when the loop state
says it is open; `update` takes the `QrSheet` event, whose data is the mint's
answer verbatim, into the `invite_qr` state key.

`qr.js` mints on open, draws the SVG from the vendored encoder, and holds the
sheet's three taps; `qr.head.html` is the one line that loads the encoder;
`qr.css` is the sheet.

`assets/join.html` is the stranger's page: check the token, name and number,
claim, code, in. `assets/qrlib.js` is the vendored encoder, unmodified.

`invite.rs` gained `invite_shape_ok(name, phone)` — the three shape rules
(a name, eight digits, a country code) lifted verbatim out of `invite_add` so
this node can obey exactly the same ones. `invite_add` calls it and behaves as
it did.
