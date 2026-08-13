# muon
*shared infrastructure for all apps and tools*

> (transcripts/2026-08-13-fm-spec.md#p32)
> so next is a collection of work I want to call "muon" : it's shared infrastructure for all "apps" / "tools" I want to build. The base of it is: it's a Progressive Web App (PWA) written in rust/wasm, that has a few useful features: 1) works even when there's no internet (using a local server cache); 2) has authentication and users built in; 3) has always-on black-box and reproducibility to catch errors; 4) supports feature management through a simple UI.

## spec

Root of the shared infrastructure feature space. Muon is a Progressive Web App written in Rust/wasm, providing four base capabilities to every app built on it: offline operation, built-in authentication and users, always-on black-box recording with reproducible replay, and feature management through a simple UI. Apps are built as subfeatures of muon; products compose muon plus an app subtree.

## user

(to be written as the capabilities take shape)

## glossary

- **muon**: the shared infrastructure layer — a Rust/wasm PWA base that all apps extend.
- **app**: a user-facing tool built as a subfeature of `/muon`, inheriting its base capabilities.

## code description

No implementation yet — container node; subfeatures will hold the capabilities.
