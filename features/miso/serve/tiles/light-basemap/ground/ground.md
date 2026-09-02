# ground
*the colour under a square that has not loaded yet follows the basemap*

> (transcripts/2026-09-02-self-check.md#p47)
> when tiles haven't loaded yet, we get a bright coloured tile square - could we make those dark grey to match the new map colour

## user

While the map is still fetching a square, the gap is the same dark grey as the map around it, not a bright block.

## spec

`/light-basemap` painted the map's ground `#eae8e4` to match the light OpenStreetMap squares it introduced. The ground became Stadia's Alidade Smooth Dark on 2026-09-02 (`/fresh-tiles` `g=3`), and a square still on its way now showed as a bright block against dark land — the colour of the old ground. This node paints the ground the new basemap's land colour, `#333333`, sampled from a live square, so an unloaded square reads as more map. It is the basemap's companion: when the ground changes again, this colour changes with it and `/fresh-tiles`' tag bumps.

## hostile cases

- Offline, no square at all: the whole map is the dark land colour, pins on it, credits below — the same shape the dark ground had before `/light-basemap`.
- Node unticked: `/light-basemap`'s `#eae8e4` returns.

## glossary

- **ground**: the map's own background, seen wherever a square is missing.

## code description

`ground.css` — one rule on `#misoMap`, composed after `/light-basemap`'s.
