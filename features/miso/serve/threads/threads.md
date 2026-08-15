# threads
*each connection gets its own thread — the server can hold conversations open*

> (transcripts/2026-08-13-fm-spec.md#p115)
> (the three-instance shared-tap demo requires long-polling, which a single-threaded server cannot hold open)

## spec

The base server handles one connection at a time, which made `/messaging`'s long-poll impossible — one waiting client would block everyone. This node wraps the connection handler so every accepted connection runs on its own thread; a parked long-poll costs one thread, not the server. Untick it and miso is honestly single-threaded again. Honest limit, recorded: file-backed state (counters, queues) is now touched concurrently with no locking — read-modify-write races are possible and accepted at demo scale; a locking story arrives when data does.

## user

Nothing visible — miso just serves many clients (and holds `/messaging`'s open waits) at once.

## glossary

(no new terms)

## code description

`threads.rs` is a single `handle` /extension/: spawn a thread, run `existing.handle(s)` inside it — the entire concurrency model as one toggleable chain link.
