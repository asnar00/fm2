struct feature_Vocabulary;
impl feature_Vocabulary {
    // ---- the list ----------------------------------------------------------
    // coarse to fine: constituency, district, ward, then the geocoder's own
    // address parts, then the nearest streets. Cut at the budget from the far
    // end, so what survives is what the speaker is most likely standing on.

    fn vocab_budget() -> usize {
        40
    }

    fn vocab_streets_wanted() -> usize {
        30
    }

    fn transcribe_vocab(card: String) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let place = card_place_of(card.clone());
        let name = vocab_constituency_name();
        if !name.is_empty() {
            out.push(name);
        }
        if place.is_null() {
            return out;
        }
        let lat = place["lat"].as_f64().unwrap_or(0.0);
        let lon = place["lon"].as_f64().unwrap_or(0.0);
        let geo = vocab_geocode(lat, lon);
        for key in vocab_geo_keys() {
            let v = geo[key.as_str()].as_str().unwrap_or("").trim().to_string();
            if !v.is_empty() {
                out.push(v);
            }
        }
        for s in vocab_streets(lat, lon) {
            out.push(s);
        }
        vocab_tidy(out)
    }

    // the order the parts are read in. Ward, district and constituency come
    // from postcodes.io; the rest is Nominatim's address, and they are the
    // same fields fieldnote's buildWhisperPrompt used.
    fn vocab_geo_keys() -> Vec<String> {
        let mut k: Vec<String> = Vec::new();
        k.push("constituency".to_string());
        k.push("district".to_string());
        k.push("ward".to_string());
        k.push("road".to_string());
        k.push("quarter".to_string());
        k.push("suburb".to_string());
        k.push("village".to_string());
        k.push("town".to_string());
        k.push("city".to_string());
        k.push("county".to_string());
        k
    }

    // no empties, no repeats, no more than the budget — and case-insensitively
    // no repeats, because "Sevenoaks" the constituency and "Sevenoaks" the
    // town are one word to a speech model.
    fn vocab_tidy(list: Vec<String>) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        for v in list {
            let v = v.trim().to_string();
            if v.is_empty() || v.len() > 60 {
                continue;
            }
            let key = v.to_lowercase();
            if seen.contains(&key) {
                continue;
            }
            seen.push(key);
            out.push(v);
            if out.len() >= vocab_budget() {
                break;
            }
        }
        out
    }

    // ---- where things are kept ---------------------------------------------
    // the op store, spelled out rather than borrowed from /remember, for the
    // reason /pic-beside gives: an untick elsewhere must not stop this node
    // linking. The two lines must stay in step if the store moves.

    fn vocab_context_dir() -> String {
        match std::env::var("MISO_CONTEXT_DIR") {
            Ok(d) if !d.is_empty() => d,
            _ => format!("{}/.miso-context", std::env::var("HOME").unwrap_or_default()),
        }
    }

    fn vocab_streets_file() -> String {
        format!("{}/streets.json", vocab_context_dir())
    }

    fn vocab_streets_doc() -> serde_json::Value {
        let raw = std::fs::read_to_string(vocab_streets_file()).unwrap_or_default();
        serde_json::from_str(&raw).unwrap_or(serde_json::Value::Null)
    }

    fn vocab_constituency_name() -> String {
        vocab_streets_doc()["constituency"].as_str().unwrap_or("").to_string()
    }

    // ---- the nearest streets ------------------------------------------------
    // a flat-earth distance: at this latitude a degree of longitude is about
    // 0.62 of a degree of latitude, and over one constituency that is exact
    // enough to sort by. No square root — the ordering does not need one.

    fn vocab_streets(lat: f64, lon: f64) -> Vec<String> {
        let doc = vocab_streets_doc();
        let empty: Vec<serde_json::Value> = Vec::new();
        let items = doc["items"].as_array().unwrap_or(&empty);
        let mut scored: Vec<(u64, String)> = Vec::new();
        for it in items {
            let n = it["name"].as_str().unwrap_or("").trim().to_string();
            if n.is_empty() {
                continue;
            }
            let dlat = it["lat"].as_f64().unwrap_or(0.0) - lat;
            let dlon = (it["lon"].as_f64().unwrap_or(0.0) - lon) * 0.62;
            let d = dlat * dlat + dlon * dlon;
            scored.push(((d * 1000000000.0) as u64, n));
        }
        scored.sort();
        let mut out: Vec<String> = Vec::new();
        for s in scored {
            out.push(s.1);
            if out.len() >= vocab_streets_wanted() {
                break;
            }
        }
        out
    }

    // ---- the geocode --------------------------------------------------------
    // fieldnote's two lookups, unchanged in what they ask for: Nominatim for
    // the address, postcodes.io for the ward, district and constituency. Both
    // through curl, which is how this server already reaches the network
    // (/reports), and both with a user-agent that says who is calling —
    // Nominatim's terms ask for one and fieldnote sent one.

    fn vocab_agent() -> String {
        "miso/1.0 (campaign field notes; miso.xn--nb-lkaa.org)".to_string()
    }

    fn vocab_cache_key(lat: f64, lon: f64) -> String {
        // three decimal places: about a hundred metres, so a canvasser working
        // one street geocodes it once and the rest of the road is free.
        format!("{:.3}_{:.3}", lat, lon).replace('-', "m")
    }

    fn vocab_cache_file(lat: f64, lon: f64) -> String {
        format!("{}/geocode/{}.json", vocab_context_dir(), vocab_cache_key(lat, lon))
    }

    fn vocab_get(url: String) -> String {
        let out = std::process::Command::new("curl")
            .arg("-s")
            .arg("--max-time")
            .arg("12")
            .arg("-A")
            .arg(vocab_agent())
            .arg(url)
            .output();
        match out {
            Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
            Err(_) => String::new(),
        }
    }

    // a cached answer is used whatever the network is doing. A lookup that
    // fails writes NOTHING: an empty answer cached is an empty answer for
    // ever, and the next clip on this street deserves another try.
    fn vocab_geocode(lat: f64, lon: f64) -> serde_json::Value {
        let file = vocab_cache_file(lat, lon);
        let raw = std::fs::read_to_string(&file).unwrap_or_default();
        let cached: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::Value::Null);
        if cached.is_object() {
            return cached;
        }
        let mut got = serde_json::json!({});
        let nom = vocab_get(format!(
            "https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=json&addressdetails=1",
            lat, lon));
        let n: serde_json::Value = serde_json::from_str(&nom)
            .unwrap_or(serde_json::Value::Null);
        for key in vocab_nominatim_keys() {
            let v = n["address"][key.as_str()].as_str().unwrap_or("").to_string();
            if !v.is_empty() {
                got[key.as_str()] = serde_json::json!(v);
            }
        }
        let pc = vocab_get(format!(
            "https://api.postcodes.io/postcodes?lon={}&lat={}", lon, lat));
        let p: serde_json::Value = serde_json::from_str(&pc)
            .unwrap_or(serde_json::Value::Null);
        let first = p["result"][0].clone();
        if first.is_object() {
            got["ward"] = first["admin_ward"].clone();
            got["district"] = first["admin_district"].clone();
            got["constituency"] = first["parliamentary_constituency"].clone();
        }
        if got.as_object().map(|o| o.is_empty()).unwrap_or(true) {
            return got;
        }
        let _ = std::fs::create_dir_all(format!("{}/geocode", vocab_context_dir()));
        let _ = std::fs::write(file, got.to_string());
        got
    }

    // Nominatim's own field names, which are not ours: `county` is what it
    // calls what postcodes.io calls a district, and both are kept because
    // either may be the one a speaker says out loud.
    fn vocab_nominatim_keys() -> Vec<String> {
        let mut k: Vec<String> = Vec::new();
        k.push("road".to_string());
        k.push("quarter".to_string());
        k.push("suburb".to_string());
        k.push("village".to_string());
        k.push("town".to_string());
        k.push("city".to_string());
        k.push("county".to_string());
        k
    }
}
