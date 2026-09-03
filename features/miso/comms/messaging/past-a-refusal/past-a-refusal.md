# past-a-refusal
*one write the server will never accept must not stop every write behind it*

> (transcripts/2026-09-03-invite-test.md#p159)
> do everything now
> *(the second of the two things #p158 asked for: a worker's contact report
> showed the outbox is a FIFO queue whose `flush()` does `if (!r.ok) break;`,
> so an op the server answers 400 to sits at the head forever and everything
> minted after it never leaves the device.)*

## user

If the app ever sends the server something it will not accept, that one
message is set aside and everything else keeps going. Before, it stuck at the
front of the queue and nothing sent after it ever left your phone — you would
have seen a phone that looked normal and synced nothing.

## spec

The outbox is a persistent FIFO and its order is a promise: an op minted
before another must land before it. `flush()` kept that promise with
`if (!r.ok) break;` — every refusal is treated as "not yet". That is right for
every refusal that can change its mind, and wrong for the one kind that
cannot.

The kind that cannot is a body the server will not parse. `/messaging`
truncates a body over `msg_body_cap()`, the truncated JSON fails to parse, the
message reads as untyped and the endpoint answers **400**. Retrying sends the
identical bytes, so the answer is identical, forever. Measured on a rig:
177,704 bytes → 200; 197,903 → 400; 217,153 → 400. `/room-for-a-team` raises
where that line falls; it cannot remove the line, so the queue needs to know
what to do when a message is on the wrong side of it.

**The exact test, from what this server actually answers.** `POST /msg` has
exactly two refusals of its own: **401** `log in first` when a tunnel request
carries no valid cookie, and **400** for an untyped or unparseable body. A 401
changes its mind the moment the user logs in; a 400 never does — both of its
causes (truncated-because-oversize, and genuinely untyped) are properties of
the bytes, not of the moment. A handler's own complaint is *not* a refusal at
all: `/converge` answers a malformed `CtxOp` with **200** and `{ok:false}`, so
nothing in the chain reaches this decision by accident.

So: **drop on a 4xx that is not 401, 403, 408 or 429; wait on everything
else.** Those four are named rather than assumed — auth arrives, a rate limit
lifts, a timeout was the network — and 5xx and a thrown fetch keep the old
`break` exactly. Today that rule reduces to "drop a 400", which is what it is
for; it is written as a class so a later node answering 413 or 422 gets the
right behaviour without editing this file.

**Nothing is lost silently.** A dropped message is reported to `/diag` with
its type, its size in bytes, the status, and how many messages were behind it
in the queue, so a build's log says which device dropped what and how big it
was. Because a 400 can only be received from a server that answered, the
report has a network to travel on by construction. It is also written to a
bounded local record (`misoDropped` in localStorage, last 10, oldest evicted)
so the evidence survives a failed report and a probe or a rig can read it back
without ssh. The record is diagnostic, not the data: what it evicts is an old
report, never a message.

The two records fail independently and neither failure stops the drop: with
`/diag` unticked or its POST lost the local record still holds the row, and
with localStorage full or unavailable the report still goes. Both can fail at
once — a device whose storage is full and whose network died in the moment
between the 400 and the report — and then that drop is silent. That is a
narrower window than the one it replaces, where every drop was silent *and* the
queue was dead behind it, but it is not nothing. It is why the report carries
the queue depth: a device that starts dropping usually drops again, and the
next report says how much was waiting.

**Where the dropped value goes, for `cards` specifically.** A `cards` write is
a whole-list `set` op: the value it carries is the device's entire cards list
as of that turn, and the device's own world already holds it — the op is the
copy, not the original. So the next turn that touches `cards` at all mints a
fresh op carrying the whole list again, this one included, and if that one
fits, the server catches up completely. Nothing needs to be replayed and no
merge is needed, because a whole-list `set` is idempotent in the value.

The honest limit of that: while the list is over the cap, *every* write is
over the cap, so every one is dropped and reported and the server stays at the
last write that fit. The device is never wrong; the server and the user's
other phones go stale, and the diag log says exactly when it started. That is
strictly better than the jam it replaces — where the same staleness happened
**and** every unrelated op (a `Join`, a `RecShared`, a tap, another var
entirely) was stuck behind it with no record anywhere — but it is not a repair.
The repair is the list fitting: `/room-for-a-team` today, the per-card
foundation after it.

**What happens when it fires wrongly.** If a node ever answers 4xx to a
message that *would* succeed on retry, this drops it and the value is gone
from the wire — recoverable only if some later turn re-sends it. That is why
the retryable statuses are named explicitly and why the report carries the
message's type: the diag log is how a wrong answer is found. Untick this node
and `flush()` breaks on every refusal exactly as it did before — the old jam
returns, which is the state the tree shipped in until today.

## glossary

- **refusal**: an HTTP answer to an outbox POST that is not ok; *permanent* if
  retrying the same bytes cannot change it.

## code description

`past-a-refusal.js` redefines `/messaging`'s `refused(status, msg)`: false for
401, 403, 408, 429 and for anything outside 400–499, true otherwise. On true
it reports the drop through `feature_Diag.report` (typeof-guarded — without
`/diag` the drop still happens and the local record is still written) and
pushes it onto `misoDropped`, a 10-entry ring in localStorage. Every read and
write of that ring is wrapped, so a full or unavailable localStorage costs the
record and never the decision.
