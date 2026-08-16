# mirrored-words
*a transcript made on one device reaches all your instances, like the audio does*

> (transcripts/2026-08-16-fm-spec.md#p2)
> yeah, I think I'd like to first fix the distribution issue with the transcript

## spec

The field found the gap (notes.md, fm-spec-2 #p22): the phone transcribed a
recording on-device, but `/mirror` moves only metadata and audio — the words
never left the phone, and the laptop set about re-transcribing from scratch.
This node makes transcripts part of the mirrored record. Words are small
facts, so they travel eagerly like metadata: a `TranscriptShared` message
carries `{id, text, rung, grade}` through the persistent outbox; the exchange
stores words per user beside the blob index and broadcasts them to the user's
audience. Collisions follow `/phone`'s standing rule — a better grade replaces
a rougher one, in the store and on every device; equal grades keep what's
there (which also makes the origin's echo a no-op), with one honest
exception: real words beat the empty stamp a failed local attempt leaves
behind, even at the same grade — otherwise a device whose own attempt
failed would refuse the words another device actually made. Empty
transcripts (the failed-attempt stamps) never travel — never replicate a
lie about what was said.

Catch-up is the same two-speed doctrine as `/mirror`: the boot `RecIndex`
reply arrives with words already stamped on its items, which also repairs the
day-3 reseed loss — `RecList` wipes the stamps, the next catch-up restores
them, and the laptop stops re-deriving what the phone already said.

## user

Record a note on your phone and let it transcribe. Open miso on your laptop:
the note's words are already under its tile — no waiting for the laptop to
work them out again. Wherever a transcript is made, every logged-in instance
gets it; the "local draft" stamp travels with the words, and better versions
still replace rougher ones everywhere when higher rungs arrive.

## glossary

- **words store**: the exchange's per-user `words.json` — the best transcript
  known for each recording, keyed by id, graded like every rung result.

## code description

`mirrored-words.rs` server half extends `/mirror`'s exchange. `handle_msg`
claims `TranscriptShared`: sanitise the id, refuse empty text, and merge into
the sender's `words.json` only when the grade beats the stored one; on a real
improvement, publish the same message to the `user.<sender>` audience. For
`RecShared` it pre-enriches the announcement's metadata with stored words
before delegating to the chain, so a recording announced after its words
arrived (the offline-outbox ordering race) carries them into the index and
the broadcast. For `RecIndex` it delegates first, then stamps the reply's
items from the words store — boot catch-up delivers words without a second
round trip.

`mirrored-words.rs` client half: `update` claims `TranscriptShared` (stamp
the matching `dict_files` entry) and post-processes `RecIndexed` (adopt
words onto entries `/mirror`'s merge skipped because the file was already
here). Both routes go through `adopt_words` — non-empty text, better grade
or the equal-grade-beats-empty exception — and re-run `transcribe()` so the
scheduler drops intent for work the mirror just did. Known accepted gap: `/dictate`'s own `Transcribed` handler stamps
unconditionally, so a slower local result can momentarily overwrite an
equal-grade mirrored one — same grade, equally honest; revisit when a
higher rung exists.

`mirrored-words.js` is the announcer: it wraps `feature_Loop.apply` and after
each apply scans `dict_files` for transcripts not yet shared (a
`localStorage.misoWordsShared` map of id → announced grade), queueing
`TranscriptShared` through `feature_Messaging`'s persistent outbox —
offline-safe, flushed on reconnect, silent during `/replay`. A mirrored
transcript gets re-announced once by each receiving device; the server's
grade check makes that a no-op, an accepted cost for keeping the page half
free of provenance bookkeeping.
