# one-level
*‹ goes one level up the tree of tools, however deep*

> (transcripts/2026-09-02-self-check.md#p88)
> tools are arranged in a tree, effectively: users -> invite; the < button should go to the parent level no matter how "deep" we are.

## user

On the add-person page (👤 → the plus), ‹ takes you to your 👤 page. Tap ‹ again and you are at the toolbar. Open a post from the posts list and ‹ takes you back to the list, not out of posts. Every ‹ is exactly one level.

## spec

`/tools`' own instruction says it: the interface is a tree, ‹ goes to the parent level, always exactly one, however deep. `/back` drew the ‹ and gave it `tools_home` — leave the tool — which was one level when a tool was one level deep. It is not any more: `/under-account` put inviting under 👤, and `/browse` gave every tool a card page below its set. Ash named the rule (#p88). One reading, so it builds.

**The tree already knows what one level back means; ‹ did not.** A tool's own button has stepped back a level since cards arrived — `/browse`'s `tool_cards`, `/posts`, `/projects`, `/reports` and `/people` each answer their own `tool_<id>` with "back to the set" when a card is open, and `/tools` answers it with "home" when one is not. So this node does not navigate. It **renames the tap**: on `tools_home`, while a tool is open, it hands the chain beneath the `tool_<id>` event for the level above and lets every link do exactly what it does for a finger on that button.

- **A registry tool is open** (posts, 👤, projects, cards, reports — anything `tools_list` names): the level above is that tool's own button. With a card showing that is the set; with the set showing it is the toolbar. One rule, both levels, and no node had to be taught anything.
- **A nested tool is open** (one `tools_list` does not name — invite, under 👤): the level above is the tool that opened it, and `tool_<parent>` is what `/tools` already turns into "open the parent".

**Which tool is the parent, recorded rather than guessed.** `/current-only` finds a parent by reading the row for a registry tool's `tool_<id>` control — and then removes it, so by the time a newer link could read the row the parent button is gone. So this node remembers the way in instead of re-deriving it: `parents` is a device-scoped stack of tool ids, pushed when a tap leaves one tool open and lands on a **nested** one, popped when ‹ climbs. It is device state exactly as `open_tool` is — navigation is per-instance and never travels — and it is emptied whenever a tap lands on a registry tool or on the launcher, so it can only ever hold the chain you are actually standing in. It is not bridged to the page: nothing on the page half needs it, and a var written from a link newer than `/payload` would paint a stale frame (misses.md, "navigation from the wrong side"). For the same reason nothing here writes `open_tool`: the rewritten event does the moving, at the links that own it.

**Any future sub-tool gets the rule for nothing.** The rule reads the registry, never a list of names: a tool that is not in `tools_list` is nested, and whatever tool was open when it opened is its parent. A second level of nesting is the same push and the same pop — the stack is a stack — so a sub-tool of a sub-tool climbs to its own parent, not to the root, without a line of new code. And a long-press "home" that jumps to the root, if it is ever asked for, is a card on `/back`'s ‹ rather than a change here.

Untick and ‹ leaves the tool, as it did.

## hostile cases

- **The gate.** `/me/profile-first` drops navigation taps before the chain sees them while a profile is incomplete. The rewritten event is `tool_account` on the own card page — the very event the gate names — so it is dropped like the ‹ it stood for; nothing is written and nothing repaints. The pop is guarded on the chain having actually arrived (`open_tool` is the parent afterwards), so a dropped tap cannot eat a level either.
- **A nested tool with no remembered parent** (a stack emptied by something this node did not see): the event is left alone and ‹ goes to the launcher, exactly as before this node. The failure direction is the old behaviour, never a dead button.
- **The stack fills.** It is capped at eight; a ninth push drops the oldest, which is the end furthest from where you are — the level you are about to climb to is always kept.
- **Re-entry.** A tool re-opened from the launcher is a tap landing on a registry tool, which empties the stack: no path can accumulate a second copy of a chain. A relaunch empties it too (`init`), and `open_tool` is emptied there already, so there is no stale pair.
- **The rewritten event cannot re-enter this rule**: `tool_<id>` is handed straight to `existing`, and this link's own `tools_home` branch is never reached again in the same turn.
- **A long-press on ‹** is `/long-press`'s card, not a tap; no event is sent and no level is climbed.
- **`/browse` unticked, or a tool with no card page**: the registry branch sends the tool's own button, which `/tools` alone answers with "home" — this node reads no card state and names no card feature.

## glossary

(no new terms)

## code description

`one-level.rs`, `update()` /extension/: on a `tools_home` click with a tool open, replaces the event's `ev` with the `tool_<id>` of the level above and passes that to `existing`, then pops the parent stack if the chain really climbed; on every other event it calls `existing` and notes what the tap did to the stack.

`one-level.rs`, `one_level_up()`: the id of the level above — the open tool itself when the registry names it, otherwise the top of the parent stack.

`one-level.rs`, `one_level_nested()`: whether a tool id is absent from `tools_list` — `/current-only`'s test for a sub-tool, asked of the registry rather than of the row.

`one-level.rs`, `one_level_note()`: after a tap that changed which tool is open, pushes the tool left behind when the new one is nested, and empties the stack when it is not.

`one-level.rs`, `one_level_read()` / `one_level_write()` / `one_level_push()` / `one_level_pop()`: the parent stack as a JSON array in the `parents` var, capped at eight.

`one-level.rs`, `init()` /extension/: empties the stack at boot, beside `/tools` emptying `open_tool`.

`one-level.vars` — `parents`, a device-scoped JSON array of tool ids; not bridged.

## risks

**The tour's first step still says one tap.** `/long-press/tour` step 0 points at ‹ on the own card page and waits for `open_tool === ''`. Under this rule that ‹ goes to the people list and `open_tool` stays `account`, so the step advances on the second tap rather than the first. The copy and the `done` test belong to `/tour` and are not touched here.

**`/tool-words` calls ‹ "Back to the toolbar."** True only at the last level now. Its wording is `/long-press/tool-words`' to change.
