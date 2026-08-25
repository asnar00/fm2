# tag
*the card's type, as a little coloured tag in its corner*

> (asks#1787667662051)
> let's show the card type in the top right corner as a little coloured rounded tag, different colour per type name
> *(filed from the field on 2026-08-25 by ash, birthplace `👤 @ miso/shell/panel/account`)*

## user

Every card wears a small rounded tag in its top-right corner saying what it is — **profile**, later **project**, **post** — each type in its own colour, the same colour everywhere.

## spec

Cards are told apart by their `type` field and nothing on the page said which a card was. Ash asked for the type in the top-right corner as a little coloured rounded tag, a different colour per type name. One reading, so it builds.

This node extends `card_page_html`: it takes the page `/cards` drew and inserts a `.card-tag` just inside the opening div, so the ground carries it and it scrolls with the card. The colour is `card_tag_colour(type)`: a fixed palette of eight dusty tones, picked by a hash of the type name — so a type nobody has seen yet gets a colour without anyone assigning one, and every device agrees. On `/taste` 3 (a colour is a word): here the word *is* the type name, one colour per meaning; the palette is desaturated to stay in the family. Untick and the corner is bare.

## hostile cases

- A card with no `type`: no tag, page unchanged.
- A type name with markup in it: escaped by `card_esc` on the way out.

## glossary

- **tag**: the small coloured pill naming a card's type.

## code description

`tag.rs` — `card_page_html` inserts the tag after the first `>` of the page `existing` returns; `card_tag_colour` is the djb2-style hash into the palette.

`tag.css` — the pill, absolutely placed inside the page.
