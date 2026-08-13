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

`push.rs`: `serve` /extension/ (`notify_if_updated` then `existing.serve()`); `route` /extension/ (`push/vapid-key` public, `push/subscribe` cookie-gated); subscription store (`push-subs.txt`, upsert-by-endpoint); `send_push` (encrypt → temp file → curl with `Authorization: vapid`); `encrypt_payload` (RFC 8291), `vapid_jwt` (ES256, raw r‖s), `hkdf_bytes`, `endpoint_origin`. Client halves: sw.js `push`/`notificationclick` listeners, panel enrolment button in `/shell`'s loader. `deps.toml` adds `hkdf` + `aes-gcm`; `p256` gains its `ecdh` feature (declared in `/passkey`'s deps).
