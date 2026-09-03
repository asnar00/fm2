# further
*a tool's card stands a little further above its button*

> (asks#1788448511509)
> move tooltips a bit further from the tool button
> *(filed from the field on 2026-09-03 by ash)*

## user

Hold a button and its card sits a little higher, clear of your thumb.

## spec

`/long-press` puts the card 10 px above the button. Ash asked for a bit more (asks#1788448511509). One reading, so it builds: after the card is placed, it is lifted a further 12 px — 22 px clear of the button — and never above the top of the screen. Untick and the card sits at 10 px again.

## glossary

(no new terms)

## code description

`further.js` — wraps `feature_LongPress.show` to lift the card.
