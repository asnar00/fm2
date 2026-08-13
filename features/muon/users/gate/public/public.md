# public
*the shell is public; the data is gated*

> (transcripts/2026-08-13-fm-spec.md#p44)
> I think it isn't auto-refreshing: I closed/reopened it four times but I still just see the logo, no phone. Maybe we should build enough diagnostic features to let you reach out and debug the app on my phone?

## spec

The prompt's first half exposed a policy bug: gating the app shell froze logged-out installed PWAs solid — the service worker only caches 2xx responses, so every background refresh (even of sw.js itself) got the 401 login page and the app could never update. Policy corrected: the shell (index, sw, wasm, manifest, icons, login/install pages, version, changes) is publicly served — it's just code, no secrets — and only data routes gate. The shell asks `auth/whoami` and routes logged-out visitors to login itself. `/features/` later joined the public list under the same reasoning.

## user

Login protects what matters (your data and actions), not the app's own machinery — which is what lets an installed app always keep itself current.

## glossary

(no new terms)

## code description

`is_public` in `gate.rs`, consulted by the gate's `route` extension before the tunnel/cookie checks; the whoami-driven redirect lives in `/shell`'s loader.
