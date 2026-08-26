# rects
*the readout says where things are*

> (transcripts/2026-08-26-session.md#p164a)
> run user-level interaction tests on the ios simulator, at high speed (i.e. not waiting for screenshots to tell you what's on there)

## user

Nothing to see. The builder's rig can put a real finger on any control by name.

## spec

`/readout` serialised what was on screen but not where, so a simulator test had to read coordinates off screenshots (#p164a). Each visible node now carries `r: [left, top, width, height]` in CSS pixels — on an iPhone, screen points, so `tools/simrig.py` taps the centre of a selector with `idb`. `ctl` (the toolbar's page control), `face` (its pencil or tick), and `ce` (a block open for writing) ride along; the root carries the visual viewport's offset and height (the keyboard's shift), the scroll, what has focus, and the screen and web-view sizes — a standalone app's web view sits below the status bar, and the rig adds that inset to every finger. Untick and the readout is shape only.

## hostile cases

- A hidden node: no rectangle, as before.
- The keyboard up: rectangles are layout-viewport; the root's `vv`/`sy` say how far the screen has moved, and the rig subtracts.

## glossary

(no new terms)

## code description

`rects.page.js` — wraps `feature_Readout.capture`; reposts on viewport resize/scroll, scroll, and focus changes.
