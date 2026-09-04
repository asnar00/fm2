# rubber-band
*at the ends of the list the sweep pulls against a spring*

> (transcripts/2026-09-04-field-walk.md#p34)
> the first and last in the list allow you to scroll past the start/end of the list, but scroll in the same post, which is disorienting - they should rubber-band scroll

## user

On the newest post, drag down; on the oldest, drag up. The card comes with your finger but less and less, as far as a thumb's width and no further, and settles back when you let go. It never flies off the screen and returns, and no other post arrives.

## spec

`/carries-the-card` let every sweep run its course: at the end of the list the card slid off, `/flick`'s event went, the Rust found no neighbour, and the same card came back — 140 ms out and 190 ms back to say "there is nothing that way". Ash read that as the list scrolling past its end into the same post, and asked for a rubber band instead (#p34).

**Where the ends are.** `/reel` writes the surface's ids on `#mapData` — the very `cards` vector `/browse` hands the surface, which is the list `/flick` walks. Rust draws the card page instead of `#mapData` while a post is open, so the list is kept from the last paint that carried it; the world has not changed under the sweep. The open card's place in that list is the answer: first and sweeping down, last and sweeping up, and the sweep is asking for a post that is not there.

**The pull.** The parent is handed a *damped finger* rather than a damped answer: the travel is put through a curve asymptotic to 160 px and given to `/carries-the-card.move` as though the finger had moved that far. The parent then computes its own offset, its own transform and its own spring-back from the same number, and none of its rules need to know this node exists. The first pixels track the finger almost exactly, the last hardly move, and the card can never be pulled clear of the screen.

**And no switch.** `/flick`'s `release` still fires — its sixty-pixel threshold is the finger's own travel, not the card's — so `go` is dropped for that direction here, outside `/carries-the-card`'s own wrapper. Nothing is sent, so the Rust is never asked a question whose answer is no, and the card is never flown off and brought back. `/carries-the-card`'s release then finds no switch running and springs the card back from wherever the pull left it.

Untick and the ends fly the card off and bring it back.

## hostile cases

- **The middle of the list.** Not an end; the sweep is 1:1 and switches as before.
- **A card whose id is not in the kept list** (a project card opened from a person's page, a surface with no map). The ends are unknown, so the sweep behaves as it did before this node — the fly-off and back.
- **No list kept yet** (the map view never opened this visit). The same: as before.
- **One post in the list.** Both ends at once; every vertical sweep is a pull.
- **A sweep across the end and back within one gesture.** The damping is computed from the finger's travel each move, so it follows the finger back down to nothing.
- **`prefers-reduced-motion`.** `/carries-the-card` carries nothing, so there is nothing to damp; `go` is still dropped at the ends, which is the part that matters.
- **The list changing under the sweep** (a post arrives at the head). The kept list is the one the card was opened from; at worst one sweep is refused that would now have worked.

## glossary

(no new terms)

## code description

`rubber-band.js` — `feature_RubberBand`.

`remember()` keeps `#mapData`'s `data-ids` on every paint that has it, and the
wrapper on `feature_Loop.paint` is where it runs.

`end(dy)` answers whether the sweep is asking for a post beyond either end of
that list, and `endOf(dir)` asks the same question in `/flick`'s words.

`damp(dy)` is the curve: asymptotic to `PULL`, so the pull is resisted more the
further it goes and stops at 160 px.

`feature_CarriesTheCard.move` is redefined to hand the parent a damped finger
at the ends and the real one everywhere else, and `feature_Flick.go` is
redefined to drop the switch at the ends — outside `/carries-the-card`'s own
wrapper, so its slide-off never starts.
