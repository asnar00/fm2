# miso-button
*the ask button says the app's own word*

> (transcripts/2026-08-15-fm-spec-2.md#p14)
> NEW ASK [proposed] … :: 'let’s change the “ask” button to say “miso”'
> *(a field ask, filed from the launcher on 2026-08-15, miso build 170)*

## user

Type what you want and press **miso** — make it so.

## spec

The button beside the ask box reads **miso** instead of "ask". You type
a wish and press the name — "make it so". Nothing else about the row
changes: the placeholder still explains what the box is for, Enter
still fires, and the button's behaviour is untouched.

## glossary

(the ask box and its parts are defined at `/ask`)

## code description

`miso-button.index.js` re-labels `#askGo` at load: the parent builds
the ask row once when its fragment runs, and this fragment runs after
it (fragments load in provenance order, newest last), so a plain
`textContent` assignment lands on the finished row. Guarded on the
button existing; unticked, the label is the parent's "ask" again.
