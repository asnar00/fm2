#!/bin/sh
# Deploy the miso product to the Mac mini (public at https://miso.nøøb.org via
# cloudflare tunnel, ingress miso.xn--nb-lkaa.org -> localhost:8095).
#
# Builds the miso product (fm linker: native server + wasm client + site/),
# ships the server binary and site to ~/miso on the mini (both machines are
# arm64 darwin, so the local build runs there), and restarts com.noob.miso.
set -e
SRC="$(cd "$(dirname "$0")/.." && pwd)"

# the mini on the home LAN, else its public address; MISO_HOST overrides both
pick_host() {
  [ -n "${MISO_HOST:-}" ] && { echo "$MISO_HOST"; return; }
  # running on the mini itself (fm2 lives there since 2026-08-28): ship to
  # localhost — the ssh/rsync steps below then just loop back
  [ "$USER" = microserver ] && [ -d "$HOME/miso" ] && { echo localhost; return; }
  for h in microserver@microservers-Mac-mini.local microserver@185.96.221.52; do
    if ssh -o BatchMode=yes -o ConnectTimeout=5 "$h" true 2>/dev/null; then echo "$h"; return; fi
  done
  echo "deploy: can't reach the mini on the LAN or its public address" >&2
  exit 1
}

# a release is a committed state; 'deploy.sh --dirty' overrides for a hotfix
if [ "${1:-}" != "--dirty" ] && [ -n "$(cd "$SRC" && git status --porcelain)" ]; then
  echo "deploy: working tree is dirty — commit first (or 'deploy.sh --dirty'):" >&2
  (cd "$SRC" && git status --short) >&2
  exit 1
fi

HOST="$(pick_host)"
echo "deploying to $HOST"

# the toggle-proof gate (/confined, settings #p4–#p5): a commit whose tree
# footprint is one node plus a tick in its parent's order.md has its toggle
# proof implied — the unticked build is the last release. Any other shape
# must carry a 'Toggle-proof:' trailer saying the untick was done. Checked
# for every first-parent commit since the last released sha (written below
# once the ship lands); PROOF=skip overrides for a hotfix — say so in the
# commit.
RELEASED="$SRC/products/miso/build/released.sha"
if [ "${PROOF:-check}" != "skip" ]; then
  if [ -f "$RELEASED" ]; then
    python3 "$SRC/tools/toggle_proof.py" --since "$(cat "$RELEASED")" || {
      echo "deploy: a commit needs its toggle proof recorded — nothing shipped" >&2; exit 1; }
  else
    python3 "$SRC/tools/toggle_proof.py" HEAD || {
      echo "deploy: HEAD needs its toggle proof recorded — nothing shipped" >&2; exit 1; }
  fi
fi

python3 "$SRC/tools/fmlink.py" miso

# replay sessions are local-only test data — never ship one
rm -f "$SRC/products/miso/build/site/replay.json"

# the loader instantiates the wasm with ZERO imports — refuse to ship a build
# that quietly grew import requirements (a dependency's wasm-bindgen glue once
# turned the deployed app into a black screen)
node -e '
const fs = require("fs");
WebAssembly.instantiate(fs.readFileSync(process.argv[1]), {})
  .then(({instance}) => { if (!instance.exports.fm_entry) throw new Error("no fm_entry"); })
  .catch(e => { console.error("deploy: wasm smoke test FAILED:", e.message); process.exit(1); })
' "$SRC/products/miso/build/site/client.wasm"

# the smoke gate (tools/smoke.py, accounts #p96): the ten things a user does,
# headless, against this build on a scratch port — cold, warm (world cache
# primed) and throttled — so a boot-timing race that one pass hides and
# another shows is caught here and not on a phone. Green or nothing ships.
# SMOKE=skip bypasses it for a hotfix whose rig is known to be the problem;
# say so in the commit.
if [ "${SMOKE:-run}" != "skip" ]; then
  # the whole gate transcript is kept beside the build, so a failure can be
  # read pass by pass afterwards (three first-attempt failures on 2026-08-26
  # went undiagnosed because only a grep of the output survived)
  python3 "$SRC/tools/smoke.py" --port "${SMOKE_PORT:-8140}" 2>&1 | tee "$SRC/products/miso/build/smoke.log"
  if [ "${PIPESTATUS[0]}" != "0" ]; then
    echo "deploy: the smoke gate failed — nothing shipped (products/miso/build/smoke.log)" >&2; exit 1; fi
fi

# provenance visibility: which feature nodes does this release touch, and did
# any capability ship without a node? informational — the judgment stays
# human, but the omission becomes visible at the moment of shipping
LIVE=$(curl -s --max-time 5 https://miso.xn--nb-lkaa.org/version 2>/dev/null | tr -cd '0-9')
NOW=$(cd "$SRC" && git rev-list --count HEAD)
if [ -n "$LIVE" ] && [ "$NOW" -gt "$LIVE" ] 2>/dev/null; then
  N=$((NOW - LIVE))
  echo "shipping $N commit(s) — feature nodes touched:"
  (cd "$SRC" && git diff --name-only "HEAD~$N" HEAD -- features/ 2>/dev/null \
    | xargs -n1 dirname 2>/dev/null | sort -u | sed 's/^/  /')
  if ! (cd "$SRC" && git diff --name-only --diff-filter=A "HEAD~$N" HEAD -- features/ 2>/dev/null | grep -q '\.md$'); then
    echo "  NOTE: no new feature nodes in this release — did every new capability get its node?"
  fi
fi

# the feature tree, statically rendered — served publicly at /features/
python3 "$SRC/tools/export_features.py"

# the on-device whisper model ships with the site; without it every phone's
# transcription fails silently (the mini's fresh clone shipped six builds
# without it, 2026-08-28). The recipe is tools/fetch_stt.py.
# Asked only while /phone is ticked FOR THIS PRODUCT: since 2026-09-04 miso
# transcribes on the mini and unticks it, and a deploy must not demand 133 MB
# of model that nothing composes (dictate/transcribed).
if grep -qE '^- \[x\] +phone' "$SRC/products/miso/miso/loop/dictate/order.md" 2>/dev/null \
   || [ ! -f "$SRC/products/miso/miso/loop/dictate/order.md" ]; then
  if [ ! -d "$SRC/features/miso/loop/dictate/phone/assets/stt/models" ]; then
    echo "deploy: the STT model is absent — run tools/fetch_stt.py first (or STT=skip)"
    [ "${STT:-}" = "skip" ] || exit 1
  fi
fi

# catalog embeddings for on-device semantic find (loop/compute/semantic-find);
# skipped gracefully if the potion table hasn't been fetched on this machine
if [ -f "$SRC/features/miso/loop/compute/semantic-find/assets/find/table.bin" ]; then
  python3 "$SRC/tools/embed_catalog.py"
else
  echo "  NOTE: potion table absent (tools/fetch_find.py) — vectors.json not refreshed"
fi

# deploy stamp: the client compares this on launch and self-refreshes on change.
# a plain increasing integer (the commit count — every release is a commit, so
# this needs no counter file and still names an exact commit for debugging)
(cd "$SRC" && git rev-list --count HEAD) > "$SRC/products/miso/build/site/version"

# what's-changed list for the system panel: recent commit subjects, newest
# first, each tagged with its build number (count minus offset)
python3 - "$SRC" > "$SRC/products/miso/build/site/changes.json" <<'PY'
import json, re, subprocess, sys
src = sys.argv[1]
count = int(subprocess.check_output(["git", "rev-list", "--count", "HEAD"], cwd=src))
lines = subprocess.check_output(["git", "log", "--format=%H%x09%s", "-12"],
                                cwd=src, text=True).splitlines()

# release kind, from the tree discipline: a commit that ADDS a feature node
# spec (features/**/<name>/<name>.md) ships new behaviour; else it's a fix.
# paths = feature node dirs the commit touched, added = node specs it added
# (both features/-relative, the same paths tree.json speaks) — the review
# workflow reads these to badge and pre-tick proposed additions (#p54, #p2)
def touched(sha, filt):
    args = ["git", "diff-tree", "-r", "--name-only", "--no-commit-id"]
    if filt:
        args.append("--diff-filter=" + filt)
    return subprocess.check_output(args + [sha], cwd=src, text=True).splitlines()

def node_paths(files):
    out = set()
    for p in files:
        m = re.match(r"features/((?:[^/]+/)*[^/]+)/[^/]+$", p)
        if m:
            out.add(m.group(1))
    return sorted(out)

entries = []
for i, line in enumerate(lines):
    sha, subject = line.split("\t", 1)
    added = [re.match(r"features/(.*)/[^/]+$", p).group(1)
             for p in touched(sha, "A")
             if re.match(r"features/(?:.*/)?([^/]+)/\1\.md$", p)]
    entries.append({"build": count - i, "text": subject,
                    "kind": "feature" if added else "fix",
                    "paths": node_paths(touched(sha, None)),
                    "added": sorted(added)})
print(json.dumps(entries))
PY

# the update delta's ground truth: a content hash per site file (see
# review/delta). version/changes.json/hashes.json are always-fresh data, not
# cached app files — excluded so a data-only release reads as "no change"
python3 - "$SRC/products/miso/build/site" > "$SRC/products/miso/build/site/hashes.json" <<'PY'
import hashlib, json, os, sys
site = sys.argv[1]
skip = {"version", "changes.json", "hashes.json", "replay.json"}
hashes = {}
for root, _, files in os.walk(site):
    for f in files:
        p = os.path.join(root, f)
        rel = os.path.relpath(p, site)
        if rel in skip or f == ".DS_Store":
            continue
        h = hashlib.sha1()
        with open(p, "rb") as fh:
            for chunk in iter(lambda: fh.read(1 << 20), b""):
                h.update(chunk)
        hashes[rel] = h.hexdigest()[:16]
print(json.dumps(hashes, sort_keys=True))
PY

# ---- shipping it, without the port ever going quiet (features/miso/serve/
# reuseport/handover, accounts #p54) --------------------------------------
#
# The binary goes first and the site goes last, with the handover between
# them. That order is deliberate:
#
#   * the binary is rsynced by rename, so the running server keeps executing
#     the inode it started with and nothing about it changes;
#   * the successor starts from the NEW file, binds beside the incumbent
#     (SO_REUSEPORT) and asks it to leave, so the port is held throughout;
#   * site/ lands only once the new server is answering, so /version — the
#     deploy stamp every device compares — flips at the moment the release is
#     actually live, and never announces a build the server is not serving.
#
# Two handovers, not one, because the LaunchAgent must end up owning the new
# process: the first is to a plain background process started from the new
# binary, the second hands it back to launchd. Each is sub-second.

miso_pid() {  # the pid answering the port right now, or empty
  ssh "$HOST" 'curl -s --max-time 3 http://127.0.0.1:8095/admin/whoami' 2>/dev/null \
    | sed -n 's/.*"pid":\([0-9]*\).*/\1/p'
}

wait_for_pid() {  # $1 = the pid that must be answering, $2 = seconds
  n=0
  while [ "$n" -lt "$2" ]; do
    [ "$(miso_pid)" = "$1" ] && return 0
    n=$((n + 1))
    sleep 1
  done
  return 1
}

wait_for_change() {  # $1 = the pid that must STOP answering, $2 = seconds
  n=0
  while [ "$n" -lt "$2" ]; do
    p="$(miso_pid)"
    [ -n "$p" ] && [ "$p" != "$1" ] && return 0
    n=$((n + 1))
    sleep 1
  done
  return 1
}

# is the live system ready for a handover? Three things must be true, and any
# of them being false is a reason to deploy the old way rather than to fail:
# the running build must carry /handover (it answers /admin/whoami), and the
# LaunchAgent must carry the two keys tools/com.noob.miso.plist explains.
LIVEPID="$(miso_pid)"
PLIST=$(ssh "$HOST" 'cat ~/Library/LaunchAgents/com.noob.miso.plist' 2>/dev/null || true)
HANDOVER=no
case "$PLIST" in
  *SuccessfulExit*) case "$PLIST" in *MISO_HANDOVER*) [ -n "$LIVEPID" ] && HANDOVER=yes ;; esac ;;
esac

if [ "$HANDOVER" = yes ]; then
  echo "handover: pid $LIVEPID is serving; shipping the binary first"
  rsync -a "$SRC/products/miso/build/server/target/release/miso_server" "$HOST:miso/"

  # 1. a successor from the new binary, beside the incumbent. Its pid is not
  #    read back over ssh: a backgrounded remote command keeps the channel
  #    open until it exits, so `echo $!` through ssh hangs for the life of the
  #    server. All three descriptors are detached and the pid is learned the
  #    honest way — by asking the port who is answering it now.
  ssh "$HOST" 'cd ~/miso && MISO_HANDOVER=1 nohup ./miso_server \
                 < /dev/null >> /tmp/miso.log 2>&1 &' > /dev/null 2>&1
  echo "handover: a successor is starting beside it"
  if ! wait_for_change "$LIVEPID" 40; then
    echo "deploy: the successor never took over — pid $(miso_pid) is still" >&2
    echo "  serving, which means nothing is down. Check /tmp/miso.log." >&2
    exit 1
  fi
  NEWPID="$(miso_pid)"
  echo "handover: pid $NEWPID is answering the port"

  # 2. hand it back to launchd, which starts from the same new binary and
  #    evicts the successor in turn (its plist carries MISO_HANDOVER=1)
  ssh "$HOST" '
    launchctl kickstart "gui/$(id -u)/com.noob.miso" 2>/dev/null ||
      launchctl bootstrap "gui/$(id -u)" ~/Library/LaunchAgents/com.noob.miso.plist
  '
  AGENTPID=$(ssh "$HOST" '
    launchctl print "gui/$(id -u)/com.noob.miso" 2>/dev/null |
      sed -n "s/^[[:space:]]*pid = \([0-9]*\).*/\1/p" | head -1')
  echo "handover: launchd took it back as pid ${AGENTPID:-?}"
  if [ -n "$AGENTPID" ] && ! wait_for_pid "$AGENTPID" 40; then
    echo "deploy: WARNING: the LaunchAgent is not the process answering the port." >&2
    echo "  pid $(miso_pid) is serving; check /tmp/miso.log before the next release." >&2
  fi

  # 3. and only now the site, so /version names a build that is being served
  rsync -a --delete "$SRC/products/miso/build/site" "$HOST:miso/"
else
  # the old way: rsync everything, restart in place. Used on the release that
  # first ships /handover (the running build has no /admin/whoami yet), and
  # any time the LaunchAgent has not been updated.
  rsync -a --delete \
    "$SRC/products/miso/build/server/target/release/miso_server" \
    "$SRC/products/miso/build/site" \
    "$HOST:miso/"

  ssh "$HOST" '
    launchctl kickstart -k "gui/$(id -u)/com.noob.miso" 2>/dev/null ||
      launchctl bootstrap "gui/$(id -u)" ~/Library/LaunchAgents/com.noob.miso.plist
  '
  if [ -z "$LIVEPID" ]; then
    echo "  NOTE: the running build has no /admin/whoami — this release restarts"
    echo "  the server the old way. The next one can hand over."
  else
    echo "  NOTE: ~/Library/LaunchAgents/com.noob.miso.plist is missing KeepAlive"
    echo "  {SuccessfulExit:false} and/or MISO_HANDOVER=1, so this release"
    echo "  restarts the server the old way. tools/com.noob.miso.plist is the"
    echo "  version that hands over; install it once, by hand."
  fi
fi

# one server per state directory (6a's ruling). The refusal itself lives at
# boot, where a second process actually appears — features/miso/loop/context/
# remember/sole-tenant claims ~/.miso-context and refuses to start if another
# live miso holds it. Deploy asserts the OUTCOME: more than one server on the
# mini means the release just restarted one of two, which no release should
# hide. Read-only, and never fatal to a deploy.
live=$(ssh "$HOST" 'pgrep -x miso_server | wc -l' 2>/dev/null | tr -d ' ' || echo 0)
if [ "${live:-0}" -gt 1 ]; then
  echo "  WARNING: $live miso_server processes are live on $HOST."
  echo "  One state directory, one server: the extra one is not serving the"
  echo "  tunnel, and both are writing ~/.miso-context. Stop it, or give it its"
  echo "  own MISO_CONTEXT_DIR."
fi

# the ask inbox: shipping without answering becomes visible at the moment of
# shipping — print every user ask still status "asked" (see noob-button/ask).
# Asks live in each asker's world since rung 7, so the source is the context op
# logs, not the var store rung 8 deleted; the last `asks` op in a log is that
# user's current list. Until 2026-08-22 this read /tmp/miso-vars and had been
# finding an empty cupboard since the ladder — reporting "nothing outstanding"
# whatever was true, which is the one thing /honest forbids.
ssh "$HOST" 'cat ${MISO_CONTEXT_DIR:-$HOME/.miso-context}/*.log 2>/dev/null' | python3 -c '
import json, sys
latest = {}
for line in sys.stdin:
    line = line.strip()
    if not line or "\"asks\"" not in line: continue
    try: op = json.loads(line)
    except Exception: continue
    if op.get("name") != "asks": continue
    v = op.get("value")
    try: asks = json.loads(v) if isinstance(v, str) else (v or [])
    except Exception: continue
    for a in asks:
        if a.get("t") is not None: latest[a["t"]] = a
for _, a in sorted(latest.items()):
    if a.get("status") in ("asked", "proposed"):
        where = (" (in %s)" % a["tool"]) if a.get("tool") else ""
        print("  %s awaiting the builder: \"%s\"%s"
              % (a["status"].upper(), a.get("text", ""), where))
' || true

# the released sha: where the next deploy's toggle-proof gate starts from
(cd "$SRC" && git rev-parse HEAD) > "$RELEASED"

echo "deployed — https://miso.nøøb.org"
