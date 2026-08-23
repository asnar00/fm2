# attention
*how a message from the builder gets your attention: in place, a gentle flash, or a notification*

> (transcripts/2026-08-23-plans.md#p17)
> yeah - the panel will be open, but could also be closed. In the former case it should update, in the latter case, the noob-button should flash gently as it does now to indicate an update is available. That should be the general rule for all messages / alerts sent to the user.

> (transcripts/2026-08-23-plans.md#p18)
> add to that rule: if the app is active / foreground, just flash the icon; if it's backgrounded, use a notification.

> (transcripts/2026-08-23-plans.md#p19)
> well don't send notifications if there's nothing to notify about

## user

When the builder sends you something — a did-you-mean question, a
lifecycle stamp, a note beside a build — it reaches you the way that
suits where you are. Panel open: the row simply appears, no
interruption. App in front of you with the panel closed: the nøøb
lozenge pulses gently until you open the panel, and opening it stops
the pulse. App in the background: a notification, carrying the
question or the note in plain words.

Exactly one of the three happens. And nothing happens at all when
there is nothing to say: a stamp that changed nothing, or a change with
no words on it, is silent.

## spec

The three channels are one rule, and the rule is chosen by the app's
state at the moment the message lands.

**Panel open — in place.** `/lifecycle` already re-renders its sections
on every applied payload while the panel shows, and `/converge` already
relays an applied edit to the owner's instances (`/to-owner`). Together
those are the open-panel case, complete, with nothing added here. This
node's only job in that state is to stay quiet: a panel that just
re-rendered has already delivered the news, so it must not also be
flagged.

**Foreground, panel closed — the gentle flash.** The page watches the
`asks` value across applied payloads. A value that changed, was not
changed by this device, and arrived while the panel was shut puts a
pulse class on the nøøb lozenge — the same 1.6s ease as `/update`'s
`#build.update`, its own class and keyframes, because that class means
"a newer build is waiting" and must keep meaning only that. Opening the
panel clears it. Two arrivals in a row leave one pulse, not two:
the class is a state, not a queue.

**Whether this device did it is read from the outbox, not guessed.** An
edit minted on this device rides out as a `CtxOp` in the very payload
that changed the value; an edit arriving from elsewhere is applied by
assignment and mints no op. So the payload carries its own provenance,
and your own filed ask never flashes at you. The payload has to be read
as it arrives rather than out of `feature_Loop.state`, because
`/messaging` lifts `_send` out of the state as soon as it has it.

**A loading page is catching up, not being interrupted.** A page joins,
and then its long poll hands it everything it missed while it was gone
— the world arriving, and a backlog of relayed edits replayed from
`v = 0`. Every one of those is a change to the `asks` value that this
device did not make, and none of them is news. So the page is not
*awake* until a second after it joins: until then arrivals only set the
baseline. The clock starts at the join rather than at script load, so a
slow wasm fetch cannot eat the window, and a page that never joins
never wakes — it has no live connection to be interrupted by. The cost
is a stamp landing in that first second, which the panel shows on
opening anyway.

**Backgrounded — a notification.** The server tells the user's push
subscriptions when a builder edit lands on their `asks`. `/push` stores
the subscriber's phone on every subscription line already, and a world
key is `phone:<number>`, so the targeted send is the deploy
announcement's own walk with one match added — same encryption, same
VAPID, same expired-subscription cleanup. A user with no subscriptions
matches no line and nothing is sent.

**Nothing to notify about, nothing sent (#p19).** The push rides a real
change: the world is read on both sides of the applied op, and an op
that left the value identical — a repeated stamp — sends nothing. It
also rides real words: the body is the changed entry's question text,
else its note, and an entry that changed with neither says nothing at
all. A notification reading "miso" with an empty line under it is worse
than no notification.

**The foreground/background fork is made in the service worker,** which
is the only place that knows both. It wraps `showNotification`: with a
visible window client the payload is posted to that page — which
flashes, or has already updated in place — and nothing is displayed;
with none, it displays as before. The wrap rather than a second `push`
listener, because two listeners would both fire and the second could
not un-show the first's notification.

*Named risk, unproven on device:* a browser may insist that a push
which shows nothing shows *something*, and substitute a default
notification ("this site was updated in the background"). Chrome is
documented to do this after repeated silent pushes; iOS Safari's
tolerance is not knowable from a rig. If a device shows the default
card, the honest fallback is to stop suppressing and accept a
notification beside the flash — the wire is right either way. This is
labelled a hypothesis until a phone answers it.

**When it fails.** `clients.matchAll` rejecting means the fork cannot be
made; the worker rings, because a notification too many is a smaller
failure than an alert that never arrives. A push whose payload is not
JSON is already caught by `/push`'s own handler and becomes the default
title with no body; it does not crash the worker. Unticking this node
removes all three rungs: no pulse class, no fork (every push rings), no
targeted send — and `/to-owner`'s relay still reaches the open panel,
because that half is not this node's.

**Scope.** `asks` is the only attention-worthy var today. The rule is
the seam, not a registry: a second var joins by being named here, when
a second var wants it.

## glossary

- **attention rule**: panel open → update in place; foreground and
  closed → gentle flash; backgrounded → notification. One of three,
  never two.
- **the gentle flash**: a slow opacity pulse on the nøøb lozenge,
  cleared by opening the panel.
- **targeted push**: a web push addressed to one user's subscriptions
  rather than to everyone's.

## code description

`attention.index.js` wraps `feature_Loop.apply`: it parses the arriving
payload itself, lets the chain beneath apply it, then judges it in
`saw()` — the page is awake, the `asks` string changed, no `CtxOp` on
`asks` in the payload's `_send`, panel not open — and calls `flash()`.
`wake()` starts the one-second catch-up window on the first payload
carrying `_joined`, and until it closes every payload only records what
"already seen" means. `flash()` adds the `attention` class to `#build` and is idempotent;
`clear()` removes it and is wired to `feature_Panel.open`. A
`serviceWorker` `message` listener turns a forwarded push into the same
flash. Every cross-feature reference is typeof- or truthiness-guarded.

`attention.index.css` styles `#build.attention`: a warm muted colour and
`fm_attn`, a 1.6s ease-in-out opacity pulse — kin to `fm_pulse`, its own
name and its own meaning.

`attention.sw.js` replaces `self.registration.showNotification` with a
wrap that asks `clients.matchAll` for visible window clients: none, and
the original shows the notification; some, and each is `postMessage`d
`{fm: "attention", title, body}` while nothing is displayed. A rejected
`matchAll` falls back to showing.

`attention.rs`, `handle_msg()` /extension/: an unsigned `CtxOp` on
`/ask`'s `asks` var — the builder speaking to a user — is applied by the
chain beneath, with the world read through `/ask`'s `asks_read()` on
both sides of it; unchanged means no push, and `attention_news()` then
picks the changed entry's question text or note, an empty answer also
meaning no push. Everything else falls straight through.

`attention_push_to_user()`: walks `/push`'s `push-subs.txt`, matching
`phone:<field four>` against the world key, and sends the same encrypted
payload `send_push` builds for the deploy announcement, dropping
endpoints that answer 404 or 410. This node needs `/push` composed —
`subs_file`, `send_push`, `endpoint_origin` and `remove_sub` are its
functions — so unticking `/push` and leaving this on is a link failure,
which is the tree telling the truth about the dependency.
