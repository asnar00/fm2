# invite-tool
*inviting is a tool in the toolbar: a person with a plus*

> (asks#1787667557161)
> Let's make "invite someone" a tool in the toolbar - the icon can be a person silhouette with a "plus"
> *(filed from the field on 2026-08-25 by ash, birthplace `👤 @ miso/shell/panel/account`)*

## user

If you may invite people, the toolbar has a person-with-a-plus button. Tap it: the name and phone boxes, **invite**, and the list of people you have invited. Your own card no longer carries the invite rows.

## spec

`/invite` put its rows under the 👤 card and `/invite-someone` folded them behind a pill. Ash asked for inviting to be a tool of its own in the toolbar, with a person-and-plus icon. One reading, so it builds.

Three moves. The `tools_list` chain gains `{id: "invite", icon: 👤}` — but only when the fetched `invite` state says `may`, so a member's toolbar never shows it; the page half asks `users/invited` once at load so the answer is there before any page opens. The `render` chain draws, when `open_tool` is `invite`, a card-shaped page holding `/invite`'s rows — `invite_rows_html`, extracted from `/invite`'s `me_under` for this node with its behaviour intact. And `me_under` is redefined to nothing, so the rows leave the card (this drops any other filler of that seam too; `/invite` is its only filler today, and a second one should get its own tool or its own place).

On the tool's page the form is the page: `/invite-someone`'s pill is hidden and its fold released, so the boxes are simply there. The plus is drawn by CSS beside the emoji rather than being a second glyph, so it stays monochrome and inverts with the selected state like the rest of the toolbar. Untick and the rows return under the card, folded as before.

## hostile cases

- A member: no tool, and `/invite`'s server-side `may:false` still guards every route.
- The fetch fails at load: no tool until `/invite`'s next pull (opening 👤) succeeds; nothing thrown.
- The tool opened before the pull answered: an empty page for a beat, then the rows arrive with the state.

## glossary

(no new terms)

## code description

`invite-tool.rs` extends `tools_list` (gated on `invite.may`), redefines `me_under` to empty, and extends `render` with the `invite` tool's page built from `invite_rows_html`.

`invite-tool.js` pulls `users/invited` once at load through `feature_Invite.pull`.

`invite-tool.css` — the `+` badge on `[data-ev="tool_invite"]`, and the page rules that unfold the form.
