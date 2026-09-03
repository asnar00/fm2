# player-in-place
*the player stands where the poster stood: above the words*

> (transcripts/2026-09-03-housekeeping.md#p3)
> I made a video post last night and that's showing the text above the media, which is supposed to have been fixed.

## user

Tap a video post's poster and the clip plays where the poster was — above the words, like a photo. It stays there through every repaint.

## spec

`/poster` puts the face where the picture stood, and `/picture-first` and `/titled/above` order the picture ahead of the words with an `order` rule on the flex column; the poster carries the same rule. The tap that opens it (`open`) takes the `post-poster` class off the holder and puts `/capture/video`'s `post-video` on, and no rule orders THAT — so the moment the clip starts, the holder falls back to its place in the DOM, which `/posts` put after the words: the words jump above the video, and `restore` re-opens it after every paint, so it stays wrong for the rest of the visit. Ash saw exactly that on last night's post (#p3). One reading, so it builds: the player gets the poster's order. A video post with no face — a clip whose frame could not be taken — gains the same: its player leads the words too. Untick and the clip drops under the words again when it plays.

## hostile cases

- A poster never tapped: unchanged, the poster's own rule orders it.
- A foreign copy (the dim row): ordered ahead of the words as well, where the poster would have been.
- A photo post: no `post-video` on the page, nothing to order.

## glossary

(no new terms)

## code description

`player-in-place.css` — one rule on `.card-page.post .post-video`, the poster's own `order`.
