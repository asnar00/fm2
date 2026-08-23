# login-guard
*passkey sign-in: consume-first, still-a-guest, race-free*

> (transcripts/2026-08-23-fm-spec.md#p3)
> yeah, fix all the weaknesses

## user

Face ID sign-in is unchanged to use. Under it: a passkey belonging to someone who's been removed from the guest list no longer works, and the sign-in no longer answers questions about which credentials exist to a caller who hasn't got a live challenge.

## spec

The base passkey verification was itself sound — origin, rpIdHash, user-verified flag, single-use challenge, ECDSA-P256 all correct. This node closes the three softer edges the red-team found, without changing the crypto.

**Consume the challenge first.** The base looked the credential up by the caller's `id` and returned "unknown passkey" *before* consuming the challenge — a probe for which credential ids are registered. Now the login challenge is validated and consumed first, so a caller without a live challenge learns nothing.

**Still a guest.** The base issued a session for the passkey's stored phone without re-checking the guest list, so a de-listed member's Face ID kept working. It now refuses if the phone has left `users.json`, matching `/harden`'s SMS-path rule.

**Race-free challenge.** The whole login runs under `/harden`'s store lock, so two concurrent sign-ins can't both consume the same one-time challenge (the base rewrote `challenges.txt` unlocked).

Not done here: the WebAuthn signature counter. Apple passkeys are iCloud-synced and always report counter 0, so counter-based clone detection would be dead weight; left out deliberately rather than half-built. Depends on `/harden` for `with_store_lock` (hard link dependency).

## code description

`login-guard.rs` redefines `/passkey`'s `passkey_login`: same checks as the base, reordered so the challenge is consumed before the credential lookup, with a `find_user` guest-list gate before the token issues, all wrapped in `with_store_lock`.
