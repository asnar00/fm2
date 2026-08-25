# under-account
*inviting is a sub-tool of 👤, not a tool of its own*

> (asks#1787668032781)
> invite tool should be a sub tool of user tool, not at top level
> *(filed from the field on 2026-08-25 by ash)*

## user

The toolbar no longer has an invite button of its own. Open 👤 and the person-with-a-plus sits in its control row; tap it for the invite page, whose control row shows 👤 to take you back to your card.

## spec

`/invite-tool` registered inviting as a top-level tool. Ash asked for it to be a sub-tool of the user tool. One reading, so it builds.

Two chains. `tools_list` drops the `invite` entry, so the launcher never shows it and — because `render_toolbar` only draws the open tool's icon from that list — an open invite page shows no icon of its own. `tool_controls` supplies what the row needs instead: with 👤 open (and the server having said you may invite) the person-with-a-plus, which fires `tool_invite` exactly as the old button did; with the invite page open, a 👤 button (`tool_account`, the way back to the card) followed by the plus, selected. Both go in front of `/undo`'s button, never after it: undo is the last button in every control row, and a newer node's links land after undo's by provenance, so keeping the invariant is the newcomer's job (`before_undo`; the first build appended and broke it — accounts #p36). The page itself, its fetch and its rows are `/invite-tool`'s unchanged. Untick and the top-level button returns.

## hostile cases

- A member: `may` is false, so 👤's row has no plus; the invite page's row is unreachable.
- The pull hasn't answered yet when 👤 opens: no plus for a beat; the next paint after the answer has it.

## glossary

- **sub-tool**: a button in an open tool's control row that opens a page of its own; the parent's icon in that page's row is the way back.

## code description

`under-account.rs` — `tools_list` filters out `invite`; `tool_controls` appends the plus under 👤 and the 👤-plus-selected pair under the invite page; `invite_sub_button` draws the plus, reusing `/invite-tool`'s badge by `data-ev`.
