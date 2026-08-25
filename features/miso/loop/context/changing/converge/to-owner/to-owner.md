# to-owner
*a relayed edit reaches the world's owner, not the edit's signer*

> (transcripts/2026-08-23-plans.md#p17)
> yeah - the panel will be open, but could also be closed. In the former case it should update, in the latter case, the noob-button should flash gently as it does now to indicate an update is available. That should be the general rule for all messages / alerts sent to the user.

> (transcripts/2026-08-23-plans.md#p18)
> add to that rule: if the app is active / foreground, just flash the icon; if it's backgrounded, use a notification.

## user

A change the builder makes to your world now arrives while you are
looking at it. Before this, an edit stamped from the bench — a
lifecycle status, a did-you-mean question — only appeared the next
time the page loaded, because the live relay was addressed to whoever
*signed* the edit and the bench signs nothing. It is addressed to
whoever the world *belongs to* now, so the panel updates in place.

## spec

`/converge` relays every applied op to the editor's other instances so
a phone and a laptop agree within a beat. It chose that audience from
the op's `_from` — the cookie-proven sender `/messaging` stamps — which
is right for the case it was written for and wrong for every other one:
an edit made through `POST /diag/context` on localhost has no session,
so `_from` is empty and the relay went nowhere. The question the
builder asked walked into a panel that would not learn about it until
the next page load.

The audience is a property of the **world that changed**, not of the
hand that changed it. A request already runs under exactly one world key
(`/per-user`), and since `/whole-number` a person's identity on this
server *is* that key: `sender_of` answers `phone:<number>` and the
audience their own long-poll filters on is `user.<that key>`, opaque on
the wire and spelled identically at both ends. So the relay asks the
world who owns it and addresses them by the name they already answer to.

For a user editing their own world this is the same person the old rule
named, so nothing that worked before changes. Only a `phone:` world has
a listener: a `local:` world is tooling's own, `_global` is the shared
layer `/overlay` borrows this seat for, and the wasm place never sets a
world key at all. All three fall back to the sender rule beneath, which
for an unsigned edit means no relay — as before.

Unticking this node returns the relay to the sender's audience: your own
edits still cross your devices, and a bench edit is silent again until
the page reloads.

## glossary

- **relay audience**: the `/messaging` address a `/converge` update is
  published to — now the edited world's owner.
- **world key**: the `phone:<number>` or `local:<name>` string a request
  runs under (`/per-user`).

## code description

`to-owner.rs`, `ctx_relay_audience()` /extension/: answers the owner's
audience when there is one, and otherwise defers to the chain beneath,
which is `/converge`'s sender rule.

`ctx_owner_audience()`: reads the request's world key with
`context_user_now()` and answers `user.<key>` for a `phone:` key —
character for character what `/whole-number`'s `sender_of` produces for
that person, so both ends of the long poll spell the audience the same
way and the opaque translation applies to both. Any other key shape, and
the empty key the wasm place always has, answer with the empty string,
which is this node declining rather than deciding.

`/converge`'s own `handle_msg` gained the seam this extends: the
publish now asks `ctx_relay_audience()` instead of formatting the
sender inline, and the base definition of that function is the old
expression verbatim, so `/converge` alone behaves exactly as it did.
