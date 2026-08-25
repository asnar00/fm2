# stay
*dismissing the nøøb panel leaves you where you were*

> (asks#1787666840922)
> dismissing the nøøb panel should keep us in our current tool page, rather than dumping us back to home screen
> *(filed from the field on 2026-08-25 by ash)*

## user

Open the nøøb panel from your card, tap the shade to put it away, and you are still on your card. Nothing sends you home.

## spec

`/account` wraps the panel's close so that a shade-tap, while the account tool is open, also leaves the tool — correct when the panel *was* 👤's surface (toolbar state must not lie), and wrong since `/me` gave 👤 a page of its own: open the panel from the card, dismiss it, and the tool closed under you. Ash filed it from the field (`asks#1787666840922`). One reading, so it builds.

The dismissal became a seam on `/account` for this node — `feature_Account.dismissed()`, default behaviour unchanged — and this node replaces it with nothing. It is a child of `/me` because it is only right while `/me` is on: with `/me` unticked this node leaves too, and the panel is 👤's sheet again with the old rule intact.

## hostile cases

- `/account` unticked: no `feature_Account`, nothing to replace, nothing thrown.
- The panel opened from another tool (taps, dictate): never affected either way — the rule only ever fired for `account`.

## glossary

(no new terms)

## code description

`stay.js` sets `feature_Account.dismissed` to a no-op at load; `/account`'s close wrap calls it by name at dismissal time, so the replacement holds.
