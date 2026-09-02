# aside
*undo shows only when there is something to undo, and stands alone at the far right*

> (transcripts/2026-09-02-self-check.md#p71)
> on all toolbars, the "undo" button should only be visible when there's something to undo; when it is visible, it should always appear as far to the right as possible, rather than grouped with the others.

## user

Open a tool and its control row has no undo. Change something — a tap, an edit, a delete — and the undo arrow appears at the far right edge of the row, on its own, apart from the other buttons. Undo the last thing and, with nothing left to undo in that tool, the arrow goes again. Every tool's row alike; the launcher never had one.

Undo no longer undoes itself: pressing it walks back through what you did, one step per press, and stops when there is nothing left. Getting a change back after undoing it is a redo, which is not built yet.

## spec

`/undo` drew its button in every open tool's control row unconditionally — shaded when the tool had no step — and put it last among the controls, which `/ember`'s layout centres as one group. Ash asked for two things (#p71): the button only when there is something to undo, and, when it shows, as far right as possible, apart from the others. One reading, so it builds.

**Visibility follows the stack.** This node is the newest link on `tool_controls`, so it runs after every node that inserts a control in front of undo (`before_undo` and its copies search for the button and still find it, because they run inside). With the open tool's stack empty — `undo_has`, `/undo`'s own test for the shaded state — the button element is removed from the row's HTML; otherwise the row is left as it was. The row is composed on every paint, so the button follows the stack turn by turn: an edit brings it, the undo that empties the stack takes it away, and the ten-deep bound evicting a tool's last step takes it away too.

**Position is one rule.** `/ember` anchors ‹ and the open tool's button left by giving the selected button an automatic right margin, and centres the controls by giving the last control one too. Undo is the last control in every row (`/glyphs`), so it carried that centring margin. This node gives the undo button an automatic *left* margin and no right one: the free space now splits between the tool button's right and undo's left, which puts the other controls in the middle of the space between the tool and undo, and undo against the right edge, with the split of the free space as the gap that sets it apart. With no other control the whole free space sits between the tool and undo. When a row is full — taps on a 390px phone has no free space at all — undo is still last, and the gap is whatever the squeeze leaves.

**A quiet turn files no step.** Two turns edit the world without the person having done anything to undo, and this node's `update` link, the outermost, marks each for its duration so its `undo_record` link answers without recording (`undo_quiet`, a seam a later node extends with its own event). The first is the undo press itself: `/undo` recorded the inverse it minted as a step like any other, so pressing undo twice redid — and the stack was never empty after a press. The ask says the button is visible only when there is something to undo, and its in-hand reading, confirmed in conversation, is that undoing the last thing leaves nothing; so the stack now walks back one step per press and empties, and the oscillation that stood in for redo is retired by the ask. Redo is a sibling button with the same rule, parked until asked for. The second is `CardEnsure`, the blank profile `/me` makes on a person's first 👤 open: found on the rig with a fresh user — the row showed undo the moment 👤 opened, nothing done, and its press would have taken the profile away (which `/guard` would then refuse at the server, leaving the two sides apart until the next join). Before this node that button was lit, not shaded, and nobody had noticed; the ask makes it a defect, so it is quiet here. The seam is this node's rather than `/me`'s because the rule — what a person can undo — is undo's to hold; the line naming `CardEnsure` is the one a `/me` child could take over.

**Reordering never reaches undo.** `/reorder` sorts the registry (`tools_list`) and its page half drags only the `tool_*` buttons; `ctx_undo` is neither, so a person's arrangement of the row cannot move it and it stays right of any arrangement.

Untick and today's row returns: undo always present, shaded when idle, grouped with the controls, and redoing itself.

## hostile cases

- A fresh user opens 👤 for the first time: `/me` writes their blank profile in that turn; the turn is quiet, so no step and no button. Proven with a rig user who had never opened 👤.
- The stack for this tool is empty and a stale tap reaches `ctx_undo` (a repaint racing a finger): `undo_take` finds nothing, nothing is applied, and the press turn records nothing — the row stays without the button.
- An inverse fails silently (`/undo`'s named silence — a var that left the composition): the step is spent, the world unchanged, and the button goes rather than staying shaded over nothing.
- The eleventh step evicts a tool's only step: that tool's button goes on its next paint; the tool that made the newer steps keeps its own.
- Another tool's step is on the stack: this tool shows no button — the filter is per tool, as `/undo` filed it.
- The row is full (taps at 50px on a 390px phone): undo is last with no visible gap; nothing overflows, because `/undo`'s squeeze guard still governs the widths.

## glossary

(no new terms)

## code description

`aside.rs`, `tool_controls()` /extension/: calls `existing` and, when the open tool has no step (`undo_has`), removes the undo button element from the row.

`aside.rs`, `undo_quiet()` /seam/: is this turn's edit not the person's to undo — the `ctx_undo` click, and `CardEnsure`. A later node that mints a value on the person's behalf extends it with its own event.

`aside.rs`, `update()` / `undo_record()` /extensions/: the first holds the quiet mark for the length of a quiet turn; the second returns without recording while the mark is set and passes through otherwise.

`aside.rs`, `aside_strip()`: removes one `<div …data-ev="ctx_undo"…>…</div>` from a row; the button holds no nested div (the arrow is an SVG), so the first close after the marker is its own.

`aside.lib.rs`: the turn's quiet mark, a static the loop's one thread sets and clears.

`aside.css`: the undo button's automatic left margin and zero right margin, stated twice so it beats `/ember`'s `:last-child` centring at equal specificity whatever order the sheets compose in.

## risks

**Redo is gone until it is built.** A person who undoes one step too many has no button to bring it back; before this node the same press brought it back. Named as parked in the brief; the ask's wording is explicit.

**The gap is not guaranteed on a full row.** Where the buttons already fill the width, undo is last but not visibly apart. The squeeze is `/undo`'s, not new.
