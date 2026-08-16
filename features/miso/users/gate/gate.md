# gate
*login wall for tunnel traffic*

> (transcripts/2026-08-13-fm-spec.md#p39)
> How about user login?

## user

Visiting miso.nøøb.org logged out shows the login screen: phone number → texted code → in, for a year on that device. On the LAN there is no login.

## spec

Extends the `/serve` route chain: `/auth/request`, `/auth/verify` and `/auth/whoami` answer everywhere; the app shell and the feature tree (`/features/`, statically exported at deploy — a deliberately public, shareable artefact) are served without login; all other traffic that arrived through the cloudflare tunnel (cloudflared always stamps `cf-connecting-ip`) needs a valid session cookie or receives the login page with status 401 (`no-store` — Safari reuses cached 401s). Local/LAN requests hit the port directly, carry no tunnel header, and pass ungated — the dev loop stays frictionless.

## glossary

- **tunnel traffic**: requests arriving via cloudflared, identified by the `cf-connecting-ip` header it always adds.

## code description

`gate.rs`'s `route` /extension/ decides in order: auth endpoints answer first; public paths (`is_public`: the app shell and `/features/`) pass through; non-tunnel and authed requests pass through via `existing.route(r)`; everything else gets `login_page` — `site/login.html` with 401 and `no-store`.

The login page is a `/shell`-styled port of ftr's, keeping its hard-won iOS-autofill and cookie-race fixes.

It also runs **first-login device setup** (`enrolDevice`): after a successful PIN verify it automatically enrols Face ID (`/passkey`) and notifications (`/push`) — both ride the login tap's user activation. Failures log to `/diag` and the system panel buttons remain as the retry path.

Face ID sign-in on an already-enrolled device marks the passkey flag (one evidently exists) and picks up notifications only — never a second passkey ceremony.
