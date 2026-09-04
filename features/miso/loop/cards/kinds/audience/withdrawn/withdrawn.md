# withdrawn
*a post that rises out of your reach leaves your phone*

> (transcripts/2026-09-04-field-walk.md#p113)
> another bug: the post with text = "you" (date 4 sep) shows "visible to the project's admins only" in my phone, but on "Tara"'s phone, I see that post, even though "Tara" is a candidate.

## user

Promote a post and it reaches further. Undo that promote — or otherwise put
the audience back up — and it stops reaching: the people it had gone out to
who are no longer at or above its floor lose their copy on the same tap, the
way a deleted post goes. It comes back if you promote it again. Your own copy
is never touched: this is about who else is holding one.

## spec

Ash saw a post of his own that says *visible to the project's admins only*
sitting on Tara's phone, and Tara is a candidate (#p113). The world logs said
what happened: at 18:15:03 the floor of `asnaroo.1788512223669` walked
candidate → team → volunteer → supporter → public and then back up again, one
millisecond apart — five promotes and five undos. Each promote handed Tara a
copy at the lower floor through `/audience`'s gate, correctly. Each undo
raised the floor, and the gate simply **did not send** the raised card. Her
copy stopped at `candidate` and stayed.

`audience.md` says the floor "only goes one way", and while that was true of
promote it was never true of the *world*: `/undo` raises it, and a raise that
does not withdraw leaves copies behind at every rung it passed through.

**A refusal is not a withdrawal.** `exchange_give` is the one door into
another world, and `/audience`'s link on it drops a card the recipient may not
hold. Dropping is right for a card they never had and wrong for one they are
holding: nothing else in the system will ever revisit that copy. So this node
takes the same door, outside that gate, and for a card the gate is about to
refuse **that the recipient already holds** hands them a **tombstone** rather
than nothing — `/delete`'s shape (`deleted` and `edited` stamped, one empty
title block, no links), which `/guard` names as the only write that removes
anything. Everything else `/exchange` and `/audience` do is untouched.

**The stamp has to win.** `/guard` merges a cards set per id and keeps the
newer `edited`, so the tombstone is stamped one past the copy it replaces
unless the arriving card is already newer. An undone promote arrives newer by
itself — `/guard/revert` restamps a reverted list to one past the world's
newest — but the log shows promotes and undos landing in the same millisecond,
and a tie decided by clock luck is not a mechanism.

**The way back is the same rule read the other way.** Promote it again and the
gate lets the card through; `/guard` takes a card entire, so a live copy
newer than the tombstone replaces it, `deleted` and all, and the post is on
their phone again. That works today except in the one case the stamp
paragraph names — a promote in the same millisecond as the withdrawal it
undoes would tie and lose — so a card the recipient holds **as a tombstone**
is given one past that stone. This is `/revert`'s own trick: a deliberate
write is a new edit in time and must say so.

**Out of scope, named.** A person *removed* from a project, or moved down a
grade, keeps the copies they were given: no card is written, so no give is
triggered, and nothing walks their world. The repair below fixes the data as
it stands today; the mechanism for it is a role edit that walks that person's
held copies, which is `/projects`' road and not this one.

**The repair.** `tools/withdraw_copies.py` (scaffolding, the shape of
`prune_posts.py`): for every world, every copy whose floor stands above the
holder's grade in that project becomes a tombstone through the op door, so
the copies handed out before this node existed leave too. Dry by default,
`--go` to write, `--world` for one world, `--port` for a rig.

## hostile cases

- **A holder who never had it.** Nothing is sent — a tombstone for a card
  nobody holds would be a card, arriving, that says it is gone.
- **A holder whose copy is already a tombstone.** Left alone.
- **The floor lowered again later.** The card is re-given live and replaces
  the stone; if the two stamps would tie, the live one is given one past it.
- **A promote and an undo inside one message.** Each cards write is its own
  give: the promote hands the copy over, the undo hands the stone. The last
  write of the turn is what the world is left holding, in order.
- **A card with no `in` link, or one the gate allows.** Passes untouched; the
  only cards this node adds anything for are those `/audience` refuses.
- **The recipient holds a different owner's card at that id.** `/exchange`'s
  own `exchange_not_theirs` refuses it as it always did — the tombstone keeps
  the held copy's owner, so it can never be that card.
- **This node unticked.** The gate refuses as before and copies stay behind:
  the bug, exactly.

## glossary

- **withdraw**: to take back a copy already given, by handing its holder a
  tombstone — the only write that removes.

## code description

`withdrawn.rs`, `exchange_give()` /extension/: reads the recipient's cards
once through `/exchange`'s `exchange_cards_of`, passes every card on to
`existing`, and appends one tombstone for each card `/audience`'s gate will
refuse that the recipient holds live. `audience_may_hold` and `audience_in_of`
are `/audience`'s own — the gate is asked, not restated.

`withdrawn_held` finds the recipient's copy by id (a copy keeps the owner's
id). `withdrawn_stamp` is the arriving card's `edited` or one past the held
copy's, whichever is later. `withdrawn_revive` gives a card the recipient
holds as a tombstone one past that stone, and leaves every other card alone.
The tombstone itself is `/delete`'s `delete_tombstone`.

`tools/withdraw_copies.py` (scaffolding) is the one-off repair for the copies
given before this node existed: it reads each world through
`POST /diag/context`, works out each copy's project and the holder's grade in
it by `/audience`'s rules, and writes the tombstones back through the same
door. Dry by default; `--go` writes.

## risks

**A grade demotion still leaves copies.** Named above: the withdrawal rides a
card write, and a role edit is not one.

**The repair reads the guest list to name a world's holder.** A world key is
a phone and the grade is keyed by name, so the repair maps one to the other
through `~/.miso-auth/users.json`, as the server's `exchange_name_of` does. A
world whose phone is not on the guest list is skipped and said so.
