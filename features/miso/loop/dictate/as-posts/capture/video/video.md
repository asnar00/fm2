# video

*a video is a recording that you can see, and its speech becomes the post's
words*

> (transcripts/2026-09-01-saturday.md#p2)
> clean up the "make posts" interface to include video, audio, photo+type,
> transcription

## user

The camera-with-a-lens in the posts row records video. A viewfinder opens
above the toolbar so you can see what you are filming; tap stop and the clip
becomes a post, playable on its page, with whatever was said in it written
into the post's words a few moments later. A minute is the longest clip.

## spec

`/as-posts` already answers the hard part of this. A recording becomes a post
through one pass over `dict_files` after every event — the same answer owed to
a recording made here, one another device announces, one in the boot index and
one already in IndexedDB — and `/phone`'s transcript lands in the post's words
without ever overwriting words a person typed. A video is a recording with
pictures in it, so this node adds a kind, not a pipeline: the same meta, the
same `<owner>.<t>` id on every device, the same minting, the same landing.
Only the block's `kind` differs, and only that is redefined.

**The wire, named.** The bytes do not go on the card. A video is megabytes
and the whole cards list travels as ONE `/msg` op — the cap `/wider` set at
192KB — so putting a clip there would jam the outbox forever, which is the
failure `misses.md` records under "the picture cap". The clip goes the way a
recording's audio goes: `POST blob/<id>`, `/mirror`'s per-user blob route, a
raw body the serve layer reads up to 16MB. What travels on `/msg` is what
travelled before — the `RecShared` meta (about 140 bytes) and the card, whose
video block is `{kind, id, dur, mime}`, about 90. **A minute at 1Mbit is
about 8MB**, and that is why there is a minute: over 16MB the exchange would
refuse the upload, `/mirror` would break out of its loop and retry forever,
and the clip would be one that could never be handed on. The cap makes that
unreachable rather than unlikely.

**Transcription.** `/phone` decodes a recording to 16kHz mono PCM before the
engine sees it, and getting the audio track back out of a recorded *video*
container is not a thing every browser will do. So a second, audio-only
recorder runs beside the first on the same microphone track, and its blob is
kept next to the video under a key the exchange could never accept as an id —
never listed, never uploaded, never a second post. `/phone`'s decode step is
wrapped to reach for it. With no companion — an old file, a device that gave
no microphone — the original decode runs, and if it fails `/phone` stamps the
attempt and the scheduler moves on, exactly as it does today: no loop, no
words, the post still a post.

**The three ways it fails**, each proven rather than reasoned about. The
camera is refused: the intent is written straight back to off, so the control
does not sit there pretending, no viewfinder opens, no post is made, and the
app says "no camera here" in `/cards`' own voice. The minute runs out: the
recording stops itself and becomes a post exactly as a tap on stop would.
There is no room to store it: the meta is written LAST and only if the bytes
are down, so a failure says "no room for that video" and leaves nothing —
never a post pointing at a clip that is not there.

**One camera at a time.** Starting a video turns the audio recording off and
starting an audio recording turns the video off; each page half watches its
own flag and sees an edge, so a recording cut short this way is *saved*, not
lost — it takes `/dictate`'s ordinary stop path.

Costs and gaps, named. A render is a whole-DOM swap, so the player is put
back after every one; where it had got to and whether it was playing are
remembered on this side, so a repaint mid-clip is invisible rather than a
restart — but the first frames after a repaint come from a seek, not from
nowhere. A copy handed to someone else carries the block and the words and
not the bytes, because `/mirror`'s route is per-user — the same honest note
`/as-posts` gives a foreign recording. Editing or trimming a clip, and more
than one clip on a post, are not here and were not asked for.

Untick and the control leaves the row, the block kind is never minted, and
every recording, photo and typed post is untouched.

## glossary

- **companion audio**: the audio-only recording made beside a video, kept
  locally so transcription never has to decode a video container.

## code description

`video.rs` redefines `/capture`'s `capture_extra` to append the control —
two faces, the camera and the stop with `/dictate`'s breathing dot, chosen on
`vid_recording`.

`update` takes `vid_rec` and `vid_stop` into `vid_recording`, and clears the
other recorder's flag on either that or `dict_rec`.

`as_posts_card` is redefined to retag the block `/as-posts` makes, from
`audio` to `video`, for a file whose meta says `kind: "video"`. Everything
else about the card — the `rec` key that survives a delete, `when` for
`/post-time`, the transcript landing — is `/as-posts`' and is not touched.

`card_page_html` puts a mount before the words, where `/as-posts` puts its
play row, carrying `data-rec` so `/as-posts`' "transcribing…" hint finds a
video too. A foreign copy gets the note instead. `card_tile_html` marks the
grid tile, beside the audio mark rather than over it.

`video.js` drives the camera off state's edges (replay-guarded, `/dictate`'s
rule), runs the companion recorder, stores blob + companion + meta in
`/dictate`'s own IndexedDB store — all three inside one `try`, meta last —
and sends `RecSaved`, wraps `/phone`'s
`pcm16k`, opens the viewfinder outside `#app`, and re-mounts the player after
every render. Every cross-feature reach is typeof-guarded.

`video.css` is the viewfinder, the player, the foreign note and the tile
mark — the house ground, 12–14px radii, the 0.18s ease-out the toolbar uses.
