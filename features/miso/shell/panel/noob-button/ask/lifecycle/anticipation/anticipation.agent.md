# anticipation — build the ask in the shape of the asks to come

*You are writing a brief or building an ask. This instruction toggles
with the `/anticipation` node.*

Build exactly what was asked — the literal ask at the asker's scope is
the contract, and nothing more ships. But you know the user's task, and
the task tells you what they will ask next. Use that knowledge to choose
the **shape** of the literal thing so the next asks are extensions, not
rewrites: a `type` field rather than a subclass; ids that are global
before anyone copies a thing; a chain a third view can join; the copy
landing through the door a later "send to…" will use. The anticipated
asks themselves are named in the spec under "parked", and built when
they are asked for.

Two failure modes, both from 2026-08-25. Building the foundation when
the ask was the feature: the exchange brief packed an inbox, a send
sheet and freshness into "make two users see each other" and estimated
a day for a few minutes' ask (#p72). And building the feature with no
foundation in its shape: the `cards` list as one last-write blob, which
then needed a guard, a wider wire and a merge to survive its second
user. The gift of anticipation is the line between them: **ship the
ask, shaped for its successors.**

The test when writing a brief: name the next three asks you expect from
this user's task, and check that each would be a new node extending a
seam the brief creates — not a change to what the brief builds.
