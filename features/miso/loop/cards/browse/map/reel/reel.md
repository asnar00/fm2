# reel
*a reel of the latest posts along the bottom of the map; scroll it and the map follows*

> (transcripts/2026-09-03-housekeeping.md#p18)
> in the bottom area of the map, I'd like a zone that shows posts as a most-recent first scrolling horizontal list; each post should appear as a short-form lozenge showing the image thumbnail and part of the text, with date/time and author. the zone should show about 1.5 posts horizontally, so you can see when there's more than one; scrolling the list horizontally should smoothly move the map to the current post's location. New posts should appear at the head of the list (leftmost).

## user

On the posts map, a band along the bottom holds the posts, newest first: each one a lozenge with its picture, the first of its words, who wrote it and when. About one and a half show at once, so a second one is always peeking in. Flick the band sideways and the map glides to wherever the post you land on was made. Tap a lozenge and the post opens. A new post arrives at the left end.

## spec

`/map` draws pins from `#mapData`'s rows and the page keeps the cards; this node reads both. It lives outside `#app` like the map host, made once and shown only while the posts tool's map view is up (the row shows posts selected and `#mapData` is on the page — read from the screen, not the state mirror, which lags a frame after a way-back tap and hid the band, #p19); the map host is inset by the band's height while it shows, so the band is a zone of its own under the map and above the toolbar, not a cover over the pins. The posts are the world's cards of type post, newest first by `/post-time`'s `when`, else `created` — which is what puts a new post at the left. A lozenge is the post's picture (the poster for a clip, the initial in a square if it has neither), up to two lines of its words, and a line with the author and the time in plain words (today: the time; this week: the day and time; else the date). Each is `/browse`'s own `browse_open:<id>`, so a tap opens the post through `/loop` as a tile does. The band is 66% of the width per lozenge with scroll snapping, so one and a half show. Scrolling it is read on `scrollend` (a short settle where the browser has none): the lozenge nearest the left edge is the current one, and if the post has a place the map pans there, animated; a post with no place leaves the map where it is. Re-rendering is by signature — the ids in order — so a repaint that changed no post keeps the band's scroll where the finger left it; a new post at the head is a new signature and the band shows it from the left. Untick and the map runs to the toolbar again and no band shows.

## hostile cases

- No posts: no band, the map to the toolbar.
- A post without a picture or place: a lozenge with an initial; landing on it moves nothing.
- The people map, a project's map: not the posts tool, no band.
- A repaint mid-scroll with nothing changed: the band is left alone.
- The map not yet mounted when the band renders: the pan is skipped, the next scroll pans.

## glossary

- **reel**: the band of post lozenges along the bottom of the posts map.

## code description

`reel.js` — `feature_Reel`: `host` made at load outside `#app`; `posts()` (the world's posts, newest first, joined to `#mapData`'s places); `render()` by signature; `current()` and `follow()` (the lozenge at the left edge, the pan); `sync()` wrapped around `feature_Map.sync` so it runs after every paint that could show or hide the map.

`reel.css` — the band, the lozenges, the inset on `#misoMap` while `body.fm-reel` is set.
