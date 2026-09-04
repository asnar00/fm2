# keeps-trying
*the server retries on its own clock, drops nothing, and says on the engineer
sheet what it is stuck on*

> (transcripts/2026-09-04-field-walk.md#p74)
> let's fix transcription and make it retry immediately if dropped - we can add notifications to the engineer section if need be.

## user

Nothing to do, and that is the point. A note whose words did not come back is
tried again a few seconds later, then a little later, then every hour, until
they do — whether or not anybody opens the app. If something on the server is
broken, the engineer section on the nøøb sheet says which clip is stuck and
why, and the line goes when the words land.

## spec

`/transcribed` had no clock. A drain ran when a message arrived — a recording
shared, a phone booting — so a job nothing could do at the time waited for
somebody to open the app before it was tried again, and after five tries it was
deleted. On 2026-09-04 that cost ash's 15:30 clip: the rung failed because
ffmpeg was not on the server's PATH, the job was re-queued by hand, and it sat
there until his phone next spoke. Three things change.

**The server tries at boot and keeps trying.** One thread, started before the
listener, looking every **ten seconds**. Ten and not thirty because ten is also
the first step of the backoff, and a schedule finer than the clock reading it
is a fiction. A look is a handful of directory entries; a drain starts only
when a job is actually due, so an idle box does no work but the look.

**Nothing is dropped, and the try count becomes a schedule.** A job that did
not land is rescheduled: **10 s, 30 s, 2 min, 10 min, then hourly**, for as
long as it takes. Quick enough that a rung you fix while watching is tried
while you are still watching; slow enough that a clip nothing can do costs one
attempt an hour. A job older than a day is **parked** — moved to
`queue/parked/`, not deleted — so it is out of today's way and still nameable.
The parent's five-tries-and-delete is where those two answers used to be; they
are seams now, and this node's answers replace them.

**An empty transcript is an answer, not a failure.** A rung that ran and heard
nothing is a clip with no speech in it: the job is done, the post keeps its own
words, and nothing is retried. Two silent morning clips were retried five times
each and then dropped because `/api` read Speechmatics' perfectly good empty
transcript as a failure — that classification is fixed at the rung, where the
difference between "I ran" and "I could not run" is actually known.

**And it says so.** `GET diag/transcribe` answers with one line per waiting
clip (id, whose, tries, when it is next due, why it did not land), one per
parked clip, and — when nothing on the box can transcribe — the reason: no key,
no script, no worker beating. The engineer section draws them; the line goes
when the job does.

**A lock whose holder is gone is no lock.** The parent's queue lock goes stale
after twenty minutes, which is right for a process still working and wrong for
one that has died. During a `/handover` two servers share the file and the
incumbent is draining: if it exits mid-clip, twenty minutes of nothing would
follow. A lock naming a pid that is not running is dropped at once, and `kill
-0` is the question — so the successor picks the work up on its next look, not
twenty minutes later. A lock held by a *live* pid is still respected, which is
what keeps the handover's two servers from both draining.

**What is still dropped, silently:** a clip whose post is a tombstone, and a
job whose clip is not on disk. Both are unchanged and both are right — a
deleted note is never re-worded.

## glossary

- **parked**: a job kept but set aside after a day of failing; it is on the
  engineer sheet and it is not deleted.
- **the keeper**: the thread that looks every ten seconds and starts a drain
  when a job is due.

## code description

`keeps-trying.rs` is server-side but for its notice file.

`serve` is the entry: one keeper thread, then `existing.serve()` — which never
returns, so the thread is started first. `keeps_trying_pass` is one look: write
the notice, and if anything is due and a rung is reachable, drain.

`transcribed_jobs` is narrowed to the jobs whose `next` has passed — that one
filter is what turns the parent's count into a schedule, and the parent's drain
needs no other change. `keeps_trying_all` reads the queue directly for the
notice, because a chain call belongs to its own function and the notice needs
the ones that are *not* due.

`transcribed_retry` and `transcribed_expire` are the parent's two seams,
redefined: reschedule on the backoff and never delete; park past a day.
`keeps_trying_park` moves the file and keeps it.

`transcribed_take_lock` drops a lock whose pid is not alive and then defers to
the parent's, so the twenty-minute rule still governs a holder that is running.

`keeps_trying_why_not` is diagnostic only and deliberately duplicates the
rungs' readiness facts: a rung node may be unticked, and a sentence that has
drifted is better than a node that will not link. `transcribe_best_grade()`
remains the only authority on whether a rung is reachable; if the two disagree,
the number is right.

`route` answers `diag/transcribe` by reading the notice file — screened as
`/self-check`'s GET is, free on localhost and owner-only through the tunnel,
because it names clip ids. Worlds are named by their four-digit tag, never by
the phone number that keys them.

`keeps-trying.index.js` fills the engineer section through `/engineer`'s own
capture-replace-append idiom. The fill is synchronous and the answer is a
fetch, so the block draws from what is held and the fetch calls `refresh()`
when it lands; one request in flight at a time, and never within five seconds
of the last.
