# private-vapid
*the push signing key, owner-only*

> (transcripts/2026-08-23-fm-spec.md#p3)
> yeah, fix all the weaknesses

## user

Nothing to see. The key that lets the server send notifications is now readable only by its owner, so nobody who can glance at the mini's files can start sending notifications as the campaign.

## spec

The VAPID private key (`~/.miso-auth/vapid-secret`) signs every push; anyone who reads it can send notifications to every subscriber as the campaign. The base wrote it with default `0644`. This tightens it to `0600` on every read, exactly as `/harden` does the session secret — a fresh key is born private, an old world-readable one repaired. Depends on `/harden` for `fm_own_only` (hard link dependency).

## code description

`private-vapid.rs` redefines `/push`'s `vapid_secret` to `fm_own_only` the key file after the base reads or generates it.
