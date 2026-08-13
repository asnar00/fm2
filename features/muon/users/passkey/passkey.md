# passkey
*Face ID login via WebAuthn*

> (transcripts/2026-08-13-fm-spec.md#p63)
> Shall we do face ID next?

## spec

Passkey authentication: while SMS-logged-in, the system panel offers "enable Face ID login" — the phone creates a P-256 keypair guarded by Face ID (`navigator.credentials.create`, platform authenticator, resident key, user verification required, attestation none) and the server stores credential id + public key against the phone number (`~/.muon-auth/passkeys.txt`). Thereafter the login page offers "sign in with Face ID": one-time server challenge (5-min expiry, single use), `credentials.get`, and the server verifies clientData (type/origin/challenge), authenticatorData (rpIdHash, UP+UV flags), and the ECDSA P-256 signature over `authenticatorData ‖ SHA256(clientDataJSON)` before issuing the same year session cookie as the PIN flow. SMS-PIN remains bootstrap and recovery. RP id is the public hostname (punycode) — passkeys only function via the tunnel domain, not LAN/localhost. Passkeys sync across the user's devices via iCloud Keychain.

## user

Once: open the system panel (tap the build number) and choose "enable Face ID login", confirm with Face ID. From then on, after any logout, the login screen's "sign in with Face ID" button logs you in with a glance — no SMS. Each device enrols once (or arrives pre-enrolled via iCloud Keychain).

## glossary

- **passkey**: a device-held P-256 keypair unlocked by biometrics; the server holds only the public key.
- **challenge**: a one-time random value the authenticator must sign, preventing replay.

## code description

`passkey.rs`: `route` /extension/ (four endpoints ahead of `/gate`); `passkey_register_challenge`/`passkey_register` (cookie-gated; parses attestationObject CBOR → authData → credential id + COSE EC2 key via `ciborium`); `passkey_login_challenge`/`passkey_login` (public; full verification chain, `p256` for ECDSA-DER, `sha2` for hashes). One-time challenges and stored keys live as flat files beside the other auth state. `deps.toml` adds `p256`, `ciborium`, `base64`, and `getrandom` with its `js` feature (wasm builds). Client halves live in `/gate`'s login page (sign-in button) and `/shell`'s panel (enrolment button); failures report via `/diag`.
