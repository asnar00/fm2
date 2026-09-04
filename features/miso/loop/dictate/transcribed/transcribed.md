# transcribed
*the words come back from the server: a note is recorded, uploaded, and
transcribed where the model lives*

> (transcripts/2026-09-04-field-walk.md#p7)
> next: upload and transcription. The quality of the on-device transcription is really poor, so let's not do that. Instead, let's do the following: 1) stream audio and video to the server as we're recording it (or to a local cache with an upload queue if we're on a slow/nonexistent connection) 2) do transcription using the best available on the mac mini [look at the fieldnote project - that method got decent results; and maybe look online for other options also]; 3) seed the transcription with words taken from a context document based on our location (streetnames) and maybe a later a briefing document; 3) as soon as the post is complete, it should appear on other users' grid/list/maps, as appropriate to the publishing level.

## user

Make a video note. It uploads while you record. A moment later the post's
words fill themselves in — on this phone and on every other device of yours,
and then on the phones of everyone the post reaches. Nothing runs a model on
your battery, and you can lock the phone and walk on while it happens.

## spec

A **transcribed note** is a note whose words were written by the server, not
by the phone. The phone records and uploads; the mini owns transcription; the
words arrive as an ordinary edit to the post — the same edit a thumb would
have made, so it travels by the road every edit travels by (`/exchange`) and
needs no protocol of its own.

This node is the plumbing all of that shares, and no transcriber. It holds a
**queue**, a **rung interface**, a **landing**, and a **grade ladder**; the
rungs that actually turn sound into words are its children (`/api`, `/mini`),
and the words they are seeded with come from `/context`. With every rung
unticked this node queues nothing, which is the same as not being here.

**The queue is on disk, per world, beside the clips.** A clip that has
arrived and has no words yet is one small file in
`~/.miso-blobs/<world>/queue/<id>.json`; a clip that has been transcribed
leaves one in `queue/done/<id>.json` saying which rung wrote it and at what
grade. Nothing is held in memory, so a server restarted mid-job finds the
work where it left it, and a queue survives a deploy.

**A drain is triggered, never scheduled.** There is no timer. The arrival of
a clip (`RecShared`) and a device's boot catch-up (`RecIndex`) each spawn one
drain thread, and a lock file with a stamp on it (`queue.lock`, stale after
twenty minutes) means one clip is transcribed at a time on the whole box —
which is what a mini with 8.6 GB and one warm model requires. A drain makes
at most three rounds twenty seconds apart and then exits, so a job whose card
has not synced yet is retried inside the same drain and no thread lives
longer than a minute.

**The rung interface is one extensible function, and the ladder is walked
here.** `transcribe_rung(job)` takes `{world, id, path, vocab, want}` and
returns `{text, rung, grade}`, or the empty string. `want` is the grade being
asked for: a rung answers only when `want` is its own grade and otherwise
passes straight to `existing`. This node asks for the best grade
(`transcribe_best_grade()`, which each rung raises to its own only when it is
truly reachable), and on an empty answer asks for the grade below, down to
one. So each rung is tried at most once per clip, a rung that cannot work
today falls to the one beneath it rather than failing the clip — and no rung
has to know where in the chain it sits. That last part is the point: the
rungs share one anchor, and same-anchor siblings load in name order
(misses.md, 2026-09-03), so neither may assume it is outermost.

**The upgrade in place is the server's, not the phone's.** `done/<id>.json`
carries the grade that wrote the words. When a better rung comes into reach —
a key added, the network back — the next drain re-queues the clips whose
grade is below the best now available, at most twenty per pass and at most
twice per clip, so a rung that advertises a grade it cannot deliver costs two
runs and then stops. `/dictate`'s own scheduler keeps its ladder for the
reason it always had one: it is what draws "transcribing…".

**Silence is an answer.** A rung may come back with "there was nothing said
in this", and that stops the ladder: the clip is stamped done and the post
keeps its own words. It is not passed down to a worse rung, because a worse
rung asked about a hiss writes subtitle credits into somebody's note.

**Words land as an edit, and are dropped for the dead.** A returned text is
written into the post through `/as-posts`' own `as_posts_land` — the same
function, the same never-over-your-own-words rule (a hash of what was last
written; one keystroke and the words are yours for good) — and the card is
stamped into its owner's world through the door `/exchange` gives a card by,
so it reaches a phone that was switched off. A `Transcribed` message to the
owner's audience repaints every instance of theirs that is awake. If the post
is a tombstone, or has no card at all after the retries, the job is dropped:
a deleted note is never re-worded.

**And the landing hands the card on itself.** `/exchange` spreads a changed
card from its *route* link — it watches the caller's own cards across one
`POST /msg` — and a transcript landed on a background thread has no request
and no cookie to be that caller. Without the same two reads and the same
`exchange_share` call made here, the words would stop in the author's world
and a colleague who can see the post would watch it stay empty for ever. That
is what the rig showed on the day this was built, and it is the difference
between the design's part (4) working and only looking as though it does.

**What the phone is told.** The server answers a device's boot catch-up with
`TranscribeRungsAre {best, rungs}`, which lands in the world as `dict_rungs`;
the rung nodes' `transcribe_server` / `transcribe_api` slots read it, so
"transcribing…" appears exactly when a rung really is reachable. If that
message is missed the words still arrive — only the hint is absent, which is
the right way round.

## glossary

- **transcribed note**: a note whose words were written by the server from
  its recording, and may be replaced by its author at any time.
- **rung**: one way of turning sound into words, with a grade; the highest
  reachable grade wins, and a better one may re-write an earlier answer.
- **drain**: one pass over the queue, one clip at a time, under a lock.

## code description

`transcribed.rs` is server-side but for one link.

`handle_msg` is the trigger. `RecShared` (a clip has arrived) queues that id
and starts a drain; `RecIndex` (a device booting) starts one too, since that
is when a clip stranded by a restart gets its second chance. Both publish
`TranscribeRungsAre` to the sender's audience. Everything else goes straight
to `existing`.

`transcribed_drain` is the pass. It takes `queue.lock` (or returns, if a fresh
one is held), then for three rounds: every world with a queue, every job
oldest first, through `transcribed_run`; twenty seconds between rounds; and
the lock is released on every way out. `transcribed_upgrade` runs once per
drain and re-queues the `done` entries whose grade is below
`transcribe_best_grade()`.

`transcribed_run` is one job. It drops a job older than a day, one whose clip
is not on disk, and one whose post is a tombstone; it keeps and retries a job
whose post has not synced yet (five tries, then dropped with a line in the
log); otherwise it builds the vocabulary (`transcribe_vocab`) and walks the ladder,
calling `transcribe_rung` once per grade from the best down until one answers,
then lands the words and writes the `done` stamp.

`transcribed_land` is the landing: `as_posts_land` into the card, then
`transcribed_stamp` (a `CtxOp` `set` of that one card, handed to `handle_msg`
with the thread acting as its owner — `/reports`' idiom) and a `Transcribed`
publish. Landing twice is landing once: `as_posts_land` changes nothing when
the words are already there.

`transcribe_rung(job)`, `transcribe_best_grade()` and `transcribe_vocab(card)`
are the three extension points, with bases that answer "nothing here" — which
is what makes this node's presence alone a no-op.

`transcribed_root` and `transcribed_log_dir` spell out the two paths this node
needs rather than borrowing `/mirror`'s and `/remember`'s, so an untick
elsewhere cannot stop this node linking (`/pic-beside` documents the same two
lines for the same reason); they must stay in step if the stores ever move.

The one client link is `update`, which stores `TranscribeRungsAre` as
`dict_rungs`. The words themselves arrive as a card edit through `/converge`,
which needed no help.
