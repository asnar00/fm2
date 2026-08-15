# live-panel
*news arrives while the panel is open — the panel notices*

> (transcripts/2026-08-15-fm-spec.md#p22)
> One tweak: if the noob panel is open when an upgrade arrives, it should automatically update its contents to reflect the new update.

## spec

The panel's update freight — the build line, the awaiting section, the
upgrade button — was rendered at open and then froze; a build arriving
mid-view stayed invisible until the next open. Now the two moments
news can land both refresh an open panel in place: `/watch` learning a
new server build (the poll, the visibility wake, the online wake), and
a quiet apply finishing (the build line should flip to up-to-date the
moment it happens). Refresh re-runs `/review`'s section — which
already re-fetches the live tree, re-renders or removes the awaiting
block, and re-dresses through `/upgrade`, whose session-local draft
ticks survive re-rendering by design — and `/less-busy`'s build line.
A closed panel costs nothing: the refresh only fires when the panel is
showing, and only when the server build actually changed.

## user

Leave the panel open and updates walk in as they happen: a new build
shows up as an awaiting entry the minute it ships, and the build line
flips to up-to-date on its own. No closing and reopening to see the
news.

## code description

`live-panel.index.js` wraps `feature_Watch.check`: after the original,
if the server build differs from the last one this node saw and the
panel is showing (`#panel`'s display is the panel's own open state),
`refresh()` runs — `feature_Review.section()` then
`feature_LessBusy.refresh()`, each typeof-guarded. It also wraps
`feature_Delta.quiet` the same way, so a no-reload apply flips the
build line and clears the awaiting block in place. The last-seen
server memo starts from the current value at load, so the wrap stays
silent until something actually changes.
