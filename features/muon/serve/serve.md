# serve
*static file server for the muon site*

> (transcripts/2026-08-13-fm-spec.md#p38)
> let's think about the "first light" muon app we want to build. I'd want to serve it off my mac mini (look in ../ftr/scripts/deploy.sh for details) on a subdomain (muon.nøøb.org) via cloudflare tunnel. Let's do a little "hello muon" PWA that displays the nøøb logo "ᕦ(ツ)ᕤ" and then build features one by one. We'll stick to a mobile format for now.

## spec

Serves the built site directory (`site/`, relative to the working directory) over HTTP on port 8095, using only the Rust standard library — no crates. GET requests map to files under `site/`; `/` serves `index.html`; paths containing `..` are refused; unknown paths get 404. `Cache-Control: no-cache` lets the service worker own caching. Runs as the entry of the `server` place in the muon product; cloudflare tunnel maps muon.nøøb.org to this port on the mini.

## user

Run the server place binary from the product build directory (so `site/` resolves); browse to the port or the tunnel hostname.

## glossary

- **site**: the assembled static output of the client place — html/js/manifest/wasm/icons — served verbatim.

## code description

`serve.rs` declares the `request` struct (method, path, cookie, body, tunnel — where tunnel means cloudflared's `cf-connecting-ip` header was present) and the `response` struct (status, ctype, body, set_cookie, cache). `serve` binds port 8095; `handle` runs each connection through `parse_request` → `route` → `write_response`. **`route(request) -> response` is the extension point**: the base implementation serves static files from `site/` with a directory-index fallback (`site/<path>/index.html` — the exported feature tree at `/features/` relies on it); features add endpoints and gating by redefining it and delegating via `existing.route(r)` (see `/gate`). Helpers: `clean_path` (query stripped, `..` refused, `/` → index.html), `text_response`/`json_response` constructors, `content_type` (`.wasm` → `application/wasm` so streaming instantiation works), `status_name`.
