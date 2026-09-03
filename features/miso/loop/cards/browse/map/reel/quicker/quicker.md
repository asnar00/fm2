# quicker
*the reel answers a flick at once: the mark moves as you scroll, the map follows sooner*

> (asks#1788448728397)
> in reel view, response time
> *(filed from the field on 2026-09-03 by ash)*

## user

Flick the band and the outline jumps to the new post as it settles, and the map is already moving — no pause first.

## spec

`/reel` waits 140 ms after the last scroll event before it reads the current post and pans, and the pan itself takes 0.45 s; `/current` marks the lozenge only then. Ash's ask names the response time (asks#1788448728397) — read as "quicker", the likelier meaning, and the stamp says so. One reading built: the current lozenge is marked on every scroll event, not after the settle; the settle is 60 ms; the pan 0.3 s. Untick and the band answers as before.

## glossary

(no new terms)

## code description

`quicker.js` — the shorter settle and pan, and a scroll listener that marks as it goes.
