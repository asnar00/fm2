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

rsync -a --delete \
  "$SRC/products/miso/build/server/target/release/miso_server" \
  "$SRC/products/miso/build/site" \
  "$HOST:miso/"

ssh "$HOST" '
  launchctl kickstart -k "gui/$(id -u)/com.noob.miso" 2>/dev/null ||
    launchctl bootstrap "gui/$(id -u)" ~/Library/LaunchAgents/com.noob.miso.plist
'

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
# shipping — print every user ask still status "asked" (see noob-button/ask)
ssh "$HOST" 'find /tmp/miso-vars -name "user.*.asks.json" -exec cat {} + 2>/dev/null' | python3 -c '
import json, sys
for line in sys.stdin:
    line = line.strip()
    if not line: continue
    try: asks = json.loads(json.loads(line).get("v") or "[]")
    except Exception: continue
    for a in asks:
        if a.get("status") in ("asked", "proposed"):
            where = (" (in %s)" % a["tool"]) if a.get("tool") else ""
            print("  %s awaiting the builder: \"%s\"%s"
                  % (a["status"].upper(), a.get("text", ""), where))
' || true

echo "deployed — https://miso.nøøb.org"
