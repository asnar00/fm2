# everypage
*every page carries the icon links — any page can be the install point*

> (transcripts/2026-08-13-fm-spec.md#p47)
> I just got a capital M instead of the logo - maybe some issue with deploy

## user

Whichever screen you install from, the tile is the logo.

## spec

iOS takes the icon from whatever page is showing when the user adds to home screen; a page without `apple-touch-icon` links produces a generated monogram tile ("M", from the title). The login page was such a page. Rule: every user-visible page carries the full head — icon links, manifest, theme colour — because any of them can be the install point.

## glossary

- **install point**: the page being viewed at Add-to-Home-Screen; iOS takes both the icon and the app URL from it.

## code description

This node owns `everypage.page.head.html`: the `apple-touch-icon` and favicon links, composed into the head slot of every page — so any page is a safe install point. Untick it and the monogram-tile behaviour honestly returns.
