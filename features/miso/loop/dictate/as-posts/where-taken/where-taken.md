# where-taken
*a recording's post is placed where the recording was made, the moment it is made*

> (transcripts/2026-09-03-housekeeping.md#p10)
> all posts should carry the location at which the media was taken, in preference to the time/place the post was made or opened.

## user

Film a clip or speak a recording and the post it makes stands on the map where you were when you made it — as a photo post already does — and its place reads the time you made it. Not where you happened to be when you first opened the post.

## spec

A photo post takes its place from the picture at capture (`/from-picture`, source `picture`). A recording's post took none: `/location` gave it one the first time its page was looked at, from the device and stamped then — so last night's Brixton clip stood in Soho, placed at 09:30 this morning, and "still overlapped" (#p9a) was two posts that were never at one place. Ash's ruling (#p10): every post carries the location at which its media was taken, in preference to where or when it was made or opened. One reading, so it builds, once for every recording: `/as-posts` mints a card for each `RecSaved` and writes the recording's id on it as `rec`; this node watches that one event on `/loop`'s send, asks the device once for its position (a minute's cached fix, ten seconds' patience — the prompt is one `/live` and `/location` already earned), finds the card by `rec`, and places it with `/location`'s own `CardPlace`, source `device`, stamped with the recording's own time rather than the fix's. The card page opens as the post is made and `/location`'s dim pill asks too; whichever lands second wins, and both say the same place — the phone has not moved. Denied, absent or slow: nothing is written and `/location`'s ask remains the road it was. A card that cannot be found (the mint never happened) writes nothing. Recordings already misplaced cannot be put back — nothing recorded where they were made — and are left as they are (`/retrofit`: the shape of new posts changes; old ones keep the place they have). Untick and a recording's post is placed on first viewing again.

## hostile cases

- Location denied: no block; the pill on the page asks as before.
- Two recordings in a minute: both placed from one cached fix — a minute's walk at most.
- The post deleted before the fix arrives: the card is gone from the list, nothing is sent.
- A recording announced by another device (`RecShared`), not saved here: not this event, not placed here.
- `/replay` re-sending a `RecSaved`: the position asked again is the replaying device's — `/replay` pauses recording, and a replayed place is the one thing here that could differ; accepted, replay is a diagnostic.

## glossary

(no new terms)

## code description

`where-taken.js` — wraps `feature_Loop.send`: after a `RecSaved`, one `getCurrentPosition`; on a fix, the card whose `rec` is the recording's id gets a `CardPlace` stamped with the recording's `t`.
