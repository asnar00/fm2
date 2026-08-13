# gate
*login wall for tunnel traffic*

> (transcripts/2026-08-13-fm-spec.md#p39)
> How about user login?

## spec

Extends the `/serve` route chain: `/auth/request`, `/auth/verify` and `/auth/whoami` answer everywhere; all other traffic that arrived through the cloudflare tunnel (cloudflared always stamps `cf-connecting-ip`) needs a valid session cookie or receives the login page with status 401 (`no-store` — Safari reuses cached 401s). Local/LAN requests hit the port directly, carry no tunnel header, and pass ungated — the dev loop stays frictionless.

## user

Visiting muon.nøøb.org logged out shows the login screen: phone number → texted code → in, for a year on that device. On the LAN there is no login.

## glossary

- **tunnel traffic**: requests arriving via cloudflared, identified by the `cf-connecting-ip` header it always adds.

## code description

`gate.rs`: `route` /extension/ — auth endpoints first, then pass-through for non-tunnel or authed requests via `existing.route(r)`, else `login_page` (serves `site/login.html`, a `/shell`-styled port of ftr's page including its iOS-autofill and cookie-race fixes).
