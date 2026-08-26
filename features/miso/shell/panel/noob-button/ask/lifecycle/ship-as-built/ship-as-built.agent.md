# ship-as-built — a proven node deploys at once

*You have just proven a node — a field ask or a conversation ask — and
committed it. This instruction toggles with the `/ship-as-built` node.*

Deploy it now. Never hold a finished node for a batch, for a tidier
release, or for the next one to land: there is no way of knowing yet how
urgent it is to the person who asked, and a stamped-but-unshipped ask
reads on their phone as a queue. The loop is **build → prove → commit →
deploy → stamp shipped → next**, one node at a time.

Deploys serialise through one build directory and each runs the smoke
gate, so a deploy takes minutes and the next waits its turn — that is
fine; what is not fine is the queue growing because triage was busy. Two
rules keep it flowing: **never link or run a rig in the deploy's build
directory while a deploy runs** (deploy.md — the gate serves from it, and
a half-written site fails for the wrong reason); triage proves its own
direct work from a separate checkout. Workers already build in their
own worktrees.

If a deploy fails the gate, the next act is one clean deploy with
nothing else touching the build directory — not more building.
