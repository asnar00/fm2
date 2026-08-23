# did-you-mean
*one question, two readings, one tap — the builder asks what you meant instead of guessing*

> (transcripts/2026-08-23-plans.md#p12)
> I think it should be a matter of agent discretion. Eg. if I'm editing text and I select a word and say "italic", well, I don't need the full conversation, it's clear what I want, just build it and ship it quick. But in the "square" example, where I was in taps and it could have meant two things (square tap button shape, or square the tap count) it would have been better to ask rather than build the wrong thing.

## user

When the builder can't tell which of two things you meant, your request
turns into a short question in the panel — "did you mean the button's
shape, or squaring the count?" — with the readings beside it. Tap the
one you meant; the request goes back to the builder with your answer
on it. If the builder guessed and built one anyway, it says which, so
a tap is a correction rather than a delay.

## spec

The ask that survives its context with two readings gets one question,
not two features. The precedent is on the record: "square", asked from
inside taps, was read as the button's shape *and* as squaring the count,
and both were built six minutes apart. A disambiguation is the one
question doctrinally allowed to travel to a user — which thing you
*meant* is the single fact no trace can settle — so this node gives it
a road and a tap to answer it.

An ask entry may carry `status: "question"` and a `question` object,
`{text, options: [{key, label}, …], likely}`, stamped from the bench.
Such an ask leaves the plain requests section (which shows `asked` and
`proposed` only) and renders as its own quiet block above it: the ask's
own words, the question beneath them, and one chip per reading. The
`likely` reading is marked faintly — it is what silence gets built —
and an optional `note` carries the builder's hedge ("built the button
shape for now — tap if you meant the count") beneath the chips.

Tapping a chip files an `AskAnswer` event. The server stamps `answer`
onto the entry and flips its status back to `asked`: the ask is
actionable again, with its intent settled, and the bench's ask monitor
fires on the restamp with no change to the monitor at all. The
`question` object stays on the entry — the record of what was asked and
what was chosen outlives the row that displayed it.

The tap is a `/loop` event like any other, so it is durable offline and
arrives at the builder over the same road as the ask itself. A chip whose
key is not one of that entry's own options is refused, an answer for a
timestamp no entry carries is a no-op, and the same answer arriving twice
changes nothing: the world is written only when an entry actually
changed, so a stale page cannot grow the op log.

Unticking this node removes both halves. Question-status asks then render
nowhere — absence is the unticked state — and an `AskAnswer` that arrives
anyway falls through the chain unhandled.

## glossary

- **did-you-mean**: a question from the builder naming two or more
  readings of one ask, answered by a single tap.
- **likely reading**: the option the builder would have built on
  silence — marked, not chosen.

## code description

`did-you-mean.index.js` wraps `feature_Lifecycle.render` the way
`/being-built` does: after the original draws, it renders `#didyoumean`
— one `.crow`-grammar row per ask with `status: "question"`, the ask
text bold with a `dstatus` pill, the question and its `.dymchip` chips
in the row's own block, the `note` last when present — placed after
`#building` (or `#awaiting`), before `#requests`, and removed when
empty. Its own click listener turns a chip into
`feature_Loop.send({type: "AskAnswer", data: {t, choice}})`. Every
cross-feature reference is typeof-guarded.

`did-you-mean.rs` extends the `update` chain: on an `AskAnswer` event it
reads the asks list through `/ask`'s `asks_read()`, finds the entry whose
`t` matches, checks the choice against that entry's own option keys, and
on a match stamps `answer` and sets `status` to `asked`; with no match it
writes nothing. The same tap twice writes nothing either — a stale page
repeating an answer costs the op log nothing — while the other chip is a
correction, and lands.

`did-you-mean.index.css` styles the pill and the chips, borrowing the
sibling pills' shape and the panel's own chip colours.

`tools/stamp_ask.py --question` (scaffolding) is the bench half:
`--question "text" --option key=label … [--likely key] [--note "text"]`
sets `status: "question"` and the question object on every matching
entry, through the same `POST /diag/context` door as an ordinary status
stamp. `--note` may accompany a plain `--status` stamp too, which is how
the hedge rides along with a build the user did not ask for yet.
