# instant
*type a name, show the code, and they are in — no number, no text message*

> (transcripts/2026-09-01-saturday.md#p8)
> for the QR code invite - I was thinking, what if the workflow was: you type
> just the user name, we flash up a QR code that goes to a coded link
> (containing the user details somehow) that's valid for a short time (same as
> the PIN code), then if a device goes to that URL, we log that user/device as
> logged in on that device, and we're done. No SMS details or challenge needed.

> (transcripts/2026-09-01-saturday.md#p10)
> I was actually thinking of it more to allow canvassing team members to
> quickly get on the app without having to share their phone numbers, which is
> slow and fiddly.

## user

Getting a colleague onto miso takes one field and one scan. Open 👤, tap **add
someone now**, type their name, and hold up the code. They point a camera at
it, and they are in the app as themselves — no number typed, no code texted,
nothing to read out at a doorstep.

The code is for that one person and it dies the moment it is used, or after
five minutes, whichever comes first. Show it again and it makes a new one. If
someone scans a code that has already been used they are told so plainly and
asked for a fresh one, rather than being dropped somewhere confusing.

An account made this way has no phone number, and that is a real state, not a
half-finished one: their cards sync, their notifications arrive, everything
works. The number is only needed to log in on a *second* device later, and
`/add-number` is where they can add one whenever they like.

## spec

The shipped `/qr` puts a code at a door for strangers to type their own name
and number into. This node is the other half of the same idea, pointed inward:
a code minted **for one named person**, which logs the device that scans it
straight in. The canvasser types the name because the canvasser knows it; the
person scanning types nothing at all.

**The account is real before the code is shown.** Minting creates the
users.json entry first and binds the token to it, so the token names an account
that already exists rather than carrying user details for a later route to
trust. Nothing about the person travels in the link — the token is 32 random
hex characters and the entry it points at lives only on the server.

**The world key is synthetic, and it is spelled as a number.** A minted account
is keyed `phone:+9<16 more digits>` — 17 digits, where E.164 caps a real number
at 15, so a synthetic key can never collide with a real phone. This is the
whole of why the feature fits in two nodes. miso's identity channel is
digits-only and enforced at both ends: `/harden`'s `token_valid` re-checks a
session against `find_user`, and every consumer matches on
`normalise_phone(entry.phone)`, which keeps only ASCII digits. Twenty-one sites
across twelve features build or read the key as `phone:<digits>` — including
two that fail *silently* rather than loudly on a differently-shaped key
(`/to-owner`'s `starts_with("phone:")` audience test, `/attention`'s
subscription-owner comparison). A `user:<hex>` key would have needed all
twelve taught a second shape. A synthetic *number* needs none of them touched:
authority, push filing, card exchange, converge and `joined`-stamping all keep
working because they only ever handled an opaque digit string. (The map is in
misses.md, "the second key shape".)

**The key never changes for the life of the account**, which is what makes
`/add-number` a login alias rather than a migration.

The synthetic value lives in the entry's own `phone` field rather than a
separate `key` field, because that is the choice with **no foreign function
extensions at all** for identity: `find_user`, `authority_of`, `token_valid`
and `exchange_key_of` resolve a minted account natively. A separate `key` field
would have needed each of those four redefined to look in a second place. The
entry also carries `"instant": true` as a marker for the eye, but every test in
the code is the length test — a number of more than 15 digits is synthetic —
because that answers from a bare string, wherever one turns up.

**Two things must never leak.** A synthetic number is not a number anyone can
be reached on, so `auth_request` refuses one plainly instead of trying to text
seventeen digits into the void, and no surface renders it: the invited list
already showed only names, and this node blanks the number out of the row data
as well, so the string does not reach a page at all.

**The claim is the login page's own success path.** `GET /go?t=…` serves
`site/login.html`, and this node's `.login.js` fragment — which lands in that
page's own script scope — notices the token, posts it, and then runs exactly
what a finished PIN entry runs: `whoami`, `feature_Enrol.run()`, and
`location.replace("/?in=…")`. So Face ID and notification enrolment happen on
first entry precisely as they do after an SMS login, which matters *more* here,
not less: with no number on the account, that device's credentials are the only
way back in. Serving the page from this node's own route rather than leaning on
`/gate`'s 401 is deliberate — the 401 road only fires for tunnel traffic, and
the rig is not the tunnel.

**Single use is enforced at the store, inside the lock.** The token row is
marked spent in the same locked section that reads it, so two devices racing
the same code cannot both be let in; the second gets the spent page. Expiry is
300000 ms — the PIN's own window, as the ask asked for.

Mint-time the synthetic number is checked against every existing entry's last
four digits and redrawn on a clash. Nothing today depends on that — `/to-owner`
discards the `_from` that `exchange_audience_of` truncates — but the truncation
is still there under an untick of `/to-owner`, and a mint is a cheap place to
be certain.

Unticking this node removes the pill, the sheet, the routes and the page; `/qr`
returns to its shipped scan-and-type flow, and no minted account is created —
though accounts already minted keep working, because they are ordinary
guest-list entries with an unusual number.

## glossary

- **`/instant code`**: the single-use 32-hex token bound to one freshly minted
  account, alive for five minutes.
- **`/synthetic key`**: a world key spelled `phone:+9…` with 17 digits — a real
  identity that is not a real telephone number.

## code description

`instant.rs` defines `feature_Instant`.

`route` (line 176) is the entry point. It answers the mint, the claim and the
`go` page, then defers. This node is the newest in the tree, so it is outermost
on the route chain and sees a request before `/gate` — which is what lets the
claim and the page answer a device that has no cookie yet.

`instant_mint` takes a name from a support-or-above caller, draws a synthetic
number, writes the guest-list entry and the token row under one lock, and
answers with the token and its expiry. `instant_claim` is the crossing: it
looks the token up, refuses a spent, expired or unknown one with the sentence
that fits, marks it spent, and answers with the `miso_auth` cookie for the
bound account. `instant_page` serves `site/login.html` for `GET /go`.

`instant_new_number` draws 17 digits from `random_bytes`, forces the leading
digit to 9, and redraws while the last four collide with an existing entry.
`instant_is_synthetic` is the length test the rest of the node asks.

`auth_request` (redefined) refuses a synthetic number before the base can try
to text it. `invite_invited` (redefined) blanks the number on a minted row so
it never reaches the page.

`instant_list`, `instant_save`, `instant_prune` and `instant_index_of` are the
token store, following `/qr`'s discipline exactly: a missing file is an empty
list, a broken one is `null` and nothing writes on it, and a save is
temp-write, `fm_own_only`, rename.

`invite_rows_html` (redefined) adds the second pill. `render` (redefined) draws
the sheet — the name box, or the code once minted. `update` (redefined) files
the mint's answer under this node's own transient state key.

`instant.js` holds the name box's draft outside the DOM (`#app` is repainted
wholesale), mints on the button, and draws the SVG from the token by
observation. `instant.login.js` is the claim half, in login.html's script
scope. `instant.css` follows `/qr`'s sheet: one bright field, quiet pills.
