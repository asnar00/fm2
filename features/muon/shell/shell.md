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

`shell.rs` defines the `render() -> String` chain base (lines 5-7, returns empty). `assets/index.html` is the loader: fetches `client.wasm`, instantiates it, calls the exported `fm_entry()`, unpacks the returned ptr/len pair from wasm memory, and sets `#app`'s HTML; registers `sw.js`. After paint it checks the deploy stamp (`site/version`, written by deploy.sh from the git hash) against the one it launched from — on change it drops the cache and reloads once, so a deploy is picked up on the next launch. `assets/sw.js` caches GETs stale-while-revalidate, bypassing `/auth/*` and `version` (never answered from cache). `assets/manifest.json` + icon PNGs make the app installable, standalone, black-themed.
