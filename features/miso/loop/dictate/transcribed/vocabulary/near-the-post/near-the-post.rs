struct feature_NearThePost;
impl feature_NearThePost {
    // ---- the radius --------------------------------------------------------
    // four hundred metres: far enough to hold the streets a person can see from
    // where they are standing, near enough that nothing across town gets in.
    // The parent took the thirty nearest of the CONSTITUENCY's list with no
    // distance test at all, which is why a clip made in Soho was seeded with
    // Sevenoaks and came back "Bourke Street and Bloor" (field-walk #p154).

    fn near_radius_m() -> f64 {
        400.0
    }

    // metres between two points, flat-earth with a real cosine — over four
    // hundred metres that is exact to well under a metre, and the parent's
    // fixed 0.62 was only ever right at one latitude.
    fn near_metres(alat: f64, alon: f64, blat: f64, blon: f64) -> f64 {
        let dlat = (blat - alat) * 111320.0;
        let dlon = (blon - alon) * 111320.0 * (alat.to_radians().cos());
        (dlat * dlat + dlon * dlon).sqrt()
    }

    // ---- the seed ----------------------------------------------------------
    // the stocked list first, because it is free and offline; a live pull when
    // the post is outside it; and NOTHING when neither can answer. Never the
    // parent's answer: "the thirty nearest anywhere" is exactly the wrong one
    // for a post out of area, and an empty seed beats a wrong one — an empty
    // seed loses a little accuracy, a wrong one invents street names.

    fn vocab_streets(lat: f64, lon: f64) -> Vec<String> {
        let mut near = near_from_list(lat, lon);
        if !near.is_empty() {
            return near;
        }
        near = near_from_overpass(lat, lon);
        near
    }

    // every stocked street within the radius, nearest first.
    fn near_from_list(lat: f64, lon: f64) -> Vec<String> {
        let doc = vocab_streets_doc();
        let empty: Vec<serde_json::Value> = Vec::new();
        near_pick(doc["items"].as_array().unwrap_or(&empty).clone(), lat, lon)
    }

    fn near_pick(items: Vec<serde_json::Value>, lat: f64, lon: f64) -> Vec<String> {
        let mut scored: Vec<(u64, String)> = Vec::new();
        for it in items.iter() {
            let n = it["name"].as_str().unwrap_or("").trim().to_string();
            if n.is_empty() {
                continue;
            }
            let d = near_metres(lat, lon, it["lat"].as_f64().unwrap_or(0.0),
                                it["lon"].as_f64().unwrap_or(0.0));
            if d > near_radius_m() {
                continue;
            }
            scored.push(((d * 100.0) as u64, n));
        }
        scored.sort();
        let mut out: Vec<String> = Vec::new();
        let mut seen: Vec<String> = Vec::new();
        for s in scored {
            let key = s.1.to_lowercase();
            if seen.contains(&key) {
                continue;
            }
            seen.push(key);
            out.push(s.1);
            if out.len() >= vocab_streets_wanted() {
                break;
            }
        }
        out
    }

    // ---- the live pull -----------------------------------------------------
    // one Overpass radius query per CELL, not per post: a walk down one street
    // makes a dozen posts and they share the answer. The cell is about five
    // hundred metres square and the pull is eight hundred metres around its
    // centre, so every point in the cell has its own four hundred covered.

    fn near_cell() -> f64 {
        0.005
    }

    fn near_pull_m() -> u64 {
        800
    }

    fn near_cell_key(lat: f64, lon: f64) -> String {
        let c = near_cell();
        format!("{:.3}_{:.3}", (lat / c).round() * c, (lon / c).round() * c)
            .replace('-', "m")
    }

    fn near_cell_centre(v: f64) -> f64 {
        let c = near_cell();
        (v / c).round() * c
    }

    fn near_cache_file(lat: f64, lon: f64) -> String {
        format!("{}/near/{}.json", vocab_context_dir(), near_cell_key(lat, lon))
    }

    // a miss is remembered for an hour. Overpass rate-limits, and asking it
    // again for every post of a walk is both rude and useless — the answer
    // will be the same 429.
    fn near_miss_ms() -> u64 {
        3600000
    }

    fn near_from_overpass(lat: f64, lon: f64) -> Vec<String> {
        let file = near_cache_file(lat, lon);
        let raw = std::fs::read_to_string(&file).unwrap_or_default();
        let cached: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::Value::Null);
        if cached.is_object() {
            if cached["miss"].as_bool() == Some(true) {
                if now_ms() < cached["at"].as_u64().unwrap_or(0) + near_miss_ms() {
                    return Vec::new();      // still backing off
                }
            } else {
                let empty: Vec<serde_json::Value> = Vec::new();
                return near_pick(cached["items"].as_array().unwrap_or(&empty).clone(),
                                 lat, lon);
            }
        }
        let items = near_ask_overpass(near_cell_centre(lat), near_cell_centre(lon));
        let _ = std::fs::create_dir_all(format!("{}/near", vocab_context_dir()));
        if items.is_empty() {
            // could be a genuinely empty patch of countryside or a refusal; both
            // are answered the same way and re-asked in an hour.
            let _ = std::fs::write(file, serde_json::json!({
                "at": now_ms(), "miss": true }).to_string());
            println!("near-the-post: no streets came back for {}, {} — the seed is the address alone",
                     lat, lon);
            return Vec::new();
        }
        let _ = std::fs::write(file, serde_json::json!({
            "at": now_ms(), "items": items.clone() }).to_string());
        println!("near-the-post: pulled {} named places around {}, {}",
                 items.len(), near_cell_centre(lat), near_cell_centre(lon));
        near_pick(items, lat, lon)
    }

    // the same query tools/streets.py makes, as a radius instead of a boundary.
    fn near_query(lat: f64, lon: f64) -> String {
        format!(concat!("[out:json][timeout:50];",
                        "(way[\"highway\"][\"name\"](around:{},{},{});",
                        "node[\"place\"][\"name\"](around:{},{},{}););",
                        "out center tags;"),
                near_pull_m(), lat, lon, near_pull_m(), lat, lon)
    }

    fn near_ask_overpass(lat: f64, lon: f64) -> Vec<serde_json::Value> {
        let mut out: Vec<serde_json::Value> = Vec::new();
        let said = std::process::Command::new("curl")
            .arg("-s")
            .arg("--max-time")
            .arg("60")
            .arg("-A")
            .arg(vocab_agent())
            .arg("--data-binary")
            .arg(near_query(lat, lon))
            .arg("https://overpass-api.de/api/interpreter")
            .output();
        let o = match said {
            Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
            Err(_) => return out,
        };
        let v: serde_json::Value = serde_json::from_str(&o)
            .unwrap_or(serde_json::Value::Null);
        let empty: Vec<serde_json::Value> = Vec::new();
        for el in v["elements"].as_array().unwrap_or(&empty) {
            let name = el["tags"]["name"].as_str().unwrap_or("").trim().to_string();
            if name.is_empty() || name.len() > 60 {
                continue;
            }
            // a way answers with a `center`, a node with its own coordinates
            let elat = if el["lat"].is_number() { el["lat"].as_f64() } else { el["center"]["lat"].as_f64() };
            let elon = if el["lon"].is_number() { el["lon"].as_f64() } else { el["center"]["lon"].as_f64() };
            let elat = match elat { Some(v) => v, None => continue };
            let elon = match elon { Some(v) => v, None => continue };
            out.push(serde_json::json!({ "name": name, "lat": elat, "lon": elon }));
        }
        out
    }
}
