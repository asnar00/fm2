# tour
*a first run teaches the app: a card over the real screen points at a real control and waits for the tap*

> (transcripts/2026-09-02-self-check.md#p61a)
> I'd also like an onboarding workflow that a) prompts the user to fill in their profile, including a picture, immediately (and mandatorily) before they learn to use the app and b) teaches them how to use the app, using a "demo" workflow.

*(This node is (b). The profile gate, (a), is `/profile-first` under
`/me`; the tour runs only once that gate has lifted.)*

## user

The first time you use the app, a small card sits over the screen and
points at one button: *posts are what you make in the field. tap the
bubble.* You tap it — the real button, the real tool — and the card moves
on: back, then people, then their map, then projects. Three moves, a
line each, in your own app with your own things in it. From the second
card on there is a **skip**; the tour ends where you leave it and never
comes back — not on this phone, not on another.

## spec

Nobody teaches a new person the app; `/enrol` ends at the toolbar and
the rest is guessing. Ash asked for a demo (#p61a). Two shapes were
weighed. A ghost replay (`/replay`) seeds state from a keyframe and
dispatches a recorded session's events on a timer: somebody else's taps
on somebody else's world, playing over yours, with recording paused and
the login redirect suppressed — a reproduction, not a lesson, and it has
no way to wait for *your* tap. A scripted tour has: a card over the real
screen, pointing at the real control, advancing on the real state
change. This node is the tour. The demos-as-foundation pillar is served
the way it says — the demo runs on the shipped surface and its steps
are the shipped controls, so a control that moves or goes is a step
that moves or goes, never a stale recording.

**It is `/long-press`'s card, driven.** Hold a tool's button and a card
tells you what it does; this node is that card as a sequence — the same
plane, the same look, a pointer added — placed under `/long-press`
because that is what it extends. Each step is *a target, a line, a test*:
a selector for the real control, the words, and a predicate on the loop
state (or the DOM) that says the move has been made. Steps advance on
`feature_Loop.apply`, which is where a real tap's consequence lands, so
a tap that did not reach the loop advances nothing and a move made by
any road (‹ instead of the tool's own button) counts.

**The seven steps, three moves.** *That's your card. tap ‹ to go on* —
only when the person arrives from the gate with their card open; skipped
otherwise. *Posts are what you make in the field. tap the bubble* → the
posts tool opens. *+ makes a post from where you stand. tap the bubble
again to come back* → home. *People: everyone whose card you hold. tap
the person* → 👤 opens. *The map puts them where they are. tap the map*
→ `/map`'s pill lights. *Tap the person again to come back* → home.
*Projects: a campaign, and who is in it. tap the flag* → projects opens.
*New makes one. tap the flag again, and it's yours* → home, and the tour
is over. A step whose control is not on the screen when its turn comes
(a tool unticked; ‹ when nothing is open) is passed over, so the tour is
never stuck pointing at nothing. The list is the seam a longer tour
extends.

**Offered once per user, and it travels.** `tour_seen` is a user-scoped
var (`/policy`'s idiom: `(user, last-write, own)`, bridged to the page as
`tour_seen`). It is written `true` when the tour ends — finished or
skipped — by a `TourSeen` event this node's Rust half turns into the op,
so it reaches the person's other devices through the world like every
other var. A page mirrors it in `localStorage` too (`/policy`'s idiom
again), so a relaunch before the op has shipped does not offer the tour
twice. A second page for the same person, joining after the end, reads
the var at join and shows nothing.

**After the gate, never before.** The tour starts only when the page's
world has joined (`_joined`), the var says unseen, and `/profile-first`
is not standing (its marker on the page; with that node unticked there
is no gate to wait for). While the tour runs the card sits above
everything but the sheets, points at its target with a quiet triangle,
and rings the target with a thin grey line — no colour (colour is a
word, and none of the existing ones means "look here"), no motion but
the card's 0.18 s fade.

**Skip.** From the second step on the card carries a dim *skip*; it ends
the tour exactly as finishing does. The first card has none — one line
is the least a person can be asked to read — which is the brief's
"skippable after step one".

## hostile cases

- A step reads the screen, never the state mirror: after a tool's own back the mirror can carry one stale frame (`open_tool` `''` then `account`), and the first cut believed it, marked ‹ done and skipped the three posts steps (the one-level review, 2026-09-02). Since `/one-level`, ‹ from the card reaches the people set, so a card says so and points at the person for the toolbar.

- **The tour and the gate**: a gated page never starts the tour; the
  first apply after the gate lifts does, pointing at ‹ over the card the
  person has just finished.
- **A relaunch mid-tour**: nothing is written until the end, so the tour
  is offered again from the first step whose control is present — a
  person who was interrupted gets it back; a person who wants out has
  skip from the second card.
- **A tool unticked** (no posts, say): its two steps are passed over at
  their turn; the tour continues with the next control that exists. With
  every control absent the tour ends at once and marks itself seen.
- **`/map` unticked**: the map step is passed over; 👤 opens and the
  next card says to come back.
- **The person makes a post during step three** (taps +): the card
  stays — the step's test is "home", and the way home is the same. A
  card page over the tour's target: the target is the lit tool button
  in the row, which stays put.
- **`/profile-first` unticked**: no marker to read; the tour starts at
  the first joined apply, as a first-run tour should.
- **Two pages at once, both unseen**: both offer; the first to end
  writes the var; the other is mid-tour and finishes on its own — one
  extra tour, never a missing one.
- **`_joined` never comes** (offline): no tour this launch; nothing is
  written.
- **The engineer section, the lozenge, the sheet**: untouched — the
  tour draws only its own card and one class on its target.

## parked, named

- A "show me again" entry on the nøøb sheet: `feature_Tour.start()` is
  the door; the entry is a one-line node under `/panel`.
- Per-authority tours (support learns inviting): a second step list
  chosen by `auth/whoami`'s authority; the list is already the seam.
- A tour that teaches *inside* a tool (the picture's crop, a project's
  add): steps whose targets are a tool's own controls — the same shape.

## glossary

- **tour card**: `/long-press`'s tool card, driven: one target, one
  line, one test, advancing on the real tap.

## code description

`tour.vars` — `tour_seen: bool`, user-scoped, last-write, own, bridged as
`tour_seen`.

`tour.rs` — `update` turns a `TourSeen` event into `tour_seen_write(true)`,
which is the `edit_op` at this node's path; nothing else on the Rust
side.

`tour.js` — `feature_Tour.steps` is the list: `{at, say, done, skipIf}`.
`check()` runs on every `feature_Loop.apply` and on a mutation of `#app`:
while not started it asks `may()` (joined, unseen, not gated) and starts;
while running it passes over steps whose control is absent or whose
`skipIf` says so, ends when the list runs out, advances when the current
step's `done(s)` is true, and places the card. `place()` puts `#tourCard`
above the target (below it when there is no room), centres the pointer
on the target with `--tour-px`, and moves the `tour-here` ring. `end()`
sends `TourSeen`, mirrors `localStorage.misoTourSeen`, and hides the
card; `start()` is the public door for a later "show me again".

`tour.css` — the card (the `#toolCard` family: dark, 1px border, 10px
radius, 13px), its pointer, the skip, and the ring.
