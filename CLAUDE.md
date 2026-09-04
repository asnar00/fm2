# fm2 — notes for Claude

**Read `agents.md` first and follow its five-step loop for every user request.**
It is the canonical development discipline (node placement → node first →
implement inside the node → prove the toggle → ship); the laws and mechanics
live there, not here. `fm.md` is the user-authored doctrine — never edit it.

**Fresh session? Read `handover.md`** — current build, today's doctrine,
tooling state, and the named next rungs. It is rewritten at each session
end; trust it over older prose in notes.md when they disagree.

**Usage watch (ash, 2026-09-02):** a SessionStart hook runs
`tools/usage_log.py --report` (the Fable weekly percentage, its burn rate,
and whether the budget lasts to the reset). Relay its estimate to ash in
plain words in the first message of every session — days remaining, or
"lasts the week" — before anything else. Hourly samples come from the
launchd job `com.noob.usagelog`; the log is `~/.claude/usage-log.jsonl`.

**Field asks sit at `asked`; triage stamps building or proposed, or answers
with a note (ash, 2026-09-04, field-walk #p199).** No stamp is written by a
machine: the ask monitor runs piped through `tools/ask_ack.py`, which now only
announces each new ask and names the asker's authority. Rearm it as
`python3 tools/ask_monitor.py --local | python3 -u tools/ask_ack.py`. Triage
then stamps by hand — `building` for an admin or support asker, `proposed` for
anyone else (anyone may ask; the payer decides what is built: ash accepts and
orders proposals, they build in a batch, everyone gets them — notes.md
"feature flow"), `shipped` (with the build), a did-you-mean question, or, when
the thing already exists, `answered` with a note saying how to do it in the UI
(`stamp_ask.py --status answered --note "…"`, which also rings the asker's
phone once). A proposal is built only on ash's word.

**Building and shipping: see `deploy.md`** — build/run commands, what
deploy.sh does, the mini, tunnel, state locations, and how to check on the
live system.

**Agent instructions are the tree's third language** (2026-08-23, plans
#p29): a node may carry `<name>.agent.md` — instructions to agents,
build-time or in-app — which fmlink assembles, provenance-ordered and
toggle-obeying, into `products/<product>/build/skillset.md`. Read the
composed skillset at session start alongside this file; `/taste` (the
aesthetic standard) lives there, not in a root doc.

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
  breath — and most nodes no longer need one: `fmlink.py miso --prove`
  says when the proof is implied (`/confined`, agents.md step 4).

## Verbatim libraries

A node may carry `<name>.lib.rs`: full Rust (generics, traits, comma-bearing
types) the linker emits as-is — no chains, no merging, provenance-commented,
toggleable with the node. Use for library types like scope's `SyncVar<T>`; the
regex parser never sees these files, so its limits don't apply inside them.
