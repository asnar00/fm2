# handover
*a release replaces the server without the port ever going quiet*

> (transcripts/2026-08-25-accounts.md#p54)
> sounds like we need to work on some kind of "upgrade code without restarting server" workflow so this doesn't happen - once we have multiple users making changes, we'll want that to be silky smooth

## user

Ash deploys while Tara is typing in her card. Her long-poll returns the way it returns every twenty-five seconds; her next one connects to the new server; her edit lands. She never sees a spinner, a login page, or a lost keystroke, and nothing on her screen says a release happened until the build number changes.

## spec

A release used to be a cliff: the old process exited, the port answered nothing for a moment, every open long-poll died in the same instant, and every device rejoined an empty world just as the server came back. `/reuseport` removed the reason the port must go quiet. This node is the sequence that uses it.

**The successor arrives before the incumbent leaves.** The new process binds beside the old one, and only then sends it a SIGTERM. The order matters twice: the port is held by somebody at every instant, and a binary that cannot bind — a bad build, a missing file — dies before it has touched a server that was working. Eviction is by pid and not by a request to the port, because with `SO_REUSEPORT` a request to the port may come back to the sender; a pid names exactly one server.

**One server holds the state directory at every instant, still.** `/sole-tenant`'s rule is not relaxed, it is sequenced. The successor defers the claim rather than refusing on it, evicts, waits for the incumbent to actually be gone, and only then writes its own pid into `server.pid`. If the incumbent does not go, the successor refuses to start and says so: it is the one that gives up, because the incumbent is serving perfectly well and nothing is down. Two servers are bound to the port for the length of the handover, but only one of them is accepting, so no two processes ever write one op log.

**A long-poll is answered, not killed.** The parked `/msg/wait` calls are the reason a drain would otherwise take twenty-five seconds, and a drain that slow would stall the successor's backlog for the same twenty-five seconds — the outage moved, not removed. So the wait horizon is read every tick rather than once, and a draining server shortens it: a parked poll returns its ordinary empty answer within 200ms, the page re-asks at once as it does after any timeout, and the new ask lands on the successor. Nothing is lost by this, because a wait carries `since` and the broadcast slot is a file both processes read — an entry published during the handover is delivered by whichever server answers next.

**Leaving is: stop accepting, finish what is in flight, exit clean.** Stopping is closing the listening descriptor, which removes this process from the port's group in one syscall and hands every new connection to the successor. Finishing is a count of requests inside `route`, waited on for at most `drain_grace_ms` — five seconds, generous for a set of requests that no longer contains a parked poll. The exit is `0` on purpose: the LaunchAgent's `KeepAlive` is `SuccessfulExit=false`, so a drained server stays down and a *crashed* one is still restarted.

**`/admin/whoami` is how a deploy knows the new process is answering.** Both processes serve one `site/`, so the build stamp cannot tell them apart mid-handover; the pid beside it can. `tools/deploy.sh` starts the successor, polls until `whoami` answers with the successor's pid, and only then calls the release done. `POST /admin/drain` is the same drain by hand, for an operator with no successor waiting — the port does go quiet then, which is what asking for it means. Both are localhost-only by `/gate`'s rule (`!r.tunnel`, sound because `/loopback` binds the port there), so neither is reachable from outside the machine.

Untick it and a release is a restart again: the successor refuses to start beside the incumbent (`/sole-tenant`), SIGTERM kills the server where it stands, and `/admin/*` is 404. `/reuseport` stays, and with it the property that the port alone no longer excludes a second server.

## glossary

- **the handover**: the seconds in which two servers are bound to one port, one accepting and one finishing.
- **drain**: stop accepting, answer what is in flight, exit 0.
- **the successor / the incumbent**: the arriving server and the one it replaces.

## code description

`handover.rs` is the sequence and the two endpoints.

`bind_listener` is the whole handover, in the order that keeps the port held: take the listener `/reuseport` made, remember it for the drain, install the SIGTERM watchdog, and — only under `MISO_HANDOVER` — evict the pid in `server.pid` and write this process's own claim in its place. A refusal to start is reserved for the one case that would corrupt data: an incumbent that will not go.

`claim_state_dir` defers under `MISO_HANDOVER` and is otherwise `/sole-tenant`'s, unchanged. The claim it skips is written by `bind_listener` after the eviction, which is the same claim at a later and more honest moment.

`msg_wait_ticks` returns 2 while draining, so a parked wait answers within a tick and a fresh one within two. `/messaging` reads it every tick for exactly this.

`route` answers `admin/whoami` and `admin/drain` for same-host callers and otherwise counts itself in and out of `FM_INFLIGHT`, which is what the drain waits on. This is the outermost link of the route chain, which puts it outside `/edit`'s turn boundary — the risk `edit.md` names. It is sound here only because neither endpoint reads or writes a var: a counter and a pid are all this link touches, and everything that meets the world still enters through the boundary beneath.

`drain_grace_ms` and `handover_grace_ms` are the two waits: how long a leaving server gives its in-flight requests, and how long an arriving one gives the incumbent to go.

`handover.lib.rs` is the machinery, verbatim because it needs `libc`, `#[cfg]` and process-global statics.

`fm_drain_begin` is idempotent — a second SIGTERM, or a drain request racing the signal, sets no second exit timer — and does its waiting on a thread so the request that triggered it can return.

`fm_close_listener` closes the remembered descriptor. The accept loop then fails forever, which `/serve`'s `Err` arm answers by sleeping instead of spinning; the process exits seconds later, so the `TcpListener` is never dropped and the descriptor never double-closed.

`fm_handover_evict` sends SIGTERM and polls `kill(pid, 0)` until the pid is gone or the grace runs out.

`fm_sigterm_handler` and `fm_handover_install`: the handler touches one atomic, which is all a signal handler may honestly do, and a watchdog thread reads it every 50ms and starts the drain.

Off unix — the wasm place compiles this body too — every one of these is a stub, and `MISO_HANDOVER` means nothing there.
