# sync
*one shared tap count across every miso instance*

> (transcripts/2026-08-13-fm-spec.md#p111)
> What if we had three instances of muon running: one on my iphone, one in the ios simulator on this laptop, and the third in a browser tab also on this laptop. the idea is that the "taps" number would be distributed across all three instances, and tapping any button on any instance would increment the number on all instances.

## user

Tap the pill on any logged-in miso — phone, simulator, tab — and the count moves on all of them within a beat.

## spec

The first shared state, and `/messaging`'s proof: every tap sends `TapSync` through the message pipe; the server keeps one **global** total and publishes `TapTotal` to every listening instance; arriving totals simply overwrite `tap_count`, so `/tap`'s own pill displays the shared number unchanged. Local taps increment optimistically (instant feel), the authoritative total sweeps through a beat later — convergence, not ceremony. Scope is deliberately global (any user, any instance, one number — demo-charming); per-user or per-session scoping is the named follow-up. Requires `/messaging` (without it, sends accumulate undelivered in state — recorded as a known coupling).

## glossary

- **optimistic update**: acting locally at once and letting the authoritative answer sweep through after.

## post-migration (rung 7)

This node no longer ships an op by hand. It declares the shared counter —
`tap_count_shared: Counter = Counter::zero() (global, counter, own)` — and
redefines the three-function seam `/tap` exposes so that reads, taps and resets
all address it. The op that reaches the other devices is the one the declared
merge produces; the escalation this feature is for is now a declaration rather
than a message.

Reads and writes both go to the LAYER. A global var's authority is the shared
layer and its resolver never looks at a user's own field, so a local edit has to
land there too — which is also what keeps the tap optimistic: the number moves
on the screen before the server has heard, and the authoritative total replaces
it a moment later. Offline, the ops queue in the outbox exactly as they always
did, and rung 6b's dedupe means a reconnect lands each of them once.

## code description

`sync.rs` collapsed from ~60 lines to one `update` /extension/ the moment `/scope` existed: on a tap event, `Var::<u64>::global("tap_count").add_op(&mut s, 1)` ships the already-applied increment as an op on the global counter. Server keying, storage, broadcast and arrival are all `/scope`'s generic machinery — this node now contains exactly its intent and nothing else.
