# redo — the ledger of the expunged session

*Written 2026-08-21 (transcripts/2026-08-21-hybrid.md#p7–p9). On ash's
ruling, main was rewound to `431b39a` (the Aug 15 handover, build 181) and
the entire Aug 16 session's code — Fable morning and Opus afternoon alike —
was expunged, to be redone under the hybrid protocol (`hybrid.md`). The
expunged history is fully preserved on branch `archive/aug16-pre-rewind`;
nothing here starts from a blank page unless it says so. The records
survived the rewind by design: `agents.md` (with 4a and the law above the
laws), `notes.md` (all Aug 16 entries and the model comparison), both
transcripts, and `fm.md` at its post-change form — the doctrine that
session paid for is the rulebook the redo runs under.*

*Every item re-enters through triage: Fable writes the brief (in-hand line,
named acceptance evidence), an Opus worker runs the five-step loop in a
worktree, Fable reviews against the brief before integrate → deploy →
stamp. Anchors below cite transcripts/2026-08-16-fm-spec.md.*

## first, because everything sits on it

**1. The fm.md format migration** (#p5). `fm.md` is kept at its
post-change form (the user paragraph moves above the spec) but the ~113
node specs reverted to the old order. Re-migrate — mechanical, nodeless by
the export-tooling precedent, and it re-normalises the tree before any
other work lands on it.

**2. `tools/look.py` / `shot.py` — the review seat's eyes** (remedy R2;
the expunged `tools/shot.py` on the archive branch is a seed). The hybrid
review gate demands screenshots; build the tool that takes them before
the first worker ships anything visual. R3 (the visual bar, a doc) and R5
(`withdrawn` in the ask lifecycle) are cheap and belong early too — R5
matters immediately because the grid asks below have server-side stamps
citing builds that no longer exist.

## the priority redo

**3. The context manager — feature enable/disable wiring, redone
completely** (#p4, #p4a; ash's explicit ask, hybrid #p8). The old
mechanism (linker-emitted gates at chain heads, `fm_unticked` raw-scan,
`trusted.md` base) is on the archive branch for reference, not for
copying — this is a fresh design under the protocol. Standing ruling of
record: **nothing is exempt** (#p4); the exemption question returns as a
privilege question (notes.md #p12). Paid-for lessons harvested from the
expunged spec, so nobody re-pays:

- The ticks map's raw text also rides inside queued `VarSet` messages as
  a *value*; a scanner must match the ticks KEY (the `':'` discriminator),
  not any occurrence, or it fails open on the wrong string.
- Gating the transport of the ticks gates the enforcer's senses: the
  first build gated `/scope` and froze an untick irreversibly within
  minutes. Whatever delivers the ticks is part of the enforcer (the
  trusted-base principle), however the new design expresses it.
- An update-chain gate reads the *incoming* state: the event that flips a
  tick processes under the old context; the render that follows sees the
  new one. That is the correct Elm-style boundary — don't fight it.
- The linker half (gate emission in `fmlink.py`, ~114 lines) was expunged
  with the node; the redo owns both halves or neither.

**4. The sovereign restart — from scratch** (ash's explicit ask, hybrid
#p8; context #p15–p20a). The ort shim is NOT redone: no clamped-limits
experiment, no `tamed-request` (which broke its own fallback promise —
notes.md, the model comparison), no ort at all. The old `sovereign.md`
went to the archive branch deliberately: the restart begins with a fresh
Fable-led plan, and the prior research conclusions (the Moonshine pivot,
the 500MB ceiling, static allocation) are inputs to re-verify, not
premises to inherit. One lesson survives on its merits: module files must
be served as javascript, not anonymous binary — the `.mjs` MIME bug cost
a field afternoon.

## the rest of the session, in redo order

**5. Transcript mirroring + self-heal** (#p2, #p21a). Transcripts join
the mirrored record so words reach all instances like audio does;
better-replaces-rough decides collisions; a device that cannot transcribe
must not erase words another device worked out.

**6. Release-list bookkeeping** (#p3). Diary/notes commits stop
masquerading as releases — a release-worthiness seam, not a hardcoded
filter.

**7. The updates picker survives redraw** (#p19a). The bug is latent in
the rewound tree. Lesson: the picker's DOM was on loan to a parent that
redraws — own the element or re-render with the owner.

**8. Instance names, engine receipts, per-feature logging** (#p13a,
#p23–p28). Devices sign reports with short names; transcriptions carry
engine/ms/build receipts; logging is switchable per feature at runtime
(notes.md carries the full doctrine entry). Two latent defects shipped
last time are design constraints this time: **logging must not evict the
flight recorder it exists to explain, and must not pollute replay.**

**9. The grid field asks** (#p29–p31a). Grid aligned to the toolbar and
centred, one size named once; brighter dots; twice-fine grid. Real users'
asks — restamp on re-ship (`stamp_ask.py`'s fix that a re-shipped ask
names the build that actually has it, expunged with 84babe3, comes back
here). The never-ask rule and the `asks#<t>` provenance judgment both
date from these asks; they stand.

## not redone

- **The map** (#p32–p39): interactive with ash, later, under the
  protocol, and only when its gate opens (R1 tokens + R2 look + R4's
  first vector rung — notes.md, the remedies). The proxy/cache/projection
  code is on the archive branch.
- **The ort shim** (#p6–p9): superseded by item 4.
- The doctrine and post-mortems: never expunged — they are the point.

## standing cautions

- The mini last deployed build 208 from the expunged line; the rewound
  tree rebuilds from 182. Build = commit count is now discontinuous with
  what deployed phones remember — resolve before the next deploy (fresh
  deploy from the rewound line supersedes on the update channel, but the
  panel's old stamps cite builds that no longer exist; R5/restamp is the
  honest fix).
- `origin/main` still holds the expunged history; pushing the rewound
  main needs a deliberate force-push — ash's call, not automatic.
