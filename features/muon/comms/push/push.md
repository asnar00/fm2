# push
*notifications that reach the device even when the app is closed*

> (transcripts/2026-08-13-fm-spec.md#p66)
> OK let's do notifications next :-D

## spec

Web Push for the installed PWA (iOS 16.4+; installed-only, per the PWA-only principle). Enrolment: the system panel's "enable notifications" button (permission must ride a user gesture) subscribes via the service worker with the server's VAPID public key and stores the subscription (endpoint + client public key + auth secret) per logged-in device. Sending implements the protocol directly: VAPID ES256 JWT (`p256`) + RFC 8291 payload encryption (ECDH → HKDF-SHA256 → AES-128-GCM, aes128gcm framing; `hkdf`, `aes-gcm`), delivered with `curl`. First use: deploy announcements — `push` extends the `serve()` chain, so on every restart (= every deploy) the server compares `site/version` with the last build it announced and notifies every subscription, message body drawn from the changes list. Expired subscriptions (404/410) are dropped. Verified end-to-end with a synthesized browser: captured push decrypted per-RFC, JWT signature checked.

## user

Once per device: tap the build number → "enable notifications" → allow. From then on, deploys announce themselves as notifications — app closed or not. Tapping the notification opens muon.

## glossary

- **subscription**: a device's push address (endpoint at the platform's relay) plus the keys needed to encrypt messages only it can read.
- **VAPID**: proof to the push relay that the sender is muon's server, via a signed token.

## code description

`push.rs` extends two chains. Its `serve` /extension/ runs `notify_if_updated` before `existing.serve()` — since the server restarts on every deploy, comparing `site/version` with the last announced build at startup *is* the deploy-notification trigger. Its `route` /extension/ adds `push/vapid-key` (public) and `push/subscribe` (cookie-gated).

Subscriptions live in `push-subs.txt`, upserted by endpoint; delivery failures with 404/410 prune the line.

`send_push` does the wire work: encrypt the payload, write it to a temp file, and POST it with `curl` under an `Authorization: vapid` header. `encrypt_payload` implements RFC 8291 (ephemeral ECDH → HKDF-SHA256 → AES-128-GCM, aes128gcm framing); `vapid_jwt` signs the ES256 token (raw r‖s signature); `hkdf_bytes` and `endpoint_origin` support them.

The client halves live elsewhere: sw.js carries the `push`/`notificationclick` listeners, and `/shell`'s panel has the enrolment button.

`deps.toml` adds `hkdf` and `aes-gcm`; `p256` gains its `ecdh` feature (declared in `/passkey`'s deps, which owns the p256 line).
