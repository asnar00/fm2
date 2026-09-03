# opens-over-map
*a post opened from the map view grows into place with the map still behind it*

> (transcripts/2026-09-03-housekeeping.md#p19)
> animate the post opening up to full view, and keep the map in the background rather than the dot-grid

## user

Tap a post on the map view and its page grows into view over the map. The map stays behind it, not the dotted ground.

## spec

When a post opens in the posts tool, Rust draws its page instead of `#mapData`, `/map` hides its host, and the dotted ground shows around the page. Ash asked for the map to stay and the page to animate in (#p19). One reading, so it builds: after `/map`'s own sync, if there is no `#mapData` but a card page is up and the picker shows the map view selected, the map host is shown again and the body is marked `fm-map-behind`; the marked body's ground is transparent so the map shows around the page, and the page arrives with a short grow from 96% and a fade (the toolbar's own 0.18 s ease-out). The map is the ground now and `/backdrop` leaves the map alone, so a plain tap on the map (Leaflet's click, never a drag) puts the page away the way `/backdrop` does, by the tool's own button. The reel is hidden while the page is up — it would lie across the page's foot — and returns with the map when the page closes. Untick and the page opens over dots, unanimated, as before.

## hostile cases

- A post opened from the grid or list: no map view selected, no map behind, no animation.
- A repaint while the page is up: the mark is re-applied after every sync.
- The map view left while a page is up (the picker tapped): `/browse` puts the set back; the next sync sees `#mapData` and the mark goes.

## glossary

(no new terms)

## code description

`opens-over-map.js` — wraps `feature_Map.sync`: the map kept and the body marked while a page is up over the map view; one map click listener that puts the page away.

`opens-over-map.css` — the marked body's transparent ground, the page's grow-in.
