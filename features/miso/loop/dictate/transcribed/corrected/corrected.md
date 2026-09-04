# corrected
*the names the recogniser misheard, matched against the map before the words
land*

> (transcripts/2026-09-04-field-walk.md#p154)
> the "bourke and bloor" was taken at a specific GPS location, so we could (if we were being clever) look up the map to figure out what those words were actually likely to be :-) maybe there's a post-transcribe process where we can look over the text and identify words that are "interesting" or seem to be names, and see if we can match them to where we actually are.

> (transcripts/2026-09-04-field-walk.md#p155)
> build both now, I think. We'll probably need a manual transcription error fix UI later - we'll build that once we have some real field data.

## user

Nothing to see. A note that says "Bourke Street, corner of Bourke Street and
Bloor" arrives saying Berwick Street and Broadwick Street, because that is
where you were standing. Anything you have typed yourself is never touched.

## spec

Seeding gets the recogniser most of the way (`/near-the-post`), but it is a
hint and not a constraint: a name can still come back wrong. This is the
second look — the one that has the map in front of it and the sentence
already written.

**Before the words land, not after.** This is the newest link on
`transcribed_land`, so it runs first and hands the corrected sentence to
everything inside it: `/as-posts` writes the corrected words, `edited` is
bumped once, `/exchange` carries them, and `/from-the-words` titles what was
actually said rather than what was misheard. Landing twice would have put the
wrong words on every phone for a second and then titled them. The cost is that
the words arrive one model call later; a name that is wrong is worse than a
note that is two seconds late.

**Only where there is something to do.** Grade 2 or better (the on-device
whisper was never worth correcting), a nearby list to match against, and at
least one token that looks like a name — a capitalised word that does not open
the sentence, or any word beside a street word. No names, no call.

**Never over a thumb.** `/as-posts` stamps a text block with a hash of the
words it last wrote; a block that no longer hashes to its stamp was edited by
hand and this stops. `/keep`'s rule, checked before the call is even made.

**Haiku 4.5, given the sentence and the nearby list**, and asked which of those
the speaker most likely said — answer with the corrected sentence, or the
sentence unchanged. Key from `~/.agent-config.json` on stdin, never on argv
(`/off-argv`). A cost line per call.

**Three guards, and any one of them throws the answer away.** The word count
may not move by more than two. A word that appears must have been in the
original or in the nearby list — otherwise the model wrote something instead of
correcting something. A word that disappears must have been name-shaped —
otherwise it rewrote the prose. A discarded answer is logged, because the
discards are what say whether the guard is set right.

**Every correction is written down.** `corrections.jsonl` under the context
directory: what was heard, what was written instead, whose it was, and whether
it was taken. Ash wants a manual fix screen after the field test; this file is
its seed, and it is the only record of what the field actually sounded like.

## glossary

- **name-shaped**: a capitalised word that does not open a sentence, or a word
  beside one of "street, road, lane, close, hill…" — the cheap test for
  whether a correction pass is worth a call.

## code description

`corrected.rs` redefines `transcribed_land` and adds nothing else to any chain.

`corrected_pass` is the whole decision: grade, card, author's hand, a nearby
list, a name-shaped token, the call, the guard. Every refusal returns the
original text unchanged, so the failure mode of this node is the behaviour
without it.

`corrected_author_edited` reads `/as-posts`' own `auto` hash. `corrected_names`
is the name-shaped test. `corrected_nearby` is `/vocabulary`'s list for this
post, which with `/near-the-post` is the streets within four hundred metres.

`corrected_safe` is the guard, over word bags rather than an alignment: the
length test, the "appeared from nowhere" test against the nearby phrases split
into words (so "Broadwick Street" allows both), and the "disappeared and was
not a name" test.

`corrected_log` appends one line per correction, taken or discarded.

`corrected_ask` and `corrected_reply` are the call and its reading — error
first, then `stop_reason`, then content, then the cost line. This node keeps
its own copy rather than sharing `/from-the-words`': that one is writing a
title, this one is choosing between what was heard and what is on the map, and
a shared prompt would serve neither.
