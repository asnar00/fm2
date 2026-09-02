# while-editing
*an update never lands mid-sentence: writing is a task that finishes first*

> (transcripts/2026-09-02-self-check.md#p24)
> another thing: I want to make the update policy work better. Under "auto", I shouldn't be asked to OK updates, they should just happen (I should still be notified when the app isn't in focus). Otherwise, the app should just be up to date at all times without any action from me. The only exception should be that we shouldn't update while the user is recording or editing. As long as update doesn't switch the UI state, it should be completely seamless.

## user

If you are writing when a build ships — a post open with the pencil
pressed, words typed and not yet saved — nothing happens. Save (the tick,
the save pill, or a tap away), and the update follows on its own, your
words kept.

## spec

`/seamless` lets a task finish before an accepted build applies, and reads
busyness from the features that can be busy: `/dictate`'s mic and speaker,
`/phone`'s transcriber. Editing was not among them, and with `/by-policy`
stamping the acceptance the instant a build is known, an update could
reload a page out from under a sentence. This node adds writing to the
tasks that finish first.

An instance is **editing** when its open card is in edit mode — `/editing`'s
flag for the page on screen, the state the toolbar's pencil/tick reflects —
or when a text block still holds the caret, whose words reach the store
only on the tap away (`/keep/manual`'s rule). Either is enough: a card
unlocked but not focused is still being written; a focused block on a page
`/editing` does not govern is still unsaved text.

The deferral and the retry are `/seamless`'s: the build parks in `deferred`
and fires on the first state change that finds the instance idle. Saving is
usually that change — the tap away sends the block's text, the store
answers, the deferred build goes. The one save that changes no state is the
tick pressed with nothing focused, so this node also nudges the retry when
`/editing` locks the card, and the build follows the lock either way.

Recording stays covered by `/seamless` itself; nothing here re-reads it.

## parked

- "don't update while I'm on a call / in the camera" extends `busy()` the
  same way: a sibling node under `/seamless` reading the capture feature's
  live flag.

## glossary

- **editing**: (adds to `/seamless`'s mid-task) an own card open for
  writing, or a text block that still holds the caret.

## code description

`while-editing.index.js` extends `feature_Seamless.busy()` at load — the
current function is captured, the property replaced, and the captured one
is called first, so recording and transcribing keep their say — with
`feature_WhileEditing.editing()`: true when `feature_Editing.page()` is a
page whose id is in `feature_Editing.open`, or when `document.activeElement`
is a `contenteditable` element carrying `data-block` (the test `/manual`'s
save pill uses to show itself).

`retry()` fires a parked build the way `/seamless`'s own `feature_Loop.apply`
wrap does — `deferred` cleared, `feature_Review.apply(build)` called — when
one is parked and the instance is idle. A wrap of `feature_Editing.lock`
calls it on the next tick after the lock, covering the save that changes no
state. Every reference to another feature is typeof-guarded: without
`/editing` only the caret test remains; without `/seamless` the node is
inert.
