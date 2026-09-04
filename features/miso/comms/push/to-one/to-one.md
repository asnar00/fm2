# to-one
*a notification for one person, on the same wire that rings the whole team*

> (transcripts/2026-09-04-field-walk.md#p199)
> in the feature request (ask) workflow, it's a bit confused - I make a request and it goes straight to building, but also pops up a suggestion. Let's drop the suggestion part. Instead, go to "asked", and if the feature exists already (concierge, i.e. you, determines), you send a text message ad-hoc explaining how they can use the UI to do it. I think that makes more sense.

## user

When the builder answers your request, your phone rings — yours only. Nobody else is told.

## spec

`/push` sends to everybody, because until #p199 every notification was
news for everybody: a deploy. Answering one person's request is news for
that person alone, so this node adds the one road that was missing —
the same VAPID and the same RFC 8291 encryption, one recipient.

`POST push/one` takes `{phone, title, body}` and rings every
subscription that person holds (a phone and a laptop are two lines, and
both ring). Numbers match on their digits alone, so `+44…` and `44…` are
one person — the comparison `tools/ask_ack.py` already makes against the
guest list. A 404 or 410 prunes the subscription exactly as `/push`'s
`send_all` prunes it, and is not counted as sent.

The door is screened the way `POST pic/retrofit` is: the bench on the
box may call it, and a caller arriving through the tunnel must be logged
in. **This is a builder's door, not a way for one phone to ring
another** — without the screen a phone could name any number and any
words. Nothing on a device calls it; `tools/stamp_ask.py` does, when it
writes an answer.

It lives under `/push` rather than beside its caller because it uses
`/push`'s wire directly (`send_push`, `subs_file`, `remove_sub`): a
child cannot compose without its parent, which is the honest shape for
a hard dependency. Untick it and the road is gone; `/push` is exactly
what it was.

## hostile cases

- **The person never enabled notifications.** No line matches; the
  answer comes back `{"ok":true,"sent":0}`. Not an error — the sheet
  holds the words, the ring was the courtesy.
- **The subscription has expired.** 404/410 prunes the line and does not
  count it, so a caller is told the truth about what rang.
- **A phone tries it through the tunnel.** 401 unless logged in; a
  logged-in caller can reach it, so the screen is the same one
  `pic/retrofit` carries and no stronger. Named as a limit, not
  a claim: this door trusts anyone with a session on the box.
- **An empty phone or body.** 400 — a silent no-op would look like a
  delivery.
- **`/only-news` composed.** It judges only bodies matching `updated to
  build N`; an answer's words do not, so it passes through untagged and
  rings as any non-update notification does.

## glossary

(the terms are `/push`'s: subscription, VAPID)

## code description

`to-one.rs` extends `/push`'s `route` chain with `push/one`.

`push_one_route` screens the request (POST; logged in if it came through
the tunnel), reads `phone`, `title` and `body`, builds the payload with
`serde_json::Value::String` so the words are escaped rather than
interpolated, and answers `{ok, sent}`.

`send_one(phone, payload)` walks `subs_file()`, matches the line's
fourth field on digits, sends through `/push`'s `send_push`, prunes on
404/410 through `remove_sub`, and returns how many rang.

`digits_only` is the comparison both sides use.
