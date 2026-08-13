# deploy.md — building and shipping muon

## Build and run locally

- Link + build: `python3 tools/fmlink.py muon` (add `--run` to start the
  server from the build dir; serves `site/` on port 8095).
- Products live in `products/<name>/build/`; the linker emits one crate per
  place (muon: `server` native + `client` wasm) plus the composed `site/`.

## Ship

`./tools/deploy.sh` does, in order:

1. refuses a dirty tree — a release is a committed state (`--dirty` overrides
   for a hotfix-in-progress);
2. links the muon product;
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
(muon/push extends the serve chain) — no deploy-side notification step.

## The mini (the public server)

- Host: `microserver@microservers-Mac-mini.local` on the LAN, else
  `microserver@185.96.221.52`; `MUON_HOST` overrides. Both ends are arm64
  darwin, so locally-built binaries run there.
- App: LaunchAgent `com.noob.muon`, working dir `~/muon`, port 8095,
  log `/tmp/muon.log`. **Do NOT touch `com.noob.muon-server`** — despite the
  name it is the dev surface (dev.nøøb.org), a different thing entirely.
- Tunnel: cloudflared (system daemon, config `~/.cloudflared/config.yml`)
  maps `muon.xn--nb-lkaa.org` (= muon.nøøb.org) → localhost:8095. Restart:
  `sudo launchctl kickstart -k system/com.cloudflare.cloudflared`.

## State on the mini (outside the synced tree — deploys can't touch it)

- `~/.muon-auth/`: guest list `users.json` (`_`-prefixed names are test users
  whose PINs go to the log, no SMS), signing `secret`, `pending.txt`,
  `passkeys.txt`, `push-subs.txt`, `challenges.txt`, `last-notified`.
- `~/.agent-config.json`: Vonage credentials (shared with ftr).

## Checking on it

- Live build: `curl -s https://muon.xn--nb-lkaa.org/version`
- Server log: `ssh <mini> tail -f /tmp/muon.log` (auth events, push sends)
- Device reports: `ssh <mini> tail -f /tmp/muon-diag.log` (launches, errors,
  enrolment outcomes — the remote eyes on installed phones)
- The system panel on any device shows its running build and recent changes.
