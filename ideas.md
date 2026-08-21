# ideas
*ash's passing whims, dated on arrival — not commitments, not designs*

## 2026-08-15

- bigger-buttons should convert into a real UI feature with a per-user
  "size" var (evening session, after the +25% field ask shipped as
  build 170): the literal ask — "increase the size of the tool buttons
  by 25%" — is a naive form of "let me adjust the UI scale", and the
  proper shape is a user-scoped variable the buttons derive from, the
  way `feature_ticks` is per-user. There's a bunch of learning here
  about how best to implement naive user requests: the shipped literal
  answers the ask, and the generalisation it gestures at arrives as a
  later node that subsumes it.

- panel reorder, "less busy" (transcripts/2026-08-15-fm-spec.md#p14):
  top-to-bottom becomes **ask muon** first → awaiting-update list +
  update button → update policy picker → a **features** button that
  opens the long feature list (no longer always inline) → last row:
  "logged in as asnaroo" and log out combined into ONE row at the very
  end.

## 2026-08-14

- colour palettes from https://github.com/carpdiem/ember (noted mid-transcription-build)

## 2026-08-21

- build/deploy speedups, to land AFTER the contexts ladder's last rung
  (ash's ruling, hybrid #p37): shared CARGO_TARGET_DIR across worktrees
  (cold worker builds are the latency king — warm link is only 4.8s);
  a --quick fmlink profile (debug, no lto) for toggle-proof cycles while
  deploy keeps release; skip deploy's feature re-export when no .md
  changed. The zero-build path for tunable asks needs no work — it's
  rung 6 of the ladder itself.
