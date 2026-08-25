# revert
*undo still works on cards, guard or no guard*

> (transcripts/2026-08-25-accounts.md#p50)
> always fix residuals before calling a job done
> *(the residual: with `/guard` on, an undo of a card edit was accepted locally and then overwritten by the server's reply — proven on the rig after `/late` made undo see card edits at all)*

## user

Undo in your card puts your words back and they stay put.

## spec

`/guard` merges every cards write by "newer `edited` wins per card". An undo restores the *prior* list, whose cards carry their older stamps — so the guard read the revert as a stale device, kept the newer card, and the reply relayed that back over the phone's revert. A deliberate revert is a new edit in time, and must say so. This node redefines `undo_apply`: for a change on `miso/loop/cards`/`cards`, every card in the prior list is restamped to one past the newest `edited` the world holds (no wall clock is needed in wasm, and "newer than anything the guard will compare against" is all the stamp must mean), then `existing.undo_apply` issues the inverse as before. Untick and the guard wins over undo again.

## glossary

(no new terms)

## code description

`revert.rs` — `undo_apply` calls `existing` with the restamped step; `cards_revert_restamp` rewrites the `prior` strings; `revert_stamp` is max `edited` + 1 over `cards_read()`.
