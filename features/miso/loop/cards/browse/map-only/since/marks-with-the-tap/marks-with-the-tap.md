# marks-with-the-tap
*the pill's own tap carries the three midnights, so the filter can never be waiting to be told the time*

> (asks#1788532331774)
> bugfix: today/week/month doesn't actually filter the posts I can see
> *(filed from the field on 2026-09-04 by ash, on build 649)*

## user

Tap **today** and you see today's posts. It works the first time, on a phone
that has just been opened, whatever the app was doing while it started.

## spec

`/since` computes local midnight on the page — the wasm half has no clock and
no time zone — and sends it in once, at load, as a `SinceMarks` event. On ash's
phone that send never arrived, so `since_cut()` answered 0, so every period
behaved as **all** and the pills did nothing. That fail-open is `/since`'s own
designed state *before the marks arrive*; the bug is that they never did.

**Two faults in one road, both in `feature_Since.tell`.**

The readiness test is the weaker of the two available: `tell` is called from a
poll on `typeof feature_Loop !== 'undefined'`, and that object exists as soon
as its fragment is parsed — long before the wasm world is up and
`feature_Loop.state` is anything. `/restore`, two nodes away, waits for
`feature_Loop.state !== null`, which is the real test. On a desktop rig the
wasm was up by then anyway; on the installed app it was not.

And the send is latched before it is made: `this.sent = m` is assigned *above*
`feature_Loop.send(...)`, inside a `try` whose `catch` says "not up yet" and
does nothing. So a send that failed is recorded as sent, and every later
call — the `visibilitychange` road included — returns early on `m ===
this.sent`. One missed send at boot is permanent for the life of the page.

**The cure is to stop depending on any particular send.** The three midnights
ride **every** event the page sends. `feature_Loop.send` is the one choke point
every road goes through — the loop's delegated `[data-ev]` listener,
`/on-release`'s synthetic click for a press held past 120 ms, `/drive`, and
every node that mints an event of its own — so the marks are attached there, to
all of them, and the Rust half takes them off and writes them before the chain
runs. Whatever the phone does next refreshes the marks; the filter can be stale
only until the very next event of any kind, and no single send is load-bearing.

They ride at the **top level** of the event, beside `type`, not inside `data`:
`data` belongs to whichever node minted the event and its shape is that node's,
while nothing in the tree reads an unknown top-level key.

**Stale marks are worse than none, which is why the tap alone is not enough.**
Ash's case (the ruling that sharpened this node): *shut the app down for two
days, come back, and don't tap the filter selector* — the stored marks are two
days old, so **today** means the day before yesterday, and the map shows a day
that is not today while looking perfectly correct. A fail-open filter announces
itself; a filter set to the wrong day does not. So the marks must be refreshed
without anyone asking, and they are: by every event, and by three timers of
last resort.

**The three roads that need no finger.** At boot: a poll that waits for
`feature_Loop.state` — the real readiness test, which `/restore` two nodes away
already uses and `/since` did not — and keeps trying until a send gets through,
setting `sent` only *after* one does. On `visibilitychange`, for a phone
carried into another zone or woken in a pocket. And at the **next local
midnight**, for the app left open on the map overnight: the timer clears `sent`,
re-sends, and the turn that lands repaints the map on the new day. It re-arms
each time it fires, so night after night.

**The one frame this does not reach.** The first paint after a cold launch is
drawn from whatever the device last stored, before any event has been sent, so
a phone that has been shut for two days can show one stale frame. The boot
chase fires as soon as the loop has state, which is well before the map has a
world to draw, so in practice the marks are right by the time anything is on
screen — the simulator confirms it — but the honest statement is that the
correction is one turn, not zero. Making it zero means not persisting the marks
at all, which is the parent's var to change.

**`/since`'s own `tell` is left exactly as it is.** It is the parent's code and
unticking this node must give the parent back unchanged. Its send is harmless
where it works and inert where it does not; this node's send is idempotent
beside it, because the Rust side ignores marks that have not changed.

## hostile cases

- **An event while the marks are already right.** The same string arrives, the
  write is skipped, and nothing is queued — which is every event after the
  first each day.
- **An event from before this node** (a replayed black-box log, a message from
  another device): no top-level `marks`, nothing is written, and the event does
  what it always did.
- **An event that already carries `marks`** — the wrapper leaves it alone, so
  a replay keeps the marks it was recorded with rather than today's.
- **The app shut for two days, opened, no pill touched.** The boot chase sends
  today's marks as soon as the loop has state; the filter is right from the
  turn they land, and every later event keeps them right.
- **Midnight while the app sits open.** The timer fires at 00:00:01, the marks
  move on a day, and the map repaints on that turn.
- **A long press on a pill.** `/on-release` dispatches the click at the armed
  button and `/since`'s own swallow drops it, so no period change and no
  marks — correct, because nothing was chosen.
- **The clock moves between the tap and the render.** They are one turn; it
  cannot.
- **A launch with `today` already stored and the loop slow.** The map is
  unfiltered for as long as the marks take to land — which is now bounded by
  the poll rather than unbounded — and then filters itself on the turn they
  arrive. Every period is **all** until then, never empty.
- **`/since` unticked.** This node is its child and goes with it.
- **A second device.** `period` is a user var and follows the person;
  `day_starts` is device-scoped and each device tells its own.

## parked

- Reading the marks back to confirm they landed: `day_starts` is not bridged,
  so the page cannot check its own work. A `js:` column on the parent's var
  would let the resend stop exactly when it has succeeded rather than when a
  send did not throw.

## glossary

(no new terms — **the marks** is `/since`'s)

## code description

`marks-with-the-tap.js` — `feature_MarksWithTheTap.marks()` is `/since`'s three
midnights, recomputed at the moment it is asked. `up()` is the readiness test
the parent should have used: `feature_Loop.state` present, not merely
`feature_Loop` defined. `tell()` sends `SinceMarks` and records it as sent only
*after* the send returns, so a failure is retried. `chase()` polls `tell()`
until one gets through and then arms `arm()`, the timer for the next local
midnight, which clears `sent` and chases again.

The block at the end wraps `feature_Loop.send` once the loop object exists:
every event gets a top-level `marks`, unless it already carries one. It then
chases at boot and on `visibilitychange`.

`marks-with-the-tap.rs` extends `update`: any event carrying top-level `marks`
writes them through `/since`'s own `since_marks_write` **before** the chain
beneath runs, so anything inner that reads them this turn already sees them,
and `render` — which follows the whole chain — draws with them. `e_marks` pulls
the field off the event. Unchanged marks are not rewritten.
