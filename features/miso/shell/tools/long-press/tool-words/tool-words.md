# tool-words
*the card under a long press says what the tool does today, in one place*

> (transcripts/2026-09-02-self-check.md#p64)
> the tooltips on the tools need to be brought up to date - the 'users' tool still shows "account" and talks about "a user page is coming". We should do a sweep through all tool button tips and check they're current

## user

Hold a finger on a tool button and the card tells you what that tool does now. 👤 is **people**: your page and everyone you hold, with the map; the plus is how you invite. The words no longer describe what a tool was going to be.

## spec

`/long-press` draws its card from the feature tree: the registering node's name and its user paragraph (`feature_Chooser.flat`, matched by the tool id). A node's paragraph describes the increment that node made when it was written — `/account` said the 👤 button was a placeholder and a profile page was coming, and that was true on 2026-08-14 — so the card ages as the tool grows. This node keeps **the current words for each tool** in one table and hands them to the card in place of the node's paragraph; a tool not in the table keeps its node's words. The sweep (2026-09-02): 👤 becomes **people** (page, everyone held, the map with live pins, the plus that invites); invite says the two doors; posts mentions +, photo and video; dictate no longer says nothing leaves the phone (it becomes a post); taps says what it is for; projects and reports are tidied. The words are the tool's, so a future change to a tool edits this table, and the node's own paragraph stays the history it is.

## hostile cases

- A tool with no entry here (a new one): its node's words, as before.
- `/long-press` unticked: no card, nothing to say.
- The feature tree failed to load: the card falls back to the button's title, and this node's words still apply when the id is known.

## parked

- Words per authority (a member's card for reports).

## glossary

- **tool words**: the name and sentence the long-press card shows for a tool — current, not historical.

## code description

`tool-words.js` — `feature_ToolWords.WORDS`, the table keyed by tool id; at load, wraps `feature_LongPress.contentFor` so an id in the table answers with these words.
