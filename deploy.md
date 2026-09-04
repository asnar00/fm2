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

**The basemap (2026-09-02):** Stadia Alidade Smooth Dark, set by `MISO_TILE_URL`
(with ash's Stadia key) and `MISO_TILE_ATTRIBUTION` in the live plist's
environment — the key is never in the repo (the reference plist carries
`STADIA_KEY`). Changing the ground means: the env, then **`launchctl bootout` +
`bootstrap` of `com.noob.miso`** (launchd reads a plist only at load, and the
deploy's handover starts the successor from the deploy's own shell — build
474 shipped with the old env until the job was reloaded), then
`rm -rf ~/.miso-context/tiles`, and a bump of `/fresh-tiles`' ground tag so
no phone's cache answers with the old squares.

**Product overrides (2026-09-02):** `products/miso/miso/<path>` is a
symlink into `features/miso/<path>` until a product needs its own
`order.md` there; then that path is *materialised* — real directories of
symlinks to each sibling, one real `order.md` — the shape
`products/miso/miso/loop/tap/counter` set and `users/invite/qr` follows
(`instant` unticked for miso). `loop/dictate`, `as-posts` and `capture` were materialised on 2026-09-04 to untick `photo`. Two hazards: writing to
`products/miso/miso/<path>/order.md` while it is still a symlink edits
the shared tree (it happened twice on 2026-09-02 — check `test -L`
first); and a new sibling landing in the shared tree under a materialised
directory needs its symlink added, or the link fails with "includes 'X'
but the folder does not exist" — loud, and the deploy refuses.

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
  `passkeys.txt`, `push-subs.txt`, `challenges.txt`, `last-notified`,
  `invite-qr.json` (`/qr`'s live canvassing codes — one row per inviter,
  expiring; deleting the file revokes every code and costs nothing else).
- `~/.miso-blobs/`: `<world key>/` holds `/dictate`'s clips and their index;
  `pics/<id>` holds `/pic-beside`'s pictures, one file per picture, addressed
  by id alone and never rewritten. A picture is readable by whoever holds a
  card naming it, which is why it can be served to a recipient where a clip
  cannot. **The retrofit is `tools/pics.py`** — dry by default, `--go` to
  write, `--back --go` for the tested inverse; the work is the server's
  `POST pic/retrofit`, screened as `POST diag/context` is. Run the dry first
  and read the byte counts: on the rig a planted old-shape world went
  13,150 → 515 bytes, and `back` restored it byte for byte.
- `~/.agent-config.json`: Vonage credentials (shared with ftr).
- **Wifi watchdog** (2026-09-03, housekeeping #p4): the mini is on wifi
  (`en1`; the ethernet port is empty), and its link dropped four times on
  the evening of 2026-09-02, once for 67 minutes. `tools/wifi_watchdog.sh`
  runs every 30 s from launchd `com.noob.wifiwatchdog` (reference plist in
  tools/): ping the gateway, then 1.1.1.1; after two misses cycle the radio
  once, five-minute cooldown. Transitions (DOWN, CYCLE, UP) and an hourly
  OK line go to `~/wifi-watchdog.log`. A cable into `en0` is still the
  better answer for a field day.

## Checking on it

- Live build: `curl -s https://miso.xn--nb-lkaa.org/version`
- Server log: `ssh <mini> tail -f /tmp/miso.log` (auth events, push sends)
- Device reports: `ssh <mini> tail -f /tmp/miso-diag.log` (launches, errors,
  enrolment outcomes — the remote eyes on installed phones)
- The system panel on any device shows its running build and recent changes.
- **Reset a test user** (2026-09-03): `python3 tools/reset_user.py <name>`
  on the mini (`--list` shows the guest list, `--dry-run` says what would
  go). Their copies in other people's worlds become tombstones through the
  op door, their guest-list row goes to `~/.miso-auth/removed.json`, their
  auth lines go, their world log moves to `~/.miso-context/removed/`, and
  the server restarts by handover so it forgets the world. Nothing is
  deleted; the same number can be invited again at once. Refuses a user
  with authority without `--force`.

- **Delete every non-video post** (2026-09-04, asks#1788503662808):
  `python3 tools/prune_posts.py` lists the still and audio posts in every
  world, `--go` tombstones them (`/delete`'s shape, in the owner's world and
  every copy, through the op door). Recordings in `~/.miso-blobs` stay; the
  op log holds the prior lists. Run once on 2026-09-04: four of ash's posts,
  thirteen copies over four worlds.

## The toggle-proof gate (added 2026-09-02, settings #p4–#p5)

Before the build, deploy.sh runs `tools/toggle_proof.py --since <last
released sha>` (the sha is written to `products/miso/build/released.sha`
when a ship lands; with no file it checks HEAD alone). A commit whose
feature-tree footprint is one node (subtree and own order.md included)
plus a tick added to its parent's order.md is *confined*: it cannot alter
the build without that node, so the toggle proof is implied — for a new
node, that build is the last release. Any other shape must carry a `Toggle-proof:`
trailer in its message or nothing ships. `PROOF=skip` overrides, as
`SMOKE=skip` does — say so in the commit. From a working tree,
`fmlink.py miso --prove` gives the same verdict before you commit. Only
`features/` and `products/` count; tools and documents are not a toggle.

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

**The gate's first-attempt failures were the app** (2026-08-26, five
times on builds 365→370, diagnosed on the sixth): under a freshly installed
service worker with new content, the chooser's `cache: 'no-store'` fetch of
`features/tree.json` never returned, `/panel`'s open awaited it, the sheet
never showed, and when the stuck open finally resolved it raised the shade
over the page — every later click timed out. Only a real change reproduced
it (a relink is old content). Fixed by `/chooser/arrives`. The gate now
keeps its full transcript at `products/miso/build/smoke.log`, and on a
lozenge failure dumps what is under it, the panel's state, fetch timings
and service-worker probes (`SMOKE_DUMP=1` forces the dump on a pass, for
comparison). On a failure: read the log first; theories without a log
went wrong four times that day.

**Nothing links in `products/miso/build` while a deploy runs** (2026-08-26):
deploy.sh links into it and the smoke gate serves from it — a relink hands
the gate a half-written site. Triage proves its own work from the rig
worktree `.claude/worktrees/triage-rig` (its own build dir). Rigs in other
directories are safe alongside a deploy since `/own-slot`: before it, every
server on the machine shared `/tmp/miso-broadcast.json` and a rig's stream
reached the gate's page (three failed gates that morning).

## The simulator rig (2026-08-26, #p164a)

User-level tests of the **installed** app on an iPhone simulator, with real
touches and no screenshots in the loop. `tools/simrig.py`; tests in
`tests/sim/*.json`; the tree's side is `/diag/rig` (a server started with
`MISO_RIG=1 MISO_PORT=8099` tells every page to switch on `/readout`,
`/drive`, a 1 s black-box flush, and to drop its service worker — a rig runs
the code it was given), `/readout/rects` (every element's rectangle, so a
finger goes by selector), and `/blackbox/touches` (what the finger did).

Bring-up, once per device:
1. Rig server from a worktree build: `MISO_RIG=1 MISO_PORT=8099 HOME=<scratch>
   MISO_CONTEXT_DIR=<scratch>/ctx ./server/target/debug/miso_server` — never
   the main checkout's build dir. `fmlink --quick` rebuilds the binary; the
   port lives in the environment, not the source. One rig at a time: readout
   and drive share `/tmp/miso-readout.json` and `/tmp/miso-drive.json`.
   **On the mini port 8099 belongs to `com.user.deadman`** (a launchd
   agent of ash's that bound it the moment a rig let go, 2026-09-02): the
   simulator rig there runs on `MISO_PORT=8098` (8097 is `com.noob.learn`,
   8096 `com.noob.rsc`, 8090 muon; survey `lsof -nP -iTCP -sTCP:LISTEN` and
   the LaunchAgents plists before choosing), and the web clip's URL carries
   the port — re-add the clip if the port changes. **A rig is ended by its OWN
   PID — the one its start wrote to its own file, checked against
   `ps -o command=` before the signal — never by clearing its port, and
   never by a search over every `miso_server` (a worker's fallback killed
   the live server on 2026-09-03; misses.md)**: `lsof -ti :PORT | xargs kill` killed
   ash's learn server on 2026-09-02 when a rig had moved.
   Add `MISO_RIG_KEEP=1` (`/diag/rig/keep-worker`, 2026-09-02) and the rig
   keeps the page's service worker and caches, so the cache path — a mixed
   cache after rapid updates, the self-check's hashing — is under test;
   without it a rig drops both and every fragment reads `uncached`.
   The mini's rig tooling (2026-09-02): `idb` is the fb-idb client in
   `~/.local/rig-venv` (linked at `~/.local/bin/idb`) with the prebuilt
   `idb_companion` from the facebook/idb GitHub release under
   `~/.local/idb-companion` (Homebrew's formula needs newer command line
   tools than the mini has). The web clip's bundle id comes from
   `xcrun simctl listapps <udid>` (`com.apple.WebKit.PushBundle.<hex>`);
   `simctl launch` refuses it — Spotlight is the launcher. The rig's HOME
   needs `.miso-auth/users.json` with the `_` test user before login
   (`[{"name":"_ash","phone":"+15550000998","authority":"admin"}]`). On
   the iPhone 17 Pro (402×874) Safari's share route is: More (344,816) →
   Share (243,542) → View More (335,797) → Add to Home Screen (150,642) →
   Add (352,111). Lessons of the live-location proof (2026-09-02): a web
   clip cannot be launched by bundle id (`simctl launch` and idb both
   refuse) — Spotlight is the only door, and its Top Hit row may carry a
   ghost tile first, so look at the screenshot before tapping; a removed
   clip lingers in `data/Library/WebClips/<id>.webclip` after
   `simctl uninstall` — delete the directory with the sim shut down; a
   Safari tab left on the rig's URL answers the drive door instead of the
   app (`simctl terminate <udid> com.apple.mobilesafari` first); a fresh
   clip asks for precise location once (`simrig press Allow`) before
   `getCurrentPosition` resolves, and the sim's location resets to Apple's
   default on reboot (`simctl location <udid> set lat,lon` after boot).
2. `xcrun simctl erase <udid>; boot`, `simctl openurl <udid> http://localhost:8099/`,
   then the share sheet (iOS 26 points: … 343,814 → Share 201,542 → More
   332,778 → Add to Home Screen 150,590 → Add 357,110). Home; Spotlight
   (201,717; type miso; top hit 63,149) launches it.
3. `simrig login _ash` types the login through `/drive` and reads the code
   from the rig's log; `simrig press Cancel` dismisses the passkey sheet.
   Grant location first: `simctl privacy <udid> grant location-always
   com.apple.WebKit.PushBundle.<id>` (the web clip's bundle, from `listapps`).

Running: `SIM_UDID=… MISO_PORT=8099 SIM_RIG_LOG=… SIM_RIG_HOME=…
python3 tools/simrig.py run tests/sim/pencil-post.json`. Steps: `tap`
(real finger at a selector's rectangle, plus the 62 px status-bar inset of a
standalone app and minus the keyboard's viewport shift), `text` (the sim's
keyboard), `js` (run on the page, value back; setup and assertions only),
`drive`, `home`, `relaunch` (a reboot — iOS keeps a home-screen app alive, so
its icon reloads nothing), `wait_for`, `assert` (`find`/`text`/`face`/`ce`/
`exists`), `shot`. Before every finger the rig checks for a native alert
with `idb ui describe-point` and presses Allow/Cancel/… by label.

Known limits: the iOS 18.6 simulator never raises its soft keyboard (26 does);
`idb ui text` cannot type into the login's tel field on 18 (use the keypad
or `/drive`); a real phone bug is diagnosed from `/touches` first, then
replayed here, then in `tools/smoke.py`.

## Working on the mini (2026-08-28)

fm2 lives on the mini by default now: `~/fm2` on `microserver@` the mini,
with a persistent tmux session `fm2` that claude runs inside. From this Mac
type **`mini`** (`~/.local/bin/mini`, not in the repo) — it attaches over
mosh via Tailscale when the mini is on the tailnet, else plain ssh to the
public address, forwarding the ssh agent so `git push` on the mini uses
this Mac's GitHub key. `mini <cmd>` runs a one-off command instead.
Detach with `C-b d`; the session, and claude in it, keep running.

**Attaching without tmux's copy mode (2026-09-02):** from iTerm2 on the laptop, `ssh -t microserver@microservers-Mac-mini.local 'tmux -CC new -A -s fm2'` — tmux control mode; the session's windows become native iTerm2 tabs with native scrollback, and closing the laptop only detaches. In plain tmux the mini's `~/.tmux.conf` has `mouse on` (the wheel scrolls the pane, no mode switch; `prefix m` toggles it off for a native select). A claude started straight in an ssh shell dies with the connection — this afternoon's session was; start inside tmux.

On the mini `deploy.sh` notices it *is* the mini and ships to `localhost`
(the box can ssh itself). Toolchain there: rustup + wasm32 target, brew
python 3.12 with playwright (`~/.local/bin/python3` points at it; brew's
3.14 pip is broken), Chrome for the smoke gate, Xcode, tmux, mosh, claude
(native install in `~/.local/bin`; the old npm one is parked as
`/usr/local/bin/claude.npm-2.1.56.old`). Claude's memory for the project
lives at `~/.claude/projects/-Users-microserver-fm2/memory/` — a copy of
this Mac's, made once; the mini's is the live one from here on.

**Fetched artifacts the site needs** (gitignored; a fresh clone has none —
builds 401–406 shipped without them, misses.md 2026-08-28): the whisper
model (`tools/fetch_stt.py` → `features/miso/loop/dictate/phone/assets/stt/`,
133 MB) and the semantic-find table (`tools/fetch_find.py` →
`features/miso/loop/compute/semantic-find/assets/find/`). deploy.sh refuses
to ship without the model.

This Mac stays a working clone; nothing stops a local session, but the
mini is where sessions start unless there's a reason.
