# enrol
*first login sets the whole device up*

> (transcripts/2026-08-13-fm-spec.md#p67)
> Note for future: enable faceID and notifications should just happen automatically on first login on a device.

## spec

A device's first login is its complete setup: after a successful PIN verify, the login flow automatically enrols Face ID (`/passkey`) and notifications (`/push`) before entering the app — both permission ceremonies ride the login tap's user activation. Failures degrade silently into the app (reasons to `/diag`; the `/panel` buttons remain as retry). A Face ID sign-in on a fresh install marks the passkey as present (one evidently exists via iCloud) and picks up notifications only — never a second passkey ceremony.

## user

New device: install, one SMS code, allow the two permission sheets — done forever. Nothing to find in menus.

## glossary

- **device setup**: the passkey + push enrolments that make a device fully muon-capable, run once at first login.

## code description

`enrolDevice` in `/gate`'s login page: the passkey branch (register-challenge → `credentials.create` → register), then the push branch (`pushManager.subscribe` with the VAPID key → subscribe endpoint), each guarded, each diag-logged; called from both the PIN-verify and Face-ID success paths.
