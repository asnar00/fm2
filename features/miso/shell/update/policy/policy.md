# policy
*three update policies: automatic, fixes-auto, everything by consent*

> (transcripts/2026-08-14-fm-spec-2.md#p25)
> There should be three update policies: 1) automatic everything; 2) automatic bugfixes, interface-breaking changes by consent (with optional fine-grained control); 3) everything by consent.

## user

Open the system panel: the "updates" row offers **automatic** (everything
applies by itself), **fixes auto** (bug fixes apply by themselves; anything
with new behaviour pulses the corner handle and waits for you to press
update), and **ask me** (nothing applies without the button). Your choice
follows you to all your devices.

## spec

The update policy is the user's, not the product's: a user-scoped var
(`update_policy`: `auto` | `fixes` | `consent`, default `auto`) chosen from a
picker in the system panel, synced across the user's devices by `/scope` and
restored on boot by `/join`. Releases classify themselves from the tree
discipline: a commit that adds a new feature node ships new behaviour
(`kind: feature`); commits that only edit existing nodes are fixes — deploy
stamps each changes.json entry. Enforcement: `auto` updates as today; `fixes`
auto-applies releases whose pending changes are all fixes and asks otherwise
(unknown coverage counts as asking); `consent` always asks. Asking = the
existing pulsing handle and update button. Known limit: consent gates when
the switch happens (mid-session auto-reload, launch stamping) — the freshness
cache is not a hard version pin. Fine-grained per-feature consent is a named
refinement awaiting runtime contexts (dark-shipped nodes gated by consent
vars).

## the policy moved into the context (rung 7)

`update_policy` is a declared `/var` now — `(user, last-write, own)` with a
`js:update_policy` column — rather than a key in the loop's JSON state. Nothing
a user sees changed: the picker still writes their choice, the choice still
reaches their other devices, and `policy.index.js` still reads
`s.update_policy`, unedited, because rung 7a's bridge republishes the resolved
value before every paint.

The declared default is the empty string, not `auto`, and that is deliberate.
The old var read back `Default::default()` when nothing had been chosen, which
is what makes `policy.index.js`'s `if (s.update_policy) … else
localStorage.misoPolicy || 'auto'` fall through to the launch-time mirror. A
default of `auto` would have made the key truthy from the first frame and
silently disabled the mirror; `auto` stays where it already lives, in the
fallback the fragment writes.

**What this rung takes away, and it is named rather than smuggled:** a
user-scoped SyncVar was in the store `/join` snapshots, so a device booting for
the first time was told the user's policy before it decided anything. A
declared var is not in that store. Today the only thing that carries a var's
value to a never-seen-before instance is the 50-entry broadcast backlog, so a
policy chosen long enough ago has nothing to arrive on. See the risks.

## glossary

- **update policy**: (revises `/auto`'s definition) what an instance does
  upon learning a newer build exists — now the user's three-way choice rather
  than a product-baked behaviour.
- **release kind**: `feature` (adds new behaviour nodes) or `fix` (edits
  within existing nodes), derived from the commit's tree diff.

## code description

`policy.rs` extends `update`: a panel picker click (`policy_auto` /
`policy_fixes` / `policy_consent`) writes `Var::<String>::user
("update_policy")` — storage, sync and boot restoration are `/scope` and
`/join` machinery.

`policy.index.js` owns the page side: it inserts the picker row into the
panel (the `/source` idiom), reflects the current value from loop state and
mirrors it to localStorage (launch runs before join, so launch-time decisions
read the mirror), and enforces: it replaces `feature_Update.consented` (the
extension point `update` exposes) and wraps `feature_Auto.act`, both via
`consentNeeded()` — false for `auto`, true for `consent`, and for `fixes` a
changes.json check: any pending entry of kind `feature`, or pending builds
the list doesn't cover, means ask.

`policy.index.css` styles the three-way segment; the selected choice carries
the `sel` class.

The release classifier lives in deploy (scaffolding): each shipped commit's
kind comes from whether its diff adds a `<name>/<name>.md` spec under
features/.

`policy.vars` declares `update_policy` — user, last-write, own, bridged back to
the page at its own name — and `policy.rs`'s `update_policy_write` is the one
place its address is written.

## risks

**A new device of this user is not told the policy.** `/join` snapshots the
SyncVar store, and a declared `/var` is not in it; the only thing that carries a
migrated var to a never-seen-before instance is the broadcast backlog, which is
fifty entries wide and shared by everyone. Measured on the two-instance rig: with
the backlog rolled over, a fresh profile logged in as the same user reads
`update_policy` as the empty string and the fragment falls back to `auto`. The
same test before the migration read the user's real choice, delivered by
`VarJoin`.

The blast radius is the whole of rung 7, not this node: the already-shipped
`asks` migration fails the same test the same way (a fresh device sees `[]`
once its broadcast has aged out). What the ladder is missing is a **context
join** — a booting instance asking the server for its own world, the exact
analogue of `VarJoin` — and it should be ruled and built before rung 8 declares
absorption complete. Named here rather than worked around, because the honest
workaround (declaring `auto` as the default) would have hidden the gap behind a
value that looks right and is not the user's.

Until then the mitigation is the one the fragment already has: `misoPolicy` in
localStorage, which covers every device that has run once and no device that
has not.
