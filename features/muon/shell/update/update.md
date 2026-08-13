# update
*deployed changes reach every device by themselves*

> (transcripts/2026-08-13-fm-spec.md#p41)
> having installed the PWA on my homescreen, how do we get it to refresh to the latest version?

## spec

Keeps installed apps current without user effort. Every deploy stamps a /build number/ into `site/version` (the commit count — no counter file, and each build names an exact commit); the shell compares it with the build it launched from and, on change, drops the cache and reloads once. Mid-session, foregrounding, regaining connectivity, and a 60-second visible poll re-check; a pending build lights the `/panel` handle. Failed checks surface as "can't reach the server" — never as "up to date" (#p59). A what's-changed list (`changes.json`, commit subjects tagged with build numbers) is generated at deploy for the panel; `/push` announces deploys to enrolled devices even when the app is closed.

## user

Nothing to do. Online launches are simply current; a deploy mid-session makes the logo button pulse — tap it and press update. The corner of the app never shows raw numbers; builds are listed in the panel.

## glossary

- **build number**: a plain increasing integer naming each release — the repo's commit count at deploy (#p51).
- **deploy stamp**: the build number written to `site/version`, compared by the shell on every check.

## code description

The mechanics live in `/shell`'s loader (`checkForUpdate`, the launch comparison, the retry loop and foreground/online/poll triggers) and in deploy.sh (the stamp and `changes.json` generation).

The service worker's network-first freshness deadline is what makes launch-time currency automatic; the stamp comparison covers assets caught mid-deploy.

This node records the intent and the rules; see `/shell` for the code walk-through.
