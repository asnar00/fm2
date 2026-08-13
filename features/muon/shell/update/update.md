# update
*deployed changes reach every device by themselves*

> (transcripts/2026-08-13-fm-spec.md#p41)
> having installed the PWA on my homescreen, how do we get it to refresh to the latest version?

## spec

The base mechanism: every deploy stamps a /deploy stamp/ into `site/version`; on launch, after paint, the shell compares it with the build it launched from and on change drops the cache and reloads once. Together with the service worker's network-first freshness deadline, an online launch is simply current. Refinements live in the subfeatures: `/buildnum` (what the stamp is), `/watch` (noticing deploys mid-session), `/honest` (what to say when the check can't run).

## user

Nothing to do — relaunch the app online and you're on the latest build.

## glossary

- **deploy stamp**: the build identifier written to `site/version` at deploy, compared by the shell on every check.

## code description

The launch comparison lives in `/shell`'s loader (fetch `version` cache-bypassed, compare with localStorage, `caches.delete` + reload once); deploy.sh writes the stamp.

This node records the intent and the rules; see `/shell` for the code walk-through.
