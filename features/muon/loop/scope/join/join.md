# join
*a booting instance catches up: scoped vars arrive before first paint*

> (transcripts/2026-08-14-fm-spec-2.md#p22)
> yeah - the next muon update should let me restart an instance and have it start at the same nTaps as the others

## spec

Vars previously converged on writes only: a restarted instance booted with
`Default` values (zero taps) and stayed wrong until the next write anywhere.
Joining fixes the gap: on boot the instance sends a `Join` message; the server
replies with a snapshot of every scoped var the sender may hear (its `/scope`
audience: global plus its own user scope), and the values flow into state
through the normal event loop. A fresh boot is just a maximally-stale replica;
the same queued `Join` performs reconnect catch-up after offline, because the
outbox holds it until the network returns. Snapshot application is
last-write-wins at boot; presence and instance identity are named future
refinements (fm-spec-2 #p21).

## user

Nothing to operate: restart an instance (or come back online) and it shows
the same shared values as everyone else, without waiting for someone to act.

## glossary

- **join**: the boot-time (or reconnect-time) act of asking the authoritative
  store for the current values of every var in your hearable scopes.

## code description

`join.rs` server half: `handle_msg` claims the `Join` type — it scans the var
store for `global.*` and `user.<sender>.*` entries, strips the scope prefix to
recover each bare state key, and replies `{"type":"VarJoin","data":{"values":
{key: value, …}}}` (user entries overwrite same-named global ones; other
types delegate to `existing`).

`join.rs` client half: `update` claims `VarJoin` — it writes every entry of
`data.values` into state under its bare key, the plural form of `/scope`'s
single-key `VarUpdate` arrival, then the render chain repaints with true
values.

`join.rs` boot half: `init` extends the init chain to queue `{type:"Join"}`
into the state `_send` outbox — the canonical send path, so the join is
blackbox-visible and a replayed boot re-queues it without delivering
(messaging declines to flush during replay). Offline, the persistent outbox
holds the Join until reconnect.

`join.js` is a boot-race nudge: once boot state exists, it calls
`/messaging`'s drain so the queued Join ships immediately rather than on the
next event (messaging normally drains on apply, but the boot payload can be
applied before messaging wraps apply). The reply becomes an event via
messaging's reply-to-event rule. Absence-guarded like every fragment.
