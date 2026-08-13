# sync
*one shared tap count across every muon instance*

> (transcripts/2026-08-13-fm-spec.md#p111)
> What if we had three instances of muon running: one on my iphone, one in the ios simulator on this laptop, and the third in a browser tab also on this laptop. the idea is that the "taps" number would be distributed across all three instances, and tapping any button on any instance would increment the number on all instances.

## spec

The first shared state, and `/messaging`'s proof: every tap sends `TapSync` through the message pipe; the server keeps one **global** total and publishes `TapTotal` to every listening instance; arriving totals simply overwrite `tap_count`, so `/tap`'s own pill displays the shared number unchanged. Local taps increment optimistically (instant feel), the authoritative total sweeps through a beat later — convergence, not ceremony. Scope is deliberately global (any user, any instance, one number — demo-charming); per-user or per-session scoping is the named follow-up. Requires `/messaging` (without it, sends accumulate undelivered in state — recorded as a known coupling).

## user

Tap the pill on any logged-in muon — phone, simulator, tab — and the count moves on all of them within a beat.

## glossary

- **optimistic update**: acting locally at once and letting the authoritative answer sweep through after.

## code description

`sync.rs` extends both halves. Its `update` /extension/ runs the chain first, then: a tap event appends `TapSync` to state's `_send`; an arriving `TapTotal` overwrites `tap_count` with the shared total. Its `handle_msg` /extension/ claims `TapSync`, bumps the file-backed global total, publishes `TapTotal` to all listeners, and returns the same `TapTotal` as the direct reply — so the tapper and the bystanders converge by different roads.
