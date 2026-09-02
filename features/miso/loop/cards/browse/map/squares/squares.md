# squares
*the map's squares when the signal is poor: what stands in for a missing one, and what is kept ahead of time*

> (transcripts/2026-09-02-self-check.md#p49)
> ok do the fallback first, then the pre-load

## user

Nothing to do. These two are the map's manners when your phone has no signal: a square it never fetched is drawn from the one above it, blurry but there, and the district was quietly kept in the phone while you had signal so most squares are there anyway.

## spec

A grouping node, made when `map` reached its seventh child (`/recentre`, 2026-09-02) and the tree's cap of six asked for a regroup. Its two children are the two halves of one answer to "what does the map do with no signal": `/stand-in` (a missing square draws its parent, three levels of reach) and `/stocked` (the project's area at zooms 12–16 fetched into the service worker's cache while online). Regrouping rewires nothing: composition order is provenance order, and both children keep their citations; the linked site before and after this move is the same set of fragments in the same order.

## glossary

- **square**: one map tile, `tiles/{z}/{x}/{y}.png`.

## code description

None of its own: a grouping.
