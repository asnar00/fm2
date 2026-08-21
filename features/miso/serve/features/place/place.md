# place
*the conversation drawer opens at the right prompt*

> (transcripts/2026-08-13-fm-spec.md#p78)
> wee bug: on mobile, the conversation pane isn't updating to the right place when we tap a node.

## user

Tap a node, open ❝: you're at the highlighted prompt.

## spec

Tree links carry the provenance `#pN` fragment, but mobile Safari refuses to fragment-scroll inside the off-canvas transcript drawer — so opening it landed at the top. A small scroll-position shim (the pages' only JavaScript, strictly progressive enhancement) places the drawer's scroll on load, on every in-page fragment tap, and only when meaningfully off-target so desktop's native behaviour is untouched.

## glossary

(no new terms)

## code description

The `place()` function at the end of explorer.py's template: resolves `location.hash` inside `#right`, adjusts `scrollTop` by the measured delta when it exceeds 40px; wired to load and `hashchange`.
