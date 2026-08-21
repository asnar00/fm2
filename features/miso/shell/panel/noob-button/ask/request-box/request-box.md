# request-box
*the ask box invites you to do something*

> (transcripts/2026-08-15-fm-spec-2.md#p15)
> NEW ASK [proposed] … [in taps @ miso/loop/tap/counter] :: 'Instead of “ask miso…” let’s say “request”'
> *(a field ask, filed 2026-08-15 on miso build 173, moments after the button became `miso-button`)*

## user

The box says **do something** — type what you want, press miso.

## spec

The ask box's placeholder reads **do something** — playful and
imperative, where "ask miso — find a tool, or wish for one" used to
explain at length. It stops repeating the app's name next to a button
that now says it, and pairs with that button as what → do: *do
something → miso*. The whole placeholder is the phrase, the calm-panel
taste applied to prose.

## glossary

(the ask box is defined at `/ask`; requests at `/lifecycle`)

## code description

`request-box.index.js` re-sets `#askText`'s placeholder at load, the
same late-fragment move as `miso-button` beside it: the parent built
the row already, this runs after (provenance order), one attribute
assignment, guarded on the input existing. Unticked, the parent's
longer placeholder returns.

*(Revised in place, transcripts/2026-08-15-fm-spec-2.md#p17 — a
same-evening wordsmithing pass, churn the two-phase lifecycle blesses:
"request" became "do something" after a hunt for a word between
request and command landed on neither. The node keeps its founding
name; the box holds the phrase.)*
