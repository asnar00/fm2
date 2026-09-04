# unbroken
*the next post comes up as this one leaves, so the sweep is one strip*

> (transcripts/2026-09-04-field-walk.md#p96)
> when scrolling between posts, the old post scrolls completely offscreen before the new one scrolls in, leaving the screen empty for a short time. I'd like the new one to start scrolling in immeidately as we scroll the old one out, so it feels like an unbroken series.

## user

Sweep from one post to the next and the two move together: the top of the new one comes up as the bottom of the old one goes, with no empty screen in between. Let go and the strip settles onto the new post. Let go before you have gone far enough and both come back.

## spec

`/carries-the-card` sent the switch on the release, so the old card left, the screen was empty for the length of a turn, and the new card came in after it. Ash asked for one unbroken series (#p96).

**Two cards on the screen means switching early.** The turn is sent the moment the sweep passes `/flick`'s own threshold *with the finger still down* — far enough, not sideways, quick enough, at the end of the card the sweep needs, and only when there is a post that way. Before the event goes, the card being left is moved out of `#app`: `.card-page` is fixed to the viewport by its own rule, so being reparented moves it not one pixel, and it keeps the transform the finger gave it. The paint then replaces `#app`'s contents with the new card, and the card that was there is still on the screen because it is no longer in `#app` to be replaced.

**And then one number moves both.** The card being left sits at the finger's travel; the card arriving sits a card's height and a gap behind it. On release past the threshold the strip completes — the arriving card to rest, the other away — and the ghost is dropped. Short of the threshold the switch is undone by a real turn back through the same event, because the card that came in arrived by a real turn too; the restored card then springs back from where the finger left it.

**Why this is a child and not a rewrite.** Everything the parent does still does it: `arm`, `move`, `end` and the spring-back are its own, `/flick`'s rule is still the only rule about what a sweep means, and `/rubber-band` still owns the ends — this node never starts a strip where there is no neighbour, so the pull is untouched. What the child replaces is one decision: *when* the switch is sent. It takes that by watching the finger through the parent's own `move` and by standing the parent's release down while a strip is live, so unticking it returns the sweep to sending on release with nothing else moved.

**One switch per crossing.** `/flick`'s release still fires on the way up and would send a second; it is dropped while a strip is live, and the parent's own deferred road never starts.

Untick and the old card leaves, the screen is briefly empty, and the new card follows.

**What was measured**, on the rig throttled to Slow 3G in a browser that held
none of these posts' pictures — the phone's case, never the rig's default —
four sweeps in a row: **two cards on the screen at once in every one of them**
(the card being left at −42…682 while the card arriving stood at 696…1420 and
came up behind it), the strip settling on the new post each time with **no
ghost left behind**, and across all thirteen paints **no video element made, no
media load and no `play()`** before any tap.

## hostile cases

- **A sweep at either end of the list.** No post that way; no strip, and `/rubber-band`'s pull answers as it does now.
- **A release short of the threshold after crossing it.** The switch is undone by the opposite event and both cards come back. The restored card is rebuilt, so its picture is fetched again — `/prewarmed` has it in the cache by then.
- **A sweep that turns sideways after crossing.** The strip is already live and completes or springs back on the release; `/flick`'s sideways limit is only asked at the crossing.
- **A second finger, or a new gesture, while the strip is live.** The parent's own `busy` is set, so a new sweep is refused until the strip has settled.
- **`prefers-reduced-motion`.** The parent never arms a carry, so `move` never engages and no strip is ever started.
- **The card page gone under the strip** (the tool left, the post closed). The ghost is dropped and the gesture released; nothing is held.
- **A card taller than the screen.** The step is that card's own height plus the gap, so the strip is as long as the card is.

## glossary

- **the strip**: the two cards moving as one under the finger — the post being left and the post arriving.

## code description

`unbroken.js` — `feature_Unbroken`.

`remember()` and `has(dir)` are the list and the question of whether there is a
post that way, from `#mapData`'s own ids.

`crossing(a, x, y)` is `/flick`'s rule asked mid-drag rather than at the
release. `start(C, dir, dy)` moves the card being left out of `#app`, sends the
switch and places both. `place(dy)` is the whole of the motion: one number, the
card being left at it and the card arriving a step behind.

`release(C, dy)` completes the strip or undoes the switch; `glide` is the
settling animation, `drop` takes the ghost away and `done` gives the gesture
back to the parent.

The wrappers are the parent's `move`, `end` and `begin`, and `/flick`'s `go` —
the last so the release's own send is dropped while a strip is live.

`unbroken.css` — the card being left keeps every `.card-page` rule and only
needs the open card's own depth over the map.
