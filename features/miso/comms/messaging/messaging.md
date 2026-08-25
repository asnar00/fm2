# messaging
*typed messages between places: one pipe, offline-first, replies are events*

> (transcripts/2026-08-13-fm-spec.md#p110)
> let's talk about messaging :-)

## user

For agents building features: to send, append `{type: "YourTag", data: {…}}` to state's `_send` in an update extension; to serve it, extend `handle_msg`, claim your tag, return a reply (or `{}`); to react to replies or broadcasts, handle them in `update` — they arrive as events. Offline behaviour is inherited, not written.

## spec

Miso's one crossing point. Client-side: features *send* by appending type-tagged messages (`{type, data}`) to the `_send` key of `/loop` state — sending is data, purity is preserved, the drain moves them into a persistent outbox (localStorage) flushed to `POST /msg` in order (interval-while-visible, on reconnect, immediately on drain); offline, messages simply wait. Server-side: one cookie-gated endpoint feeds the **`handle_msg(msg) -> reply` extension chain** — each feature claims its own type tags, delegating the rest to `existing`; the base answers `{}`. Replies ride the HTTP response back and enter the update chain as ordinary events — receiving needs no new machinery. **Identity**: the endpoint stamps the cookie-proven sender into each message as `_from` before the chain runs — handlers key user-scoped data by it and cannot be lied to by the payload. **Broadcast**: server features `publish(audience, msg)` into a versioned, capped entry list; every client holds a long-poll open (`POST /msg/wait {since}`, ~25s cycles, needs `/threads`) and receives **only the entries its identity may hear** — `global` plus its own `user.<me>` — so scoped values cannot leak between users; arrivals inject as events. Replay-aware throughout: a ghost neither sends nor listens. Deferred deliberately: typed-struct routing (a v2 linker generation over `handle(T)` chains — tags are the future type names), WebSockets (a faster transport for the same shapes), delivery ids/exactly-once, and per-user or per-session scoping of broadcasts.

## glossary

- **message**: a type-tagged JSON value crossing between places.
- **outbox**: the persistent queue of unsent messages; the `_send` state key drains into it.
- **broadcast**: a message published to every listening place via the long-poll.

## code description

`messaging.rs`: the `handle_msg` chain base (unknown tags → `{}`); a `route` /extension/ — `POST msg` (guard: localhost free, tunnel needs a cookie) parses the body and returns the chain's reply; `POST msg/wait` long-polls the broadcast slot (5 checks/second, ~25s timeout, sleeping only its own `/threads` thread); `publish()` bumps the versioned slot other features call.

`messaging.js`: `feature_Messaging` wraps `feature_Loop.apply` (lazily — comms linearises before loop, so the wrap installs by a short poll once `feature_Loop` exists): after every turn, `drain()` moves `_send` into the outbox and flushes; `flush()` posts FIFO, injecting non-empty replies via `feature_Loop.send`; `wait()` is the perpetual long-poll injecting broadcasts; both stand down while `/replay` is active.

*(Refactored 2026-08-25, accounts #p21, behaviour unchanged: the 16KB body
limit in `msg_endpoint` moved into `msg_body_cap()` so a later feature can
widen it — `/roomier` is the first.)*
