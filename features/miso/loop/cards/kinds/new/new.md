# new
*one door for making a card of any type*

> (transcripts/2026-08-25-accounts.md#p88)
> my hitlist before tara comes in: 1) projects; 2) posts; 3) map view.

## user

For agents: send `CardNew {owner, type, title, t}` and a card of that type appears in the world with that title, an empty picture and an empty paragraph, and opens on its page.

## spec

Projects and posts (#p88) both need "make one" — the same act with a different type. This node is that act, once: a `CardNew` event on the update chain appends a card built by `/cards`' own `card_new` (the profile's three-block body: title, picture, text), sets its title, writes the list through `cards_write` (so `/guard` and `/exchange` treat it like any other write), and opens it through `/browse`'s `browse_open_write`, so the surface that asked lands on the page ready to write. The owner comes from the event as `CardEnsure`'s does — the name lives behind the cookie, not in the world.

## hostile cases

- Empty title: a card titled nothing, editable; the surface's copy decides what to show.
- Unknown type: made anyway; a type is a word (#p10).

## glossary

(no new terms)

## code description

`new.rs` — `update` handles `CardNew`: `card_new` + title, `cards_write`, `browse_open_write`.
