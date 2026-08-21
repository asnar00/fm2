# auto
*instances update themselves — the policy is this node*

> (transcripts/2026-08-14-fm-spec.md#p132)
> as easy as it is to update muon, I'm already chafing at having to press the update button on three separate devices. How about an auto-update policy we can switch off and on?

## user

You never press update. A deploy reaches every open instance within the minute (the corner button may pulse for a moment first); backgrounded and closed instances update on their next launch as always.

## spec

When `/watch` detects a newer build, don't ask — act: the instance performs the same steps as the update button (record the new build, drop the cache, reload) by itself, but only while visible, and never during a `/replay` (a ghost mid-performance is not interrupted). The on/off switch is fm's native one, at the right layer: **inclusion is a product decision** — to turn auto-update off for a product, give it a local override of `shell/update/order.md` with this node unticked (the `/hello_only` mechanism: the shared tree stays untouched; the product carries its own selection). The next deploy of that product returns its fleet to the pulsing button. Named future refinement: a runtime per-user toggle via `Var::<bool>::user` awaits scoped-variable boot hydration (vars currently converge on writes; a relaunched device would forget the setting).

## glossary

- **update policy**: what an instance does upon learning a newer build exists — ask (`/watch` alone) or act (this node).

## code description

`auto.js` wraps `feature_Watch.check` (the JS extension idiom): after the original runs, if a newer build is known, the instance is visible, and no replay is active, it records the server build, deletes the cache, and reloads. The reload-loop guard is inherent: after reloading, running equals server, so the wrapped check goes quiet.
