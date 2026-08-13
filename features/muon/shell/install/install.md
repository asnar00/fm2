# install
*phone browsers are directed to add muon to the home screen*

> (transcripts/2026-08-13-fm-spec.md#p49)
> so I think a good feature would be: if we detect we're running on a phone browser, we direct the user to add to home screen before we let it continue.

## spec

A phone browser visiting muon is sent to `install.html` — logo, platform-appropriate add-to-home-screen instructions (iOS share sheet / Android menu) — before anything else (including login). The installed PWA and desktop browsers proceed as normal. Detection: mobile user agent (incl. modern iPad's Mac-like UA via touch points) and not `display-mode: standalone`. The page carries the full head (manifest, apple-touch-icon) because it *is* the install point — and since iOS installs the current page's URL, it self-redirects to `/` if ever launched standalone. A small "continue in the browser" escape link appends `?browser=1`, which the shell honours for that visit.

## user

Visit muon.nøøb.org on a phone: you're shown how to add it to your home screen. Open the installed app and continue to login/app as usual. The dim link at the bottom lets you carry on in the browser anyway.

## glossary

- **install point**: the page being viewed when the user adds to home screen; iOS takes both the icon and the app URL from it.

## code description

`assets/install.html`: static public page (added to the gate's `is_public` shell list); standalone self-redirect (line ~35), Android copy swap, and a `/diag` funnel line on view. The detection + redirect lives in `/shell`'s loader: phone-and-not-standalone (and no `?browser=1`) → `location.replace('install.html')`, checked before the whoami/login step.
