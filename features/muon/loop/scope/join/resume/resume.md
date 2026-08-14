# resume
*coming back to foreground is a join moment too*

> (transcripts/2026-08-14-fm-spec-2.md#p31)
> 2) join on background->foreground as well as startup

## spec

A backgrounded instance is frozen (iOS suspends PWA JavaScript), and its
long-poll resumes with `since = lastV` against a broadcast ring that only
holds 50 entries — a long absence can have a hole in it. So returning to
foreground re-joins: the same `Join` through the same outbox, deduplicated,
also fired on the browser's `online` event (a warm tab regaining network).
This completes join's generalisation: boot, reconnect and resume are one act
— "catch up whenever you might be stale".

## user

Nothing to operate: switch back to muon after any absence and the values on
screen are current, not what they were when you left.

## glossary

(no new terms)

## code description

`resume.js` listens for `visibilitychange` (to visible) and `online`; both
queue `{type:"Join"}` into `/messaging`'s persistent outbox and flush,
skipping if a Join is already queued. By resume time messaging is long
initialised, so the direct queue push is race-free (unlike boot, which goes
through the state outbox — see `/join`); the reply is the same `VarJoin`
event, applied by the same machinery.
