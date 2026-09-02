# confined — a change inside one node needs no toggle proof

*You have built a node and are at agents.md step 4, the toggle proof.
This instruction toggles with the `/confined` node.*

The linker never reads an unticked node's files. So the composition
WITHOUT your node is decided entirely by the files OUTSIDE it — and if
you changed none of those, your change cannot alter it: the untick can
never observe what you did. For a new node that composition is
byte-for-byte the last release, built, smoke-gated and shipped;
unticking, relinking and re-ticking would only re-prove it.

**The pattern.** Run `python3 tools/fmlink.py miso --prove`. Your change
is *confined* when every changed file under `features/` and `products/`
lies inside one node (its descendants and its own `order.md` count as
inside) plus, at most, lines ADDED to that node's parent `order.md` —
the tick that admits it.
Confined: the toggle proof is implied; skip the untick and move on.
Tools, notes, transcripts and handover do not move a node's toggle and
are not counted (a linker change is the smoke gate's business).

**Not confined — prove it.** A parent refactored to open an extension
point, two nodes in one commit, a sibling unticked or reordered, a
product-level order.md override, anything outside a node: these are the
changes the proof was written for. Do the full step 4 — untick, relink,
confirm the code left and nothing else changed, re-tick — and record it
in the commit message as a trailer, `Toggle-proof: <what was proven>`.
deploy.sh runs the classifier over every commit since the last release
and refuses a non-confined commit without that trailer; `PROOF=skip`
overrides for a hotfix, and the commit must say so.

**Two premises, both checked by the gate.** Ship-as-built: the previous
commit was released (the gate checks from the last released sha, not from
a count). And your worktree's base is main's tip — a confined change on a
stale base passes the pattern and is still wrong; fast-forward first.

Nothing here weakens the ticked side: the flows you ran and the smoke
gate still prove the node ON. This only retires re-proving it OFF.
