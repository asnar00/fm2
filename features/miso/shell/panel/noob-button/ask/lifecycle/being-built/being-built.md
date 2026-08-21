# being-built
*the list's missing verse: features under construction, live from the builder's bench*

> (transcripts/2026-08-15-fm-spec.md#p38)
> can we add the thing that shows the "being built" list of features? same format as the "list of features in next update". Then when they're done, they appear in the next-update list

## user

When the builder takes your request on, it moves into a **being
built** list in the panel — same shape as the update list, live while
you watch. When it's done it vanishes from there and reappears
moments later as a real feature in the next update, waiting for your
OK.

## spec

Between "proposed" and "in the next update" a request disappears into
the builder's silence; this node fills that verse. Asks with
`status: "building"` render as their own section — directly
below the awaiting block, above the plain requests, headerless and
title-only with the description a tap away (#p39), the amber pill
saying everything the old header said — and when a build ships, the stamp flips to
`shipped`, the entry leaves this section, and the feature itself
walks into the awaiting-update list by the standing machinery.

The half that makes it live: the builder's stamps ride the server's
own broadcast. `tools/stamp_ask.py` (scaffolding) updates the ask's
status in the mini's var store **and appends a `VarUpdate` for that
user's `asks` var to the broadcast file** — the same file the server's
`publish` writes and every client long-polls — so an open panel sees
"proposed" become "being built" within the poll's beat, no relaunch.
(Named risk, accepted: the stamp tool and the server both write the
broadcast file; single-writer in practice, a real lock if it ever
isn't.)

## glossary

- **being built**: an ask the builder has taken on — stamped
  `building`, displayed until the ship flips it to `shipped`.

## code description

`being-built.index.js` wraps `feature_Lifecycle.render`: after the
original draws the requests, it renders `#building` — one
`.crow`-grammar row per ask with `status: "building"`, header **being
built** — placed after `#awaiting` (before `#requests`), removed when
empty. The live arrival is free: `/scope` writes the broadcast
`VarUpdate` into state, `/lifecycle`'s apply-wrap re-renders, this
wrap rides it.

`tools/stamp_ask.py --text <substring> --status building|shipped`
edits every matching entry in the mini's `user.*.asks.json`, bumps
`/tmp/miso-broadcast.json` with the per-user `VarUpdate`, and prints
what it did. `--local` targets the dev store instead of the mini.
