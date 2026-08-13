# fm2 — working rules for Claude

## The rules that got broken once: NODE-FIRST, ONE PROMPT PER NODE

**Before building any user request that adds a capability, create its feature
node** — an `.md` with the provenance quote and transcript anchor — *then*
write the code, wherever it lands. A request whose implementation is edits to
another feature's files STILL gets its own node: the tree tracks intent, not
file boundaries. (The update system and system panel were once built straight
into shell's assets and only got nodes when the user noticed the gap.)

**One prompt per node.** A node cites exactly one user request. When a new
prompt refines an existing capability, it gets a subfeature node under it —
never a second quote folded into the parent. (The bundled update/ node had to
be decomposed into update/{buildnum,watch,honest} when this rule landed.)

**Pointer nodes are FORBIDDEN (user law, #p91).** The only nodes allowed to
contain no code are grouping nodes. Every other node owns its implementation —
code, assets, or chain extensions. Existing violations (client-behaviour nodes
pointing into shell's asset JS) are scheduled debt: asset composition + the
event core legalise them; do not create new ones. 4-6 children per node
(fm.md rule); shell is at 6 — its next child forces a regroup (mind
linearisation order). Also learned: a chain extension must linearise AFTER
its base — a node early in DFS order (serve/features) cannot extend a chain
defined later (gate's is_public); tree position constrains what a node can own.

Deploy prints which nodes a release touches and flags releases with no new
nodes — treat that flag as the question "did a request go nodeless?"

## Documents

- `fm.md` is written ENTIRELY by the user — never edit it; point out errata
  for them to fix. It is the source of truth for intent.
- `notes.md` is co-written and freely editable — decisions, proposals, status.
- `transcripts/` is the immutable record: regenerate with
  `python3 tools/export_transcript.py --slug fm-spec --title "fm spec discussion"`
  BEFORE citing a new `#pN` anchor, and again at session end.
- Do not excavate pre-fm2 experiments (miso, microserver.fm, ftr internals…);
  the user found that counterproductive. (Reading ftr for a specific proven
  mechanism, when pointed there, is fine.)

## Specs

- fm.md defines the format. House additions: code descriptions are short
  paragraphs, one per thing described (entry/extension points first, then
  mechanics, then helpers) — never one dense block.
- Glossary terms are written backticked with a leading slash: `` `/term` ``.
- Commit subjects are user-visible (changelog + push notifications) — write
  the first line for the user, not for git.

## Building and shipping muon

- Link/build: `python3 tools/fmlink.py <product> [--run]`. Products: muon
  (two places: server native + client wasm), demo/hello_* (test tree).
- Deploy: `./tools/deploy.sh` — refuses dirty trees (a release is a committed
  state), smoke-tests that client.wasm instantiates with ZERO imports, exports
  the feature tree to /features/, stamps the build number (= commit count).
- The mini runs LaunchAgent `com.noob.muon` (`~/muon`, port 8095). Do NOT
  touch `com.noob.muon-server` — despite the name it is the dev surface.
- Auth state lives in `~/.muon-auth/` on the mini, outside the synced tree.
- wasm gotcha: getrandom needs its `custom` feature, never `js` (wasm-bindgen
  imports black-screen the glue-free loader).

## fmlink parser limits (regex-level, not a real Rust parser)

- One `feature_` struct per feature node.
- No commas inside fn parameter types (`Vec<(A, B)>` breaks the param split);
  tuple RETURN types are fine.
- Braces must balance everywhere, including inside string literals.
- `existing.fn()` may only call the enclosing function's own chain.

## Asset fragments (page-language implementations)

- A node may carry css/js/html implementation files beside its spec:
  `honest.js`, `pinned.page.css`, `enrol.login.js`. Filename infix names the
  target page — {index login install sw}, bare = index, `page` = all html
  pages; `.head.html` targets the head slot.
- Page-owning assets carry slot markers (`<!-- fm:head -->`, `/* fm:style */`,
  `<!-- fm:body -->`, `// fm:script`); the linker fills them in linearisation
  order, provenance-commented.
- One `const feature_<Name> = {...}` object per JS fragment. Cross-feature
  references are ALWAYS typeof-guarded — absence is the unticked state, and
  fragments must survive their siblings being toggled off (guard DOM lookups).
- After changing fragments, sanity-check with toggle tests: untick the node,
  relink, grep the composed page for its provenance comment.
