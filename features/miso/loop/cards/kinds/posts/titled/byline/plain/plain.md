# plain
*a post page without the clip's length, the owner note, and the map pill*

> (asks#1788463728652)
> remove video duration, "video stays with its owner" and "map location"
> from card / post view

## user

A post's page is the title, the byline, the picture or clip, and the words.
The clip's length, the note that a video stays with its owner, and the
*map location* pill are gone from it.

## spec

Three things on a post page said less than they cost (the ask): the
duration beside a clip, `/poster`'s note on a copy, and `/location`'s pill.
This node's stylesheet hides all three on post pages, scoped by the page's own
`post` class; profiles keep their map pill, where it is the way to
set a location. The code that draws them is untouched — untick and they
are back — and the map still places a post by its location block.

## hostile cases

- **A profile or project page.** Not scoped; the pill stays.
- **`/byline` unticked.** This node goes with it (its parent); the class
  is the page's own.
- **This node unticked.** All three shown, as before.

## code description

`plain.css` — `.card-page.post` hides `.post-dur`,
`.post-play-note` and `.card-place`.
