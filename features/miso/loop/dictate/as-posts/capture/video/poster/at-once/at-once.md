# at-once
*the face is taken while you film, so a new post has its picture the moment it appears*

> (asks#1788532355525)
> bugfix: thumbnail doesn't appear on new video post in reel lozenge. Should appear immediately

## user

Stop recording and the post is there with its picture on it — in the reel's lozenge, on the map's pin and on the card — in the same breath. No blank square that fills in a few seconds later.

## spec

`/poster` takes the frame *after* the clip is saved: it reads the whole recording back out of IndexedDB, makes a detached `<video>`, waits for its metadata, seeks past the end to learn the duration a MediaRecorder webm will not report, seeks again to a chosen moment, waits for that, draws and only then sends the op. On a phone that is seconds, and the post exists from the first of them — so the lozenge, the pin and the card all stood empty while it happened. Ash filed it from the walk.

**The frame is already on the screen.** The viewfinder is a live `<video>` playing the camera. A frame is kept off it every 400 ms while filming — a `drawImage` onto a canvas, no decode, no seek, no read-back — and the two most recent are held. At the stop, one of them becomes the picture.

**Which one.** The newest is passed over when it is younger than 400 ms: the last half second of a clip is a hand reaching for the stop button, which is the moment `/poster`'s own chooser exists to avoid. The chosen canvas goes through `feature_Poster.draw` — so the framing is `/square-crop`'s central square and the quality ladder is `/cards`' own, exactly as the slow road's frame would have been — and under `/pic-beside` that returns a `pic/<id>` naming bytes this device already holds. **Only one frame is ever minted**: a mint writes to the device's store and puts an id on the upload queue, so minting each tick would leave a minute's filming as a hundred and fifty stored pictures and a hundred and fifty uploads for a post that shows one.

**Into the card in the same turn as the mint.** `/capture/video`'s `metaFor` seam puts the reference on the recording's own metadata, so it rides in on `RecSaved`; `/as-posts`' `as_posts_land` — asked of every file both when the card is minted and on every later pass — writes it into the card's first *empty* picture block and marks it a poster. A picture the user chose outranks it, which is `/poster`'s own rule in `/poster`'s words. So the post is drawn with its face from its very first paint, from bytes on this device, with no request made; `/pic-beside`'s resolver swaps the reference for the local copy in the reel's row and on the map's pin as it does everywhere else, and the upload follows on its own queue.

**And the slow road stands down.** `feature_Poster.make` returns nothing when the recording's card already carries a poster: its op would find the block filled and write nothing, and the frame it decoded would be bytes on the upload queue that no card names. So `/until-play`'s promise holds too — the element is never swapped under the reader, because there is no second picture.

Untick and the face arrives seconds later, by the road it took before.

**What was measured, and where.** On the rig, with a real camera stream, a real
`MediaRecorder` and the app's own save: the lozenge shows the frame **182 ms
after the stop**, drawn from a `blob:` URL — the device's own copy, no request
made — the card's block holds the same reference the metadata carried, one
picture is minted for the recording and no other, and four seconds later the
same element is still there with the same source. `/poster`'s expensive road
ran **zero** times. The camera half could not be run on the iPhone simulator:
an iOS simulator has no camera for `getUserMedia`, so the grab has no live
frame to take there. The rendering half it feeds — a `pic/<id>` in a card
drawn into the reel's lozenge from the local store — is the road `/pic-beside`
already runs on that device for every other picture.

## hostile cases

- **A clip shorter than one tick.** No frame kept; the metadata carries no face and `/poster`'s own road runs as it always did.
- **A clip of exactly one tick.** One slot, so that frame is used however new it is — better a hand reaching for stop than nothing.
- **The camera refused, or the stream stopped early.** `drawImage` throws and the tick disarms itself; the slow road covers it.
- **The stop.** The frames stop being kept before the tracks are stopped, so the last one held is one the camera was still making.
- **A picture the user put on the post already.** The block is not empty; nothing is written, on this road or the old one.
- **`/pic-beside` unticked.** `draw` returns a data URL instead of a reference and it goes into the block as one — bigger, and exactly what the card would have carried before that node existed.
- **The upload fails or the device is offline.** The picture is the device's own copy throughout; it shows, and the id waits on `/pic-beside`'s queue.
- **A recording whose metadata is republished later without the face** (an index from another device). `as_posts_land` finds no `poster` field and leaves the card alone.
- **Two recordings in quick succession.** The frames are stamped with the recording they belong to; a set that does not match the file being saved is refused.
- **A sibling taking the same seam.** `/streams` *assigns* `feature_Video.metaFor` rather than wrapping it, from an install that runs after this node has loaded — so a wrapper put on once is simply gone. The seam is taken again whenever it is not this node's, which is `/poster`'s own `hook()` idiom for the same reason (measured: without it the face never reached the metadata at all).
- **`/poster` asking before the card exists.** It asks the moment the recording is saved, and the card is not minted yet at that moment — so the stand-down is decided by what this node handed over, not by what the world holds.

## glossary

(no new terms)

## code description

`at-once.js` — `feature_AtOnce`.

`grab()` keeps one frame off the viewfinder as a canvas, naming `videoWidth`
and `videoHeight` on it so `/poster`'s `draw` can take it as it takes a video.
`arm()` and `disarm()` run the tick, armed from `/capture/video`'s
`viewfinder` and disarmed at the head of its `stop`.

`frameFor(id)` picks the frame — the newest, or the one before it when the
newest is younger than `OLD` — and mints exactly that one through
`feature_Poster.draw`.

`already(id)` reads the world for a card carrying this recording and a filled
poster block, and the redefinition of `feature_Poster.make` uses it to stand
the slow road down.

`feature_Video.metaFor` is the seam the face rides in on: the reference is
added to the recording's metadata, which is what `RecSaved` carries.

`at-once.rs` — `as_posts_land` is extended to write `file["poster"]` into the
card's first empty picture block and mark it a poster, after whatever the
chain beneath it did with the transcript.
