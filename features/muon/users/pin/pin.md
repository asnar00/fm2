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

`pin.rs` provides the four JSON endpoints, wired into routing by `/gate`: `auth_request` (guest-list check, rate limit, PIN issue, send), `auth_verify` (constant-time check, attempts/expiry, cookie on success), `auth_whoami` (names the user via the token's phone), and `auth_logout` (clears the cookie — stateless tokens can't be revoked server-side).

The pending store is a flat file, one line per phone: `load_pending`, `set_pending_line`, `save_pending`, `clear_pending`.

Rate limiting is file-backed too (`sms_count_last_hour`, `record_sms` — 5 texts per phone per hour); `make_pin` draws from urandom.

The base `send_sms` prints to the console — the dev/test delivery; `/vonage` extends it with the real thing.
