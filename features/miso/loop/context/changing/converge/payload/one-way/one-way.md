# one-way
*the bridge complains when a page write is about to be thrown away*

> (transcripts/2026-08-21-hybrid.md#p56)
> let's fix all residuals next.

## user

For agents writing fragments. The keys the bridge publishes into the loop state
— `open_tool`, `update_policy`, `asks` and the rest — are written BY the context
and never read back. A fragment that assigns one is writing into the sea: the
next paint puts the context's value back. That used to happen in silence. Now
the console says so, once per key:

> miso: the page wrote state["open_tool"] and the context overwrote it. That key
> is published BY the context and never read back — send a CtxOp instead.

## spec

Rung 7a's bridge is one-way by design: `Context::republish` writes a resolved
value into the loop payload at a declared legacy key, so fragments that predate
the context keep reading what they always read. Nothing reads those keys back
into the context, so a page-side write to one survives exactly until the next
republish. The failure is silent, and silence is what makes it expensive.

**The bridged key set costs nothing to learn.** `republish` writes its keys
unconditionally, so republishing into an empty object yields both the list of
keys and the values the context would publish this turn. No linker change, and
no second declaration to keep in step with the first.

**The comparison is against what we published, not against the context.** A key
is reported lost when the page's copy differs from the value this mechanism put
there last time. Comparing against the context's current value instead would
accuse the context of every change it legitimately made. The last-published
value is remembered per user and per key, because the server composes this code
too and one process republishes for everybody.

**The complaint travels as data and is printed by the page**, because the place
that can print it is the page: the payload carries `_bridge_lost` when there is
something to say and does not carry it otherwise, so a warning never outlives
what caused it.

**Known noise, named rather than discovered later.** `/seamless` restores a
whole state object across an update, which is a page-side write of every bridged
key at once; this will complain about each of them, once, after a seamless
upgrade. The complaint is technically true — those restored values are indeed
overwritten by the context, which is the correct outcome — so it is left loud
rather than special-cased.

## glossary

- **bridged key**: a page-state key a var declares with `js:`, published by
  `Context::republish`.

## code description

`one-way.rs` extends `ctx_republish` (line 10): probe the bridged keys, compare,
republish through the chain beneath, and attach the complaint.

`bridge_lost` (line 23) is the comparison, and answers the keys whose page copy
has drifted from what was last published into it.

`bridge_shadow` (line 42) answers the previous published value for one key and
remembers the new one in the same call, seated by user.

`bridge_announce` (line 58) attaches `_bridge_lost` when there is something to
say and removes it when there is not.

`one-way.index.js` prints it: once per key per load, at the outermost link of
the paint it can reach.
