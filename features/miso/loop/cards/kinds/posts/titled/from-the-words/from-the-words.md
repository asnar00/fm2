# from-the-words
*once the transcript lands, the post names itself*

> (transcripts/2026-09-04-field-walk.md#p77)
> we should also show a "transcribing..." indicator on the post with some animation to let the user know that something is happening. I'd also suggest that we use our agent to figure out a title for the post once the transcription comes through

## user

A note you recorded arrives with its words and then, a second later, with a
name — one short line in the app's own voice, on the card and on its tile in
the grid. If you had already typed a title, nothing touches it. If you type
one while the name is being written, yours wins.

## spec

A post minted from a recording has an empty title until a thumb types one, and
a grid of untitled notes is a grid you cannot read. The words are here by the
time the transcript lands, so that is the moment to name it.

**Only ever into an empty title.** `/keep`'s rule, checked three times because
the call takes a second and a thumb does not wait: before the call, before the
write, and again on the card that is actually being written. A title the
author typed is never replaced, and a note deleted while the call is out is
dropped.

**No speech, no call.** A clip with nothing said in it lands no words, and a
title from no words would be an invention.

**Haiku 4.5, and the cut is made at the model.** The cheapest current model
that can do this, and six words is well inside it: `max_tokens` 64, a system
prompt that asks for the title and nothing else, and `none` as the answer when
the words are too garbled to name. The tidy afterwards only strips quotes and
a trailing stop and enforces the six — the shortening is the model's job, not
the stylesheet's. Rust has no Anthropic SDK, so the call is raw HTTP by
`curl`, with the key on stdin inside a `-K` config and never on argv
(`/off-argv`; `/reports` made the same call the same way). Every call logs what
it read, what it wrote, and what that came to at the published rates.

**The prompt carries the place.** The note's words, the project's name, and the
first dozen phrases `/vocabulary` seeded the transcriber with — so a title may
name the street the note was made on, spelled the way the map spells it.

**A failure is retried on the same rhythm as everything else.** Titles have a
small queue of their own beside the clips', worked by `/keeps-trying`'s keeper
on the same backoff. It is a separate queue because a clip whose words have
landed is finished, and re-queueing it would ask for the transcript again. No
key on the server is one of those failures: the title stays empty and the log
says why.

**And it travels like the words.** The block, a bumped `edited`, a stamp into
the owner's world, and `/exchange`'s hand-on — which a background thread has
to do for itself, for the reason `/transcribed`'s landing documents.

## glossary

- **auto title**: a title this node wrote; marked `auto` on the block, and
  replaced by anything the author types.

## code description

`from-the-words.rs` is server-side entirely.

`transcribed_land` is the hook: after the words are in, the job is written to
the titles queue and tried at once. `keeps_trying_pass` is extended so the
keeper works the ones that failed, on `keeps_trying_wait_ms`' backoff, giving
up after a day.

`from_the_words_try` is one attempt, and it is the place all the refusals
live: no card, a tombstone, a title already there, no key, a bad answer.

`from_the_words_around` builds the context half of the prompt — the project's
name off the author's own cards, and `/vocabulary`'s nearest phrases.

`from_the_words_ask` is the call; `from_the_words_reply` reads it in the order
that matters (error, then `stop_reason`, then content) and prints the cost
line. `from_the_words_tidy` strips what the model was asked not to add.

`from_the_words_write` re-reads the card, refuses a title that appeared while
the call was out, writes the block, bumps `edited`, stamps and hands on.

This node needs `/transcribed` and `/keeps-trying` present — it hangs off the
landing and the keeper. That is a real coupling and the price of the title
being written where a title belongs rather than inside the transcriber.
