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

`users.rs`: `find_user` (guest-list lookup via `normalise_phone`), `secret`/`random_bytes` (urandom-backed key material), `hmac_sha256` (standard construction over the `sha2` crate — see `deps.toml`), `make_token`/`token_valid` (constant-time compare via `constant_eq`), `cookie_token`/`authed` (cookie → verdict), `tag` (log-safe last-4-digits).
