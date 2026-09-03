# one-hour
*a code lives an hour, not a day*

> (transcripts/2026-09-03-invite-test.md#p17)
> one hour

> (transcripts/2026-09-03-invite-test.md#p16, the question it answers)
> A day is a long life for a code shown once at the start of a session. If
> you'd rather it died after an hour, or the moment you put the sheet away
> with **done**, that's a one-line change to the token's life and I'll make it.

## user

The code you show at the start of a session stops working an hour after you
made it. Show the sheet again later and it makes a fresh one.

## spec

`/qr` gave a code a day, sized for a canvassing session that might run all
afternoon. The corrected context (`/scan-is-proof`, #p15) is a team signing
itself up in the first minutes of a session; a code that outlives that by a
day is exposure for nothing. Ash: one hour (#p17).

`qr_ttl_ms` is the extensible function `/qr` left for exactly this: the
token's life, read once at mint into the row's `expires`. This node redefines
it to 3 600 000 ms. Everything else — the cap of 25, the two-second floor
between claims, revocation by **new code**, pruning of expired rows — is
unchanged and still measured against `expires`.

**Parked, named:** the two-second floor between claims on one code
(`qr_gap_ms`) was sized for doorsteps; a team scanning one code together may
meet "one moment — try that again" and tap join twice. Its own ask, its own
node.

## hostile cases

- **A code minted before this build.** Its `expires` was stamped at mint and
  stays a day; the next mint is an hour. No retrofit — a live code is not a
  user's data.
- **This node unticked.** A day again.

## code description

`one-hour.rs` — `qr_ttl_ms` returns 3 600 000.
