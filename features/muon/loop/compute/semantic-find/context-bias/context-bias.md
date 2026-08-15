# context-bias
*where you're standing tilts the search: the open tool's family scores higher*

> (transcripts/2026-08-15-fm-spec.md#p27)
> The system should be smart enough to know 1) that I'm in the tap tool right now, so my request probably pertains to it
> *(fired at #p30; #p78 named context sensitivity — this is its first cash-out)*

## spec

An ask made inside a tool probably pertains to it, so the finder leans
that way: every catalog entry in the open tool's **family** — its
registering node, that node's ancestors, and its descendants — gets a
small fixed bonus (0.08) on top of its semantic score, enough to lift
the tool you're using over a global near-tie ("reset" asked inside
taps finds taps first) and far too small to drag an unrelated family
over a real match elsewhere. On the launcher, no tool is open and
nothing tilts.

## user

Ask while you're inside a tool and that tool gets the benefit of the
doubt: vague words like "reset" or "undo" find the thing you're
looking at before anything else.

## glossary

- **family**: the open tool's registering node, its ancestors, and its
  descendants — the paths the bias lifts.

## code description

`context-bias.index.js` wraps `feature_SemanticFind.score`: after the
original returns the catalog scores, `home()` resolves the open tool
(loop state) to its registering node's path via `/chooser`'s flat list
(typeof-guarded; no tool or no catalog means no change), and every
path equal to it, prefix of it, or prefixed by it gains 0.08. The
biased scores flow into the standing threshold and top-three
selection unchanged.
