# muon
*first light: the muon PWA — server + wasm client*

> (transcripts/2026-08-13-fm-spec.md#p38)
> let's think about the "first light" muon app we want to build. I'd want to serve it off my mac mini ... on a subdomain (muon.nøøb.org) via cloudflare tunnel. Let's do a little "hello muon" PWA that displays the nøøb logo "ᕦ(ツ)ᕤ" and then build features one by one.

Two places from one feature tree (see `places.md`): `server` (native, entry `serve`) runs on the mini behind the cloudflare tunnel; `client` (wasm, entry `render`) is built into `build/site/` alongside the shell assets and served to browsers. Deploy with `tools/deploy.sh`.
