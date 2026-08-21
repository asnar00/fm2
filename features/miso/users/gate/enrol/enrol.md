# enrol
*first login sets the whole device up*

> (transcripts/2026-08-13-fm-spec.md#p67)
> Note for future: enable faceID and notifications should just happen automatically on first login on a device.

## user

New device: install, one SMS code, allow the two permission sheets — done forever. Nothing to find in menus.

## spec

A device's first login is its complete setup: after a successful PIN verify, the login flow automatically enrols Face ID (`/passkey`) and notifications (`/push`) before entering the app — both permission ceremonies ride the login tap's user activation. Failures degrade silently into the app (reasons to `/diag`; the `/panel` buttons remain as retry). A Face ID sign-in on a fresh install marks the passkey as present (one evidently exists via iCloud) and picks up notifications only — never a second passkey ceremony.

## glossary

- **device setup**: the passkey + push enrolments that make a device fully miso-capable, run once at first login.

## code description

This node owns `enrol.login.js`: `feature_Enrol.run()` — the passkey branch (register-challenge → `credentials.create` → register), then the push branch (`pushManager.subscribe` with the VAPID key → subscribe endpoint), each guarded, each reported via `log()`. Called by the login page's PIN-verify path and by `/passkey`'s Face ID sign-in path.
