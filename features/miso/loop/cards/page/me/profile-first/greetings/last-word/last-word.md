# last-word
*the set-up page is only set-up, and a last page says that's it — no tour*

> (transcripts/2026-09-03-invite-test.md#p131)
> let's split the "notifications" and "hold button to see what it does" into
> two pages; the notifications page should just be about login +
> notification. Then the next page should say "that's it! hold any button to
> find out what it does" - we don't need any of the special demo stuff.

## user

After your card, a page about two things to switch on: Face ID to log in
and notifications so the team can reach you, with the two rows and **got
it**. Then one last page: **that's it!** — *hold any button to find out what
it does* — and **done**. The app is yours. The old guided tour does not run.

## spec

`/greetings` put the long-press line and `/set-up`'s rows on one page, and
let `/tour` run after. Ash (#p131): two pages, and no tour.

**Page two is set-up.** `greetings_sheet(2)` is redefined: the words become
*two things to switch on* / *Face ID to log in, and notifications so the
team can reach you.* — `/set-up`'s rows and gating are untouched, since
that node sits inside this one on the chain and this one only changes the
words around them.

**Page three is the last word.** `render` gains the third moment: gate
down, world joined, `greeted` at 2 — *that's it!* / *hold any button to
find out what it does.* / **done**. The click is the same `greet_next`;
this node's `update` steps `greeted` from 2 to 3 (the base stops at 2)
and marks the tour seen (`tour_seen_write(true)`, `/tour`'s own), so the
tour never starts: the last page said the one thing it was for.

## hostile cases

- **A person at greeted 2 from build 583** (they tapped got it before
  this build). They see page three once, and the tour is marked seen.
- **`/tour` unticked.** `tour_seen_write` is not composed and the linker
  refuses this node — the two travel together, as this node exists to
  silence it.
- **This node unticked.** Two pages and the tour, as `/greetings` drew them.

**The tour is told on the tap.** `tour_seen` written from this link reaches
the state one frame late (`/payload`'s older link republishes it — misses.md,
"navigation from the wrong side"), and `/tour` would take that frame. So the
page half also ends the tour on the tap of any welcome button: `at = -2` and
the local mirror `/tour` already keeps.

## code description

`last-word.rs` — `greetings_sheet(2)` rewords; `greetings_sheet(3)` is the
last page; `render` shows it at greeted 2; `update` steps 2→3 and marks
the tour seen. `last-word.js` — ends the tour on the tap of a welcome
button, before the frame.
