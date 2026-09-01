# capture

*one set of controls for making a post: photo, video, audio, words*

> (transcripts/2026-09-01-saturday.md#p2)
> I demoed to Tara - she liked it, and our first actual canvassing session is
> on Saturday! So we have some work to do to get ready. In no particular
> order: 1] show constituency and ward boundaries on the map; 2] **clean up the
> "make posts" interface to include video, audio, photo+type, transcription**
> 3] make a quicker QR-code based invite workflow that we can use instantly
> during canvassing 4] AI interrogation / report generation.

## user

On the posts tool the ways to make a post sit together in one row, in one
order, wearing one colour: a photo, a video, a recording, and the plus for
words. Undo stays where it always is, last.

## spec

The make-a-post surface grew a control at a time. `/plus-at-home` put the
plus on the posts toolbar; `/as-posts` put the recording dot beside it, each
inserting itself in front of `/undo` as it arrived, so the row's order was
the order the features shipped in. Ash asked for that cleaned up and widened
to every way a canvasser captures a doorstep (`#p2`).

This node is the row. It takes the two controls that are already there out of
wherever they landed and re-lays the set in one deliberate order — the
capture kinds first, then the plus, then undo — and it opens `capture_extra`,
the extensible function a kind plugs into to join the set. Its two
subfeatures are the two kinds the tree lacked: `/photo` and `/video`. Audio is `/as-posts`' own dot,
unchanged and simply placed; transcription is `/phone`'s, and it reaches a
photo or video post through the same `dict_files` pass that already carries a
recording's words.

The controls are lifted by their events, not by position: a control whose
event is not in the row was never drawn, and this node skips it. So with
`/as-posts` unticked the row is photo, video, plus; with both subfeatures
unticked the set is what it was and `tool_controls` re-emits the same two
buttons it was handed. Untick this node and every control returns to the
place its own feature put it.

Seven buttons is one more than the row was squeezed for (`/undo`'s note:
six 50px buttons want 340px in a 296px bar). The gap tightens to 6px while a
capture control is present — scoped with `:has()`, so it applies exactly when
this node's set is on screen and never anywhere else.

Nothing is added to the wire here: the buttons are markup in the toolbar,
which travels in the rendered page, not in a var.

## glossary

- **capture kind**: one way of starting a post — a photo, a video, a
  recording, or typed words. Each is a control in the set and, below the
  first two, its own node.

## code description

`capture.rs` redefines `tool_controls`. While the posts tool is open with no
card open (`/plus-at-home`'s rule for the plus, which `/as-posts` adopted for
the dot), it grabs the plus and the recording control out of the row, cuts
them, and inserts `capture_extra` + the recording control + the plus in front
of `/undo` through `/posts`' own `posts_before_undo`.

`capture_extra(state)` is the extensible function: the base returns nothing,
and a kind's function extension appends its own button, so the row order is
provenance order among the kinds.

`capture_grab` returns the whole `<div>…</div>` of the control carrying an
event, and `capture_cut` removes it. A control row nests no divs inside a
button, so the first `</div>` after the opening tag closes it — the rule
`/posts` already relies on. Both answer with an empty string for a control
that is not there.

`capture_button` is the shared button: `tool-button ctrl capture`, tinted
with the posts tool's own colour through `/ember`'s `tool_colour`, so the
whole set reads as one thing (`/taste` 3, `/glyphs`' rule that a tool's make
button wears the tool's colour).

`capture.css` tightens the toolbar's gap while a capture control is present,
and takes the drawn glyphs to black on the tint the way `/as-posts` does for
the dot.
