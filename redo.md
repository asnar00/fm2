# redo — the ledger of the expunged session

*Written 2026-08-21, revised same day after review with ash
(transcripts/2026-08-21-hybrid.md#p7–p16). On ash's ruling, main was
rewound to `431b39a` (the Aug 15 handover) and the entire Aug 16 session's
code — Fable morning and Opus afternoon alike — was expunged, to be redone
under the hybrid protocol (`hybrid.md`). The expunged history is preserved
on branch `archive/aug16-pre-rewind`; every item below is a from-scratch
rewrite that may consult the archive for lessons, never for copying. The
records survived by design: `agents.md` (4a, the law above the laws),
`notes.md` (the Aug 16 doctrine and the model comparison), both
transcripts, and `fm.md` at its post-change form.*

*Standing correction from the review (#p16): the screenshot remedy (old
R2, `shot.py`/`look.py`) was a red herring — **`/diag/readout` is the
agent's eyes**: DOM-as-JSON, in the tree since Aug 13, three days before
the map. Opus built a vision path instead of finding it. Verification
asserts on readout; questions of appearance go to ash interactively.
hybrid.md's evidence rules now say so.*

## batch one — trivial and foundational, in order

**1. The fm.md format re-migration** (#p5). fm.md mandates `## user`
above `## spec`; the ~110 reverted specs have it inverted. Mechanical,
nodeless by the export-tooling precedent. First, so the tree is
normalised before anything lands on it.

**2. Release-list bookkeeping** (#p3) — pulled forward on ash's ruling.
Diary/notes commits stop masquerading as releases: a release-worthiness
seam, not a hardcoded filter. Also the honest answer to the features
list topping out at 175 while the panel says build 182+.

**3. The updates picker survives redraw** (#p19a) — fix from scratch;
the bug is latent in the rewound tree now. Lesson kept: the picker's DOM
was on loan to a parent that redraws — own the element or re-render with
the owner.

**4. Instance names, engine receipts, per-feature logging** (#p13a,
#p23–p28) — all needed, all rewritten from scratch; foundational, so it
precedes the headliners (diagnostics arm everything after them). The two
latent defects shipped last time are design constraints this time:
**logging must not evict the flight recorder it exists to explain, and
must not pollute replay.** notes.md carries the full doctrine entry
(promotion rule fires on telemetry; instances get names).

**5. Transcript mirroring + self-heal** (#p2, #p21a) — redo from
scratch. Transcripts join the mirrored record so words travel like audio;
better-replaces-rough decides collisions; a device that cannot transcribe
must not erase words another device worked out. Ash's two phone
recordings are the standing test case.

## the headliners — design conversations first, then build

**6. Contexts — the enable/disable wiring, redone completely** (#p4,
#p4a; ash's priority). **A proper design conversation with ash comes
before any implementation — that conversation is what didn't happen last
time, and ash has thoughts.** The requirements as ruled (#p16): a user
can dis/enable features **instantly**, **without losing app context** (if
possible), and **without affecting other users**. The old mechanism
(linker-emitted gates, `fm_unticked`, `trusted.md`) is archive reference
only. Paid-for lessons, so nobody re-pays:

- The ticks map's raw text also rides inside queued `VarSet` messages as
  a *value*; a scanner must match the ticks KEY (the `':'` discriminator)
  or it fails open on the wrong string.
- Gating the transport of the ticks gates the enforcer's senses: the
  first build gated `/scope` and froze an untick irreversibly within
  minutes. Whatever delivers the ticks is part of the enforcer.
- An update-chain gate reads the *incoming* state: the event that flips
  a tick processes under the old context; the render after it sees the
  new one — the correct Elm-style boundary.
- The linker half (gate emission in `fmlink.py`) was expunged with the
  node; the redo owns both halves or neither.
- Standing ruling: nothing exempt (#p4); the exemption question returns
  as a privilege question (notes.md #p12).

**7. The webgpu restart** — `webgpu.md`, written fresh, replaces the
expunged sovereign.md entirely (ash's ruling: throw away all the
Opus-written material). No ort, no shim, no tamed-request. Prior research
conclusions (Moonshine, the 500MB ceiling, static allocation) are claims
to re-verify from sources, not premises to inherit. One lesson survives
on its merits: module files must be served as javascript, not anonymous
binary — the `.mjs` MIME bug cost a field afternoon.

## after the tunables conversation

**8. The grid asks return as live asks** (#p29–p31a context). There is
workflow here — multiple naive requests becoming tunables (notes.md
#p17a, #p18) — so the sequence is: talk the tunables design through with
ash first, then **ash re-fires the asks interactively from the app**, and
they run as the hybrid pipeline's first live field asks. Their original
`asks#<t>` records died with the server wipe; the transcript record
stands as history. The stamp_ask fix (a re-shipped ask names the build
that actually has it, ex-84babe3) rides along here.

## not redone

- **The map** (#p32–p39): interactive with ash, later, when its
  prerequisites exist (style tokens, authored vector data — the remedies
  entry in notes.md, minus the screenshot item, which readout retires).
- **The ort shim** (#p6–p9): superseded by item 7.
- **`withdrawn` in the ask lifecycle** (old R5): its urgency died with
  the server wipe (no dead stamps left to correct); someday-material.
- The doctrine and post-mortems: never expunged — they are the point.

## standing cautions

- `origin/main` still holds the expunged history; publishing the rewind
  is a deliberate force-push — ash's call.
- The features list showing 175 while the panel shows 182+ is honest
  (doc-only builds); item 2 is the fix.
