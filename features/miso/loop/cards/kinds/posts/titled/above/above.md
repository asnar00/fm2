# above
*a post's title sits above its picture*

> (asks#1787707769401)
> Post title should sit above photo
> *(filed from the field on 2026-08-26 by ash)*

## user

A post reads title, picture, words — like a person's card.

## spec

`/picture-first` moved a post's picture ahead of everything, and `/titled` then gave posts a title that landed under it. Ash asked for the title above the photo (`asks#1787707769401`). One reading, so it builds: the page is a flex column, so the title's `order` goes one step ahead of the picture's. Untick and the picture leads again.

## glossary

(no new terms)

## code description

`above.css` — one rule.
