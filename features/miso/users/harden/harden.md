# harden
*revocable sessions, private keys, race-free stores*

> (transcripts/2026-08-23-fm-spec.md#p3)
> yeah, fix all the weaknesses

## user

Two new levers on the mini. Drop someone from `users.json` and their phone and any device they're signed in on stops working on the next request — no waiting a year for a cookie to expire. And `echo $(date +%s)000 > ~/.miso-auth/revoked-before` signs everyone out at once (they just log in again) without touching the signing key.

## spec

The base session was a bearer token an attacker could keep for a year and that a de-listed member kept too: `token_valid` checked only the HMAC and the expiry, never the guest list. This node makes sessions genuinely revocable and closes three primitive weaknesses the red-team found.

**Revocation.** The token carries an issued-at now — `<digits>.<issued>.<expiry>.<hmac>` — and `token_valid` additionally requires that the issue time is at or after the `/revoked-before/` epoch and that the phone is *still on the guest list*. Removing someone is instant; bumping the epoch is a mass sign-out that keeps the key. Changing the token shape invalidates the old one-year cookies once, so everyone re-logs-in (seconds over SMS or Face ID) after this ships.

**Private key.** The 32-byte signing secret is tightened to `0600` on every read, so a fresh key is born owner-only and an old world-readable one is repaired.

**No silent zeros.** A short or failed `/dev/urandom` read is now a hard error instead of being ignored — a failure could otherwise have produced an all-zero PIN, secret, or challenge.

**Race-free stores.** A process-global lock (`with_store_lock`) lets the flat-file stores serialise their read-modify-write. Without it, two concurrent PIN verifies both read `attempts=0` and both write `attempts=1`, so the counter never reaches its limit — unlimited guesses. `/pin` and `/passkey`'s hardening wrap their critical sections in it.

## glossary

- **revoked-before**: `~/.miso-auth/revoked-before`, a millisecond epoch; any session issued before it is dead. Absent = 0 = nothing revoked.

## code description

`harden.rs` redefines `/users`' session and key primitives. `make_token` and `token_valid` move to the four-part issued-at token and add the epoch and guest-list checks; `token_phone` reads the new shape. `secret` chmods the key after the base generates it; `random_bytes` turns a short read into a panic. `revoked_before` reads the epoch file.

`harden.lib.rs` is the verbatim library: `with_store_lock` (a global `Mutex`, poison-recovering) and `fm_own_only` (the unix `0600` chmod, a no-op off unix).
