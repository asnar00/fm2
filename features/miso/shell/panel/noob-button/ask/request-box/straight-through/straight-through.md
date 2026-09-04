# straight-through
*the box files first and looks afterwards: your words go to the builder at once, and the search only speaks when the thing already exists*

> (transcripts/2026-09-04-field-walk.md#p70)
> let's change the suggested text in the miso input field to "request a fix, tweak or feature"; instead of doing the semantic search, file it through as a feature request straight away; if it turns out the feature exists already, then we should pop up the guide to the feature.

## user

The box says **request a fix, tweak or feature**. Type what you want and
press miso: it goes to the builder there and then — no list of guesses
to read first, no second button to press. If miso already does the thing
you asked for, a card comes up over the sheet telling you how, with a ✕
to put it away. Your request stays filed either way.

## spec

`/ask`'s box put a search between you and the builder: press miso, read
three results, decide none of them is it, press *send to the builder*.
Two of those steps were the box's doubt, not the asker's. Ash asked for
the doubt removed (#p70) — file straight away, and use the search only
for the one thing it is good at: saying *this exists already*.

**The placeholder** reads **request a fix, tweak or feature** — what the
box is for, said plainly, where `/request-box` said *do something* and
`/ask` explained at length. Untick and *do something* returns.

**The send road** is `/ask`'s `send(text)` seam, redefined here: file the
ask (the same `Ask` event, the same `asks` var, the same `asked` status,
so `tools/ask_ack.py`, the requests list and the whole flow are
untouched), empty the box, and only then search. The box says nothing
about the filing: the new row in your requests list is the receipt,
which is `/quiet`'s ruling and `/taste` 8.

**The guide** is the feature's own `## user` paragraph — `tree.json`'s
`intro`, the same words `/long-press` shows under a finger — in a card
over the sheet: the feature's name, its paragraph, a ✕. A hit that
registers a tool is described in that tool's *current* words
(`/tool-words`'s table) rather than its node's founding ones, so the
card and the long press never disagree.

**The line between a match and a strong match.** `/semantic-find` calls
0.28 cosine a result worth listing; popping a guide unasked needs more.
Measured over 32 asks against the 379-node catalog — 16 things miso does
today, 16 it does not — the real ones score 0.453–0.827 and the absent
ones 0.264–0.587. **0.50** is the line: it pops the guide for 12 of the
16 real asks and for 2 of the 16 absent ones. The two are near misses of
meaning, not of arithmetic ("let me draw on a photo" finds the picture
frame at 0.538), and the cost of each is a card the asker closes, while
the ask is filed regardless. `strong` is one number on this node's
object, so moving the line is one edit.

The score is `/semantic-find`'s own — `/context-bias` included, so an ask
made inside a tool tilts toward that tool's family by 0.08 exactly as the
results list did.

## hostile cases

- **Nothing matches.** Nothing pops; the ask is filed and shows as
  `asked` in the requests list, as today.
- **`/semantic-find` unticked or its 8 MB table absent.** The box files
  and never searches (typeof-guarded, and a failed load leaves `ready`
  false). No word-overlap fallback is used: overlap is too coarse a
  reader to interrupt someone with.
- **The first ask ever.** The table loads on demand (~8 MB). The filing
  does not wait for it — the ask is sent and the box empties before the
  fetch begins — so the guide is the only thing that arrives late.
- **The asker moved on while the table loaded.** Each send takes a turn
  number and the guide only pops if it is still the latest send *and*
  the sheet is still up; a guide never arrives over a screen the asker
  went to afterwards.
- **A match on a feature with no guide text.** A node whose `intro` is
  empty falls back to its purpose line; with neither, nothing pops —
  a card holding a name and no words is noise.
- **`/chooser` unticked.** No `tree.json` catalog to name the hit, so no
  guide; the ask still files.
- **The sheet is put away with the guide up.** `feature_Panel.close` is
  wrapped: the guide goes with it and cannot outlive its sheet.
- **Offline.** The ask queues in the outbox as any event does, and the
  guide can still pop — the table and `vectors.json` are service-worker
  cached. *(Reasoned from the composition, not observed on the rig.)*
- **`/urgency`'s two buttons.** They belonged to the results footer,
  which no longer exists, so a filed ask carries `whenever` — its
  default. Urgency is now triage's stamp, not the asker's; an urgency
  choice that does not cost a second step is the named next rung.
- **`/quiet` unticked.** No "filed" line returns: this road never writes
  one. `/quiet`'s rule stands, its subject is gone.

## next (the seam is open)

`strong` (the line), `guideFor(n)` (what the card says about a hit) and
`show(n)` (how it is drawn) are each one redefinition. The three
refinements to expect: an **open** chip on the card for a hit that
registers a tool; a road from the card into `/chooser`'s full feature
page; and an urgency the asker sets without a second step.

## glossary

- **strong match**: a catalog hit at or above 0.50 cosine — the bar for
  interrupting an asker with a guide, above `/semantic-find`'s 0.28 bar
  for listing a result.
- **guide**: a feature's name and its user paragraph, shown over the
  sheet when the ask turns out to be already built.

## code description

`straight-through.index.js` owns `feature_StraightThrough` and, at load,
redefines `feature_Ask.send` and re-sets `#askText`'s placeholder — the
late-fragment move `/request-box` and `/miso-button` make beside it.

`send` files through `feature_Ask.file(text)` (so `/urgency`'s
replacement of `file` still applies), clears the input and the results
box, hides any standing guide, then awaits `match()` and shows the card
if the turn is still current and the sheet still up.

`match(words)` awaits `feature_SemanticFind.load()`, embeds the query
`/ask` built (`words(text)` — the same little-word trimming the results
list used) and takes the single highest of `score()`'s catalog
cosines; below `strong` it returns null, above it resolves the path
through `feature_Chooser.byPath`. Every cross-feature reference is
typeof-guarded and the whole body is in a try/catch: a search that
fails is a search that says nothing.

`guideFor(n)` prefers `feature_ToolWords.words('tool_' + n.tool)` when
the hit registers a tool, and otherwise takes the node's `intro`, then
its `purpose`.

`card()` builds `#askGuide` once and `show(n)` fills it; `hide()` is
wired to the ✕ and to a wrap on `feature_Panel.close`.

`straight-through.index.css` — the card over the sheet: `#161619` on a
1px `#3a3a3f` border at 14px radius, the panel's own column and width,
the name at `#fff` and the paragraph at `#c9c9d2`, a 0.18s ease-out
rise.
