# newline
*Enter makes a new line that stays*

> (asks#1787669434945)
> Problem with text editing: hitting enter creates a new line that immediately gets deleted.
> *(filed from the field on 2026-08-25 by ash, birthplace `👤 @ miso/shell/panel/account`)*

## user

Press Enter in your mission paragraph and you get a new line, and it stays.

## spec

Two things ate the line. Both save paths — `/cards`' tap-away and `/keep`'s save-as-you-type — took the block's text with `.trim()`, which strips a trailing newline, so the moment the debounced save fired the store had no line break, and `/keep`'s repaint restored the store's text over the one on screen. And a stored newline would not have shown anyway: the paragraph had no `white-space` rule, so a `\n` in the text collapsed to a space.

`/cards` grew a seam for this node, `feature_Cards.textOf(el)` — the one rule for what a block's text is when saved, default `.trim()` as before — and `/keep` routes both of its sends through it. This node replaces the rule: trim spaces and tabs at the ends, keep newlines, and fold a double trailing newline (how a contenteditable reports a fresh empty last line) to one. And the paragraph is `pre-wrap`, so stored line breaks render as line breaks. Untick and the old trim returns.

## hostile cases

- Enter pressed several times at the end: the text keeps one trailing newline; the extra empty lines are not stored (the page shows one blank line at most).
- Enter in the title: `/keep` makes it finish the edit, so no newline reaches the title.
- Pasted text with internal blank lines: kept as typed (`pre-wrap` shows them).

## glossary

(no new terms)

## code description

`newline.js` — replaces `feature_Cards.textOf` at load.

`newline.css` — `white-space: pre-wrap` on `.card-text`.
