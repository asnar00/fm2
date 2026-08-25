# invite-someone
*the form waits behind one button*

> (asks#1787665616577)
> let's just have a button saying "invite someone" that then opens a form with name and phone number

## user

Under your card there is one button: **invite someone**. Tap it and the name and phone boxes appear with **invite** beside them; send, and they fold away again. Tap the button a second time to fold them without sending.

## spec

`/invite` drew its two fields and its send word as a standing row, reasoning that the copy rule allowed "invite" and nothing else. Ash, from the phone, asked for the reveal (`asks#1787665616577`): one button, then the form. One reading, so it builds.

This node folds `/invite`'s `.invite-new` row behind a button row placed above it. The open flag is this node's own JS state, re-applied on every appearance of the rows (a `MutationObserver` on `#app`, the idiom `/invite` itself uses; never a wrapper on `feature_Loop.apply`). Opening focuses the name box when both fields are empty. A successful send — detected as both fields empty after `/invite`'s own `send` returns — folds the form; a second tap on the button folds it too.

Untick this node and the standing row returns exactly as `/invite` drew it.

## hostile cases

- A repaint while open: the observer re-applies `invite-open` and the button row; the fields' draft is `/invite`'s to restore, and it does.
- `/invite` refuses a send (duplicate, bad number): the fields keep their values, so the form stays open with the message under it.
- The card page is not on screen: nothing to apply, nothing thrown.

## glossary

(no new terms)

## code description

`invite-someone.js` — `apply` inserts the button row once per paint and toggles `invite-open` on the `.invite` box from the `open` flag; `toggle` flips it. A capture-phase click on `[data-invite="someone"]` toggles; a `MutationObserver` on `#app` re-applies; `feature_Invite.send` is wrapped at load to fold after a successful send.

`invite-someone.css` hides `.invite-new` unless `.invite-open`, and styles the button as the same quiet pill as `/invite`'s send word.
