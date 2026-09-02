struct feature_Live;
impl feature_Live {
    // ---- the routes ---------------------------------------------------------
    // three paths, and this link is the outermost on the chain (the node is
    // the newest), so it sits outside /gate's wall and the cookie is checked
    // here, /exchange's way. Nothing on these paths reaches the loop, the op
    // log or a var: a position goes into memory and comes back out of it.

    fn route(r: request) -> response {
        if !r.path.starts_with("live/") {
            return existing.route(r);
        }
        let me = live_who(&r);
        if me.is_empty() {
            return live_say(403, "who are you?".to_string());
        }
        if r.path == "live/here" && r.method == "POST" {
            return live_here(me, r.body.clone());
        }
        if r.path == "live/gone" && r.method == "POST" {
            live_drop(me);
            return live_say_ok();
        }
        if r.path == "live/near" && r.method == "GET" {
            return json_response(200, format!("{{\"ok\":true,\"live\":{}}}", live_near(me)));
        }
        existing.route(r)
    }

    // the caller off the cookie and nothing else — where a person is, is
    // theirs to say, so the localhost tooling door has no say in it.
    fn live_who(r: &request) -> String {
        let t = cookie_token(r.cookie.clone());
        if !t.is_empty() && token_valid(t.clone()) {
            return format!("phone:{}", token_phone(t));
        }
        String::new()
    }

    fn live_say(status: u16, words: String) -> response {
        json_response(status, format!("{{\"ok\":false,\"error\":\"{}\"}}",
                                      words.replace('"', "'")))
    }

    fn live_say_ok() -> response {
        json_response(200, "{\"ok\":true}".to_string())
    }

    // a heartbeat: {lat, lon}. Anything else is dropped before it reaches the
    // store — a small body, two numbers in range. The answer never echoes the
    // position back and nothing is printed: a coordinate exists in this
    // process's memory and nowhere else.
    fn live_here(me: String, body: String) -> response {
        if body.len() > 256 {
            return live_say(400, "too long".to_string());
        }
        let v: serde_json::Value = serde_json::from_str(&body)
            .unwrap_or(serde_json::Value::Null);
        let lat = v["lat"].as_f64().unwrap_or(1000.0);
        let lon = v["lon"].as_f64().unwrap_or(1000.0);
        if !live_sound(lat, lon) {
            return live_say(400, "not a place".to_string());
        }
        live_put(me, lat, lon);
        live_say_ok()
    }

    // the bounds test, written here rather than borrowed from /location so
    // the route stands with that node unticked
    fn live_sound(lat: f64, lon: f64) -> bool {
        lat.is_finite() && lon.is_finite()
            && lat >= -90.0 && lat <= 90.0 && lon >= -180.0 && lon <= 180.0
    }

    // ---- the store ------------------------------------------------------------
    // one entry per world key, in memory only: `static` inside a fn body
    // because the composition carries fns (/alive's idiom), a Mutex around a
    // map as /one-way and /adopt hold theirs. A restart forgets everyone,
    // which is the point. Sixty seconds after the last heartbeat an entry is
    // swept, on every read and write, so nothing outlives the app being open.

    fn live_ttl_ms() -> u64 {
        60_000
    }

    fn live_put(key: String, lat: f64, lon: f64) {
        let now = now_ms();
        let mut map = match live_cell().lock() {
            Ok(m) => m,
            Err(p) => p.into_inner(),
        };
        map.retain(|_, e| now.saturating_sub(e["t"].as_u64().unwrap_or(0)) < live_ttl_ms());
        map.insert(key, serde_json::json!({"lat": lat, "lon": lon, "t": now}));
    }

    fn live_drop(key: String) {
        let now = now_ms();
        let mut map = match live_cell().lock() {
            Ok(m) => m,
            Err(p) => p.into_inner(),
        };
        map.retain(|_, e| now.saturating_sub(e["t"].as_u64().unwrap_or(0)) < live_ttl_ms());
        map.remove(&key);
    }

    // everyone still live, as a JSON object keyed by world key. The keys
    // never leave the server: live_near turns them into names and faces.
    fn live_read() -> serde_json::Value {
        let now = now_ms();
        let mut map = match live_cell().lock() {
            Ok(m) => m,
            Err(p) => p.into_inner(),
        };
        map.retain(|_, e| now.saturating_sub(e["t"].as_u64().unwrap_or(0)) < live_ttl_ms());
        let mut out = serde_json::Map::new();
        for (k, e) in map.iter() {
            out.insert(k.clone(), e.clone());
        }
        serde_json::Value::Object(out)
    }

    // ---- who I may see ---------------------------------------------------------
    // /people's audience, unchanged: the profile cards in the caller's own
    // world — theirs and the copies /exchange handed them. A live entry is
    // served only if it is the caller's own or its owner's name is `from` on
    // a copy the caller holds. Read outside any turn, /exchange's way, so the
    // read is of the live world and nothing is frozen or written.

    fn live_near(me: String) -> String {
        let live = live_read();
        let held: serde_json::Value = serde_json::from_str(&exchange_cards_of(me.clone()))
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut rows: Vec<serde_json::Value> = Vec::new();
        for (key, at) in live.as_object().map(|m| m.iter().collect::<Vec<_>>()).unwrap_or_default() {
            let mine = key == &me;
            let name = exchange_name_of(key.clone());
            let mut card = serde_json::Value::Null;
            for c in held.as_array().unwrap_or(&empty) {
                if c["type"].as_str().unwrap_or("") != "profile" {
                    continue;
                }
                let own = c["from"].is_null();
                if mine && own {
                    card = c.clone();
                    break;
                }
                if !mine && !own && c["from"].as_str().unwrap_or("") == name && !name.is_empty() {
                    card = c.clone();
                    break;
                }
            }
            // somebody whose card you do not hold is not on your map — and
            // you are on your own map even before your first card exists
            if card.is_null() && !mine {
                continue;
            }
            let initial = if card.is_null() {
                name.chars().take(1).collect::<String>()
            } else {
                map_initial_of(&card)
            };
            rows.push(serde_json::json!({
                "name": name,
                "id": card["id"].as_str().unwrap_or(""),
                "face": if card.is_null() { String::new() } else { map_face_of(&card) },
                "initial": initial,
                "lat": at["lat"].as_f64().unwrap_or(0.0),
                "lon": at["lon"].as_f64().unwrap_or(0.0),
                "t": at["t"].as_u64().unwrap_or(0),
                "me": mine
            }));
        }
        rows.sort_by(|a: &serde_json::Value, b: &serde_json::Value| {
            let na = a["name"].as_str().unwrap_or("").to_string();
            let nb = b["name"].as_str().unwrap_or("").to_string();
            na.cmp(&nb)
        });
        serde_json::Value::Array(rows).to_string()
    }
}
