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
4. prints the feature nodes this release touches and warns if a release
   contains no new nodes;
5. exports the feature tree to `site/features/` (public at /features/);
6. stamps the build number (= commit count) into `site/version` and writes
   `changes.json` from recent commit subjects — **commit subjects are the
   changelog and the push-notification text; write them for the user**;
7. rsyncs the server binary + `site/` to the mini and kickstarts the agent.

On restart the server announces the new build to push subscribers by itself
(miso/push extends the serve chain) — no deploy-side notification step.

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
