# profile-first
*a new person's first screen is their own card, asking for a picture and a line — nothing else until both are in*

> (transcripts/2026-09-02-self-check.md#p61a)
> I'd also like an onboarding workflow that a) prompts the user to fill in their profile, including a picture, immediately (and mandatorily) before they learn to use the app and b) teaches them how to use the app, using a "demo" workflow.

*(This node is (a). The demo, (b), is `/tour` under `/long-press`, which
runs only once this gate has lifted.)*

## user

Log in for the first time and you land on your own card, with your name
already on it and one sentence at the top: **add a picture and a line
about you to start.** Tap the picture, choose one; write a line; tap the
tick. The toolbar's buttons appear and the app is yours. Nothing else is
reachable before that, and nothing asks again once it is done — on this
device or any other.

## spec

Today a first login drops a new person straight onto the toolbar
(`/enrol`: install, one code, two permission sheets, done). Their card
stays empty until they find 👤; a faceless card is a faceless pin on the
map and a nameless copy in everyone else's cards (`/exchange`). Ash's
ruling (#p61a): the profile comes first, immediately and mandatorily.

**The gate is a question about the joined world.** On every event, once
the page's world has really arrived (`/veil`'s `_joined` mark in the loop
state — set on the join, never on its timeout), this node asks whether
the person's own profile card is *enough to start with*: a picture with
data in it and a text block with words in it. A world with no profile
card yet is not enough either — that is a new person whose card `/me` is
about to make. Before the join nothing is asked: a page that could not
reach the server shows the app as it is, because a gate that fired on an
empty world would lock an existing person out of their own app while
offline. The predicate is a seam (`profile_first_missing(card)`) so a
later ask — "require a number too" — extends the question without
touching the gate.

**Getting to the card is two real taps, sent by the page.** A fresh
person's page boots to the launcher like anyone's. Once the gate stands,
the page half sends the same events a finger would — `tool_account`, and
then, when the card exists and its page is not yet showing, the own
tile's `browse_open:<id>` — so `/tools`, `/me`, `/people` and `/browse`
all do exactly what they do for a tap, and every frame is painted from a
real turn. This is `/restore`'s idiom, and it was chosen over writing the
navigation vars from the Rust side for a reason found on the rig:
`open_tool` is a *bridged* var, `/payload` republishes it into the state
at its own, older, link, and a write from a newer link paints one stale
frame (`/turn-end` names this) — while republishing the key from here
reads to `/one-way` as a page write and earns a false warning. The two
taps have neither problem. `/me`'s own watch sees the first and sends
the `CardEnsure` that makes the card; `/patient` still holds it until the
real join.

**Mandatory means there is no way past, not a nag.** While gated, this
node's `update` link — the outermost, by provenance — drops every
navigation tap *before* the chain sees it: `tools_home`, any `tool_`
button, the view picker's `browse_` events, and any `browse_open` that is
not the own card. Nothing repaints, nothing is written, the screen simply
stays. The two taps that lead *to* the card (👤 while it is not open, the
own tile) still pass, because they are the ones the page half sends. A
tap on the ground (`/backdrop`) sends `tool_account` while it is open,
and that is dropped like the rest. No skip, no later: the app has one
screen until the card is enough.

**The card is open, and stays open.** The page half reads the marker the
render link emitted (the sentence carries `id="profileFirst"`) and, on
each apply, marks the card open in `/editing` (`open[id]`, then its
`apply()`), which restores `contenteditable` on the blocks and lets the
picture's tap through. `/editing/toolbar`'s tick in the row is the save:
a tap blurs the block (the blur is the save, `/cards`' rule) and then
locks the card — and while the gate stands this node undoes that lock at
once, by wrapping `feature_Editing.lock`, so the tick keeps saying save
and a person who tapped it with nothing written is not left looking for
a pencil. The picture goes through `/cards`' own road — the tap on
`.card-pic` opens the file chooser, `/frame` offers the crop, `/cards`
shrinks and sends `CardPic`. Nothing here saves or stores anything of
its own.

**Every way off the card is withheld, not hidden behind a shade.** While
gated the page half puts `fm-profile-first` on the body; under it the
stylesheet keeps the tool buttons, ‹ and the view picker off the screen.
The row itself stays, because the save tick lives in it (ash's ruling
that nothing floats over a card, `/editing/toolbar`) — so the row holds
the tick and undo, and nothing that leads anywhere. The lozenge stays
too: updates and logout are the system's, not the app's. When the gate
lifts (the render no longer emits the sentence), the class comes off,
the open mark is dropped so the card reads as a page, and the buttons
slide in as they do on any mode change. The person is a user.

**The sentence is the app's word, once.** "add a picture and a line
about you to start" — inside the card page, above the name, in the prose
colour, lowercase like the card's own placeholders. It is the only copy
this node adds; the blocks' placeholders ("say what you are here to do")
do the rest.

## hostile cases

- **The join never comes** (offline, server restarting): `_joined` stays
  unset, no gate; `/patient` makes no card either; the next launch that
  joins asks. The failure direction is "no gate", never "locked out".
- **A picture too big, or not a picture**: `/cards` says so in its toast
  and stores nothing; the gate stays, as it should.
- **A tap on the ground, ‹, a tool, or the picker while gated**: dropped
  before the chain; no repaint, no write. Proven on the rig with a real
  click where the buttons were, a real click on the ground, and a
  `tools_home` sent by hand.
- **The tick with nothing written**: the (empty) save happens, the lock
  is undone, the card is still open and the tick still says save.
- **A relaunch mid-gate**: `/restore` reopens 👤 anyway; the join lands;
  the gate is asked again against the real world and stands until the
  card is enough.
- **Two devices**: the card completed on one arrives on the other as a
  world change; that apply's render emits no sentence and the gate lifts
  there too — nothing device-local decides it.
- **An existing person whose card already has both**: the predicate is
  false from the first joined event; nothing of this node reaches the
  screen. `_bob` in the rig — including the relaunch that `/restore`
  brings back to 👤.
- **`/editing` unticked**: the page half finds no `feature_Editing` and
  does nothing to the card, which is then editable on touch as it was
  before `/editing`; the gate, the sentence and the withholding hold.
- **`/people` unticked**: `browse_open:<id>` is a click nobody handles;
  `/me` draws the card page on `open_tool == account` alone; the gate
  holds.

## parked, named

- "Require a number too" — one more clause in `profile_first_missing`.
- A card that is enough but *thin* (a one-word line): not this node's
  question; ash may set a floor later.
- The lozenge while gated: reachable by design today; hiding it is a
  ruling for ash.
- Undo in the row while gated: it can take the picture back, which only
  keeps the gate standing; harmless, and left as the row's standing
  member.

## glossary

- **enough to start with**: a profile card with a picture that has data
  and a text block that has words — the gate's predicate.

## code description

`profile-first.rs` — `profile_first_missing(card)` is the predicate seam:
true unless the card holds a picture with data and a text block with
words. `profile_first_gated(state)` asks it of the own profile card
(`card_of_type` on `cards_read`, the first profile card, `/people`'s own
idiom), after checking the state's `_joined`; no card counts as missing.
`profile_first_own_id()` is that card's id. `update` asks the gate and
`profile_first_steps_off(event)` before the chain, and returns the state
untouched — no chain — for a navigation click while gated; every other
event goes down the chain as before. `render` calls the chain and, while
gated, puts the sentence just inside the card page's opening tag (after
the base, if the page is not there yet — `/me`'s "making your card…"
moment).

`profile-first.js` — `feature_ProfileFirst.gated()` reads the marker;
`ownId()` the first profile card's id from the bridged `cards`; `send()`
one navigation click per state (`sent` stops a slow turn being asked
twice); `apply()` toggles the body class, sends the two taps that reach
the card, marks the current own card open in `/editing` once per card
id, and on the gate lifting drops that mark again. Installed as a wrap of
`feature_Loop.apply`, `/me`'s idiom. At load it also wraps
`feature_Editing.lock` so a save while gated reopens the card.

`profile-first.css` — the sentence, and the tool buttons, ‹ and the view
picker withheld under `body.fm-profile-first`.
