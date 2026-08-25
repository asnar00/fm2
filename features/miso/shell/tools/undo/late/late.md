# late
*undo sees every tool's edits, including tools newer than itself*

> (transcripts/2026-08-25-accounts.md#p50)
> always fix residuals before calling a job done
> *(the residual: the undo button stayed dim after a card edit — noticed in review, then proven on a rig: the edit reached the server, undo recorded nothing)*

## user

Undo works in your card: change your name or your mission, and the undo button lights; tap it and the change is put back.

## spec

`/undo` promised a working undo for "tools that do not exist yet" and kept it by reading this turn's ops off the outbox rather than knowing what a tool is. It read them at its **own link** of the update chain — and provenance puts every newer node's link *outside* an older one's. `/cards` (2026-08-25) is newer than `/undo` (2026-08-21): cards' `update` calls `existing` first (running undo's scan, which finds nothing yet) and only then writes its op. So no card edit was ever recorded, and no tool written after Aug 21 would have been. Proven on a rig: the title edit reached the server; the button stayed dim; undo did nothing.

This node moves the *scan* to the end of the chain and leaves the *snapshot* where it is. `undo_record` is redefined: on the ordinary call from `/undo`'s link it stashes what it was given (the pre-event snapshot, the outbox mark, the open tool) and returns; this node's own `update` link — the newest, so the outermost — runs after every inner link has written, takes the stash, sets a flag, and calls `undo_record` again, which now passes through to `/undo`'s original. One step per turn, complete. The undo click itself still happens inside `/undo`'s link, so its inverse ops are queued before the late scan and recorded as before — pressing undo twice still redoes.

**The general lesson, for the ledger:** any node that observes "what this turn did" must observe at the outermost link, because newer nodes act after it; the snapshot-at-my-link / record-at-the-end split is the pattern.

## hostile cases

- A turn with no tool open: `/undo` never calls `undo_record`, nothing stashed, nothing recorded.
- A turn where `/undo`'s scan would already have found ops (taps): the late scan finds the same ops once — one step, not two, because only the late pass records.
- A newer node than this one appears: its writes land after this link's scan. The pattern holds only while this is the outermost `update` link; a node newer than `/late` that writes ops must itself be older-than-the-scan — i.e. this node must be re-ordered outermost, or the scan moved to `/turn-end`. Recorded as the known edge; today nothing newer writes ops.

## glossary

(no new terms)

## code description

`late.lib.rs` — two statics: the stash and the pass flag.

`late.rs` — `undo_record` stashes or passes through by the flag; `update` calls `existing` then records from the stash.
