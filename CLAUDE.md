# fm2 — notes for Claude

**Read `agents.md` first and follow its five-step loop for every user request.**
It is the canonical development discipline (node placement → node first →
implement inside the node → prove the toggle → ship); the laws and mechanics
live there, not here. `fm.md` is the user-authored doctrine — never edit it.

## Claude-specific operational notes

- Documents: `notes.md` is co-written and freely editable; `transcripts/` is
  regenerated via `tools/export_transcript.py` (run BEFORE citing a new `#pN`
  anchor, and at session end). Do not excavate pre-fm2 experiments; reading
  ftr for a specific proven mechanism, when pointed there, is fine.
- Spec style: code descriptions in short paragraphs, one per thing described.
  Glossary terms backticked with a leading slash: `` `/term` ``.
- Build/ship: `python3 tools/fmlink.py <product> [--run]`;
  `./tools/deploy.sh` (refuses dirty trees, wasm zero-import smoke test,
  exports /features/, stamps build number = commit count).
- The mini runs LaunchAgent `com.noob.muon` (`~/muon`, port 8095). Do NOT
  touch `com.noob.muon-server` — despite the name it is the dev surface.
- Auth state: `~/.muon-auth/` on the mini, outside the synced tree. Vonage
  creds in `~/.agent-config.json`. `_`-prefixed guests get PINs logged, not
  texted. Diag: `/tmp/muon-diag.log` on the mini.
- shell is at the 6-child cap — its next child forces a regroup.
