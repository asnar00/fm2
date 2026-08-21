# standalone
*reports say whether they came from the installed app or a browser*

> (transcripts/2026-08-13-fm-spec.md#p48)
> can the app tell whether it's running in the browser or as a PWA?

## user

Nothing visible — but when something's investigated remotely, we know which context reported it.

## spec

Yes — per launch context: `display-mode: standalone` matches (plus Safari's legacy `navigator.standalone`) when running from the home screen. The answer joins every launch report as `pwa:`, so the diag log distinguishes a device's installed app from its Safari tabs — they have separate cookie jars and separate behaviour, and debugging needs to know which one is talking.

## glossary

(no new terms)

## code description

This node owns `pwa.js`: `feature_Pwa.standalone()` (`display-mode: standalone` + Safari's `navigator.standalone`) and `phone()` (UA + iPad touch-points). `/update` includes `pwa:` in launch reports; `/install` consults it for the redirect; `/push` for enrolment visibility.
