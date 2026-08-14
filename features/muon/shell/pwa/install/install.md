# install
*phone browsers are directed to add muon to the home screen*

> (transcripts/2026-08-13-fm-spec.md#p49)
> so I think a good feature would be: if we detect we're running on a phone browser, we direct the user to add to home screen before we let it continue.

## spec

A phone browser visiting muon is sent to `install.html` before anything else — a user never sees the login screen until they've installed. The page shows only the logo and the instruction steps: tap [share icon], then [⌄] view more, then [+] Add to Home Screen (Android: browser menu → Add to Home Screen). The installed PWA and desktop browsers proceed as normal. Detection: mobile user agent (incl. modern iPad's Mac-like UA via touch points) and not `display-mode: standalone`. The page carries the full head (manifest, apple-touch-icon) because it *is* the install point — and since iOS installs the current page's URL, it self-redirects to `/` if ever launched standalone. No visible escape; `?browser=1` remains as an undocumented dev bypass (session-scoped).

## user

Visit muon.nøøb.org on a phone: you're shown the logo and how to add muon to your home screen — nothing else until you do. Open the installed app to reach login and the app itself.

## glossary

- **install point**: the page being viewed when the user adds to home screen; iOS takes both the icon and the app URL from it.

## code description

`assets/install.html` is a static public page (on the gate's `is_public` shell list): logo, the add-to-home-screen steps, an Android copy swap, and a `/diag` funnel line on view.

Because iOS installs the page being viewed, it self-redirects to `/` if ever launched standalone.

The detection and redirect live in `/shell`'s loader: phone-and-not-standalone (and no `?browser=1` dev bypass) → `location.replace('install.html')`, checked before the whoami/login step.
