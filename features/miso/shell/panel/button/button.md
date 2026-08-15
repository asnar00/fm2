# button
*the panel handle is a logo button, not a version readout*

> (transcripts/2026-08-13-fm-spec.md#p69)
> let's replace the version number (fine for debugging, not amazing to bother users with it) with a little button with the logo on it (in place of the hamburger). That can highlight subtly (or unsubtly) to indicate there's a new build; we press it to see the same version screen we do now.

## spec

Users aren't bothered with raw build numbers: the corner shows a small ᕦ(ツ)ᕤ button (the hamburger stand-in), and numbers live inside the `/panel` only. When `/watch` finds a newer build, the button highlights — a gentle accent-coloured pulse — and pressing it opens the panel as before. `/lozenge` gives it its button-like outline.

## user

The little logo in the corner is the menu. Glowing blue means an update is waiting.

## glossary

(no new terms)

## code description

This node owns `button.index.html` (the `#build` div carrying the logo glyphs) and `button.index.css` (its look, plus the `.update` accent-pulse animation). `/watch` toggles the `.update` class; `/lozenge` adds the outline; `/corner` places it.
