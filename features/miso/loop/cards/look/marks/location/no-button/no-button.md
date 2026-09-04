# no-button
*the "map location" pill goes; the place stays*

> (asks#1788558075894)
> remove "map location" button from all cards

## user

No **map location** button on any card. Where a card was made is shown by its pin on the map behind it, which is always there now.

## spec

`/location` draws the pill at the foot of a card page, `/map-pin` puts the locator glyph in it and `/to-map` makes its tap go to the map. `/byline/plain` had already taken it off a post's page — one rule, scoped by the page's own `post` class — on the grounds that a post page had enough on it. Ash has now asked for it everywhere (the ask).

**And the argument is the same one, grown.** Since `/map-only` and `/always-the-ground` the map is the ground behind every card, so a card's own pin is on the screen while the card is being read: a button whose whole job was to take you to that map has nothing left to add. This node widens `/byline/plain`'s rule from a post's page to every card page, which is one selector and the idiom the tree already used for exactly this pill.

**Nothing else is touched.** The location block is the card's own and is what `/map`'s pins, `/reel`'s band and `/where-taken` all read; `/from-picture` still reads a place off a photograph; `/to-map` stays in the tree unreached, the way `/location`'s coordinate sheet has since `/to-map` replaced it, so unticking this node brings the pill and its tap back together.

Untick and the pill is on every card page again.

## hostile cases

- **A card with a place.** The pill is not drawn; the place is in the card and on the map behind it.
- **A card with no place.** `/location` drew the pill dimmed; now it draws nothing, which is one less thing saying nothing.
- **A card with no map behind it** (a card reached where the map is not the ground). The place is still on the card and in every list that reads it; there is no on-card pill to show it, which is what was asked for.
- **`/map-pin` and `/to-map` unticked.** Nothing to hide differently: the rule is about the pill, however it is drawn or wherever its tap goes.
- **A tile or a row.** The rule is scoped to a card page, which is the only place the pill was ever drawn.

## glossary

(no new terms)

## code description

`no-button.css` — one rule: no `card-place` on a card page.
