# fan-out
*pins that share a point fan out around it, each stem still pointing at the place*

> (transcripts/2026-09-03-housekeeping.md#p5b)
> When there's more than one post / user at the same location, the markers overlap so you can't distinguish them. What we should do in this case is rotate the "arrow" part of the marker, so the posts/users "fan out" in a circle around the map point.

## user

Two posts from the same doorstep, or a person and their post, no longer sit on top of each other on the map. The pins turn about the place they mark and spread around it in a circle — two face each other across the point, three make a Y, more make a ring — every stem still pointing at the spot, every face upright, every one tappable. Zoom in and they come apart on their own; zoom out and they gather into the fan again.

## spec

`/map` draws a pin as a face above a stem, anchored at the stem's tip, and two cards at one place draw two pins at one pixel. Ash's ruling (#p5b): rotate the arrow part so the pins fan out in a circle around the point. Same place is a matter of the screen, not the coordinates — two posts made standing still are a metre apart, which is a pixel at street zoom and a screen at the kerb — so the fan is laid out from projected positions and laid out again on every zoom. After each draw, and after each `zoomend`, the pins are grouped greedily by screen distance (a pin within 30 px of a group's first pin joins it — the face is 34 px wide); a group of n pins gives pin k the angle 360·k/n, starting straight up, and the whole pin turns by that angle about the stem's tip, which is the map point, so the tip never leaves the place. Past six pins the stem grows so the ring has room for every face (34 px each, side by side on the circle), the pin lifted by the same amount so the tip stays put. The face is turned back by the same angle so it stays upright. A group of one gets no angle at all, so a lone pin is exactly `/map`'s. Live pins are `/live`'s own markers, drawn by another hand, and are not in the fan. Untick and pins at one place lie on top of each other again.

## hostile cases

- Two pins at one place: up and down, faces upright, both tappable.
- A pin whose place is a metre from another at street zoom: fanned; zoomed to the kerb: apart, and each stands straight.
- The sig short-circuit in `/map`'s `draw` (nothing changed): layout still runs — it is cheap and the zoom may have changed.
- A map with no pins, or one: nothing turns.

## glossary

- **fan**: the ring of pins about one place, each turned about the stem's tip.

## code description

`fan-out.js` — `feature_FanOut.layout()` (group by screen distance, set each pin's angle); wraps `feature_Map.draw` to lay out after it and hooks the map's `zoomend` once.

`fan-out.css` — the pin turns about its tip (`transform-origin` at the stem's point), the face turns back, both eased.
