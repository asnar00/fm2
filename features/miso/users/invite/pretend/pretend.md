# pretend
*an admin may invite pretend people*

> (transcripts/2026-08-25-accounts.md#p63)
> so shall we think about working through the invite workflow with "pretend tara"? actually, make it "pretend alice and bob"?

## user

As admin you can invite a pretend person — a name starting with `_`, like `_alice` — whose login codes go to the server log instead of a phone, so you can walk the whole invite flow through without a real number. Support users still cannot.

## spec

`/invite` refuses names starting with `_` so an inviter cannot mint a test user and read their codes off the mini. Ash wants to rehearse the invite workflow with pretend people (#p63), and the admin can read the mini's log anyway — the refusal protects nothing from them. `/invite` grew a seam for this node, `invite_name_ok(name, who)` (empty = fine, else the sentence; the default refuses `_`), and this node lets a caller of rank `admin` (≥ 3) through. Untick and no one may.

## glossary

- **pretend person**: a `_`-named guest whose codes are logged, not sent.

## code description

`pretend.rs` — `invite_name_ok` admits a `_` name when `authority_rank(who) >= 3`, else defers to `existing`.
