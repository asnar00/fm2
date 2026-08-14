# gate
*first paint waits for the join — or times out and says so*

> (transcripts/2026-08-14-fm-spec-2.md#p31)
> 1) gate first-paint on sync (or timeout and inform)

## spec

The user must never knowingly see stale state (the client-side twin of
`/honest`'s rule). The interface stays hidden behind a quiet "syncing…" veil
until the `/join` snapshot has been applied; if it hasn't arrived within the
timeout (2s from first paint-readiness), the interface shows anyway with a
small banner — "showing local state — server not reachable" — which clears
itself the moment a late join lands. Replay is unaffected: a recorded boot
contains its recorded `VarJoin`, so replayed instances reveal exactly as the
original did. First requested as a future refinement at fm-spec-2 #p29.

## user

Launching muon, you see a brief "syncing…" instead of possibly-outdated
values; on a dead or slow network the app appears after a couple of seconds
with a notice that it's showing local state. You never mistake old numbers
for current ones.

## glossary

- **gate**: the veil between boot and first reveal, lifted by the join
  snapshot or the timeout.

## code description

`gate.rs` extends `update`: when the `VarJoin` snapshot has been applied by
`/join` (gate linearises after it, so the values are already in), it stamps
`_joined: true` into state — a page-local marker, never shipped.

`gate.index.js` owns the veil: it covers the page at load, wraps
`feature_Loop.apply` to watch for the `_joined` marker (reveal + clear any
banner), and starts the 2s timeout at the first apply (paint-readiness, not
script load, so slow wasm fetch doesn't eat the budget); timing out reveals
with the informing banner.

`gate.index.css` hides `#app` until the body carries `fm-joined`, and styles
the veil and the banner (safe-area aware).
