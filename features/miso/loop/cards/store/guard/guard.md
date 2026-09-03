# guard
*the server never lets a card be lost*

> (transcripts/2026-08-25-accounts.md#p48)
> let's make sure that never happens again - data loss is a sure way to kill user trust

> (transcripts/2026-08-25-accounts.md#p47, the loss)
> the last update dropped my user picture and paragraph

## user

Your cards cannot be lost by a device that was behind. Whatever a phone sends, the server keeps every card it already holds, takes the newer edit of each, and throws away a blank duplicate of a card you already have.

## spec

After build 292 ash's profile card — picture and mission — vanished (#p47). The op log showed why: the app reloaded while the server was restarting, `/veil`'s join timed out, `/me`'s ensure read an empty world, created a fresh blank profile, and `cards`' last-write merge sent that one-card list over the real one. `me.md` had named exactly this failure and called it a later rung. Ash ruled it must never happen again (#p48).

This node is the server's last word on a `cards` set. It extends `handle_msg` ahead of `/converge`'s link and rewrites the op's value before it is applied: the incoming list is **merged into what the user's world already holds** — every held card survives (a set that dropped one is a stale write from a device that had not joined), a card carried by both sides resolves to the newer `edited` (ties to the incoming), new cards append — except a **blank profile for an owner who already has one**, which is the ensure-against-an-empty-world case and is discarded with a log line. The tooling door (`POST /diag/context`) mints the same op and passes the same guard, so a bench restore merges too.

What this means, stated plainly: **a `set` cannot delete a card.** Deleting is not a thing today; when it is, it will be its own op with its own intent, not the absence of an id in a list.

`/me/patient` is the client half of the same ruling: the ensure waits for a real join.

## hostile cases

- A device sends its whole list after being offline for a day: every card it edited lands at its newer `edited`; every card it never saw survives.
- Two devices edit two different cards at once: both edits survive (the whole-list last-write loss is gone at card granularity; same-card concurrent edits still resolve to one).
- The incoming value is not a list: passed through untouched for `/converge` to judge.
- The server holds nothing (a genuinely new user): the incoming list is taken whole.

## glossary

- **blank card**: a card whose blocks carry no text or picture beyond the seeded title.

## code description

`guard.rs` — `handle_msg` intercepts a `CtxOp` set on `miso/loop/cards`/`cards`, reads the world's current list (`cards_read`, on the sender's context as `/attention` does), merges with `cards_guard_merge`, and hands the rewritten op to `existing`. `cards_guard_merge` is the union; `card_is_blank` and `cards_guard_has_type` decide the discard.
