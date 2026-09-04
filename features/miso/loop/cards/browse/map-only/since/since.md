# since
*four pills where the view picker was: today, week, month, all — and the set is only what falls inside*

> (transcripts/2026-09-04-field-walk.md#p13)
> ok, superb. next batch of work: I want to lose the grid/list views and standardise on map view for everything - the "reel" feature, coupled with smooth open/close/scroll, does everything we need. So let's remove the grid/list/map switch, and replace it instead with a time-domain filter: options are "today", "week", "month", "all". Today just shows posts made today, week shows this week (from monday as day 1 of the week), month shows all this month's, all shows all. This applies to all the other views as well (users, projects).

## user

Top left, where the grid/list/map pill used to sit, four words: **today**,
**week**, **month**, **all**. The one you are in is lit. Tap **today** and the
map holds only what happened since midnight, and the band under it holds the
same; **week** goes back to Monday morning, **month** to the first of the
month, **all** brings everything back. It works the same on posts, on 👤 and
on projects — a post counts by when it was taken, a person and a project by
when they were made. Hold a word and the card says what it means.

The choice is yours, not this phone's: it follows you to your other devices.
It starts at **all**.

## spec

`/map-only` emptied the picker's slot and left `browse_slot_html()` as the
seam for whatever takes it. This is what takes it (#p13).

**Four pills, not a `<select>`.** `/audience`'s grade options are the house
pattern for a small closed set — round, dim, one lit in `#9db7d8`, no label
and nothing to submit (`/taste` 3, 4) — and these are those, in the picker's
own fixed place at the top left, at the picker's own height so the top strip
does not change shape. The words are ash's own (`/taste` 7): today, week,
month, all.

**The choice is a user var, and its default is `all`.** `period` is
`(user, last-write, own)` rather than device-scoped: which slice of time you
are looking at is a preference, not a position — it is worth having on your
other phone, unlike `view` and `open`, which are where-you-are. The default is
**`all`**, so a newcomer on a quiet day does not open the app to an empty map;
this is a call, not a reading of the ask, and it is one line to move.

**The filter is applied upstream of everything that draws.** Two chains carry
every browsed set in this tree: `browse_cards(state)` (`/browse`'s seam, which
`/people` and `/projects` take) and `posts_set()` (`/posts`' own, which
`/post-time`, `/delete`, `/current-project` and `/audience` already narrow and
reorder). This node redefines both and drops what falls outside the period.
Everything downstream then agrees without being told: the map's pins, the
band's `data-ids`, `/on-people-map`'s `data-post-ids`, `/flick`'s walk. In
particular **the reel's own contract is untouched** — the band still lists
exactly the set the map was handed, which is `/reel`'s promise (#p22) and
`/learned` 10.

**Which of a card's times.** A post's own moment if it has one, and otherwise
the moment the card was made: `when` if it is set, else `created`. That is
`/post-time`'s rule read off the card rather than through its function, which
is deliberate — the field is data, the function is a node, and reading the
field means this node keeps working with `/post-time` unticked (no card has a
`when` it wrote, so every card falls back to `created`, which is what the ask
says a person and a project count by anyway).

**The day starts come from the page, because the wasm has no clock.** A wasm
build has neither `SystemTime` nor a local time zone — `/browse`'s own date
arithmetic says so in as many words — so it cannot know when local midnight
was, and `render` carries no time. The page half does know both. `since.js`
computes the three marks with `Date` (midnight today; midnight of the most
recent Monday, Monday being day 1; midnight of the 1st) and sends them as one
`SinceMarks` event; `update` writes them into the `day_starts` device var as
`"<today>,<week>,<month>"` and the filter compares against one of the three.
Not a bridged var written from the page — a node newer than `/payload` moves
state with the events a finger would send, never by writing a bridged key
(misses.md, *navigation from the wrong side*).

The marks are re-sent at boot, whenever the page becomes visible again, and on
a timer armed for the next local midnight — the three cases in which they go
stale (a phone that slept through midnight, a phone carried into another zone,
a phone left open overnight). The page remembers what it last sent and stays
quiet when nothing moved, so a visibility flip costs nothing.

**Fail open, never closed.** Before the first `SinceMarks` lands — one frame at
boot — `day_starts` is empty and `since_cut()` answers 0, which keeps
everything. A filter that has not yet been told the time shows you your world,
not an empty map; the alternative is a blank first frame, which reads as a bug.

**Two cards are never filtered out.** Your own profile card, because a list of
people that does not contain you is somebody else's list (`/current-project`
made the same ruling for the project filter); and the card that is open,
because tapping a pill while reading something should narrow the band behind
it and not close what you are reading. Both are one condition each, named here
so they can be removed by name.

## hostile cases

- **A card with no `when` and no `created`.** Its time is 0. Under **all** it
  is in the set (nothing is filtered); under the other three it is out. Said
  here rather than special-cased: a card that cannot say when it happened
  cannot answer "was it today".
- **The marks have not arrived** (the first frame, or `since.js` never ran
  because the page half is missing): the cut is 0 and every period behaves as
  **all**. The pills still light; they simply do not bite yet.
- **A clock set back.** The marks are recomputed from the phone's own clock at
  the next boot, visibility change or midnight, so the filter follows the clock
  wherever it goes. Cards stamped in the future by a wrong clock are inside
  every period, which is the same answer `/post-time` gives for a camera whose
  clock is wrong: believe what you were told.
- **A period whose start is after every card.** An empty map with no band, and
  no message — `/map`'s ruling is that an empty map is still a map, and the
  pills are on screen saying why it is empty and how to leave.
- **The filter with a card open.** The open card stays open; the band and the
  pins behind it narrow. Going back leaves it out of the set, as the period
  says.
- **`/post-time` unticked.** No card carries a `when` it wrote, so every card
  counts by `created` — the ask's own rule for people and projects, applied to
  posts too.
- **`/map-only` unticked.** This node is its child and goes with it: the picker
  is back and the slot it filled does not exist.
- **A DST boundary inside the month.** The marks are wall-clock midnights taken
  today, so a card stamped in the hour either side of the change can fall the
  wrong side of the month's start by an hour. Named, not fixed: the fix is a
  per-card offset the tree does not carry.

## parked

- A period the map's pan chooses for you (look at last week, get last week).
- Saying how many cards the other periods hold, so an empty **today** shows
  where the rest went.
- A fifth pill, or a custom range: `since_periods()` is not a chain yet
  because four is the ask; the pills' renderer is one function to extend.

## glossary

- **period**: which slice of time the browsed set is cut to — today, week,
  month or all.
- **the marks**: the three epoch milliseconds the page computes for local
  midnight today, Monday's midnight and the 1st's midnight.

## code description

`since.vars` declares `period` (`"all"`, user-scoped, so the choice follows the
person) and `day_starts` (empty, device-scoped, because a day start is a fact
about this phone's clock and zone and belongs to nobody else). Neither is
bridged: nothing on the page half reads them.

`since.rs` redefines `browse_slot_html()` — `/map-only`'s seam for the picker's
place — with the four pills. `since_pill(which, on)` draws one, lit or not,
each with its `data-ev` written out as a literal so `/sub-tool-cards`' and
`/tool-words`' long press can read it out of this source.

`since.rs` extends `update` with the four pill clicks, which write `period` and
leave `open` alone (the open card stays open), and with the `SinceMarks` event,
which writes `day_starts` when the page's answer has changed.

`since_cut()` turns the period and the marks into one epoch millisecond floor,
0 meaning "keep everything" — for **all**, for a period whose mark is missing,
and for marks that have not arrived. `since_time_of(card)` is the card's own
moment: `when` if set, else `created`. `since_keep(card)` is the test, with the
two exemptions — your own profile card, and the card that is open.

`since.rs` redefines `browse_cards(state)` and `posts_set()`, the two chains
that carry a browsed set, and drops what `since_keep` refuses. Nothing further
down needs to know: the map's pins, the band's ids and `/flick`'s walk all read
what these two return.

`since.js` computes the three marks from `Date` and sends `SinceMarks` at boot,
on `visibilitychange` and on a timer armed for the next local midnight,
remembering what it last sent. It also arms the pills for `/long-press` the way
`/tool-words` arms the picker's buttons, and answers with this node's own words
for the four events before passing anything else down the chain.

`since.css` puts the pills in the picker's fixed place at the top left, at the
picker's own height, in `/audience`'s grade-pill grammar; and narrows
`/title`'s project name so the two never meet on a phone.
