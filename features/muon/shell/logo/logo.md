# logo
*displays the nøøb logo*

> (transcripts/2026-08-13-fm-spec.md#p38)
> Let's do a little "hello muon" PWA that displays the nøøb logo "ᕦ(ツ)ᕤ"

## spec

Extends the `/shell` `render()` chain to display the nøøb logo `ᕦ(ツ)ᕤ`, centred, white on black, sized for a mobile screen. First light: proves the whole path — feature code → wasm → loader → screen — end to end.

## user

Open the app: you should see `ᕦ(ツ)ᕤ` in the middle of a black screen.

## glossary

(no new terms)

## code description

`logo.rs` extends `render` (lines 3-5): it calls the previous chain via `existing.render()` and appends the logo div. The glyphs are written as escapes — `\u{1566}` ᕦ, `\u{30c4}` ツ, `\u{1564}` ᕤ — to keep the source ASCII-safe.

This node also owns `logo.css`: the `.logo` styling (size, no-select, desktop cap), composed into the app page.
