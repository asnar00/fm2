# handover
*state of play for the next session — written 2026-08-14, end of day 2. The discipline lives in `agents.md`; ops in `deploy.md`; this file is only what's current.*

## where things stand

Live: **build 70** at muon.nøøb.org, 56 nodes. The muon PWA has the full shell (install gate, SMS-PIN + Face ID login with first-login auto-enrolment, push notifications, panel with changelog / update / **view source** / log out), the Elm-style loop, messaging (outbox + long-poll broadcast, identity-stamped, audience-filtered), the scope lattice (`Var<T>`: Local/User/Group/Global as a verbatim `.lib.rs` library), and the observability suite (blackbox ring + keyframes, replay, drive, readout, demo scripts).

**Auto-update is on** (`shell/update/auto`): every visible instance reloads itself within a minute of a deploy. Commit subjects are user-readable — they are the changelog and the push notification. Deploying is the whole release act.

## today's doctrine additions (all recorded in notes.md)

- **Selection is a product decision** (#p132a): order.md in `features/` stays fully ticked (catalog + ordering); switching a feature off is a product-level order.md override (the `products/hello_only` mechanism). Never persist unticks in the shared tree; transient toggle-tests restore in the same breath.
- **The policy is a node**: auto-update is the worked example — behaviour-as-policy ships as an includable node, not a config flag. (Runtime per-user toggles await Var hydration-on-boot.)
- **Readout first**: read the screen with `drive.py readout`; screenshots only for genuine appearance questions (stacking, layout).

## tooling state

- `tools/audit_prompts.py` — the reverse index: prompt → node(s). Default = gap list; `--map`, `--coalesced`, `--orphans`. Current health: zero orphan nodes, zero dangling citations; the one genuine gap is the features-browser interaction cluster (#p9–#p22) still living in `tools/explorer.py` templates.
- `tools/export_transcript.py` — run with `--slug fm-spec --title "fm spec discussion"` BEFORE citing a new anchor. Anchors are append-only; mid-turn messages get rider anchors (`p132a`); edited resends keep their anchor marked *do-not-cite*; snapshot files of the same session alias together by session id (audit handles this).
- Local test rig: `?browser=1&drive=1&readout=1` ghost in the iOS simulator bypasses install and login gates; the drive-mode ghost + a hand-bumped `site/version` is how auto-update was proven.

## named next rungs (in rough order of pull)

1. **Var hydration-on-boot** — vars converge on writes only; a relaunching device forgets. Blocks runtime per-user policy toggles.
2. **Group scope membership model** — `Var::group` exists but group keying awaits a membership design.
3. **Features-browser template migration** — legalize drawers/place/tidy/fmdoc by moving the explorer page templates into node assets (#p9–#p22 debt, confirmed by the audit).
4. **Typed state** — `Var<T>` is the façade; swap innards from JSON to derived structs, then binary wire, without touching consumers.
5. **Typed message routing** — v2 linker generation over `handle(T)` chains; today's type tags are the future type names.
6. **First real app** — will force the muon/apps grouping question.

## small print

- shell is at the 6-child cap; its next child forces a regroup.
- fm.md's ordering section still describes unchecking as exclusion in general — the author may relocate that semantic to products when next editing (their document; report, don't edit).
- Local `_test` user is +15550001111; the mini's is seeded from ftr (always fetch from the mini's users.json, they differ).
- getrandom must stay on the `custom` feature for wasm; deploy.sh smoke-tests zero-import instantiation.
