# no-flash
*the pull at the ends settles without the media blinking*

> (transcripts/2026-09-04-field-walk.md#p117)
> swipe up/down now works nicely without a flash on post scrolling - except at the ends. When we "bounce back" to the same post, the post video flashes.

## user

Pull at the newest post or the oldest and let go: the card springs back and its picture sits still through it. Nothing blinks.

## spec

`/unbroken` ended the flash between posts, and ash's own record says why the ends are different: a sweep mid-list writes a media line at insertion with `src blob:`, `complete true` and `src changed false` a frame later, while **a bounce at an end writes no media line at all** — no card arrives, so nothing is built and nothing is fetched. Whatever blinks there is not content. It is the compositing.

**Measured in WebKit**, sampling every animation frame of a pull at the first post — the card's `will-change`, its transform, how many animations it holds, and its picture's state:

| frame | will-change | transform | animations | class | picture |
|---|---|---|---|---|---|
| 1 ms | `auto` | none | 0 | — | complete |
| 55 ms | `transform, opacity` | matrix | 0 | carried | complete |
| 289 ms | `transform, opacity` | matrix | 1 | carried | complete |
| **488 ms** | **`auto`** | **none** | **0** | **—** | complete |

The picture is complete throughout, so nothing reloads. The whole gesture has exactly one discontinuity: in a single frame the promotion, the animation and the inline transform all go at once. That is `/carries-the-card`'s own `clear` — it cancels the animation, takes its class off (and the class is where `will-change` lives) and clears the transform, in one turn — and dropping a compositing hint in the same frame as the transform it was hinting about is a re-raster of the layer the media is in.

**So the layer is held a little past the animation that needed it.** A class of this node's own goes on before the parent clears, and comes off 140 ms later on a quiet frame; the parent's own clearing is untouched. The hint outlives the hand-off, so there is no frame in which the element is both losing its promotion and changing its transform.

**And the release is written down**, in the shape `/arriving-picture` uses for an arrival: the media element's source, whether it was complete, the player's readyState and the card's `will-change` at that moment. The ends write no arrival line by their nature, so this is the line that will speak if a phone still blinks after this.

Untick and the promotion goes in the same frame as the transform, as it does today.

## hostile cases

- **A sweep that switches** (mid-list). The parent clears there too, so the hint is held there too — harmless, and one less re-raster on the road ash says is already clean.
- **Two pulls in quick succession.** The timer is replaced, not stacked; the hint is held from the last one.
- **The card gone before the hint is dropped** (the post closed, the tool left). Removing a class from a detached element is nothing; the element is collected with its card.
- **`prefers-reduced-motion`.** The parent never carries, so it never clears a carry, and nothing here runs.
- **A card with no media at all.** The hint is about the card's own layer; a card of words settles the same way and costs nothing.
- **A device that never had the layer promoted** (no compositing for this element). `will-change` is a hint; holding it longer asks for nothing that was not already asked for.

## glossary

(no new terms)

## code description

`no-flash.js` — `feature_NoFlash`. `hold(page)` puts the settling class on and
takes it off after `HOLD`, replacing any timer already running; `note(page,
when)` writes the release line into `/blackbox`'s ring.

The wrapper on `feature_CarriesTheCard.clear` holds the layer before the parent
lets go of it; the wrapper on `end` writes the release line while the finger's
own state is still there to read.

`no-flash.css` — the one rule the class carries.
