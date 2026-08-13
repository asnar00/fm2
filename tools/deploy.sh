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

rsync -a --delete \
  "$SRC/products/muon/build/server/target/release/muon_server" \
  "$SRC/products/muon/build/site" \
  "$HOST:muon/"

ssh "$HOST" '
  launchctl kickstart -k "gui/$(id -u)/com.noob.muon" 2>/dev/null ||
    launchctl bootstrap "gui/$(id -u)" ~/Library/LaunchAgents/com.noob.muon.plist
'
echo "deployed — https://muon.nøøb.org"
