# sub-tool-cards
*hold a control button and it introduces itself too*

> (transcripts/2026-08-15-fm-spec-2.md#p3)
> NEW ASK [proposed] … [in taps @ miso/loop/tap/counter] :: 'I’d like to see tooltips on sub-tools as well as top level tools'
> *(a field ask whose arrival is its own anchor, filed from inside the taps tool on 2026-08-15, miso build 165)*

## spec

The long-press tool card extends to sub-tools: hold any control button
in the toolbar — reset, ×2, −1 in taps, record in dictate — and the
same card appears, carrying the name and user documentation of the
feature that registered that control. The hold, drift-cancel, dismiss,
and swallowed-click behaviours are the parent's, unchanged; only the
set of buttons that answer, and how a button finds its documentation,
grow here.

A control resolves to its feature through ground truth, not the DOM:
the tree export stamps `subtools` on any node whose `tool_controls`
extension appends buttons (the control's `data-ev` ids, the sub-tool
twin of the `tool:` stamp), and the card looks the held button's id up
in the chooser's catalog. Without the catalog, the card degrades to
the button's `title`, as the parent does.

## user

Hold your finger on the little buttons inside a tool — like reset or
−1 in taps — and the same card explains what each one does.

## glossary

- **control**: a sub-tool button — one a feature adds to the toolbar
  while its tool is open, as opposed to a button that opens a tool.

## code description

`sub-tool-cards.js` extends the parent at two seams. It wraps
`feature_LongPress.contentFor` (redefinition + the saved original, the
JS form of `existing.fn()`): a held button whose `data-ev` is not a
`tool_` open event resolves by finding the catalog node whose
`subtools` list contains that id; `tool_` buttons pass straight to the
original, and failures fall back to the button's `title`.

It then adds its own `pointerdown` arming listener for
`.tool-button.ctrl[data-ev]` — the parent's matcher only answers
`data-ev^="tool_"`, so controls arm here, setting the parent's shared
timer/armed state so its drift-cancel, release-disarm, and dismiss
listeners govern the hold unchanged. A capture-phase click listener
mirrors the parent's swallow for control buttons: a long press reads,
it must not also fire the control.

Everything is guarded on `feature_LongPress` existing; with the parent
unticked the fragment does nothing.

The `subtools` stamp itself lives in `tools/export_features.py`
(`subtools_of`), beside `tool_of` whose pattern it follows.
