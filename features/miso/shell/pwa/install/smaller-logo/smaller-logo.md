# smaller-logo
*the install page's logo at half its size*

> (transcripts/2026-09-03-invite-test.md#p74)
> couple of notes for the install page: 1) make the logo half its current
> size.

## user

The face at the top of the install page is half the size it was, leaving
room for the words beneath it.

## spec

`/install`'s skeleton draws the logo at 16vw (80px on a wide screen) with a
2.2rem gap beneath. Ash (#p74): half. This node's stylesheet fragment sets
8vw, 40px wide, and a 1.2rem gap — the same proportions, halved, so the page
still reads as one thing. The skeleton's rule stays; this one lands later in
the style slot and wins by cascade.

## hostile cases

- **This node unticked.** 16vw again.

## code description

`smaller-logo.install.css` — `.logo` at 8vw / 40px, `margin-bottom: 1.2rem`.
