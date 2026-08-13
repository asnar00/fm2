# pinned
*screens are solid: no scroll bounce, no zoom*

> (transcripts/2026-08-13-fm-spec.md#p56)
> small tweak: the login/logo screens under PWA are "loose" in that they allow us to scroll - it would feel more solid if they didn't. (and zoom actually)

## spec

The page shell never moves: `html/body` are fixed and non-overflowing with `overscroll-behavior: none` (no rubber-band), the viewport forbids scaling and a `gesturestart` guard stops pinch, and `touch-action: manipulation` removes double-tap zoom (and the 350ms tap delay with it). Applies to all three pages. When content needs to scroll, it scrolls inside a child element — the frame stays rigid.

## user

The app feels native-solid: nothing drags, bounces or zooms.

## glossary

(no new terms)

## code description

This node owns `pinned.page.css` (the fixed, non-overflowing, non-overscrolling page frame and `touch-action: manipulation`) and `pinned.page.js` (`feature_Pinned` + the `gesturestart` pinch guard) — composed into every page.

The `maximum-scale=1, user-scalable=no` viewport attributes remain in each page skeleton (a page has exactly one viewport meta).
