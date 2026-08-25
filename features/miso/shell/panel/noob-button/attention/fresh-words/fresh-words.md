# fresh-words
*only words you have not heard ring*

> (transcripts/2026-08-25-accounts.md#p26)
> I just got a second notification for the ask-question on zoom, that seemed wrong

## user

You hear about a builder message once. When the builder later moves your ask along — *building*, *shipped* — without saying anything new, nothing rings; the row in your requests list just changes.

## spec

`/attention`'s notification body was "the changed entry's question text, else its note". That was right for a question arriving, and wrong the moment the same entry changed again for another reason: a `building` stamp on an ask that still carried its (already answered) question re-sent the question's words as a fresh notification (#p26; the handover had named this as a defensible re-notification, and ash ruled it wrong).

This node redefines `attention_news` so that an entry may only speak words it did not carry before the change: the question text if it differs from the entry's previous question text, else the note if it differs from the previous note. A status flip that leaves both as they were is silent. "Nothing rings about nothing" now covers "nothing new".

## hostile cases

- A new question replacing an old one on the same entry rings (the text differs).
- A note edited to different words rings; a note re-stamped verbatim does not.
- An entry that is new in this change (no `was`) speaks its question or note as before.

## glossary

(no new terms)

## code description

`fresh-words.rs` redefines `attention_news` with the same changed-entry walk as the base and one different question: `fresh_words_of(was, now)` — the question text if new, else the note if new, else empty.
