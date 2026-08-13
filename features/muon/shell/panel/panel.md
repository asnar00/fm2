# panel
*muon's system surface: who you are, what's running, what's changed*

> (transcripts/2026-08-13-fm-spec.md#p57)
> So I think there needs to be some way to a) log out and b) to know when there's an update available, see what's changed, and update. Any thoughts?

## spec

Muon's own UI, distinct from app content. A small logo lozenge bottom-right (#p69 — a button, not a version readout; #p58 — safe-area positioned) opens the panel: logged-in name, running /build number/ with update state ("up to date" / "N available" / "can't reach the server"), the what's-changed list, enrolment buttons for Face ID (`/passkey`) and notifications (`/push`) when the device lacks them (asking the real subscription state, not a cached flag), log out, and update. The handle pulses when `/update` finds a newer build.

## user

Tap the logo lozenge in the corner. Everything administrative lives there: see what changed, update when one's waiting, retry Face ID or notification setup, log out.

## glossary

(the /system panel/ term is defined at `/muon`)

## code description

The panel is part of `/shell`'s loader page: the `#build` handle and `#panel` markup, the open handler (live update re-check, changes fetch, enrolment-state checks), and the button handlers (update, logout, passkey and push enrolment retries).

This node records the intent and the rules; see `/shell` for the code walk-through.
