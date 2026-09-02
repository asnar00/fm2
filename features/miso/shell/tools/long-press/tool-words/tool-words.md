# tool-words
*the card under a long press says what the tool does today, in one place*

> (transcripts/2026-09-02-self-check.md#p64)
> the tooltips on the tools need to be brought up to date - the 'users' tool still shows "account" and talks about "a user page is coming". We should do a sweep through all tool button tips and check they're current

## user

Hold a finger on a tool button and the card says what the tool is for, in a line: 👤 is **people** — your page, and everyone you hold. Hold a button inside a tool — grid, list, map, undo, new post, record, reset — and the card says what that button does.

## spec

`/long-press` draws its card from the feature tree: the registering node's name and its user paragraph (`feature_Chooser.flat`, matched by the tool id), and `/sub-tool-cards` does the same for the buttons inside a tool. A node's paragraph describes the increment that node made when it was written — `/account` said the 👤 button was a placeholder and a profile page was coming, and that was true on 2026-08-14 — so the card ages as the tool grows. This node keeps **the current words** in two tables: one line per tool saying what it is *for* (ash, #p65: the description says the purpose, the detail belongs to the buttons), and one line per button inside a tool, keyed by the button's event — including the grid, list and map picker, which no card reached before. A tool or button not in the tables keeps its node's words. The sweep (2026-09-02): 👤 is **people**; invite, posts, projects, reports, taps, dictate and cards each get their line; twenty-three buttons get theirs. The words are the tool's, so a future change to a tool edits these tables, and the node's own paragraph stays the history it is.

## hostile cases

- A tool or button with no entry here (a new one): its node's words, as before.
- The picker's buttons (grid, list, map): a long press reads and does not switch the view; a tap switches as before.
- `/long-press` unticked: no card, nothing to say.
- The feature tree failed to load: the card falls back to the button's title, and this node's words still apply when the id is known.

## parked

- Words per authority (a member's card for reports).

## glossary

- **tool words**: the name and sentence the long-press card shows for a tool — current, not historical.

## code description

`tool-words.js` — `feature_ToolWords.TOOLS` (by tool id) and `BUTTONS` (by event, a `name:` suffix stripped), `words(ev)`; at load, wraps `feature_LongPress.contentFor` so a known event answers with these words, and arms the long press on `.browse-view[data-ev]` the way `/sub-tool-cards` arms controls, swallowing the click that follows a read.
