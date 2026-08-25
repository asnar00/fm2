# code-guard
*strong codes, no membership leak, race-free verify*

> (transcripts/2026-08-23-fm-spec.md#p3)
> yeah, fix all the weaknesses

## user

The texted code is six digits now instead of four, and the login page never confirms whether a number is on the guest list — a stranger and a member get the exact same reply, so nobody can turn the login form into the campaign's phone book.

## spec

Three fixes to the SMS-code path the red-team flagged.

**Uniform six-digit code.** The base drew four digits as `u16 % 10000` — modulo-biased and only ten thousand wide. This draws a uniform six-digit code by rejection sampling, a million-wide space, so the 15-guesses-per-hour ceiling (three attempts across five codes an hour) is negligible.

**No membership oracle.** The base returned `403 "not on the guest list"` for a stranger and `{"ok":true,"name":"…"}` for a member — an unauthenticated caller could probe any number and read back who's on the list and their name. Every request now gets the same opaque `{"ok":true}`; a code is sent only to a real guest under the rate limit, and the reply reveals nothing. (A timing difference remains — sending a text takes longer than not — noted, not closed here.)

**Race-free verify.** Verification runs under `/harden`'s store lock, so the load-check-increment of the attempt counter is atomic: the base did it in three steps, letting two concurrent guesses both see zero and defeat the three-strike limit.

Depends on `/harden` for `with_store_lock` (a hard link dependency — untick `/harden` and the build fails naming the symbol, the `/attention`→`/push` precedent). The login page's code affordance is length-driven (see `/gate`'s login.html) so it follows the code length without a magic number.

## code description

`code-guard.rs` redefines three `/pin` functions. `make_pin` rejection-samples a uniform six-digit code. `auth_request` is rewritten to send a code only to a rate-limited guest and always return the same opaque success, all under `with_store_lock`. `auth_verify` wraps the base in `with_store_lock` to make the attempt counter atomic.
