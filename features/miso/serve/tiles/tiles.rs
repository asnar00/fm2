struct feature_Tiles;
impl feature_Tiles {
    // ---- the route --------------------------------------------------------
    // `tiles/{z}/{x}/{y}.png` — the map's basemap, fetched from the upstream
    // renderer ONCE and served off this machine's disk for ever after. The
    // device never talks to a tile server: it talks to us, which is what makes
    // the map a dependency we own rather than a third party on the page.
    //
    // Not added to /public: a tile is an app surface, so the gate's ordinary
    // rule applies and the proxy is closed to anyone without a cookie.

    fn route(r: request) -> response {
        if r.path == "tiles/attribution" {
            return tiles_attribution_response();
        }
        if !r.path.starts_with("tiles/") {
            return existing.route(r);
        }
        tiles_serve(r.path.clone())
    }

    // ---- where the tiles live ---------------------------------------------
    // beside the op logs, under the same MISO_CONTEXT_DIR a rig redirects —
    // read here rather than borrowed from /remember, so a route in `serve`
    // carries no dependency on the loop's storage being ticked.

    fn tiles_dir() -> String {
        let home = std::env::var("HOME").unwrap_or_default();
        let base = match std::env::var("MISO_CONTEXT_DIR") {
            Ok(d) => {
                if d.is_empty() {
                    format!("{}/.miso-context", home)
                } else {
                    d
                }
            }
            Err(_) => format!("{}/.miso-context", home),
        };
        format!("{}/tiles", base)
    }

    // ---- the source, and who we say we are --------------------------------
    // OSM's tile policy asks for a User-Agent that names the application and
    // a way to reach whoever runs it; MISO_TILE_AGENT overrides the default.
    //
    // The DEFAULT source is CARTO's dark basemap — OpenStreetMap data, drawn
    // dark. That is a source choice, not a filter: notes.md's third principle
    // (2026-08-16) withdrew a map whose bright basemap had a brightness
    // filter piled on top of it. MISO_TILE_URL points this anywhere, and
    // `https://tile.openstreetmap.org/{z}/{x}/{y}.png` is the plain-OSM value.

    fn tiles_source() -> String {
        match std::env::var("MISO_TILE_URL") {
            Ok(u) => {
                if u.is_empty() {
                    tiles_default_url()
                } else {
                    u
                }
            }
            Err(_) => tiles_default_url(),
        }
    }

    fn tiles_default_url() -> String {
        "https://basemaps.cartocdn.com/dark_all/{z}/{x}/{y}.png".to_string()
    }

    fn tiles_agent() -> String {
        match std::env::var("MISO_TILE_AGENT") {
            Ok(a) => {
                if a.is_empty() {
                    tiles_default_agent()
                } else {
                    a
                }
            }
            Err(_) => tiles_default_agent(),
        }
    }

    fn tiles_default_agent() -> String {
        "miso/1.0 (https://miso.xn--nb-lkaa.org; ash.nehru@gmail.com)".to_string()
    }

    // the credit the map must show, kept beside the source that earned it so
    // the two can never drift. Plain text, one line; the page half asks once.
    fn tiles_attribution() -> String {
        match std::env::var("MISO_TILE_ATTRIBUTION") {
            Ok(a) => {
                if a.is_empty() {
                    tiles_default_attribution()
                } else {
                    a
                }
            }
            Err(_) => tiles_default_attribution(),
        }
    }

    fn tiles_default_attribution() -> String {
        if tiles_source().contains("cartocdn.com") {
            return "\u{00a9} OpenStreetMap contributors \u{00a9} CARTO".to_string();
        }
        "\u{00a9} OpenStreetMap contributors".to_string()
    }

    fn tiles_attribution_response() -> response {
        response { status: 200, ctype: "text/plain; charset=utf-8".to_string(),
                   body: tiles_attribution().into_bytes(),
                   set_cookie: String::new(),
                   cache: "no-cache".to_string() }
    }

    // ---- the coordinates ---------------------------------------------------
    // "tiles/{z}/{x}/{y}.png" -> [z, x, y]; anything else, or a tile that does
    // not exist at that zoom, is the empty vector and therefore a 404. Every
    // character is checked to be a digit before it is parsed, so no path this
    // function accepts can name a file outside the cache directory. A Vec and
    // not a tuple: the chain parser cannot read a comma-bearing return type.

    fn tiles_coords(path: String) -> Vec<u32> {
        let rest = match path.strip_prefix("tiles/") {
            Some(s) => s.to_string(),
            None => return Vec::new(),
        };
        let rest = match rest.strip_suffix(".png") {
            Some(s) => s.to_string(),
            None => return Vec::new(),
        };
        let parts: Vec<String> = rest.split('/').map(|p| p.to_string()).collect();
        if parts.len() != 3 {
            return Vec::new();
        }
        let mut out: Vec<u32> = Vec::new();
        for p in parts.iter() {
            if p.is_empty() || p.len() > 8 {
                return Vec::new();
            }
            if !p.chars().all(|c| c.is_ascii_digit()) {
                return Vec::new();
            }
            match p.parse::<u32>() {
                Ok(n) => out.push(n),
                Err(_) => return Vec::new(),
            }
        }
        if out[0] > 19 {
            return Vec::new();
        }
        let side = 1u32 << out[0];
        if out[1] >= side || out[2] >= side {
            return Vec::new();
        }
        out
    }

    // ---- disk first, upstream once ----------------------------------------

    fn tiles_serve(path: String) -> response {
        let c = tiles_coords(path);
        if c.len() != 3 {
            return text_response(404, "not found");
        }
        let dir = format!("{}/{}/{}", tiles_dir(), c[0], c[1]);
        let file = format!("{}/{}.png", dir, c[2]);
        match std::fs::read(file.clone()) {
            Ok(bytes) => {
                if !bytes.is_empty() {
                    println!("miso: tile {}/{}/{} disk", c[0], c[1], c[2]);
                    return tiles_response(bytes);
                }
            }
            Err(_) => {}
        }
        let bytes = tiles_fetch(c[0], c[1], c[2]);
        // an upstream that is unreachable, slow, or answering with an error
        // page is a MISSING TILE, never an error on the page: the map draws
        // its ground where the picture would have been and stays usable.
        if !tiles_is_png(&bytes) {
            println!("miso: tile {}/{}/{} missing", c[0], c[1], c[2]);
            return text_response(404, "not found");
        }
        let _ = std::fs::create_dir_all(dir);
        let _ = std::fs::write(file, bytes.clone());
        println!("miso: tile {}/{}/{} fetched {} bytes", c[0], c[1], c[2],
                 bytes.len());
        tiles_response(bytes)
    }

    // TLS is curl's problem, not ours — /vonage's precedent, and the reason
    // this route needs no crate. `-f` makes an HTTP error an exit code; the
    // timeouts stop one dead upstream holding a server thread.
    fn tiles_fetch(z: u32, x: u32, y: u32) -> Vec<u8> {
        let url = tiles_source()
            .replace("{z}", &z.to_string())
            .replace("{x}", &x.to_string())
            .replace("{y}", &y.to_string());
        let out = std::process::Command::new("curl")
            .arg("-s")
            .arg("-f")
            .arg("-L")
            .arg("--connect-timeout").arg("4")
            .arg("--max-time").arg("10")
            .arg("-A").arg(tiles_agent())
            .arg(url)
            .output();
        match out {
            Ok(o) => {
                if o.status.success() {
                    o.stdout
                } else {
                    Vec::new()
                }
            }
            Err(_) => Vec::new(),
        }
    }

    // the eight magic bytes: what is cached under a .png must be a .png, or a
    // captive-portal login page would be kept for a week as a piece of map.
    fn tiles_is_png(bytes: &Vec<u8>) -> bool {
        bytes.len() > 8 && bytes[0] == 137 && bytes[1] == 80 && bytes[2] == 78
            && bytes[3] == 71 && bytes[4] == 13 && bytes[5] == 10
            && bytes[6] == 26 && bytes[7] == 10
    }

    // a tile at z/x/y never changes, so the device is told it may keep it for
    // a week; the service worker keeps it beyond that.
    fn tiles_response(bytes: Vec<u8>) -> response {
        response { status: 200, ctype: "image/png".to_string(), body: bytes,
                   set_cookie: String::new(),
                   cache: "public, max-age=604800".to_string() }
    }
}
