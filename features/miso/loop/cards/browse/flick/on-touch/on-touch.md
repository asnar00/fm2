# on-touch
*the flick is read from the touch, which a phone never cancels*

> (asks#1788472415654)
> when viewing cards, swipe up and down should actually scroll the cards to
> prev and nest

## user

On the phone, a flick up at the bottom of a card goes to the next card and
a flick down at the top to the previous one — it now works where it is
used, not only on a desktop.

## spec

`/flick` read the gesture from pointer events. iOS hands a touch it has
taken for a scroll to its own recognizer and fires `pointercancel` — the
`pointerup` the node waited for never comes, so a flick on the phone did
nothing (the ask). The same lesson `/on-release` learned for taps.

**Read the touch.** This node listens to `touchstart` and `touchend`, which
a phone fires whatever it did with the gesture in between, and hands them to
`/flick`'s seams: `arm` at the start, `release` with the last touch's place
at the end. The rule — at the end you are already at, sixty pixels, quick,
not sideways — is `/flick`'s and unchanged. `go()` sends once per 400 ms, so
on a device that fires both roads for one gesture the second is ignored.
`/flick` was refactored to open those seams, behaviour unchanged (the
toggle proof is in the commit).

## hostile cases

- **A scroll that ends mid-card.** `arm` saw neither end; `release` sends
  nothing.
- **A two-finger gesture.** Only the first touch is read; a pinch on the
  frame sheet is excluded by `arm`.
- **Desktop.** No touch events; the pointer road as before.
- **This node unticked.** Pointer events only; the phone flick is back to
  not working.

## code description

`on-touch.js` — `touchstart`/`touchend` on the document, capturing, calling
`feature_Flick.arm` and `release`.
