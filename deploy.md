# deploy.md — building and shipping miso

## Build and run locally

- Link + build: `python3 tools/fmlink.py miso` (add `--run` to start the
  server from the build dir; serves `site/` on port 8095).
- Products live in `products/<name>/build/`; the linker emits one crate per
  place (miso: `server` native + `client` wasm) plus the composed `site/`.

## Ship

`./tools/deploy.sh` does, in order:

1. refuses a dirty tree — a release is a committed state (`--dirty` overrides
   for a hotfix-in-progress);
2. links the miso product;
3. smoke-tests that `client.wasm` instantiates with ZERO imports (a
   dependency's wasm-bindgen glue once shipped a black screen);
3a. **runs the smoke gate** (`tools/smoke.py`, added 2026-08-25): the ten
   things a user does — lozenge, people, edit, undo, invite, post, project,
   map — headless against this build on port 8140, three passes (cold, warm
   with the world cache primed, throttled network). Any failure stops the
   deploy. `SMOKE=skip ./tools/deploy.sh` bypasses it for a hotfix whose rig
   is the known problem — say so in the commit. Add a step to `STEPS` when a
   surface ships; the tree-owned form (`<name>.smoke.py` per node, composed
   by fmlink) is the named next rung;
4. prints the feature nodes this release touches and warns if a release
   contains no new nodes;
5. exports the feature tree to `site/features/` (public at /features/);
6. stamps the build number (= commit count) into `site/version` and writes
   `changes.json` from recent commit subjects — **commit subjects are the
   changelog and the push-notification text; write them for the user**;
7. ships it — see **the handover** below.

On restart the server announces the new build to push subscribers by itself
(miso/push extends the serve chain) — no deploy-side notification step.

## The handover (2026-08-25, accounts #p54)

A release no longer leaves port 8095 unheld. `features/miso/serve/reuseport`
binds with `SO_REUSEPORT` so two processes may hold the port, and its child
`/handover` is the sequence: the successor binds *beside* the incumbent, then
SIGTERMs it; the incumbent stops accepting, answers its parked `/msg/wait`
polls with their ordinary empty reply, finishes what is in flight and exits 0.
Sub-second, and no connection is refused.

deploy.sh ships the **binary first**, hands over twice (once to a background
process started from the new binary, once back to launchd so the LaunchAgent
owns it), and rsyncs **`site/` last** — so `/version`, the stamp every device
compares, flips only when the new server is already answering.

Two things gate it, and if either is false deploy falls back to the old
kickstart-in-place and says so rather than failing:

- the running build must answer `/admin/whoami` (localhost only; it reports
  `{pid, build, draining}` — the pid is what tells two processes apart, since
  they share one `site/`);
- `~/Library/LaunchAgents/com.noob.miso.plist` must carry
  `KeepAlive = {SuccessfulExit: false}` (a drained server exits 0 and must
  stay down; a crashed one must still restart) and `MISO_HANDOVER=1` (this
  job is always the one arriving). **`tools/com.noob.miso.plist` is that
  plist — install it by hand, once**, after checking its paths against the
  live one. Until then every release deploys the old way.

`POST /admin/drain` (localhost, POST) is the same drain by hand, for stopping
a server with no successor waiting — the port does go quiet then.

## The mini (the public server)

- Host: `microserver@microservers-Mac-mini.local` on the LAN, else
  `microserver@185.96.221.52`; `MISO_HOST` overrides. Both ends are arm64
  darwin, so locally-built binaries run there.
- App: LaunchAgent `com.noob.miso`, working dir `~/miso`, port 8095,
  log `/tmp/miso.log`. **Do NOT touch `com.noob.muon-server`** — despite the
  (old) name it is the dev surface (dev.nøøb.org), a different thing entirely.
- Tunnel: cloudflared (system daemon, config `~/.cloudflared/config.yml`)
  maps `miso.xn--nb-lkaa.org` (= miso.nøøb.org) → localhost:8095. Restart:
  `sudo launchctl kickstart -k system/com.cloudflare.cloudflared`.

## State on the mini (outside the synced tree — deploys can't touch it)

- `~/.miso-auth/`: guest list `users.json` (`_`-prefixed names are test users
  whose PINs go to the log, no SMS), signing `secret`, `pending.txt`,
  `passkeys.txt`, `push-subs.txt`, `challenges.txt`, `last-notified`.
- `~/.agent-config.json`: Vonage credentials (shared with ftr).

## Checking on it

- Live build: `curl -s https://miso.xn--nb-lkaa.org/version`
- Server log: `ssh <mini> tail -f /tmp/miso.log` (auth events, push sends)
- Device reports: `ssh <mini> tail -f /tmp/miso-diag.log` (launches, errors,
  enrolment outcomes — the remote eyes on installed phones)
- The system panel on any device shows its running build and recent changes.

## Speed (added 2026-08-21, hybrid #p37)

- `fmlink.py --quick` builds the debug profile: ~1.2s warm, ~16s cold vs
  minutes for cold release+LTO. For toggle proofs and rig cycles only —
  deploy.sh always builds release.
- Do NOT `export CARGO_TARGET_DIR` for worktree workers (advice
  retracted 2026-08-23): fmlink.py reads `<crate>/target/...` literally,
  so the build succeeds and then dies unable to find the wasm. Until
  fmlink honours the variable, workers build cold (~17s debug) in their
  own worktrees.
- `export_features.py` skips the ~4.5s bake when nothing under features/
  or transcripts/ changed since the last stamp; `--force` overrides.

## Scripting the linker (added 2026-08-25)

`fmlink.py` exits 1 on any link error — but `python3 tools/fmlink.py miso
--quick | tail -1` reports `tail`'s status, not the linker's. Use `set -o
pipefail` (or test for `build OK` in the output) in every rig script;
twice today a link error scrolled past a `| tail` and a rig ran the
previous binary, proving nothing.
