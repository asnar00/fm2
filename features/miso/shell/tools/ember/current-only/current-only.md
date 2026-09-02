# current-only
*a toolset shows its own icon on the left, not its parent's*

> (transcripts/2026-09-02-self-check.md#p72)
> when the add users tool is active, we should only see the "add users" button on the left, and not also its parent ("users") i.e. we should only show the icon for current toolset we're in, not the parent chain.

## user

Inside add-person (👤 → the plus), the row's left is ‹ and the plus alone — no 👤 beside it. On 👤's own page the row is as it was: 👤, then the plus. The ‹ is the way out.

## spec

`/under-account` made inviting a sub-tool of 👤 and, on the invite page, put a 👤 button in the row as the way back to the card, followed by the plus, selected. Ash asked for the current toolset's icon alone: not the parent chain (#p72). One reading, so it builds.

**Which buttons are the parent chain.** The row has two kinds of tool button: the ones `/tools` draws from the registry (`tools_list` — the launcher's tools, and in open mode the open one, selected) and the ones a tool's `tool_controls` link adds, which since `/under-account` include buttons that open *other* tools (`tool_<id>`). When the open tool is not in the registry — a nested one — a `tool_<id>` control naming a registry tool can only be the way back up: a sub-tool's own button is not a registry tool, and the open tool's is the one kept. So this node, the newest link on `tool_controls`, drops every `tool_<id>` control whose `<id>` is a registry tool other than the open one, and only while the open tool is nested. On a registry tool's own page nothing is touched: 👤's row keeps its plus, posts keeps its + and bin, and a tool that has not yet met a sub-tool never sees this link do anything. `/under-account`'s plus stays selected and first among the controls, so the invite row reads ‹, plus, and whatever the page adds.

**Generic by construction.** Any future toolset opened from inside a registry tool gets the same row: its own icon, not its parent's. A toolset nested two deep (a sub-tool of a sub-tool) is not a case the composition has, and this rule only knows the registry as the chain's top: the middle tool's button would stay, and a node for that day extends the rule with the chain it then has.

**The ways back.** Tapping the plus (the open tool's own button) and the ‹ both fire what they fired before this node; neither is changed here. Both go to the launcher today — `/tools` sends `tools_home` home, and a tap on the open tool's own button closes it — so the card is two taps away from the invite page rather than the one `/under-account`'s 👤 gave (the brief's "‹ goes back to 👤" is not what the tree does; named in the risks rather than built, since the ‹ is `/back`'s and the ask is about the icon).

Untick and 👤 is back in the invite row.

## hostile cases

- The registry is empty (every tool unticked): nothing is nested, nothing is dropped.
- A member who may not invite: the plus never appears under 👤, the invite page is unreachable, and the rule has nothing to do.
- The row's HTML carries a `tool_<id>` control with a style attribute or a tint (`/plus-tinted` rewrites the plus's tag): the drop reads the `data-ev` marker and the element's own open and close, so a tag's attributes do not matter; the buttons hold a span and no nested div.
- A card page under 👤 (`browse_open` set): 👤 is the open tool and in the registry, so the row is as today.

## glossary

(no new terms)

## code description

`current-only.rs`, `tool_controls()` /extension/: calls `existing`, reads the open tool and the registry (`tools_list`), and when the open tool is not a registry tool removes each registry tool's `tool_<id>` control from the row.

`current-only.rs`, `current_only_strip()`: removes every `<div …data-ev="<marker>"…>…</div>` from a row, one element at a time, the way `/plus-at-home` removes the + on a card page.

## risks

**The card is two taps from the invite page.** ‹ and the plus both go to the launcher; the one-tap 👤 is what this ask removed. If ash wants ‹ to climb one level instead of leaving, that is a `/back` child, not this node.

**A two-deep toolset keeps its middle button.** Named above; not a shape the tree has.
