# fm2 — notes for Claude

**Read `agents.md` first and follow its five-step loop for every user request.**
It is the canonical development discipline (node placement → node first →
implement inside the node → prove the toggle → ship); the laws and mechanics
live there, not here. `fm.md` is the user-authored doctrine — never edit it.

**Fresh session? Read `handover.md`** — current build, today's doctrine,
tooling state, and the named next rungs. It is rewritten at each session
end; trust it over older prose in notes.md when they disagree.

**Building the sovereign runtime? Read `sovereign.md`** — the plan for
replacing onnxruntime with our own WebGPU kernels and putting whisper on
them: layers, node placement, the rung ladder with its acceptance tests,
and the numpy-twin verification discipline. A living plan, freely
editable; open rulings sit in its §10.

**Building and shipping: see `deploy.md`** — build/run commands, what
deploy.sh does, the mini, tunnel, state locations, and how to check on the
live system.

## Claude-specific operational notes

- Documents: `notes.md` is co-written and freely editable — agent-originated
  observations and proposals go here; `ideas.md` is for the USER's passing
  whims (date entries; don't seed it with your own ideas); `transcripts/` is
  regenerated via `tools/export_transcript.py` (run BEFORE citing a new `#pN`
  anchor, and at session end). Do not excavate pre-fm2 experiments; reading
  ftr for a specific proven mechanism, when pointed there, is fine.
- Spec style: code descriptions in short paragraphs, one per thing described.
  Glossary terms backticked with a leading slash: `` `/term` ``.
- shell is at the 6-child cap — its next child forces a regroup.
- **Selection lives in products, not the shared tree**: order.md in features/
  stays fully ticked (it orders the catalog); switching a feature off is a
  product-level order.md override (see products/hello_only). Transient
  dev toggle-tests may flip the shared file but must restore it in the same
  breath.

## Verbatim libraries

A node may carry `<name>.lib.rs`: full Rust (generics, traits, comma-bearing
types) the linker emits as-is — no chains, no merging, provenance-commented,
toggleable with the node. Use for library types like scope's `Var<T>`; the
regex parser never sees these files, so its limits don't apply inside them.
