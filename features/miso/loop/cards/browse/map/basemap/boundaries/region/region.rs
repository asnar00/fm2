struct feature_Region;
impl feature_Region {
    // ---- the second ground -------------------------------------------------
    // `tiles/outdoors/{z}/{x}/{y}.png` — Stadia's Outdoors style, proxied and
    // cached exactly as /tiles proxies and caches the everyday ground, in a
    // cache directory of its own so the two grounds can never answer for each
    // other. This link sits OUTSIDE /tiles' `tiles/` handler because /tiles'
    // parser takes exactly three path segments and would 404 a fourth; being
    // newer, this node's route runs first and claims its own prefix.
    //
    // Nothing here calls a /tiles helper. `tiles.rs` reads MISO_CONTEXT_DIR
    // itself rather than borrowing /remember's `context_dir()`, for the reason
    // it states — a route should not depend on another feature being ticked —
    // and this node keeps the same discipline one step further out: it is a
    // child of /boundaries in the browse tree, and a compile-time call into
    // features/miso/serve/tiles would tie the two ticks together.

    fn route(r: request) -> response {
        if r.path.starts_with("tiles/outdoors/") {
            return region_serve(r.path.clone());
        }
        existing.route(r)
    }

    // ---- where the outdoors squares live -----------------------------------

    fn region_dir() -> String {
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
        format!("{}/tiles-outdoors", base)
    }

    // ---- the source, and the key that is never in the repo -----------------
    // Three answers, in order, and the reason for three is that the key lives
    // in exactly one place on this machine and it is not a file of ours:
    //
    //   1. MISO_OUTDOORS_URL — a whole template, for a source nobody here has
    //      heard of. The same door MISO_TILE_URL is for the everyday ground.
    //   2. STADIA_KEY — the key on its own. This is the shape the reference
    //      plist (tools/com.noob.miso.plist) documents.
    //   3. the everyday ground's own url. On the live box the key is not a
    //      variable of its own at all: it rides inside MISO_TILE_URL as
    //      `?api_key=…`. So when that url names Stadia, the style segment is
    //      swapped for `outdoors` and the query — the key — is carried over
    //      untouched. This is what makes the node work on the live server and
    //      on any rig without a new secret being set anywhere.
    //
    // With none of the three, the answer is the empty string: no url, no
    // fetch, a 404 per square, and the map draws the everyday ground inside
    // the region as well as outside it. /stand-in's rule — a missing square is
    // never an error on the page.

    fn region_source() -> String {
        match std::env::var("MISO_OUTDOORS_URL") {
            Ok(u) => {
                if !u.is_empty() {
                    return u;
                }
            }
            Err(_) => {}
        }
        match std::env::var("STADIA_KEY") {
            Ok(k) => {
                if !k.is_empty() {
                    return format!("{}?api_key={}", region_stadia_url(), k);
                }
            }
            Err(_) => {}
        }
        region_from_ground()
    }

    fn region_stadia_url() -> String {
        "https://tiles.stadiamaps.com/tiles/outdoors/{z}/{x}/{y}.png".to_string()
    }

    // the everyday ground's url with its style segment swapped. Only ever a
    // Stadia url is rewritten: any other source is left alone and answers the
    // empty string, because a style name is not a thing every tile server has.
    fn region_from_ground() -> String {
        let ground = std::env::var("MISO_TILE_URL").unwrap_or_default();
        let mark = "stadiamaps.com/tiles/";
        let at = match ground.find(mark) {
            Some(i) => i + mark.len(),
            None => return String::new(),
        };
        let rest = ground[at..].to_string();
        let end = match rest.find('/') {
            Some(i) => i,
            None => return String::new(),
        };
        format!("{}outdoors{}", &ground[..at], &rest[end..])
    }

    fn region_agent() -> String {
        match std::env::var("MISO_TILE_AGENT") {
            Ok(a) => {
                if a.is_empty() {
                    region_default_agent()
                } else {
                    a
                }
            }
            Err(_) => region_default_agent(),
        }
    }

    fn region_default_agent() -> String {
        "miso/1.0 (https://miso.xn--nb-lkaa.org; ash.nehru@gmail.com)".to_string()
    }

    // ---- the coordinates ---------------------------------------------------
    // "tiles/outdoors/{z}/{x}/{y}.png" -> [z, x, y], and the empty vector for
    // anything else. Digits only, every character checked, so no path this
    // accepts can name a file outside the cache directory — /tiles' whole
    // security argument, restated here rather than borrowed, because a parser
    // a route depends on should live beside the route. A Vec and not a tuple:
    // the chain parser cannot read a comma-bearing return type.

    fn region_coords(path: String) -> Vec<u32> {
        let rest = match path.strip_prefix("tiles/outdoors/") {
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

    // ---- disk first, upstream once -----------------------------------------

    fn region_serve(path: String) -> response {
        let c = region_coords(path);
        if c.len() != 3 {
            return text_response(404, "not found");
        }
        let dir = format!("{}/{}/{}", region_dir(), c[0], c[1]);
        let file = format!("{}/{}.png", dir, c[2]);
        match std::fs::read(file.clone()) {
            Ok(bytes) => {
                if !bytes.is_empty() {
                    println!("miso: outdoors {}/{}/{} disk", c[0], c[1], c[2]);
                    return region_response(bytes);
                }
            }
            Err(_) => {}
        }
        let bytes = region_fetch(c[0], c[1], c[2]);
        if !region_is_png(&bytes) {
            println!("miso: outdoors {}/{}/{} missing", c[0], c[1], c[2]);
            return text_response(404, "not found");
        }
        let _ = std::fs::create_dir_all(dir);
        let _ = std::fs::write(file, bytes.clone());
        println!("miso: outdoors {}/{}/{} fetched {} bytes", c[0], c[1], c[2],
                 bytes.len());
        region_response(bytes)
    }

    // TLS is curl's problem — /vonage's precedent, and the reason this route
    // adds no crate. No source configured is not an error either: it is a
    // fetch that is never made.
    fn region_fetch(z: u32, x: u32, y: u32) -> Vec<u8> {
        let src = region_source();
        if src.is_empty() {
            return Vec::new();
        }
        let url = src
            .replace("{z}", &z.to_string())
            .replace("{x}", &x.to_string())
            .replace("{y}", &y.to_string());
        let out = std::process::Command::new("curl")
            .arg("-s")
            .arg("-f")
            .arg("-L")
            .arg("--connect-timeout").arg("4")
            .arg("--max-time").arg("10")
            .arg("-A").arg(region_agent())
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

    // the eight magic bytes: a captive portal's login page, or Stadia's own
    // "over your quota" json, must never be cached under a .png name.
    fn region_is_png(bytes: &Vec<u8>) -> bool {
        bytes.len() > 8 && bytes[0] == 137 && bytes[1] == 80 && bytes[2] == 78
            && bytes[3] == 71 && bytes[4] == 13 && bytes[5] == 10
            && bytes[6] == 26 && bytes[7] == 10
    }

    fn region_response(bytes: Vec<u8>) -> response {
        response { status: 200, ctype: "image/png".to_string(), body: bytes,
                   set_cookie: String::new(),
                   cache: "public, max-age=604800".to_string() }
    }

    // ---- the chosen region -------------------------------------------------
    // an ONS code, or the empty string for "the whole constituency". Empty is
    // the default because nothing about Sevenoaks belongs in this tree —
    // /boundaries' rule that the FILE is the seam. The page resolves an empty
    // code to whichever feature in the file says it is the constituency.
    //
    // Read through with_context rather than off the bridged loop state: this
    // node is newer than /payload, so a state key it published is one turn
    // behind a write made here (misses.md, "navigation from the wrong side").

    fn region_read() -> String {
        with_context(|c| c.region_region_get())
    }

    // the closure runs twice — the live world, then the turn's frozen view —
    // so it clones rather than moves. /tools' idiom.
    fn region_write(code: String) {
        edit_context(|c| {
            let _ = c.edit_op(
                "miso/loop/cards/browse/map/basemap/boundaries/region",
                "region", serde_json::json!(code.clone()));
        });
    }

    // ---- the control -------------------------------------------------------
    // A sub-tool of the posts tool, in the control row and never a page button
    // (/tools' tree-of-tools rule). It shows while the posts tool is open on
    // the MAP view with no card page up — /recentre's gate, for /recentre's
    // reason: a control that changes what the ground looks like has nothing to
    // say over a grid of tiles.
    //
    // While the region page is itself open the same button is drawn lit, as
    // the current level's own icon; the row's ‹ is /back's, and /one-level
    // carries the climb back to posts without this node saying anything —
    // `region` is deliberately kept out of `tools_list`, which is the whole
    // definition of a nested tool.

    fn tool_controls(state: String) -> String {
        let row = existing.tool_controls(state.clone());
        let open = open_tool_read();
        let mut mine = String::new();
        if open == "posts" && browse_open_read().is_empty()
            && browse_view_read() == "map" {
            mine.push_str(&region_button(false));
        }
        if open == "region" {
            mine.push_str(&region_button(true));
        }
        region_before_undo(row, mine)
    }

    // a colour of its own, /ember's deterministic pick for the name "region",
    // and NOT the posts pink. /glyphs' rule is that two controls side by side
    // must read as two things; the row already holds the posts bubble and
    // /posts' pink plus, and a third pink beside them was a wall of one colour
    // on the rig. /recentre — the other map act that lives in whatever row is
    // open — made the same call with its own name.
    fn region_button(lit: bool) -> String {
        let colour = tool_colour("region".to_string());
        let tint = if colour.is_empty() {
            String::new()
        } else {
            format!(" tinted\" style=\"--tool-colour:{}", colour)
        };
        let sel = if lit { " sel" } else { "" };
        format!("<div class=\"tool-button ctrl{}{}\" data-ev=\"tool_region\" title=\"region\">{}</div>",
                sel, tint, region_svg())
    }

    // undo is last in every row and a newer node's links land after undo's, so
    // keeping the invariant is the newcomer's job (/glyphs). Written out here
    // rather than borrowed, so this node stands whichever siblings are ticked.
    fn region_before_undo(row: String, add: String) -> String {
        if add.is_empty() {
            return row;
        }
        match row.find("data-ev=\"ctx_undo\"") {
            Some(at) => match row[..at].rfind("<div") {
                Some(start) => format!("{}{}{}", &row[..start], add, &row[start..]),
                None => format!("{}{}", row, add),
            },
            None => format!("{}{}", row, add),
        }
    }

    // ---- the events --------------------------------------------------------

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return state;
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        if ev == "region_pick" {
            region_write(String::new());
            return state;
        }
        if let Some(code) = ev.strip_prefix("region_pick:") {
            region_write(code.to_string());
        }
        state
    }

    // ---- the surface -------------------------------------------------------
    // Two pieces of html, and neither of them names a ward.
    //
    // The pills are an EMPTY container. The names and the codes live in
    // /boundaries' geojson, which is a file on the page — and `render` is
    // compiled to wasm as well as to the server, so it cannot read a file at
    // all. The page half fills the container from the parsed collection
    // /boundaries already holds. That division is not a workaround: the file
    // is the seam (/boundaries), and this keeps Sevenoaks out of the Rust as
    // firmly as it is out of the JavaScript.
    //
    // `#misoRegion` is how the map half learns the choice, and it is a marker
    // in the html rather than a bridged state key for the reason /browse
    // states: a key republished at /payload's older link would paint one stale
    // frame after a write made here. This element is written by the render
    // that follows this node's own update, so it is never a turn behind.

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let code = region_read();
        let mark = format!("<span id=\"misoRegion\" data-code=\"{}\" hidden></span>",
                           region_esc(code));
        if open_tool_read() != "region" {
            return format!("{}{}", base, mark);
        }
        format!("{}{}<div class=\"card-page region-page\"><div id=\"regionPills\" class=\"region-pills\"></div></div>",
                base, mark)
    }

    fn region_esc(s: String) -> String {
        s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
            .replace('"', "&quot;")
    }

    // ---- the glyph ---------------------------------------------------------
    // drawn in currentColor, never an emoji (/glyphs): a patch of ground with
    // one part of it filled in — which is the whole of what this tool does.
    //
    // Not a folded map. The first cut was one, and on the rig it read as the
    // picker's own map glyph three fingers away — the same shape, twice, for
    // two different things. This is an irregular patch with one internal
    // division and one side solid: an area, and the part of it that is yours.

    fn region_svg() -> String {
        String::from(concat!(
            "<svg class=\"icon-svg\" viewBox=\"0 0 24 24\" aria-hidden=\"true\">",
            "<path d=\"M4 6.8 11 3.4 12.2 20.6 4.6 17.6z\" ",
            "fill=\"currentColor\" fill-opacity=\"0.45\"/>",
            "<path d=\"M4 6.8 11 3.4 20 6.2 19.4 17.4 12.2 20.6 4.6 17.6z\" ",
            "fill=\"none\" stroke=\"currentColor\" stroke-width=\"2.2\" ",
            "stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
            "<path d=\"M11 3.4 12.2 20.6\" fill=\"none\" ",
            "stroke=\"currentColor\" stroke-width=\"2.2\" ",
            "stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
            "</svg>"))
    }
}
