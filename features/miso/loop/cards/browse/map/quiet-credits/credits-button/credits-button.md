# credits-button
*the credit lines fold away behind one quiet **credits** button*

> (asks#1788370723169)
> put the credits behind a "credits" button
> *(filed from the field on 2026-09-02 by ash, birthplace "-")*

## user

Open the nøøb sheet and where the credit lines used to sit there is one
quiet button: **credits**. Tap it and the lines unfold — who drew the
ground, who drew the boundaries. Tap it again and they fold away. Close the
sheet and open it again and they are folded, every time. The map itself
stays clean, as `/quiet-credits` left it.

## spec

`/quiet-credits` took the credit off the map and put it at the foot of the
nøøb sheet, in the dimmest text the app has. It is still always in view
there, and the sheet is a short thing: three or four lines of small grey
text under the last row is the last thing on it, and it is the thing nobody
reads. This node folds them behind a button and leaves everything else where
it was.

**The extension point is `feature_QuietCredits.show()`** — the one function
that fills `#credits`, and the only place the section's markup is decided.
This node captures it at load and replaces the property (never a timer:
notes.md, "the apply-wrapper race"), awaits the captured one, then
restructures what it drew: the `credit-head` — the word *credits*, already
written there — becomes a button in a row of its own, and everything the
original appended after it moves into a `#creditLines` box that starts
hidden. Nothing in `quiet-credits.index.js` changes; a source added to its
`gather()` chain later arrives inside the fold with no work here.

**The fold resets because the section is redrawn.** `show()` assigns
`innerHTML` on every open of the sheet, so each open destroys the button and
the box and this node builds them again, folded. The state (`shown`) is set
to `false` in the wrapper before the redraw, not after, so a `show()` that
throws between the two leaves the flag folded rather than half-open. As a
second guard — the idiom `/engineer` uses — `feature_Panel.open` is wrapped
too and folds the section after the open resolves, which covers the one case
the redraw does not: a `show()` that rejects before it writes anything, so
the previous open's unfolded markup is still standing.

**When there is nothing to credit there is no button.** `show()` writes an
empty `#credits` and hides it whenever `gather()` answers no lines — the
tile route gone with `/tiles`, a fetch that failed, an answer that did not
look like a credit. The restructure keys off the `credit-head` the original
writes only when it has lines: no head, no button, and the section stays
hidden and empty as before. A `show()` that rejects — `gather()` throwing,
say — is not swallowed here: `/quiet-credits` starts it un-awaited behind its
own `.catch`, and hiding the failure from a later caller would buy nothing.
The fold runs in a `finally` instead, so a section left standing from the
previous open is folded even on the failing path, and the fold itself is
caught so it can never be the thing that breaks the open.

**Re-entrancy.** Two opens in quick succession run two `show()` calls that
share one memoised `gather()` promise and both assign `innerHTML`; the
restructure is idempotent — it returns early if a `#creditsBtn` is already
standing in the box — so the interleaving that matters (redraw, restructure,
redraw, restructure) ends folded and with exactly one button either way.

**The licence obligation is kept.** OpenStreetMap's attribution guideline
allows an application on a small screen to put the credit one interaction
away rather than always on screen, and Stadia's terms ask that the credit be
present and reachable, not permanently displayed; the ONS/OS boundary line
is OGL v3 and asks to be stated somewhere reasonable. From the map, the
sheet is one tap and the button is one more: the credit is two taps from the
map and one tap from the sheet that the map's own toolbar opens, which is
what "one interaction away" is for. This node must never be the reason a
credit becomes unreachable — if the lines are there, the button is there.

**The look.** A row button like the sheet's others and the quietest of them:
`/build-row`'s geometry to the pixel — the same size as `features`, sized to
its word rather than stretched — and `/engineer`'s gear colour behaviour,
`#77777e` folded and `#c9c9d2` open, so the button says whether it is open
without a caret or a chevron. It is the same size as the sheet's other small
buttons on purpose: `/taste` 2 makes a thing matter less by dimming it, never
by shrinking it, and a shrunken control is also a smaller thumb target.
Nothing new appears on the map.

**Placement.** A child of `/quiet-credits`: it is a refinement of exactly
that node's arrangement, it extends that node's own function, and unticking
`/quiet-credits` should take the button with the lines it folds. `/map` is
not the parent — with `/quiet-credits` off there is no section to fold, and
the credit is back on the map where Leaflet drew it.

## parked

- Per-source credit links (a tap on the OpenStreetMap line opening
  openstreetmap.org/copyright). The lines are plain text today.
- A licence page behind the same button: it would join `#creditLines`, which
  is why the fold holds a box rather than the lines themselves.

## code

`credits-button.index.js` defines `feature_CreditsButton`, whose `shown`
flag is the whole of its state.

`fold(box)` is the restructure. It finds `.credit-head` in the section —
present exactly when the original drew lines — and returns without touching
anything if there is none or if a `#creditsBtn` is already there. Otherwise
it moves every sibling after the head into a new `#creditLines` div, builds
a `<button id="creditsBtn">` carrying the head's own word, puts it in a
`#creditsRow`, replaces the head with the row, appends the box, and renders.

`render()` shows or hides `#creditLines` from `shown` and marks the button
`.on` and `aria-expanded` to match. `toggle()` flips `shown` and renders.

The load block wraps two functions by property replacement. `show()`: set
`shown` false, await the captured original, then `fold` the section from a
`finally`, so the fold happens whether the redraw succeeded or threw.
`feature_Panel.open()`: await the captured original, then
fold the section and render it closed, which resets a stale unfolded section
if the redraw never happened.

`credits-button.index.css` styles the row and the button, and gives
`#creditLines` the small top margin the head used to give the first line.
