# unmixed
*two people's event streams are two streams*

> (transcripts/2026-08-21-hybrid.md#p56)
> let's fix all residuals next.

## user

For operators. A line in `/tmp/miso-blackbox.log` now says whose stream it is —
`ash:44f7d39f38fd`, a name from the guest list and the stable id that person is
addressed by everywhere else. It used to say `…0123`, the last four digits of a
phone number, so two guests whose numbers ended the same wrote one interleaved
stream that could not be pulled apart afterwards.

`replay.py --who ash` picks a session by either half of the label. No phone
number is written to the log.

## spec

The last-four tag was chosen when a label only had to be followed by eye in a
login line — "enough to follow a login in the log, not enough to leak". The
blackbox reuses it as a **key**: every batch of device events is filed under it,
and `replay.py` reconstructs a session by selecting lines with it. Under a
collision that reconstruction is wrong in the worst way — it looks like one
person doing two contradictory things.

**What the log is for decides the label.** It is a debugging aid: an operator
reads it to see what happened on somebody's phone, and then usually to talk to
that person about it. That needs two properties the tag has one of — it must
separate people (the tag does not) and it must identify them (the tag roughly
does). So the label carries both, joined without a space because the reader
splits each line into timestamp, label and body: the guest list's name for the
identity, lowercased to one word, and the first 48 bits of the opaque
`sender_audience` id that `/whole-number` already uses to address the relay.

**Not the full number.** This file lives in `/tmp` on a shared machine, which is
exactly the argument that kept phone numbers out of the broadcast buffer. The
name is already what the auth lines print, so the log gains nothing new about
who exists; the id is unguessable and stable, so two sessions from one person
still join up.

**A stream from an identity that is no longer on the guest list** keeps its id
and is named `guest` — a label that is still stable and still unmixed.

**Localhost tooling** — no cookie, no identity — falls through to the chain
beneath, which is the tag as it always was. Nothing reaches this route without a
valid session anyway; the fallthrough exists so that unticking `/whole-number`
is a link error rather than a silent relabelling.

The seam this hangs on is new: `/blackbox` had the tag inline, so it was
refactored to compute its label in one function, behaviour unchanged, and this
node redefines that function.

## glossary

- **stream label**: `<name>:<id>` — what a blackbox line is filed under.

## code description

`unmixed.rs` redefines `blackbox_who`, `/blackbox`'s new labelling seam.

`blackbox_who` (line 8) builds the label from the cookie-proven identity, and
hands back to the chain beneath when there is none.

`stream_name` (line 18) is the guest list's name for the identity, lowercased
with every other character replaced by `-` so the label is one word;
`guest` when the identity is not listed.

`stream_id` (line 38) is the first 12 hex characters of the same opaque id the
relay is addressed by.
