# reuseport
*two processes may hold the server port at once*

> (transcripts/2026-08-25-accounts.md#p54)
> sounds like we need to work on some kind of "upgrade code without restarting server" workflow so this doesn't happen - once we have multiple users making changes, we'll want that to be silky smooth

## user

Nothing to see, which is the point: a release stops being a moment in which the port answers nothing. On its own this node changes no behaviour a person can observe — it removes the reason a second server cannot start. `/handover` is what uses it.

## spec

A release restarts the server, and between the old process exiting and the new one binding, port 8095 is not held by anybody: cloudflared gets `connection refused`, every open long-poll dies at once, and every device rejoins an empty world in the same second. The gap is small and it is the whole problem — it is the cliff `/guard` and `/patient` were built to survive rather than to avoid.

The gap exists only because one socket may hold one address. `SO_REUSEPORT` removes that rule: several sockets may bind the same address and port, and the kernel hands each new connection to one of them. A successor can therefore be listening *before* the incumbent stops, and the port is never unheld.

This node changes how the listener is made and nothing else. The socket is built by hand — `socket`, `SO_REUSEADDR`, `SO_REUSEPORT`, `bind`, `listen` — because the standard library exposes no socket options, and is then handed to `std::net::TcpListener` as a raw descriptor, so every line above it is unchanged. `SO_REUSEADDR` travels with it for the ordinary reason: a restart no longer waits out `TIME_WAIT`.

Which interface is bound is still `/loopback`'s decision — the socket takes whatever `bind_host` says — so nothing here widens who can reach the port. What it does widen is who may *hold* it: any process on this machine that can bind the port now can, where before the first one excluded the rest. The honest statement is that the port has stopped being the thing that enforces one server; the state directory is, and `/sole-tenant` is where that is enforced.

Untick it and the bind is std's again: a second process gets `Address already in use`, and `/handover` — a child, so it goes with it — is gone too.

## glossary

- **SO_REUSEPORT**: the socket option that lets several listening sockets share one address and port, so a successor can be up before its predecessor is down.

## code description

`reuseport.rs` redefines `bind_listener` — the seam `/serve` exposes — to call `fm_bind_reuseport` with the host `/loopback` chose and the port `serve_port` names.

`reuseport.lib.rs` is `fm_bind_reuseport`, verbatim Rust because it needs `libc` and `#[cfg]`: it makes an `AF_INET` stream socket, sets `SO_REUSEADDR` and `SO_REUSEPORT`, fills a `sockaddr_in` (with `sin_len` on Darwin), binds, listens with a backlog of 128, and adopts the descriptor with `TcpListener::from_raw_fd`. Off unix — the wasm place compiles this body too — it is a plain `TcpListener::bind`, which is what that place would have done and never runs anyway.
