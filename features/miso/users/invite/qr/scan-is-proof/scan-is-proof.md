# scan-is-proof
*a fresh number on a live code is in — no text message, no PIN*

> (transcripts/2026-09-03-invite-test.md#p6)
> OK. The QR code shouldn't require an SMS challenge - having the QR code
> within the time limit is "proof" that you're authorised.

## user

At the door, you point your camera at the canvasser's code, type your name
and number, and you are in miso — the app opens as you. No text arrives and
nothing has to be typed from it.

If your number is already in the campaign, the page asks for a texted code
exactly as before: that account is yours, and the code proves it.

## spec

`/qr` made the claim an invite with the authority moved to the token, and then
sent the person down the ordinary `/auth/request` road for a PIN. Ash's ruling
(#p6): holding a live code *is* the proof. The canvasser is standing there,
the code dies in a day or on their say-so, and a texted PIN after that proves
nothing the scan did not. So this node hands the cookie out at the claim.

**The claim logs the device in.** `qr_claim` is redefined: the base runs
unchanged — token checked, shape checked, the guest-list row appended under the
store lock, the use spent — and when it answers 200 for a number that was
**not** on the list before the claim, this node adds the session cookie
(`make_token(phone)`, the same cookie `/auth/verify` issues) and answers
`{ok:true, in:true}`; the guest-list row is stamped `joined` through
`/invite`'s own `invite_stamp_joined`, so the canvasser's invite list says
*joined* rather than *invited*. The join page, seeing `in`, confirms the cookie
with `/auth/whoami` and opens the app — the same landing `/instant` uses.

**A number already on the list keeps the PIN.** The code proves the canvasser
let you in; it does not prove you own a number somebody else already joined
on. So the base's duplicate answer — `{ok:true}` with nothing written — is
passed through untouched and the page goes to the texted code as today. This
is the one place the two roads differ, and it is the security of every
existing account: a leaked code can add a stranger to the campaign (as before,
capped and revocable), and can no longer become an existing member. It does
mean a code-holder can tell a number that is in the campaign from one that is
not, by which step follows — `/qr`'s membership-oracle rule is relaxed to that
extent, within the code's life, and ash may rule the other way (the parked
alternative: also log a duplicate in, which makes the code a key to every
account in the campaign).

**The seam in the page.** `join.html` (`/qr`'s asset) gained one inert branch:
a claim answer carrying `in` goes to the app instead of asking for a code.
The base never sends `in`, so with this node unticked the page behaves as it
did. Wrappers never touch the page's copy.

## hostile cases

- **A claim for a number already on the list.** The base answers as before,
  no cookie is set, the PIN step follows. Nothing written.
- **A dead, capped or too-fast code.** The base refuses before this node
  looks at the answer; no cookie.
- **The cookie does not stick** (a browser that drops it). The page's whoami
  check fails and says so, exactly as the PIN road does; the guest-list row
  is there, so the person can re-scan and — now a duplicate — take the PIN
  road, which still works for them.
- **A `_` name.** Refused by the base before anything is written.
- **This node unticked.** `qr_claim` is the base's; the page's `in` branch
  never fires; the PIN step is back for everyone.

## code description

`scan-is-proof.rs` — `qr_claim` reads the phone from the body, notes whether
the guest list already held it, runs `existing.qr_claim`, and on a 200 for a
fresh number sets the session cookie, stamps `joined`, and answers with `in`.
`scan_is_proof_seen(phone)` is the guest-list test, read outside the lock
because the base takes it.

`join.html` (in `/qr`): after a successful claim, `if (c.in)` confirms the
cookie with `/auth/whoami` and replaces the page with the app.
