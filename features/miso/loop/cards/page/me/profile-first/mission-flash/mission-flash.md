# mission-flash
*tap the tick with no line written and the box flashes*

> (transcripts/2026-09-03-invite-test.md#p117)
> If you hit "done" before we've filled in the mission, the mission box
> should flash to indicate you need to fill it in.

## user

Tap the tick before you have written your line and the box for it flashes
twice and takes the cursor. Nothing is lost; write the line and tap again.

## spec

The gate holds the card open until a picture and a line are in, and a tick
tapped too early simply keeps the card open — which reads as a dead button
(#p117). This node says why: on the tick's own pointerdown (`/toolbar`'s
capture, the same instant it acts), while the gate stands, if the card's
text block is empty, the block gets `mission-flash` for the length of the
animation and focus. The save still runs — it saves nothing, as before —
and `/profile-first` reopens the card as it always did.

**The flash is the accent, twice, quick** (`/taste` 3, 5): the border goes
to `#9db7d8` and back, 0.35s each, no bounce.

## hostile cases

- **A line written, no picture.** The tick behaves as before — the
  sentence at the top already says a picture is needed; this ask is about
  the mission.
- **Gate down.** Nothing happens; the tick is a plain save.
- **`/toolbar` unticked.** No tick; nothing to hear.
- **This node unticked.** The tick keeps the card open with no sign.

## code description

`mission-flash.js` — a capturing pointerdown on `[data-ctl="card_edit"]`
while `feature_ProfileFirst.gated()`: an empty `.card-page .card-text` gets
the class and focus. `mission-flash.css` — the keyframes.
