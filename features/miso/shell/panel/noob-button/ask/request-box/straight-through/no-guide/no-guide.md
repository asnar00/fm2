# no-guide
*the app stops guessing: a request just files, and when the thing already exists a person answers it*

> (transcripts/2026-09-04-field-walk.md#p199)
> in the feature request (ask) workflow, it's a bit confused - I make a request and it goes straight to building, but also pops up a suggestion. Let's drop the suggestion part. Instead, go to "asked", and if the feature exists already (concierge, i.e. you, determines), you send a text message ad-hoc explaining how they can use the UI to do it. I think that makes more sense.

## user

Press miso and your request is filed. Nothing else happens — no card
telling you it might already be there. The request sits in your list as
**asked** until a person reads it. If the thing you asked for is already
in miso, they write back: your request reads **answered**, with their
words under it saying how to do it, and your phone rings once with the
first line.

## spec

`/straight-through` filed the ask at once and then popped the matching
feature's guide. Ash read the two together as one confused answer
(#p199): the request had gone to the builder *and* the app was arguing
that it needn't have. The suggestion goes.

**Nothing searches.** `feature_StraightThrough.match()` is redefined to
answer null. That is the narrowest seam that stops the whole suggestion:
the 8 MB table is not loaded, the query is not embedded, the catalog is
not scored, and `show()` — the only caller — is never reached, so no
card is built. `/straight-through`'s filing, its placeholder and its
empty-the-box are untouched, and unticking this node brings the guide
back with `/straight-through` unedited.

**The answer comes from a person.** A new terminal status, `answered`,
joins the sheet beside `asked`, `proposed`, `building` and the
did-you-mean `question`. Its row is the same list grammar (#p39) — the
word where the build number sits, the ask's own words bold — with the
builder's `note` beneath it as the answer. The pill is the quiet grey
family, not a new accent: an answer is finished business and adds no
state to watch (`/taste` 3). The row stays in the list, because the
answer is the thing the asker came back for.

The builder writes it with `tools/stamp_ask.py --text "<the ask>"
--status answered --note "<how to do it>"`, which stamps through the op
door as every other stamp does and then rings the asker's phone once
through `/push/to-one` with the note's first line. `/stamp-stands`
already owns `note` as a field a stamp may set, so nothing there
changes.

## hostile cases

- **An answered ask that gets built after all.** `--status building`
  moves it on: `/being-built` claims it, this section drops it, and the
  note goes with it. Nothing here is terminal in the tree, only in the
  ladder.
- **A note on an ask the phone has deleted.** The stamp writes the list
  the world holds; a device that no longer has the entry has nothing to
  render, and the push still rings — the words are in the notification.
  The stamp says which worlds it wrote and prints nothing for the rest.
- **`answered` with no note.** The row shows the word and no block: a
  finished ask with nothing to read, which is honest rather than empty.
- **A note with several lines.** The block keeps them (`pre-wrap`); the
  push carries only the first, which is what a banner has room for.
- **`/straight-through` unticked.** `feature_StraightThrough` is
  undefined, the redefinition is skipped (typeof-guarded), and there was
  no guide to stop — `/ask`'s own results road returns.
- **`/lifecycle` unticked.** No requests section and no render chain to
  join (typeof-guarded); the answer lives in the world either way, and
  the push still rings.
- **`/comms/push` or `/push/to-one` unticked.** The note lands on the
  sheet; only the ring goes. `stamp_ask.py` reports the refusal and does
  not fail the stamp — the sheet is the record, the push is the courtesy.

## next (the seam is open)

`answered()` (which asks count), `rows(items)` (how the answer is drawn)
and `render()` (where it sits) are each one redefinition. The three
refinements to expect: a tap on an answered row that opens the feature
it names; an "that's not it" chip that puts the ask back to `asked`; and
answers folded away once read.

## glossary

- **answered**: an ask a person replied to instead of building, because
  miso already does the thing — terminal in the ladder, with the reply
  in the ask's `note`.

## code description

`no-guide.index.js` owns `feature_NoGuide` and does two things at load.

It redefines `feature_StraightThrough.match` to an async null
(typeof-guarded), which is where the search and the card both stop.

It wraps `feature_Lifecycle.render` (typeof-guarded), the standing
pattern `/being-built` and `/did-you-mean` use, and renders `#answered`:
`answered()` reads the `asks` var for the status, `rows()` draws the
`.crow` grammar with the note in an `.ansblock` beneath, and `render()`
places the section after `#requests`, `#didyoumean`, `#building` or
`#awaiting`, whichever is there, and at the top of the box when none is.

`no-guide.index.css` — the quiet pill and the answer block.
