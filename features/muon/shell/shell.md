# shell
*the muon PWA client shell*

> (transcripts/2026-08-13-fm-spec.md#p38)
> Let's do a little "hello muon" PWA that displays the nøøb logo "ᕦ(ツ)ᕤ" and then build features one by one. We'll stick to a mobile format for now.

## spec

The client-side base of muon: a `render()` chain returning the HTML placed into the page's `#app` element, plus the PWA scaffolding as assets — `index.html` (mobile viewport, dark theme, wasm loader, service-worker registration), `sw.js` (stale-while-revalidate cache so the app works offline after first visit), `manifest.json` and icons (installable to home screen). `render()` returns empty HTML; subfeatures extend it with content. Entry of the `wasm` place in the muon product.

## user

Open muon in a mobile browser; add to home screen to install. Works offline after the first visit. Content comes from subfeatures of shell.

## glossary

- **shell**: the app-independent PWA frame — loader, offline cache, manifest — inside which features render.

## code description

`shell.rs` defines the `render() -> String` chain base (returns empty HTML); subfeatures extend it with content.

`assets/index.html` is the loader: it fetches `client.wasm`, instantiates it, calls the exported `fm_entry()`, unpacks the returned ptr/len pair from wasm memory, and sets `#app`'s HTML. It also registers the service worker.

**Self-update**: after paint the loader checks the deploy stamp (`site/version`, the commit-count build number written by deploy.sh) against the one it launched from — on change it drops the cache and reloads once. Returning to foreground, regaining connectivity, and a 60-second visible-poll all re-check; a failed check surfaces as "can't reach the server", never as "up to date".

**The system panel**: a small logo button bottom-right (hamburger stand-in, safe-area positioned) opens it — who's logged in, the running build, recent changes (`changes.json`, generated at deploy from commit subjects), and enrolment / log out / update buttons. Build numbers appear only in the panel; the button shows no number and pulses gently when a newer build is waiting.

`assets/sw.js` implements the caching principle: network-first with a 1.2s freshness deadline. A fresh copy that arrives in time always wins; a slower network gets the cached copy while the fetch completes in the background and refreshes it; offline is just the deadline missed instantly; nothing-cached waits on the network. `/auth/*` and `version` bypass the cache entirely.

`assets/manifest.json` and the icon PNGs make the app installable, standalone, and black-themed.
