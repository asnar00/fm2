# store
*where a card is kept, and the law that keeps it: the server's merge, and the picture that lives beside the card rather than inside it*

> (transcripts/2026-09-03-invite-test.md#p159)
> do everything now

> (transcripts/2026-09-03-invite-test.md#p158, the diagnosis this groups around)
> `feature_Cards.held('', 1)` on ash's real list = **176,020**, i.e. room = **−16,020**. Verdict: **REFUSED**

## user

Nothing to see: this is where the rules about how your cards are stored live.

## spec

A grouping node, code-free. `/cards` had six children and the picture store
asked for a seventh (#p159), so the two nodes about the *store itself* — as
opposed to the page, the look, the surfaces, the handing over or the kinds —
sit here: `/guard` (the server's last word on a `cards` set: a `set` can never
delete a card) and `/pic-beside` (a picture's bytes live beside the card and
the block keeps a reference).

The pairing is not filing convenience. `/guard` is the law over the var and
`/pic-beside` is the shape of what the var holds; a change to either is a
change to what "your cards are stored" means, and the two are read together.

A regroup rewires nothing: composition is provenance-ordered, so `/guard` and
its three children keep their place in every chain and every fragment. What
*does* move is `/enabled`'s per-node flag, which is keyed by node path — the
guard's flag changes address and this node gains one of its own (misses.md,
"the regroup that moved addresses", 2026-09-02). Flags default on and stored
ticks are not enforced, so no world changes behaviour; the commit says exactly
this and nothing stronger.

Untick and the server's merge and the picture store both leave together — a
whole-family switch, which is the thing a grouping node's own flag buys.

## glossary

(no new terms)

## code description

(none — a grouping node)
