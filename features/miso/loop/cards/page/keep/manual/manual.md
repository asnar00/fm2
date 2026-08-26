# manual
*no saving while you type: tap away, or tap save*

> (transcripts/2026-08-25-accounts.md#p119a)
> the autosave while typing is mucking up text entry - it keeps losing keys, shift doesn't take, we need to stop doing that basically. just have a "save" button.

## user

While you write in a card nothing happens under your fingers. Tap away, or tap **save**, and your words are kept.

## spec

`/keep` saved a moment after you stopped typing, and the repaint that followed landed under the next keystrokes — lost keys, a shift that didn't take (#p119a). Ash's ruling: stop it; a save button. One reading, so it builds: this node switches `/keep`'s save-as-you-type off (`feature_Keep.soon` becomes nothing) and adds a **save** pill above the toolbar while a card block is being edited; tapping it is a tap-away — the blur is the save, `/cards`' own rule — so nothing new is written to the store. `/keep`'s other three promises stand: a repaint that does arrive (another device's edit) keeps your caret, Enter finishes the title, the words of a block that vanished are rescued. Untick and saving-as-you-type returns.

## hostile cases

- The tool closed mid-sentence: `/keep`'s rescue still sends the held words once.
- Tapping save with nothing focused: the pill is not shown.

## glossary

(no new terms)

## code description

`manual.js` — replaces `feature_Keep.soon` with a no-op; makes the save pill, shows it on focusin of a card block, hides it on focusout; its pointerdown blurs the block.

`manual.css` — the pill.
