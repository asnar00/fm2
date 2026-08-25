# holding
*where a world is kept: made, per person, across a restart, across a reload*

> (transcripts/2026-08-25-accounts.md#p54)
> sounds like we need to work on some kind of "upgrade code without restarting server" workflow so this doesn't happen - once we have multiple users making changes, we'll want that to be silky smooth
> *(the answer named the split: "this is a `context` child (`context` is at the cap → the holding/changing regroup the handover already named)")*

## user

Browse the children: a place builds a world and shows you it (`/alive`), the server keeps one per person (`/per-user`), it survives the server restarting (`/remember`), and it survives the device reloading (`/world-cache`).

## spec

Grouping node, created under the 4–6 children rule: `context` stood at six (alive, edit, enabled, per-user, converge, remember) when `/world-cache` arrived. The split is the one `converge.md` predicted when it took the fifth seat — *holding* from *changing* — and it is a real distinction rather than a filing convenience: the nodes here answer **where a world lives** and for how long, while `/changing`'s answer **how it moves**.

Each child extends the lifetime of the one before it. `/alive` makes a world exist for a process. `/per-user` makes one exist per person rather than per process. `/remember` makes the server's copy outlive the process. `/world-cache` makes the device's copy outlive the page. The order in `order.md` is that ladder.

Provenance-ordered linearisation means the grouping changes no behaviour — verified by an fmlink `--chains` diff before and after, identical modulo paths. Contributes no code, so it orders by its earliest child.

## glossary

(no new terms)

## code description

No implementation files — a grouping node; `order.md` orders the children.
