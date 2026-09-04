# shows-progress
*the post says its words are on the way, and stops saying it when they arrive*

> (transcripts/2026-09-04-field-walk.md#p77)
> we should also show a "transcribing..." indicator on the post with some animation to let the user know that something is happening. I'd also suggest that we use our agent to figure out a title for the post once the transcription comes through

## user

Stop recording and the post is there at once, with a dot breathing beside the
play button and the word **transcribing…** where the clip's length usually
sits. Its tile in the grid carries the same quiet dot. When the words land,
both go. If something on the server is stuck, the word changes to **still
trying** rather than breathing at you for ever.

## spec

**The hint existed and had never drawn.** `/as-posts` swaps a post's empty
placeholder for "transcribing…" when `dict_transcribe` names its recording,
and `/dictate`'s scheduler only ever sets `dict_transcribe` when a rung's
**page** slot answers ready. Every rung we have is the server's; none of them
ever redefined a page slot; so the value has been empty since the mini took
transcription over and the hint has drawn for nobody. The state was in the
wrong half of the app.

**So the server sends it.** A `Transcribing {working, stuck}` message to the
owner's audience, carrying the recording ids that world is waiting on. The
world keeps it as `dict_working` and the drawn page reads it.

**Published on change, never on a timer.** The broadcast slot holds fifty
entries and every waiting phone re-parses it five times a second, so a message
every ten seconds per world would be the most expensive thing on the box. A
world's set can change at exactly three moments — a clip joins the queue, a
clip leaves it landed, a clip is rescheduled — and those are the three links
this node adds. The last set sent is kept beside the queue and an unchanged
set sends nothing.

**Stuck is a different word, not a louder one.** A job that has failed three
times, or one that has been parked, reads **still trying**. Nothing is given
up on, so the post never goes quiet on its own; the engineer sheet has the
reason (`/keeps-trying`), and the post has the sentence a canvasser needs.

**The manner is the house's.** One dot, breathing at 1.6s ease-in-out opacity
— `/taste` 5, and `/dictate`'s recording dot is the same idea at a different
tempo. Never a spinner. Under `prefers-reduced-motion` the dot is steady: the
mark is the information, the breathing is only the manner.

**Two surfaces and no more.** The play row of an open post, and the post's
tile in the grid. `/as-posts` decided a fourth mark in a tile row was more
than was asked for and this keeps to that: the tile gets the dot beside its
title and nothing else.

## glossary

- **working**: a recording whose words the server is still fetching.
- **stuck**: a working recording that has failed enough times to be worth a
  different word on the screen.

## code description

`shows-progress.rs` is mostly server-side.

`shows_progress_set(world)` reads the queue directory and sorts the ids into
`working` and `stuck` — three failed tries, or parked, is stuck.
`shows_progress_tell(world)` compares that against the last set written to
`told.json` beside the queue and publishes only on a difference.

`transcribed_queue`, `transcribed_finish` and `transcribed_retry` are the
three parent links this node extends, each calling `existing` first and then
telling. They are the only three moments a world's set can move.

`update` keeps a `Transcribing` message as `dict_working`.

`render` marks the drawn page: `data-work="on"` or `"stuck"` onto the play
row, which already carries `data-rec`, and onto the post's tile, found through
`cards_read()` by the card that owns the recording. Marking the drawn page
from `render` is `/as-posts`' own idiom for the same reason — only one card
page is ever open and the loop state is only here.

`shows-progress.css` is the whole appearance: the duration hidden while the
words are coming, one breathing dot, one line of text, and a
`prefers-reduced-motion` block that stops the animation and keeps the mark.
