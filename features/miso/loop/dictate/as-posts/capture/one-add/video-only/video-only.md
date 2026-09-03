# video-only
*every post is a video: the add button records, and there is no kind to pick*

> (asks#1788472464749)
> all posts should be video - let's lose audio and still post types

## user

In the posts tool the add button records a video. There is no small control
beside it for choosing a kind: no audio, no still photo, no written post.
Tap add, record, and the post is made where you stand.

## spec

`/one-add` gave the row one add button and a mode control beside it wearing
the chosen kind — photo, video, audio or write. Ash's ruling from the field
(the ask): all posts are video; the audio and still kinds go. Read literally
and whole: with every post a video the chooser has one entry, and a control
with one entry is not drawn (`/taste` 7: nothing that shows what it does
needs explaining beside it). The written kind goes with it — the ask says
*all* posts.

**Two seams, both `/one-add`'s.** `one_add_mode` answers `video` whenever
the video control is in the row (otherwise the base's answer, so a
composition without `/video` still has a working add); `one_add_mode_button`
draws nothing, so the picking state is unreachable and the row is the add
button and undo. `/video`'s own recording edges, stop button and poster are
untouched — add is `vid_rec`, as it was when video was picked.

**Existing posts keep their kind** — a written or audio post already made
still opens and plays; only the making changes. Parked: the audio and photo
nodes stay in the tree and composed (their buttons are cut from the row by
`/one-add` as before); unticking them in the product is a separate act for
ash.

## hostile cases

- **`/video` unticked.** No video control; the base's fallback (`write`)
  and its button draw as before this node.
- **Mid-recording.** The stop takes the add slot as before; no mode control
  either side of it.
- **A device whose stored mode is `photo`.** Ignored; the mode is video.
- **This node unticked.** The chooser and all four kinds, as before.

## code description

`video-only.rs` — `one_add_mode` returns `video` when the video control is
present; `one_add_mode_button` returns nothing.
