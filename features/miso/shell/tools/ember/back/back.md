# back
*a ‹ at the left of an open tool's row: the way out*

> (asks#1787703450616)
> draw a "<" button to the left of the "current tool" button so it's clear that exits
> *(filed from the field on 2026-08-26 by ash)*

## user

With a tool open, a **‹** sits at the left end of the toolbar; tap it and you are back at the launcher.

## spec

`/tools` once had a ‹ (#p42) and `#p88` folded it into the open tool's own button — tap the tool to go home. With cards, that button now steps back one level (card → set → home), and ash asked for the exit to be visible again (`asks#1787703450616`). One reading, so it builds: this node extends `render_toolbar` and, with a tool open, puts a drawn ‹ (per `/glyphs`) first in the row, firing `tools_home` — the event the old ‹ fired, which `/tools` still handles. It is plain ink, not tinted: a door, not a tool. The tool's own button keeps its step-back meaning. Untick and the row starts with the tool again.

## glossary

(no new terms)

## code description

`back.rs` — `render_toolbar` calls `existing` and inserts the ‹ button after the row's opening tag; `back_svg` is the chevron.

`back.css` — the quiet button.
