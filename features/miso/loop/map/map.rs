struct feature_Map;
impl feature_Map {
    fn tools_list(state: String) -> String {
        let prev = existing.tools_list(state);
        let mut list: serde_json::Value = serde_json::from_str(&prev)
            .unwrap_or(serde_json::json!([]));
        if let Some(arr) = list.as_array_mut() {
            arr.push(serde_json::json!({ "id": "map", "label": "map", "icon": "🗺" }));
        }
        list.to_string()
    }

    // ---- server half: tiles come through miso, and stay here once fetched

    fn tile_root() -> String {
        format!("{}/.miso-tiles", std::env::var("HOME").unwrap_or(".".to_string()))
    }

    fn route(r: request) -> response {
        let p = r.path.clone();
        if let Some(rest) = p.strip_prefix("tiles/") {
            if !msg_guarded(&r) {
                return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
            }
            let name = rest.trim_end_matches(".png").to_string();
            let parts: Vec<&str> = name.split('/').collect();
            if parts.len() != 3 {
                return json_response(400, "{\"ok\":false}".to_string());
            }
            let z: u32 = parts[0].parse().unwrap_or(99);
            let x: i64 = parts[1].parse().unwrap_or(-1);
            let y: i64 = parts[2].parse().unwrap_or(-1);
            // the path is rebuilt from parsed integers, never from the
            // request text, so there is nothing to traverse with
            let n: i64 = 1 << z.min(30);
            if z > 19 || x < 0 || y < 0 || x >= n || y >= n {
                return json_response(400, "{\"ok\":false,\"error\":\"no such tile\"}".to_string());
            }
            return tile_response(z, x, y);
        }
        existing.route(r)
    }

    fn tile_response(z: u32, x: i64, y: i64) -> response {
        let dir = format!("{}/{}/{}", tile_root(), z, x);
        let file = format!("{}/{}.png", dir, y);
        if let Ok(bytes) = std::fs::read(file.clone()) {
            return tile_bytes(bytes);
        }
        // TLS is curl's problem, not ours (the /vonage idiom). OSM asks for
        // an honest User-Agent; a cached tile is never fetched twice.
        let url = format!("https://tile.openstreetmap.org/{}/{}/{}.png", z, x, y);
        let out = std::process::Command::new("curl")
            .arg("-s")
            .arg("--max-time").arg("8")
            .arg("-A").arg("miso/1.0 (personal instance; https://miso.xn--nb-lkaa.org)")
            .arg(url)
            .output();
        let bytes = match out {
            Ok(o) => o.stdout,
            Err(_) => Vec::new(),
        };
        if bytes.len() < 100 {
            return json_response(502, "{\"ok\":false,\"error\":\"tile unavailable\"}".to_string());
        }
        let _ = std::fs::create_dir_all(dir);
        let _ = std::fs::write(file, &bytes);
        tile_bytes(bytes)
    }

    fn tile_bytes(bytes: Vec<u8>) -> response {
        response { status: 200, ctype: "image/png".to_string(), body: bytes,
                   set_cookie: String::new(),
                   cache: "public, max-age=604800".to_string() }
    }

    // ---- client half: intent is the tool being open; readings are events

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let t = e["type"].as_str().unwrap_or("").to_string();
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if t == "Located" {
            s["map_fix"] = e["data"].clone();
            if let Some(o) = s.as_object_mut() {
                o.remove("map_error");
            }
            return s.to_string();
        }
        if t == "LocateFailed" {
            s["map_error"] = e["data"]["err"].clone();
            return s.to_string();
        }
        if t == "click" && e["ev"].as_str().unwrap_or("") == "map_again" {
            if let Some(o) = s.as_object_mut() {
                o.remove("map_error");
                o.remove("map_fix");
            }
            return s.to_string();
        }
        state
    }

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["open_tool"].as_str().unwrap_or("") != "map" {
            return base;
        }
        format!("{}{}", base, map_view(state))
    }

    fn tool_controls(state: String) -> String {
        let prev = existing.tool_controls(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["open_tool"].as_str().unwrap_or("") != "map" {
            return prev;
        }
        format!("{}<div class=\"tool-button ctrl\" data-ev=\"map_again\">⟳</div>", prev)
    }

    // ---- web mercator: the standard slippy-map projection, ~20 lines of it

    fn map_zoom(acc: f64) -> u32 {
        if acc <= 20.0 {
            return 18;
        }
        if acc <= 50.0 {
            return 17;
        }
        if acc <= 150.0 {
            return 16;
        }
        if acc <= 400.0 {
            return 15;
        }
        14
    }

    fn map_tile_x(lon: f64, z: u32) -> f64 {
        (lon + 180.0) / 360.0 * (2.0_f64).powi(z as i32)
    }

    fn map_tile_y(lat: f64, z: u32) -> f64 {
        let r = lat.to_radians();
        let v = (r.tan() + 1.0 / r.cos()).ln();
        (1.0 - v / std::f64::consts::PI) / 2.0 * (2.0_f64).powi(z as i32)
    }

    // metres per screen pixel, which is what makes the accuracy disc honest
    fn map_mpp(lat: f64, z: u32) -> f64 {
        156543.033928 * lat.to_radians().cos() / (2.0_f64).powi(z as i32)
    }

    fn map_metres(m: f64) -> String {
        if m >= 1000.0 {
            return format!("{:.1} km", m / 1000.0);
        }
        format!("{:.0} m", m)
    }

    fn map_view(state: String) -> String {
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if let Some(err) = s["map_error"].as_str() {
            let safe = err.replace('&', "&amp;").replace('<', "&lt;");
            return format!("<div class=\"map-view\"><div class=\"map-msg\">{}</div>\
                <div class=\"map-msg dim\">a map that guesses where you are \
                would be worse than none</div></div>", safe);
        }
        let fix = s["map_fix"].clone();
        if fix.is_null() {
            return "<div class=\"map-view\"><div class=\"map-msg\">finding you…</div>\
                </div>".to_string();
        }
        let lat = fix["lat"].as_f64().unwrap_or(0.0);
        let lon = fix["lon"].as_f64().unwrap_or(0.0);
        let acc = fix["acc"].as_f64().unwrap_or(0.0);
        let z = map_zoom(acc);
        let fx = map_tile_x(lon, z);
        let fy = map_tile_y(lat, z);
        let tx = fx.floor();
        let ty = fy.floor();
        // where our point sits inside its own tile, in pixels
        let ox = (fx - tx) * 256.0;
        let oy = (fy - ty) * 256.0;
        let mut tiles = String::new();
        let mut dy: i64 = -2;
        while dy <= 2 {
            let mut dx: i64 = -2;
            while dx <= 2 {
                let left = (dx as f64) * 256.0 - ox;
                let top = (dy as f64) * 256.0 - oy;
                tiles.push_str(&format!(
                    "<img class=\"map-tile\" src=\"tiles/{}/{}/{}.png\" \
                     style=\"left:calc(50% + {:.0}px);top:calc(50% + {:.0}px)\">",
                    z, (tx as i64) + dx, (ty as i64) + dy, left, top));
                dx = dx + 1;
            }
            dy = dy + 1;
        }
        let mpp = map_mpp(lat, z);
        let mut disc = if mpp > 0.0 { 2.0 * acc / mpp } else { 0.0 };
        if disc > 900.0 {
            disc = 900.0;
        }
        format!("<div class=\"map-view\"><div class=\"map-field\">{}\
            <div class=\"map-acc\" style=\"width:{:.0}px;height:{:.0}px\"></div>\
            <div class=\"map-me\"></div></div>\
            <div class=\"map-read\">{:.5}, {:.5} &middot; &plusmn;{} &middot; \
            &copy; OpenStreetMap</div></div>",
            tiles, disc, disc, lat, lon, map_metres(acc))
    }
}
