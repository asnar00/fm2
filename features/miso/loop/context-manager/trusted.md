# trusted base

The subtrees the context manager never runtime-gates — not a privilege
exemption (that ruling is deferred to per-user privileges, #p4a) but a
consistency requirement: these deliver the `feature_ticks` var itself, so
gating them would gate the context manager's own senses. The field proof:
unticking `miso/loop` froze a stale tick map in place — the `false` blocked
the very VarUpdate that would have cleared it, making ticks irreversible.

- miso/loop/scope
