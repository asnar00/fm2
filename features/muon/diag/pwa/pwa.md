# pwa
*reports say whether they came from the installed app or a browser*

> (transcripts/2026-08-13-fm-spec.md#p48)
> can the app tell whether it's running in the browser or as a PWA?

## spec

Yes — per launch context: `display-mode: standalone` matches (plus Safari's legacy `navigator.standalone`) when running from the home screen. The answer joins every launch report as `pwa:`, so the diag log distinguishes a device's installed app from its Safari tabs — they have separate cookie jars and separate behaviour, and debugging needs to know which one is talking.

## user

Nothing visible — but when something's investigated remotely, we know which context reported it.

## glossary

(no new terms)

## code description

The `pwa` constant in `/shell`'s loader (`matchMedia('(display-mode: standalone)')` OR `navigator.standalone === true`), included in the `diag()` launch report; the same check drives `/install`'s redirect and the panel's push-enrolment visibility.
