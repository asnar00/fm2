#!/bin/sh
# Deploy the muon product to the Mac mini (public at https://muon.nøøb.org via
# cloudflare tunnel, ingress muon.xn--nb-lkaa.org -> localhost:8095).
#
# Builds the muon product (fm linker: native server + wasm client + site/),
# ships the server binary and site to ~/muon on the mini (both machines are
# arm64 darwin, so the local build runs there), and restarts com.noob.muon.
set -e
SRC="$(cd "$(dirname "$0")/.." && pwd)"

# the mini on the home LAN, else its public address; MUON_HOST overrides both
pick_host() {
  [ -n "${MUON_HOST:-}" ] && { echo "$MUON_HOST"; return; }
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

python3 "$SRC/tools/fmlink.py" muon

# replay sessions are local-only test data — never ship one
rm -f "$SRC/products/muon/build/site/replay.json"

# the loader instantiates the wasm with ZERO imports — refuse to ship a build
# that quietly grew import requirements (a dependency's wasm-bindgen glue once
# turned the deployed app into a black screen)
node -e '
const fs = require("fs");
WebAssembly.instantiate(fs.readFileSync(process.argv[1]), {})
  .then(({instance}) => { if (!instance.exports.fm_entry) throw new Error("no fm_entry"); })
  .catch(e => { console.error("deploy: wasm smoke test FAILED:", e.message); process.exit(1); })
' "$SRC/products/muon/build/site/client.wasm"

# provenance visibility: which feature nodes does this release touch, and did
# any capability ship without a node? informational — the judgment stays
# human, but the omission becomes visible at the moment of shipping
LIVE=$(curl -s --max-time 5 https://muon.xn--nb-lkaa.org/version 2>/dev/null | tr -cd '0-9')
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

# deploy stamp: the client compares this on launch and self-refreshes on change.
# a plain increasing integer (the commit count — every release is a commit, so
# this needs no counter file and still names an exact commit for debugging)
(cd "$SRC" && git rev-list --count HEAD) > "$SRC/products/muon/build/site/version"

# what's-changed list for the system panel: recent commit subjects, newest
# first, each tagged with its build number (count minus offset)
python3 - "$SRC" > "$SRC/products/muon/build/site/changes.json" <<'PY'
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
python3 - "$SRC/products/muon/build/site" > "$SRC/products/muon/build/site/hashes.json" <<'PY'
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
  "$SRC/products/muon/build/server/target/release/muon_server" \
  "$SRC/products/muon/build/site" \
  "$HOST:muon/"

ssh "$HOST" '
  launchctl kickstart -k "gui/$(id -u)/com.noob.muon" 2>/dev/null ||
    launchctl bootstrap "gui/$(id -u)" ~/Library/LaunchAgents/com.noob.muon.plist
'

# the ask inbox: shipping without answering becomes visible at the moment of
# shipping — print every user ask still status "asked" (see noob-button/ask)
ssh "$HOST" 'find /tmp/muon-vars -name "user.*.asks.json" -exec cat {} + 2>/dev/null' | python3 -c '
import json, sys
for line in sys.stdin:
    line = line.strip()
    if not line: continue
    try: asks = json.loads(json.loads(line).get("v") or "[]")
    except Exception: continue
    for a in asks:
        if a.get("status") == "asked":
            print("  ASK awaiting the builder: \"%s\"" % a.get("text", ""))
' || true

echo "deployed — https://muon.nøøb.org"
