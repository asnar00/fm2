# stored-words
*the words the exchange already holds for a recording reach its post*

> (transcripts/2026-08-28-mini.md#p10)
> those were two posts made ages ago, and they were correctly transcribed and the text was stored with the dictaphone record. we need to find those records and extract the text, and then poke it into the post.

## user

Your old recordings were transcribed on the day you made them, and the words
were kept on the exchange beside the audio. When those recordings become
posts, the words are in them — nothing has to be heard again.

## spec

**Where the words are.** The exchange holds two records of a recording's
transcript, neither of which the client ever asked for at boot: the blob
index (`index.json`, `/mirror`'s per-user list, whose entries carry
`transcript`/`t_rung`/`t_grade` when the recording was announced after it
was transcribed) and `words.json` beside it, a per-recording `{text, rung,
grade}` map written by an earlier build. `/as-posts` lands a file's
`transcript` in its post's words — but only ever saw one on a device that
had just transcribed, because `RecList` reseeds from device metadata (no
transcript) and the `RecIndexed` reply came back without the stored words
for a file the device already listed (`merge_remote` adds unknown files and
ignores known ones). So two posts made from old recordings had empty words,
and were filled by hand through the op door on 2026-08-28 before this node
existed (the recovery move; the hash `auto` was set so the pass still owns
them).

**The reply carries the words.** `handle_msg`, server side, outermost: a
`RecIndexed` reply gets each item's `transcript` filled from `words.json`
when the item has none, with `t_rung` and `t_grade` beside it. The index
itself is untouched — it is read, not rewritten.

**A known file learns its words.** `update`, client side, outermost: on
`RecIndexed`, every item carrying a transcript stamps the matching
`dict_files` entry that lacks one (the entry `merge_remote` left alone),
with its rung and grade — so the scheduler does not queue a re-hearing —
and then `as_posts_sync` runs once more in the same turn, so the words land
in the post on the very event that brought them.

**The user's own words win**, as `/as-posts` promised: the landing goes
through `as_posts_land`, which replaces text only while it is empty or still
hashes to what the pass last wrote.

## hostile cases

- **No `words.json`, no `transcript` in the index.** Items pass through
  unchanged; the phone transcribes the file when it holds the blob, as before.
- **A word already in the post.** `as_posts_land` compares and does nothing.
- **A recording deleted as a post.** The tombstone keeps `rec` and no words
  are written into a tombstone: `as_posts_land` finds no text block.
- **`/as-posts` unticked.** This is its child and goes with it.

## code description

`stored-words.rs` extends `handle_msg` (server): after the chain answers, a
`RecIndexed` reply has `stored_words_fill` run over its items —
`stored_words_read(user)` is the `words.json` map for the sender's blob
namespace, the same `_from`-or-`_local` derivation `/mirror` uses.

`stored-words.rs` extends `update` (client): on `RecIndexed`, after the chain,
`stored_words_stamp` copies each item's transcript onto a listed file that
has none, then `as_posts_sync` lands it.
