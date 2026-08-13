# steps
*the wall shows only the logo and the exact steps*

> (transcripts/2026-08-13-fm-spec.md#p50)
> Yeah: a user shouldn't even see the login screen until they've installed it. It should just show the logo, and the instruction ("tap [share icon], then [v] view more, then [+] add to home screen")

## spec

The install wall hardens: no login, no app, no visible escape until installed. The page shows the logo and exactly three steps, each with its key drawn as a small bordered tile — tap [share icon], then [⌄] view more, then [+] Add to Home Screen (Android: menu wording). `?browser=1` survives as an undocumented, session-scoped dev bypass only.

## user

On a phone browser you see the logo and the three steps — nothing else. Install, open, and muon begins.

## glossary

(no new terms)

## code description

`install.html`'s step markup (`.step` rows with `.key` tiles, an inline share-icon SVG) and the removal of the visible continue-in-browser link; the loader's redirect honours `sessionStorage.muonBrowser` for the dev bypass.
