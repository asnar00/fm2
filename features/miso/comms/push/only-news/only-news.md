# only-news
*an update notice that is no longer news does not ring*

> (transcripts/2026-09-01-saturday.md#p33)
> notifications for old upgrades are popping up when you minimise the app - it's as if those notifications are suppressed when the app is foregrounded, but then pop up later on when minimised.

## user

Backgrounding the app after a busy deploy day rings at most one update
notice, and only if it is actually news. No pile of stale "updated to
build N" banners.

## spec

The report's reading of the mechanism is right, with the OS as the missing
actor: iOS holds push events for a foregrounded web app and hands them to
the service worker when it backgrounds. `/attention`'s fork then finds no
visible window and rings — correctly by its rule, but with whatever the OS
saved up: on a ten-deploy day, ten "updated to build N" notices, most
describing builds the phone has already taken.

This node judges update notices at the display point, the same joint
`/attention` took (`showNotification`, wrapped at load; composed after
`/attention`, so this wrapper is outermost — staleness is decided before
the visibility fork). An update notice is recognised by its own body
(`updated to build N…` — the format is `/push`'s one payload line; the
coupling is named here so a reword there knows to visit). Two judgements:

- **collapse**: every update notice carries the tag `miso-update`, so
  however many arrive, the phone shows one banner, the newest words.
- **drop stale**: the notice's build is compared with the server's current
  (`fetch('version')`, best-effort); a notice for an older build is
  dropped — the phone will meet the newer build through the ordinary
  update road, and ringing about a superseded deploy is noise. The fetch
  failing (offline) shows the notice: fail toward ringing, `/attention`'s
  own rule.

## hostile cases

- **Offline at display time.** The version fetch fails; the notice shows.
  A stale ring beats a swallowed fresh one.
- **The current build's own notice.** `N == current` is news; it rings.
- **A non-update notification** (a question from the builder). The body
  doesn't match; untouched, untagged, rings as today.
- **Dropping counts as a silent push on iOS** (the OS penalises handlers
  that show nothing). `/attention`'s visibility fork already takes this
  exposure routinely; the collapse means drops are rare — most late
  batches show their newest member.
- **This node unticked.** The pile returns — today's behaviour, no worse.

## glossary

- **update notice**: `/push`'s deploy notification, "updated to build N…".

## code description

`only-news.sw.js` wraps `self.registration.showNotification`: a body
matching the update-notice format gains `tag: 'miso-update'`, and its
build number is checked against a best-effort `fetch('version')` — older
than current resolves without showing; everything else passes down the
chain. The judge (`fm_onlyNewsJudge`) is a pure function of
`(body, current)` so a rig can test the decisions without a device.
