# loopback
*bind the port to localhost only*

> (transcripts/2026-08-23-fm-spec.md#p3)
> yeah, fix all the weaknesses

## user

Nothing to see: the server still answers on the tunnel exactly as before. What changes is that a device sitting on the same LAN as the mini can no longer reach port 8095 directly and walk straight past the login wall.

## spec

The gate treats a request that did not arrive through the tunnel as trusted local tooling (`/gate`: `!r.tunnel` routes through unauthenticated, and `/per-user` grants it a `local:` world with shared-layer write). That trust is only sound if "not through the tunnel" really means "same host". The base server binds `0.0.0.0`, so anything on the LAN could send a cookieless, header-free request and be trusted — a full authentication bypass by network position.

This node binds the listener to `127.0.0.1` instead. cloudflared connects to `localhost:8095` (see deploy.md), so the tunnel is unaffected; local dev and rigs are on the same host; only direct LAN/off-box access to the port is removed. The `!r.tunnel` trust is now backed by the kernel: a non-tunnel request can only originate on the mini itself.

## code description

`loopback.rs` redefines `bind_host` (the seam `/serve` exposes) to return `127.0.0.1`, so `serve` binds the loopback interface. Nothing else in the serve chain changes.
