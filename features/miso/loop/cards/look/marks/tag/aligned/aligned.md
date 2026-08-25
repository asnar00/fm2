# aligned
*the type tag lines up with the top of the title*

> (asks#1787669362202)
> vertically align card type indicator with top of title
> *(filed from the field on 2026-08-25 by ash, birthplace `👤 @ miso/shell/panel/account`)*

## user

The little type tag in the corner sits level with the top of your name.

## spec

`/tag` placed the pill 12px from the page's top edge; `/ground` pads the page 16px, so the title's line box starts 4px lower than the tag. Ash asked for them aligned. One reading, so it builds: the tag's `top` becomes 16px — the padding — so its top edge and the title's line box share a line (measured on the rig: tag top = title top). Untick and the 12px returns.

## glossary

(no new terms)

## code description

`aligned.css` — one declaration on `.card-tag`.
