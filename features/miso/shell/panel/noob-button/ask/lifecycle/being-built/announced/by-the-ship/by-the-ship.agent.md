# by-the-ship — the deploy stamps, so announce with the node and never stamp a ship by hand

This replaces the second call in `/announced`'s instruction. Announcing is
still the first thing you do for a build asked for in conversation; closing it
is no longer something you do at all.

**At the start of a build, announce it with the node it will ship as.** The
brief already knows the placement — that is what triage decided first:

    MISO_HOST=microserver@185.96.221.52 python3 tools/stamp_ask.py \
      --announce "<the ask, in the asker's words>" --status building \
      --node browse/map-only/since

Write the path as a brief writes it — a tail is enough (`capture/video/flip`),
and everything under the node counts. A `--announce` with no `--node` still
works and warns: nothing will be able to close it, and every deploy from then
on will list it as stuck.

**At ship, do nothing.** `tools/deploy.sh` calls `tools/stamp_ship.py` once the
release is actually live, and it stamps `shipped (build N)`:

- every announcement whose `node` this release touched, and
- every ask whose `t` a commit subject cites as `asks#<t>` — so **cite the ask
  id in the subject of the commit that answers it**, which is already the rule
  (agents.md, "field asks are provenance too"). That citation is now what
  closes the ask.

Then it prints every announcement still `building` after a day that nothing can
close — no node, or a node that has left the tree. **Read that list at every
deploy**: a superseded announcement (the build happened under other words) is
exactly what appears there, and one hand-stamp clears it:

    python3 tools/stamp_ask.py --announce "<the original words>" --status shipped --build <N>

Never stamp a ship before the deploy has succeeded. A stamp against a build
that did not carry the work is how three announcements went out wrong on
2026-09-04 (misses.md, "the fast-forward that never happened").
