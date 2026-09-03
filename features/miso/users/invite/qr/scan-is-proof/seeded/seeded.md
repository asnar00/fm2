# seeded
*a scan gets you your inviter's cards, as a texted login does*

> (transcripts/2026-09-03-invite-test.md#p57)
> OK, I signed up as Tara - but I should see the other users in the sevenoaks
> project, I just see Tara.

## user

The moment you are in from a scan, your inviter's cards are already in your
world — their profile is on your people page before you have touched
anything.

## spec

`/exchange` seeds an invitee with their inviter's cards *"the moment they
first get in"* — from the PIN road's `auth_verify`, the only login there was.
`/scan-is-proof` made the claim a login and never called the seed, so Tara's
world held her own card and the project and nobody (#p57). This node calls
`exchange_seed` on a claim that logged the device in.

**The key comes off the cookie, not the body.** `/name-only` is newer than
`/scan-is-proof` and rewrites the body's phone before the chain beneath sees
it; this node is newer still and would read the original, empty one. The
cookie the claim issued is the identity the server chose — `make_token`'s
payload opens with the digits — so the key is `phone:+<digits>` read from
`set_cookie`. Wrong-shaped or absent: no seed.

## hostile cases

- **A claim that did not log in** (a duplicate number, a refusal). No cookie,
  no seed.
- **`/exchange` unticked.** `exchange_seed` is not composed; this node is a
  link on a chain that is not there and the linker says so — the two travel
  together, as `/exchange` and `/invite` do.

## code description

`seeded.rs` — `qr_claim` runs `existing`, and when the answer carries a
`miso_auth` cookie, reads the digits before the first dot and calls
`exchange_seed("phone:+<digits>")`.
