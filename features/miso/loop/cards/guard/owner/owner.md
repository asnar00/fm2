# owner
*an id keeps its owner*

> (transcripts/2026-08-25-accounts.md#p76)
> ok let's do this. the "user" tool should show all users …
> *(the residual `/exchange` named on delivery: `/guard` merges by id and stamp alone, so any path that can write a list could change who a card belongs to; fixed in the run under the residuals rule, #p50)*

## user

Nobody can take over one of your cards by writing a card with its id: the server keeps the owner it already knows.

## spec

`/guard` resolves a shared id to the newer `edited`, without looking at `owner`. `/exchange`'s red-team showed the consequence: a linked person could mint a card carrying your id, their name and a newer stamp, and hand it over; `/exchange` now refuses to hand such a card on, but the weakness was the guard's, and any future write path would meet it again. This node redefines `cards_guard_merge` to drop, before the merge, any incoming card whose id the world already holds under a different owner — with a log line — then defers to `existing`. Untick and the guard is id-and-stamp again.

## hostile cases

- A legitimate re-send of your own card: same id, same owner → passes.
- Two guests sharing a name: ids are `<owner>.<created>`, so a same-id collision needs the same name AND the same millisecond; the drop is then the safe side.

## glossary

(no new terms)

## code description

`owner.rs` — `cards_guard_merge` filters the incoming list by `cards_owner_changed` and calls `existing` with the rest.
