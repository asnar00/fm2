# clips-too
*a clip shows as its central square too, not just its still*

> (transcripts/2026-09-03-housekeeping.md#p5)
> the video post should have a square media, rather than the portrait clip. All visual media should be cropped to a central square, not just stills.

## user

A video post's clip is square on the page, like its poster and like every picture: the middle square of what was filmed, held upright or sideways. The viewfinder shows that same square while you film, so what you frame is what the post keeps.

## spec

`/square-crop` made every stored picture the central square and said in as many words that the video itself was untouched — it played as it was shot. Ash's ruling: all visual media is a central square, the clip too (#p5). The bytes stay as they are — a clip is megabytes and re-encoding it on the phone is a cost with no ask behind it — so the square is the frame it is shown through: the player is a square of the page's width with the clip covering it, centred on both axes, which on a portrait clip shows the middle and on a landscape one the middle too; nothing is stretched. The viewfinder gets the same square, so the picture you compose is the picture the post shows. The poster was already the central square (`frameOf`), so a post reads the same before and after the tap. Untick and the clip plays at its own shape again, the viewfinder with it.

## hostile cases

- A landscape clip: the square shows its middle, the ends are outside the frame.
- A copy held by someone else (the dim row): no clip, nothing to frame.
- The clip's controls: the browser's own, inside the square; the square is the video element itself, so nothing overlays them.

## glossary

(no new terms)

## code description

`clips-too.css` — two rules: `.post-video video` a square (`aspect-ratio: 1`, the cover fit, centred) and `#vidView` the same.
