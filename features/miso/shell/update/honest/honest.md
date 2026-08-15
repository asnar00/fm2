# honest
*a failed check never claims "up to date"*

> (transcripts/2026-08-13-fm-spec.md#p59)
> actually, I had to restart and now I see "19" in the corner - just. Should it have indicated to me that there's a new buid available somehow? I tapped it and it's only showing 19 as available.

## spec

An offline launch once served the cached build and displayed the unreachable version check as "up to date" — absence of evidence shown as the hoped-for answer. Rule: a failed check surfaces as uncertainty. The `/panel` re-checks live every time it opens and says "can't reach the server" when it can't; a launch whose check failed retries every 5 seconds until an answer lands.

## user

If the panel says "can't reach the server", that's the truth — you may or may not be current, and it will resolve itself as soon as the network returns.

## glossary

(no new terms)

## code description

This node owns `honest.js`: `feature_Honest.retry()` (the every-5s launch-check retry until an answer lands) and `statusText(live)` — the panel's "up to date" vs "can't reach the server" wording. Untick it and failed checks simply say nothing.
