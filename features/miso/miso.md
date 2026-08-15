# miso
*shared infrastructure for all apps and tools — "make it so"*

> (transcripts/2026-08-13-fm-spec.md#p32)
> so next is a collection of work I want to call "muon" : it's shared infrastructure for all "apps" / "tools" I want to build. The base of it is: it's a Progressive Web App (PWA) written in rust/wasm, that has a few useful features: 1) works even when there's no internet (using a local server cache); 2) has authentication and users built in; 3) has always-on black-box and reproducibility to catch errors; 4) supports feature management through a simple UI.
> *(renamed at 2026-08-15-fm-spec.md#p50: "I want to rename muon to miso ('make it so') - a name I had hanging around for a self-modifying toolkit, which muon has become.")*

## spec

Root of the shared infrastructure feature space. Miso is a Progressive Web App written in Rust/wasm, providing four base capabilities to every app built on it: offline operation, built-in authentication and users, always-on black-box recording with reproducible replay, and feature management through a simple UI. Apps are built as subfeatures of miso; products compose miso plus an app subtree.

## user

Miso lives at miso.nøøb.org. On a phone you install it to the home screen (the browser only ever shows install instructions); first login is a texted code, after which Face ID and update notifications set themselves up automatically. The app works offline, keeps itself on the latest build, and announces deploys by notification. The logo lozenge bottom-right opens the /system panel/: who you are, what's changed, log out, update.

## glossary

- **miso**: the shared infrastructure layer — a Rust/wasm PWA base that all apps extend.
- **app**: a user-facing tool built as a subfeature of `/miso`, inheriting its base capabilities.
- **system panel**: miso's own UI surface (identity, builds, changes, enrolment, logout), opened from the logo button.

## code description

Container node — the capabilities live in the subfeatures.

`/serve`: the stdlib HTTP server and the `route` extension chain everything else plugs into.

`/shell`: the PWA client — loader, offline service worker with the freshness deadline, self-update, and the system panel; content renders through the `render()` chain.

`/users`: identity and sessions — `/pin` SMS codes (delivered by `/vonage`), `/gate` the login wall, `/passkey` Face ID.

`/push`: Web Push, first used for deploy announcements. `/diag`: remote launch and error reports from installed devices.

Products build miso as two places from one tree: `server` (native, entry `serve`) and `client` (wasm, entry `render`).
