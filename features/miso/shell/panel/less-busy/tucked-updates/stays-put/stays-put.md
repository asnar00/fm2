# stays-put
*the updates picker survives the list rebuilding itself*

> (transcripts/2026-08-16-fm-spec.md#p19)
> the upgrade-policy radio buttons have disappeared from the feature view - must have happened in a previous update

## user

The automatic / fixes auto / ask me choice stays where it belongs. It used
to vanish the second time you opened the panel in a session, and only came
back if you reloaded the app — now it survives, however many times the
feature list redraws.

## spec

A field-reported disappearance, reproduced exactly: open the panel twice
and the picker is gone until reload.

`/tucked-updates` moves `/policy`'s **actual element** into a container
inside `#changes`. `/chooser` rebuilds `#changes` wholesale on every
`mount()`, and `mount()` runs on every panel open — so the second open
deletes the picker outright. `place()` then reads `$('policySeg')`, finds
nothing, and returns early, so nothing ever restores it. The parent's
`if (!seg) return` guard, written to be careful, is what made the loss
silent and permanent.

The rule this establishes, worth stating beyond this node: **a feature
that moves another feature's element into a container it does not own must
survive that container being rebuilt.** Borrowing a DOM node is a loan,
not a transfer.

So the borrower takes responsibility for both ends of the loan. Before
the rebuild the picker is parked in a detached holder — out of the
document entirely, so no flicker and nothing to destroy — and after the
rebuild it is placed back into the fresh container. If the rebuilt box
turns out not to be the chooser's home (the chooser unticked, or the
teaser fallback showing), the picker is returned to its own place in the
panel above the log-out row instead of being stranded in the holder:
the loan is always repaid, to one address or the other.

Unticking this node returns the old behaviour exactly, disappearance and
all — it changes nothing but survival.

## glossary

(no new terms)

## code description

`stays-put.index.js` wraps two of `/tucked-updates`'s own functions,
typeof-guarded so it is inert without the parent.

`rescue()` runs *before* the original `mount()` (a wrap on
`feature_Chooser.mount` placed after the parent's, so it takes the outer
position and runs first): if the picker currently sits inside `#tucked`,
it moves to a detached holder this node owns.

`place()` is wrapped so that after the parent has run, a picker still
sitting in the holder is dealt with: into `#tucked` if the parent built
one, otherwise home to the panel before the log-out row — the position
`/policy` itself uses, so the fallback is its original layout rather than
an invention.
