# adopt
*recordings stored under the old four-digit name are claimed by the first whole-number owner*

> (transcripts/2026-08-21-hybrid.md#p32)
> just keep going until all rungs are built and working - test as you go

## user

Your mirrored recordings are still there. They were filed under the last four
digits of your number and are now filed under you; the move happens by itself
the first time you touch a recording after the update, and you will not notice
it.

If another guest's number ends in the same four digits, that old store held both
of your recordings mixed together and nothing in it says whose is whose. It goes
to whoever asks for it first; the other person starts with an empty store, and
the server log says loudly that it happened, so a human can sort it out.

## spec

`/whole-number` changed what `sender_of` answers, and `/mirror` names a blob
directory after that answer. Every recording already on the server therefore
sits in `~/.miso-blobs/…1234/` — the four-digit name — and would be invisible to
its owner, who now asks for `~/.miso-blobs/phone:+441234/`. This node moves them.

**Migration is on first touch, not at boot.** The server cannot enumerate owners
at startup: a directory named `…1234` names a *tag*, and the mapping from tag to
person is exactly the ambiguity being repaired — the guest list may hold two
matches, or none. A request, on the other hand, arrives with a proven identity.
So both doors into the namespace adopt before they read or write: the `/blob`
route through `blob_user`, and the `RecShared`/`RecIndex` messages through the
stamped `_from`.

**The rename is the claim.** `std::fs::rename` is atomic, so two colliding
guests touching at the same moment produce one winner and one fresh empty store,
and after it there is no legacy directory for a third touch to find. A user who
already has a whole-number store does not adopt: their store is their own, and
the legacy one belongs to whoever claims it. That is the ruling this node
implements — first claimant takes it, the second starts clean — and it is chosen
because the alternative honest answers are worse: refusing to migrate strands
everyone's recordings, and splitting the store by guesswork invents ownership
the data does not record.

**The un-mixable case is loud.** An adoption always logs `BLOB MIGRATION`, names
the claiming guest (their name from the guest list, and their tag — never their
number), and says in as many words that a colliding guest's recordings may be
inside. A forfeit logs once per run. Neither is an error and neither fails a
request: the recordings are data, and losing a request over filing is worse than
filing late.

**What is not migrated, deliberately.** The relay's audience strings are
ephemeral — fifty entries in a shared file, rewritten continuously — so old
`user.…1234` entries simply stop matching any listener and age out within
seconds; the durable authority is the context table, which never used the tag.
Push subscriptions and the guest list were always keyed by the whole number.

## glossary

- **legacy store**: a blob directory named by the four-digit `/tag`, written by
  every build before this one.
- **adoption**: the atomic rename of a legacy store onto its claimant's
  whole-number `/identity`.

## code description

`adopt.rs` extends two of `/mirror`'s functions and adds the migration.

`blob_user` (line 7) and `handle_msg` (line 13) are the two doors. Each calls
`blob_adopt` with the identity it already has, then delegates unchanged, so
adoption precedes the first read of an index or a blob.

`blob_adopt` (line 26) does nothing unless there is a legacy directory: it
derives the tag from the identity, and returns immediately when the tag names
nothing on disk — the common case, one `exists` call. When the directory is
there and the claimant has no store of their own, the rename runs and announces;
when the claimant already has one, `announce_forfeit` says so and leaves it.

`whole_number_of` (line 50) recovers the number from a `phone:` identity, and
answers empty for `_local`, for an unauthenticated request, and for any key
shape that never had a four-digit form.

`announce_adoption` (line 59) and `announce_forfeit` (line 69) are the operator's
half. They print the guest's name and tag, never the number, and the adoption
line states the ambiguity outright rather than implying it.

`migration_announced` (line 81) keeps a per-run set of announced conditions, so
the forfeit case — which recurs on every touch for as long as the unclaimed
directory exists — is said once.

`guest_name` (line 90) is the guest list's name for a number, or "an unlisted
guest" for a session whose owner has since been removed from the list.

Untick this node and nothing migrates: recordings made before the change stay in
their four-digit directory, unreachable but intact, and the rest of `/mirror`
behaves exactly as it does with it on.
