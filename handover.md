# handover
*state of play for the next session — written 2026-08-14, end of day 2's second session. The discipline lives in `agents.md`; ops in `deploy.md`; this file is only what's current.*

## where things stand

Live: **build 78** at muon.nøøb.org, 59 nodes. Everything from the last
handover still stands (shell, loop, messaging, scope lattice, observability,
auto-update), plus today:

- **`loop/scope/join`** — a booting or reconnecting instance sends `Join`
  through the outbox; the server replies with a snapshot of its hearable vars
  (global + user.me), applied through the update chain. Restart an instance
  and it shows the same tap count. Confirmed on device.
- **`shell/update/policy`** — the user picks **automatic / fixes auto /
  ask me** in the panel; the choice is `Var::<String>::user("update_policy")`,
  so it syncs across the user's devices and survives restarts (join carrying
  policy — the day's features composing). Releases self-classify: deploy
  stamps each changes.json entry `feature` (commit added a node spec) or
  `fix`. Enforcement gates both launch stamping (`feature_Update.consented`
  hook) and mid-session auto (`feature_Auto.act` wrap).
- **`shell/pwa`** — regroup: {icon, install, pinned} = "being an installed
  app"; shell at 4 children.

## today's doctrine (all recorded in notes.md)

- **Provenance-ordered linearisation (proposal 9) — IMPLEMENTED.** Composition
  order = the timestamp of each node's cited prompt. The tree is grouping +
  selection only; regrouping cannot rewire behaviour; newest is outermost,
  globally; a node may extend anything that existed when it was written. A
  code-bearing node without a citable anchor is a LINK ERROR — provenance is
  load-bearing now. Grouping nodes order by earliest child. Inspect with
  `fmlink.py <product> --chains` (chains + fragment slots + lib/chain ratio).
- **Two-phase feature lifecycle** (#p16): draft features churn in place
  (tweaks amend the node; prompts accumulate); publication freezes the spec —
  after that, behaviour changes are subfeatures, bug fixes move code toward
  the spec. Refinement subtrees are compatibility machinery for consumers,
  not history. Publish = the natural squash point. Open: what marks
  publication concretely.
- **Join / sessions / presence** (#p19–22): boot is a maximally-stale replica;
  join = the catch-up half of the authority model, same act as reconnect.
  No session object: session = (scope key) × (presence); presence is
  server-derived (it holds the long-polls) published as an ordinary var;
  **instance identity** is the one new noun, deferred with presence until
  something renders them.
- **Update policies** (#p25): the policy is the user's var, not the product's
  config. Release kinds fall out of the two-phase discipline. Fine-grained
  per-feature consent = the first named customer for **runtime contexts**
  (new nodes dark-shipped behind consent vars).
- **Document split**: `ideas.md` = the user's passing whims; `notes.md` = the
  co-written notebook including agent observations. Feature browser + linker
  staying outside the tree is a user decision (pinned), not debt.

## tooling state

- `fmlink.py`: `--chains` dump (diffable topology: chains, fragment slot
  orders, lib/chain ratio — 5% lib); chronological linearisation;
  optional-feature fix (`page` fragments skip absent pages; stale
  composition-target pages removed from site/).
- `export_transcript.py`: refuses to overwrite a different session's
  transcript (same-day sessions need distinct slugs — this session is
  `2026-08-14-fm-spec-2.md`).
- `deploy.sh`: changes.json entries now carry `kind` (feature/fix), derived
  from whether the commit adds a `<name>/<name>.md` spec. Conservative:
  regroups read as `feature` (over-asks, never under-asks).
- Hygiene list in notes.md: all items done or pinned.

## next session: PLACES (user's explicit queue)

The distribution/places conversation, properly. Standing material: the
CAPSTONE doctrine (code placeless; placement in the product; semantics pinned
at the distributed end; colocation = optimisation), places.md today is just
`server: native / client: wasm` with entry points. Join filled in catch-up;
what remains is the real vocabulary: stores and authoritative homes, replica
policies, the topology section of product descriptions, linker validation of
feature constraints vs product topology. The first app (located posts +
explorer, walk documenter — see ideas.md) is what will force it: where does
post data live?

## rungs after that (rough pull order)

1. **Typed state** (linker generation: declared vars → derived State struct;
   unlocks binary representation) — deserves a fresh session's full attention.
2. **Group scope membership** — groups are data (membership, invitation);
   blocked-on-need, and located posts will provide the need.
3. **Runtime contexts** — now has a concrete customer (fine-grained update
   consent; dark-shipped nodes gated by per-user vars).
4. **Instance identity + presence** — named in join's spec, awaiting a
   renderer (the multi-device single-surface use case wants them).
5. **Join gates first paint** (#p29, user-requested): hold the interface at
   startup until the join snapshot arrives or a timeout passes — no flash of
   pre-join values. Small node under `loop/scope/join` when taken up.

## small print

- `shell/update` is at the 6-child cap — its next child forces a (now
  behaviour-neutral) regroup.
- Consent gates the switch moment, not a hard version pin — the freshness
  cache will still serve new code on a fresh network load; hard pinning
  awaits versioned caches (noted in policy.md).
- fm.md errata list (notes.md) grew: ordering section is now doubly stale —
  timestamps are the rule, order.md is catalog + selection.
- The laptop's dev server (port 8095) was restarted with the current binary
  mid-session; state unknown after sleep — just `fmlink.py muon --run` fresh.
- Local `_test` user is +15550001111; the mini's is seeded from ftr (fetch
  from the mini's users.json, they differ). getrandom stays on `custom` for
  wasm; deploy smoke-tests zero-import instantiation.
