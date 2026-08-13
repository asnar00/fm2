# pin
*SMS-PIN login flow*

> (transcripts/2026-08-13-fm-spec.md#p39)
> We could start with a simple SMS-PIN authenticator (just hold a phone number per user - don't even need a username).

## spec

The code-by-text exchange: `auth_request` checks the /guest list/, rate-limits (5 texts per phone per hour), generates a 4-digit PIN (5-minute expiry, 3 attempts) and sends it via the `send_sms` chain; `auth_verify` checks the PIN (constant-time), and on success issues the /session token/ cookie. Pending PINs and send-times persist to disk in `~/.muon-auth/` — the server restarts on every deploy, and a code already texted out must survive that. Base `send_sms` prints to the console (test/dev); `/vonage` extends it with real delivery.

## user

Enter your phone number, receive a 4-digit code, type it in — logged in for a year on that device. Test users (`_` prefix) read their PIN off the server log instead.

## glossary

- **pending PIN**: a code that has been issued but not yet verified; expires after 5 minutes or 3 wrong attempts.

## code description

`pin.rs`: `auth_request` / `auth_verify` / `auth_whoami` (the three JSON endpoints, wired into routing by `/gate`); pending-store helpers (`load_pending`, `set_pending_line`, `save_pending`, `clear_pending` — flat file, one line per phone); rate-limit helpers (`sms_count_last_hour`, `record_sms`); `make_pin` (urandom); base `send_sms` (console).
