# drawers
*on phones, the side panes slide out; the laptop layout stays*

> (transcripts/2026-08-13-fm-spec.md#p76)
> SORRY: I tried features/ on my phone - let's keep the layout the same as for laptop, but make the tree and conversation sections "pop-out" based on a button or drag-from-side action. ta :-)

## spec

Desktop keeps its three columns. Under 900px the spec+code pane owns the screen; ☰ (top-left) slides the tree in from the left and ❝ (top-right) slides the conversation in from the right, with a tappable shade to dismiss. Pure CSS via the hidden-checkbox pattern — no JavaScript, pages stay static and curl-able. Drag-from-edge would need touch-event JS; buttons satisfy the request, swipe is deferred.

## user

On a phone: read in the middle, ☰ for the tree, ❝ for the conversation, tap the dimmed page to close.

## glossary

(no new terms)

## code description

In explorer.py's shared template: two hidden checkboxes ahead of the panes, corner `label` buttons, `:checked ~` transforms sliding the fixed panes in, and per-drawer shades — all inside the `max-width: 900px` media query.
