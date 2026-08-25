# singleton
*only a profile is one-per-owner*

> (transcripts/2026-08-25-accounts.md#p87)
> my hitlist before tara comes in: 1) projects; 2) posts; 3) map view.
> *(the residual `/posts` named on delivery: `/guard`'s blank-duplicate rule ate every second post and would eat every second project; fixed in the run under the residuals rule, #p50)*

## user

You can make as many posts and projects as you like, one after another; only your profile is one of a kind.

## spec

`/guard` discards a blank card arriving for an owner who already holds one of its type — the rule that stops an ensure against an empty world minting a second profile. Every other kind is blank at the instant `/new` makes it, so the second post vanished at the server with a log line (#p87's posts worker found it). This node gates the discard on a seam, `cards_type_is_singleton(type)`: true for `profile`, false for everything else until a type asks. `/posts`' own override becomes redundant and harmless. Untick and the discard applies to every type again.

## glossary

- **singleton type**: a card type of which an owner holds at most one.

## code description

`singleton.rs` — `cards_guard_has_type` answers false for non-singleton types, else defers; `cards_type_is_singleton` is the seam.
