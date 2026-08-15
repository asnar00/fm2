# watch
*an open app notices deploys by itself*

> (transcripts/2026-08-13-fm-spec.md#p62)
> I wonder - could we push-notify (or poll) so the update would become visible as soon as you deployed, regardless of bringing the app back to foreground or not?

## spec

Mid-session update detection, within what a frozen-when-backgrounded iOS PWA allows: re-check the /deploy stamp/ on returning to foreground, on regaining connectivity (`online` event), and on a 60-second poll while visible. A newer build lights the `/panel` handle. Reaching a *closed* app is beyond any polling — that half of the request became `/push`, which announces deploys as notifications.

## user

Leave the app open through a deploy: the logo button starts pulsing within the minute. Backgrounded or closed, the notification from `/push` covers you instead.

## glossary

(no new terms)

## code description

This node owns `watch.js`: `feature_Watch.check()` (re-fetch the stamp, light the `#build` handle when newer) and its three triggers — `visibilitychange`, `online`, and the 60-second visible poll.
