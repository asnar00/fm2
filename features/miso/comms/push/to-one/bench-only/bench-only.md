# bench-only
*the one-person push is the builder's door: from the tunnel it does not exist*

> (transcripts/2026-09-04-field-walk.md#p199)
> in the feature request (ask) workflow, it's a bit confused - I make a request and it goes straight to building, but also pops up a suggestion. Let's drop the suggestion part. Instead, go to "asked", and if the feature exists already (concierge, i.e. you, determines), you send a text message ad-hoc explaining how they can use the UI to do it. I think that makes more sense.
> *(the road this closes is `/to-one`, built for that answer; the hole was named in the worker's own return and closed before the team carried phones — see the risk note in `notes.md`)*

## user

Nothing changes for anyone using miso. The builder can still answer your request and ring your phone; nobody else can ring anybody.

## spec

`/to-one` shipped with `pic/retrofit`'s screen — a caller through the
tunnel had only to be logged in. That is the right bar for a retrofit
that only rewrites the caller's own data and the wrong one for a door
that sends a notification **to a number of the caller's choosing, with
words of the caller's choosing**. Every canvasser with a session could
have rung anyone on the list, in miso's own voice. On a field day with
twenty phones that is not a theoretical hole.

The door is the bench's, so it answers the bench alone. The screen is
`r.tunnel`, the one `/diag/context` already trusts: `/serve` sets it
from the `cf-connecting-ip` header cloudflared adds, and `/loopback`
binds the listener to `127.0.0.1`, so a request that did **not** come
through the tunnel can only have been made on the box itself. A cookie
changes nothing in either direction — no session, however senior, opens
this, and no missing session closes it for `stamp_ask.py`.

A refused caller gets the base's own miss for a path with no route —
`text_response(404, "not found")`, byte for byte what `/serve`'s `route`
answers for any unknown path — rather than a 401 or a 403. From the
tunnel this road does not exist, and a probe learns nothing about
whether it is composed.

`tools/stamp_ask.py` is unaffected. Its `sh()` and `sh_soft()` run every
command through `ssh <MISO_HOST> …` unless `--local` is given, so the
`curl localhost:<port>/push/one` is executed **on the box**, the way
`tools/reset_user.py` runs its op-door calls — a loopback request with
no `cf-connecting-ip`, which is exactly what this node lets through.
`--local` is the same request made on the box directly.

## hostile cases

- **A logged-in team member, through the tunnel.** 404, whatever their
  authority. This is the case the node exists for.
- **`stamp_ask.py` run from a laptop with `MISO_HOST` set.** The command
  is re-run on the box over ssh, so it arrives on loopback and rings.
  Verified as the ssh'd `curl` the tool sends, not only read.
- **A caller on the box that sends `cf-connecting-ip` itself.** 404. The
  screen fails closed: something on the box wearing the tunnel's clothes
  is treated as the tunnel. Named so that a future local proxy that
  forwards the header knows why its pushes stopped.
- **A tunnel caller stripping the header.** Not available to them:
  cloudflared adds it on the way in, and the client cannot remove what
  the proxy appends. Sending one of their own only sets the flag, which
  is the safe direction.
- **This node unticked.** `/to-one`'s own screen returns — logged-in
  through the tunnel is enough again, and the hole with it. That is what
  the untick proves, and it is the reason this is a node rather than an
  edit: the two behaviours are separable and one of them is wrong.
- **`/loopback` unticked.** The kernel no longer backs `!r.tunnel`, and
  anything on the LAN could reach the port directly and be taken for the
  bench. The trust this node rests on is `/loopback`'s, named here as
  `/loopback`'s own spec names it for `/gate`.

## glossary

- **the bench**: the box the builder's tools run on — a request that did
  not come through the tunnel, which `/loopback` makes the same thing as
  a request made on the mini.

## code description

`bench-only.rs` extends `/to-one`'s `push_one_route`: a request with
`r.tunnel` set gets `text_response(404, "not found")`, and everything
else falls to `existing.push_one_route(r)` unchanged. Nothing else in
either chain moves, and `/to-one`'s own tunnel-and-cookie branch is left
where it is so that unticking this node restores it exactly.
