# one-button
*however many builds are waiting, one awaiting group, one button*

> (transcripts/2026-09-01-saturday.md#p23)
> When there's two upgrade packages available, I'm seeing two upgrade buttons - I'd rather always just see one upgrade group and one upgrade button

## user

Any number of waiting builds show as one "awaiting update" group with one
button. Never two groups, never two buttons.

## spec

`/review`'s section is already one group for all pending builds — but
`section()` is async (two fetches deep), and it clears the old group at the
top and prepends the new one at the bottom. Two builds announcing close
together run two overlapping `section()` calls: both pass the removal
before either prepends, and the panel ends with two groups, each wearing
`/upgrade`'s button. The day this was reported, builds 443 and 445 shipped
nine minutes apart — the exact shape.

This node serialises the redraw: it replaces `feature_Review.section` at
load with a wrapper that chains every call behind the previous one's
completion, so each run's remove-then-prepend is atomic with respect to the
next. After each run it also sweeps any extra `#awaiting` nodes, keeping
the first — a post-condition that holds even against a race this spec did
not foresee. The race itself is the diagnosis (hypothesis from the code's
shape); the sweep is the guarantee.

## hostile cases

- **Overlapping calls.** Chained; the last caller draws last and its state
  is the one shown.
- **A run that throws.** The chain swallows it (`catch`), so a failed
  redraw never wedges every later one.
- **This node unticked.** The race returns — today's behaviour, no worse.
- **`/review` unticked.** The typeof guard finds nothing; no-op.

## glossary

(no new terms)

## code description

`one-button.index.js` replaces `feature_Review.section` with a wrapper
holding a promise chain: each call awaits the chain, runs the original,
sweeps duplicate `#awaiting` elements (keeping the first), and extends the
chain regardless of failure.
