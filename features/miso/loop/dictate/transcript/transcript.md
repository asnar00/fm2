# transcript
*read what a note said: playing a note opens its transcript in a scrollable panel*

> (transcripts/2026-08-14-fm-spec-3.md#p48)
> how do I view the transcription? maybe when you tap the file icon to play it, it should also pop up a scrollable text window with the transcript in? that would be cool

## user

Tap a note: it plays, and what was said appears in a panel you can scroll. Tap again to stop — the panel follows the playback. "local draft" in the corner means your phone made this rough version; better ones will replace it automatically.

## spec

The tile shows a clipped teaser (`/dictate`'s per-tile seam); this is the full reading view. Tap a note to play it and its transcript pops up as a scrollable panel over the lower display surface — full text, stamped with the rung that made it ("local draft" for now). Tap the note again (or let it finish) and the panel goes with the playback. A playing note that is still being transcribed says so ("transcribing…") rather than showing nothing — never lie about what exists. Notes with no transcript and none coming show no panel. Pure state-driven rendering: no new events, no page-half — playback state (`dict_playing`) and the transcript stamps are already in the loop.

## glossary

(no new terms; rung and grade are defined at `/dictate` and `/phone`)

## code description

`transcript.rs` redefines `render`: when dictate is the open tool and `dict_playing` names a file, it appends the panel — full transcript text (HTML-escaped, same `&`-then-`<` order as the seam), the rung stamp as a header chip, and a "transcribing…" body instead when the playing file has no transcript but `dict_transcribe` currently targets it. No transcript and no intent: no panel.

`transcript.css` styles it: a fixed panel over the lower display surface (clear of the toolbar, safe-area aware), `overflow-y: auto`, monochrome discipline, the stamp in dim small caps. `pointer-events` stay on the panel (it scrolls) but it sits below the grid's tap targets in height so the playing tile stays reachable to stop.
