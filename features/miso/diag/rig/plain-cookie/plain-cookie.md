# plain-cookie
*on a rig every cookie is plain, whichever route set it*

> (transcripts/2026-09-03-invite-test.md#p6)
> OK. The QR code shouldn't require an SMS challenge - having the QR code
> within the time limit is "proof" that you're authorised.
> *(the proof of that node on the simulator: the claim's cookie kept `Secure`
> on plain http and WebKit dropped it — the rig's own strip never saw the
> response)*

## user

Nothing a person sees. A test rig on the simulator logs a device in through
any road the app has — the PIN, a passkey, a scanned code — instead of only
the roads older than the rig.

## spec

`/rig` strips `Secure` from a localhost response's cookie, because WebKit
drops a `Secure` cookie on plain http and a rig is plain http. It does so by
wrapping `route`, and a wrap sees only what runs inside it: every route link
newer than `/rig` (`/qr`'s claim, `/instant`, `/add-number`) answers before
`/rig`'s link and keeps the flag. On the simulator the `/scan-is-proof` proof
ended at *"this browser did not keep the cookie"* for exactly this reason.

This node re-states the rule at today's provenance, so it is outermost on
`route` and sees every response: on a rig, not through the tunnel, a cookie
loses `Secure`. Same test (`rig_on`, `r.tunnel`), same replace, no other
change. A node newer than this one that sets a cookie will need the rule
re-stated again — that is the shape of a positional rule, named here so the
next proof does not rediscover it.

## hostile cases

- **Not a rig.** `rig_on` is false; the response passes untouched.
- **Through the tunnel on a rig.** `r.tunnel` is set; untouched, as `/rig`.
- **This node unticked.** `/rig`'s own strip still covers the older roads.

## code description

`plain-cookie.rs` — `route` runs `existing.route` and, when `rig_on()` and not
`r.tunnel`, removes `Secure; ` from `set_cookie`.
