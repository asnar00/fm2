# by-key
*localhost tooling can name a real user's world, not just a local: one*

> (transcripts/2026-08-21-hybrid.md#p67a)
> Also, the "proposed" -> "building" feedback isn't working right
> *(root cause, second half: stamp_ask.py wrote to the expunged var
> store, and after fixing that, the repair path itself could not reach
> the asker — `?user=` refuses `:` and `+`, so a `phone:+…`-keyed world
> was unaddressable from the bench that must stamp it)*

## user

For agents: `GET/POST /diag/context?user=<full key>` now accepts a raw
world key — a name containing `:` (e.g. `phone:+447700900123`) is taken
as the key itself, not wrapped in `local:`. Plain names keep their
`local:` namespace exactly as before. Localhost only, as ever: a tunnel
caller still gets no say in whose world it touches.

## spec

`/per-user`'s repair-path promise ("a user who unticks their own
chooser is repaired by a server-side var edit") was unkeepable for real
users: `context_user_name_ok` bounds tooling names to plain characters,
and real worlds are keyed `phone:+…`. This node extends the
`context_user_of` chain: when the standing resolution yields nothing,
the caller is not on the tunnel, and the `user` parameter contains a
`:`, the parameter is accepted as a raw key — bounded to 64 chars of
the key alphabet plus `:` and `+`, matching what `/remember` percent-
encodes for filenames. The tunnel guard is this node's own, so the
cookie-only rule for tunnel traffic survives its unticking trivially.

Unticking this node restores the plain-names-only tooling surface.

## glossary

- **raw key**: a world's full identity (`phone:<number>` or
  `local:<name>`) named explicitly, rather than built from a namespace.

## code description

`by-key.rs`: `feature_ByKey` redefines `context_user_of` — it calls the
previous definition first; only when that answers empty, the request is
not tunnel, and the `user` query parameter contains `:` does it
validate the raw-key alphabet and return the parameter as the key.
