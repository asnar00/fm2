# users
*who can log in: guest list, sessions, signing*

> (transcripts/2026-08-13-fm-spec.md#p39)
> How about user login? We could start with a simple SMS-PIN authenticator (just hold a phone number per user - don't even need a username).

## spec

Identity is a phone number. The guest list is `~/.muon-auth/users.json` — `[{ "name", "phone" }]` — outside the deployed tree (a deploy can never wipe it) and read fresh per request (adding someone needs no restart). Sessions are stateless signed cookies `muon_auth=<digits>.<expiry>.<hmac-sha256>`, valid one year, signed with a 32-byte secret generated once into `~/.muon-auth/secret`; they survive every deploy because nothing per-session is stored server-side. Ported from ftr's proven gate (itself a port of earlier nøøb muon).

## user

Add someone: append `{ "name": "x", "phone": "+44…" }` to `~/.muon-auth/users.json` on the mini. Names starting with `_` are test users (PIN goes to the server log, no SMS). Phone match is digits-only with `+` restored — country code required.

## glossary

- **guest list**: the users.json allowlist; only listed phones can request a login code.
- **session token**: self-describing signed cookie; validity = HMAC check + expiry, no server state.

## code description

`users.rs` owns identity and sessions, used by every other auth feature.

Guest list: `find_user` looks a phone up in `users.json`, comparing via `normalise_phone` (digits only, `+` restored).

Key material: `random_bytes` reads urandom; `secret` generates the 32-byte signing key once; `hmac_sha256` is the standard construction over the `sha2` crate (see `deps.toml`).

Tokens: `make_token` builds the year-long `digits.expiry.hmac` cookie value; `token_valid` re-computes and compares in constant time (`constant_eq`); `token_phone` recovers the phone.

Cookie plumbing: `cookie_token` extracts the token from a Cookie header; `authed` gives the final verdict. `tag` renders log-safe last-4-digit phone references.
