# whole-number
*a sender is a whole phone number, and the relay addresses them by an opaque id*

> (transcripts/2026-08-21-hybrid.md#p32)
> just keep going until all rungs are built and working - test as you go

## user

Nothing to do, and nothing to see. Two guests whose phone numbers happen to end
in the same four digits used to be one person to parts of the server: one could
hear the other's settings change on their own phone, and could read or overwrite
the other's mirrored recordings. They are now two people. Your recordings and
your choices are yours; another person's are theirs, however their number ends.

Your own devices are unaffected — they are all *you*, and they still find each
other and everything you have recorded.

## spec

`comms/messaging` answered "who is this request from?" with `tag(phone)` — the
last four digits, chosen when the answer was only ever printed in a log. Three
callers then used it as an isolation key: `messaging` addresses the broadcast
relay `user.<sender>`, `converge` and `overlay` publish a user's context updates
to that audience, and `dictate/mirror` names a blob directory after it. A
four-digit key collides, and the collision was reproduced end to end on the real
UI (notes.md, "the four-digit tag collision"): person B's feature list applied
person A's untick.

This node redefines `sender_of` to return the whole number, spelled
`phone:+44…` — the same string `loop/context/per-user` derives for the context
table, which never collided because it never truncated. One person, one key,
everywhere on the server. Every caller is untouched; the identity they receive
simply became unambiguous.

**The relay is addressed by a token, not by the identity.** The broadcast slot
is a file (`/tmp/miso-broadcast.json`) that outlives the request and is shared
by every process on the machine, so writing phone numbers into it would trade a
collision bug for a disclosure. `sender_audience` derives a per-user id —
HMAC-SHA256 of the identity under the existing session-signing secret,
truncated to 128 bits — and the two ends of the relay translate through it:
`publish` rewrites the audience it is given, `wait_filter` rewrites the listener
it is given. Callers keep writing `user.<identity>` and never learn there is a
token. The token is unguessable (an unsalted hash of a phone number is a
seconds-long search) and collision-free at 128 bits.

Durable state keeps the identity, not the token: `mirror`'s directories and the
context table survive the loss of the signing secret, which is an event that
already logs everyone out. The token appears only in the relay buffer, whose
entries live for fifty writes, so a lost secret costs at most one relay round —
the authority is the context table and a rejoining device is answered by
`converge/parity`.

**No sender identity reaches another user's client.** `_from` is stamped on the
inbound message server-side and is never echoed: the published records carry
`{path, name, value}` (context) or a recording's metadata (mirror), and
`wait_filter` returns `e["msg"]` only — the `aud` field never leaves the server.
That is a property to keep: a payload delivered to a different user must never
name its sender.

## glossary

- **identity**: the whole-number key for a person, `phone:+<digits>` — what
  `sender_of` returns and what `/per-user` keys the context table by.
- **audience token**: the opaque per-user id the relay is addressed by, derived
  from the identity under the signing `/secret`.

## code description

`whole-number.rs` redefines four functions and adds two.

`sender_of` (line 8) replaces `comms/messaging`'s definition. Same shape — an
invalid or absent cookie still answers with the empty string, which is what
localhost tooling and the whole client place see — and the valid case now
returns `phone:` plus `token_phone`, the full signed number.

`sender_audience` (line 22) is the identity's opaque id: `hmac_sha256` under
`secret()`, the first 32 hex characters.

`opaque_audience` (line 31) translates one audience string, leaving `global` and
anything else that does not start `user.` alone.

`publish` (line 43) and `wait_filter` (line 47) are the two ends of the relay.
The writer's audience and the reader's identity both pass through
`sender_audience`, so they meet on the same token; an empty listener stays empty
and hears only `global`, as before.

Untick this node and `sender_of` falls back to `messaging`'s four-digit tag and
the relay is addressed in clear text again — the pre-fix behaviour, entire.
