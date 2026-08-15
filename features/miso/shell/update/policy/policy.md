# policy
*three update policies: automatic, fixes-auto, everything by consent*

> (transcripts/2026-08-14-fm-spec-2.md#p25)
> There should be three update policies: 1) automatic everything; 2) automatic bugfixes, interface-breaking changes by consent (with optional fine-grained control); 3) everything by consent.

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

## user

Open the system panel: the "updates" row offers **automatic** (everything
applies by itself), **fixes auto** (bug fixes apply by themselves; anything
with new behaviour pulses the corner handle and waits for you to press
update), and **ask me** (nothing applies without the button). Your choice
follows you to all your devices.

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
