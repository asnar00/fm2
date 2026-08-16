# tools-first
*the ask finds tools; the rows are their readouts*

> (transcripts/2026-08-15-fm-spec.md#p24)
> when I typed in "record audio" I got the "open dictate" - but 1) I didn't get the dictate feature readout, so I can't check whether it's the right thing; and 2) it's showing mirror, open-chip and ask, which aren't tools, so really they shouldn't appear in the results at all

## user

Ask for a thing and you get tools: the button that opens each one, and
its description right there so you can check it does what you meant.
Only when nothing can do it directly do the explanatory feature pages
step in.

## spec

The finder's results were raw semantic hits — whichever features'
prose happened to sit nearest the query, tools or not, with the tool's
own feature sometimes outranked by its relatives. This node settles
what an ask result IS: a **tool**, shown as its open chip plus the
**registering feature's row** — the readout that lets you check it's
the right thing before you open it.

Each semantic hit resolves through `/open-chip`'s lineage walk to a
tool; distinct tools keep their hit order; each renders as the row of
the node that registered it (`counter` for taps, `dictate` for
dictate). Hits that resolve to no tool are dropped — a helper feature
matching the words is not an answer to "do this for me".

The one honest exception: an ask that resolves to **no tool at all**
falls back to the plain feature rows, because for capability questions
("control which updates arrive") the reading path is the answer, and
showing nothing would file a wish for something that exists.

## glossary

- **readout**: the tool's registering feature row in the results — the
  check-before-you-open explanation.

## code description

`tools-first.index.js` wraps `feature_Ask.features` (composing after
`/semantic-find`, so it reshapes whatever the finder returned): each
hit resolves via `feature_OpenChip.toolFor`; distinct tools map to
their registering node (the `flat` entry whose stamped `tool` matches)
and those owner rows return in hit order. Zero resolved tools returns
the original hits. `/open-chip`'s dress then chips the owner rows by
their own stamps — no coordination needed.
