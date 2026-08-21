# raise
*the logo sits one logo-height above centre*

> (transcripts/2026-08-13-fm-spec.md#p61)
> OK. So for a test, shall we move the logo upwards a bit (by its height) and call that build 22 - I'll see whether the app lets me know?

## user

The logo rides high — that's on purpose (eventually).

## spec

Born as a deliberately visible change to test the update-notification loop (build 22), never reverted, and ratified as the intended position when the audit surfaced it (#p86: "I like it where it is now"). The logo renders one logo-height above vertical centre.

## glossary

(no new terms)

## code description

This node owns `raise.css`: `.logo { transform: translateY(-100%) }` — one composable declaration. Untick it and the logo returns to centre.
