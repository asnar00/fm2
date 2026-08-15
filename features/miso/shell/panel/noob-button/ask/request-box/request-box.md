# request-box
*the ask box invites a request, in one word*

> (transcripts/2026-08-15-fm-spec-2.md#p15)
> NEW ASK [proposed] … [in taps @ miso/loop/tap/counter] :: 'Instead of “ask miso…” let’s say “request”'
> *(a field ask, filed 2026-08-15 on miso build 173, moments after the button became `miso-button`)*

## spec

The ask box's placeholder reads **request** — one word, where "ask
miso — find a tool, or wish for one" used to explain at length. It
aligns the box with the panel's own vocabulary (the lifecycle rows are
requests) and stops repeating the app's name next to a button that now
says it. Taken literally per the ask: the whole placeholder is the one
word, the calm-panel taste applied to prose.

## user

The box says **request** — type what you want, press miso.

## glossary

(the ask box is defined at `/ask`; requests at `/lifecycle`)

## code description

`request-box.index.js` re-sets `#askText`'s placeholder at load, the
same late-fragment move as `miso-button` beside it: the parent built
the row already, this runs after (provenance order), one attribute
assignment, guarded on the input existing. Unticked, the parent's
longer placeholder returns.
