# patient
*your card is made only once your world has really arrived*

> (transcripts/2026-08-25-accounts.md#p48)
> let's make sure that never happens again - data loss is a sure way to kill user trust

## user

Opening 👤 while the app is still catching up with the server waits, quietly, until it has; if it cannot (offline, or the server is restarting), no card is made this time — yours is safe where it is.

## spec

`/me` waited for `/veil`'s `fm-joined` mark before sending `CardEnsure`, but `/veil` sets that mark on its two-second **timeout** as well as on a real join, so a page loading during a server restart proceeded on an empty world and made a blank card — the client half of the build-292 loss (`/guard` is the server half).

This node replaces `feature_Me.ready` and `feature_Me.ensure` at load (`/me`'s own idiom on `/account`): ready means `feature_Veil.joined` — the real join — and nothing else; the wait is up to a minute; and if the join has not come by then, the ensure does not happen — the tool shows "making your card…" and the next opening tries again. A card can always be made later; a card cannot be un-lost. With `/veil` unticked there is no join to wait for and the ensure goes at once, as before.

## hostile cases

- Server restarting during load: no ensure; the join lands seconds later; the next 👤 tap ensures against the real world and finds the card.
- Truly offline: no card until online. Previously a blank card appeared and then fought the real one.

## glossary

(no new terms)

## code description

`patient.js` — replaces `feature_Me.ready` (real join only) and `feature_Me.ensure` (600 × 100ms, then give up silently).
