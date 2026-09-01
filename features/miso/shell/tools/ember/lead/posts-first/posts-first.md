# posts-first
*the launcher leads with posts, people, reports, projects*

> (transcripts/2026-09-01-saturday.md#p32)
> looks good again. one change: let's reorder the top-level buttons thusly: posts, users, reports, projects

## user

The home toolbar reads, left to right: posts, 👤, reports, projects — then
everything else as before. Your own hold-and-drag arrangement still beats
the default.

## spec

`/lead` set the campaign default at projects, posts, people; a day of real
use re-ruled it: posts are the thing reached for most, people second,
reports third, and projects — set once, visited rarely — go last of the
led set. This node redefines `tools_list` the way `/lead` did, with the new
list (`posts`, `account`, `reports`, `projects`), composed after `/lead` so
it wins; `tools_order_chosen()` still yields to a person's own arrangement,
and a tool absent from the composition (reports for a member — its server
gate hides it) simply doesn't appear, exactly as `/lead` behaves.

## hostile cases

- **A member (no reports tool).** The id matches nothing; the row is posts,
  👤, projects, rest — no gap.
- **A person's own order stored.** `tools_order_chosen()` is true; theirs
  wins untouched.
- **This node unticked.** `/lead`'s order returns.

## glossary

(no new terms)

## code description

`posts-first.rs` redefines `tools_list(state)`: yields to a chosen order,
else re-sorts `existing.tools_list`'s array by the new lead list, everything
unlisted following in registration order.
