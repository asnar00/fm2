# panel
*muon's system surface: who you are, what's running, what's changed*

> (transcripts/2026-08-13-fm-spec.md#p57)
> So I think there needs to be some way to a) log out and b) to know when there's an update available, see what's changed, and update. Any thoughts?

## spec

Muon's own UI, distinct from app content: a corner handle opens a small panel showing logged-in name, running /build number/ with update state, the what's-changed list, enrolment buttons for Face ID (`/passkey`) and notifications (`/push`) when the device lacks them (asking the real subscription state, not a cached flag), log out, and update. The handle's form and placement are subfeatures: `/button` (the logo lozenge) and `/corner` (safe-area positioning).

## user

Tap the logo lozenge in the corner. Everything administrative lives there: see what changed, update when one's waiting, retry Face ID or notification setup, log out.

## glossary

(the /system panel/ term is defined at `/muon`)

## code description

This node owns the panel's markup (`panel.index.html`: shade, sheet, who-line, changes list, enrolment and update/logout rows), its styling (`panel.index.css`), and `panel.index.js` — `feature_Panel.open/close`, which re-checks the build live via `feature_Watch`, words the status via `feature_Honest`, and invites `feature_Passkey`/`feature_Push` to offer enrolment; plus the logout and update button handlers. All references are typeof-guarded, so sibling features can be unticked freely.

The corner button's tap goes through a seam, `feature_Panel.buttonTap`, defaulting to `open()`; `/account` redefines it (and owns the panel's toolbar life).
