# authority
*graded authority beside the guest list*

> (transcripts/2026-08-23-fm-spec.md#p3)
> yeah, fix all the weaknesses

## user

A guest-list entry can now carry an authority: `{ "name": "Sam", "phone": "+44…", "authority": "support" }`. Leave it off and someone is an ordinary **member** — they act only on their own world, exactly as before. Mark someone **support** and they can act on shared state that everyone sees (the support person who isn't ash). **admin** is full authority; localhost tooling on the mini is always admin.

## spec

Until now the tree had a single privilege check — `overlay`'s `ctx_may_write_layer` — and it was binary: localhost tooling could write the shared layer, and every logged-in user could write only their own world. That holds for two people and fails for a campaign team, where a support person who is *not* the mini needs to act on shared state without being handed the whole machine.

This node puts authority beside the guest list and grades that one check. `authority_of` reads an entry's `"authority"` (default `member`); `authority_rank` orders it (none < member < support < admin); `may_write_shared` is the first, coarsest **blast-radius** test — "shared" is the whole radius and support-and-above covers it. `ctx_may_write_layer` becomes `localhost OR authorised`, so the default stays least-privilege (a member is refused exactly as before) and elevation is explicit, set by ash in `users.json`.

This is the foundation the sketched model builds on, not the whole of it: the richer form — authority as *reachable subtrees* and "enactment requires authority ⊇ blast radius" over graded blast radii — needs enactment machinery the tree doesn't have yet, and is the next rung. What ships here is the authority datum and the graded gate at the real choke point.

## glossary

- **authority**: an optional guest-list field — `member` (default, own world only), `support` (may act on shared state), `admin` (full). Localhost is always `admin`.
- **blast radius**: the reach of an action. Today the only graded distinction is own-world vs shared; `may_write_shared` is the check that a caller's authority covers a shared-reach action.

## code description

`authority.rs` extends `/users` and `overlay`. `authority_of` maps an identity (`local:` → admin, `phone:` → its entry's field, else none) to a level; `authority_rank` orders it; `may_write_shared` is the shared-reach test; `ctx_may_write_layer` is redefined to pass localhost (via `existing`) or any caller `may_write_shared` admits.
