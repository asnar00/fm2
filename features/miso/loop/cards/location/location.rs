struct feature_Location;
impl feature_Location {
    // ---- the datum -------------------------------------------------------
    // a card's place is one block in its own body: {kind:"location", lat,
    // lon, acc, t, source}. /cards' page renderer draws nothing for a kind it
    // does not know, so the block is invisible to everything but this node.
    // A garbage block reads the same as no block at all — null.

    fn card_place_of(card: String) -> serde_json::Value {
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in c["blocks"].as_array().unwrap_or(&empty) {
            if b["kind"].as_str().unwrap_or("") != "location" {
                continue;
            }
            let lat = b["lat"].as_f64().unwrap_or(f64::NAN);
            let lon = b["lon"].as_f64().unwrap_or(f64::NAN);
            if card_place_sound(lat, lon) {
                return b.clone();
            }
            return serde_json::Value::Null;
        }
        serde_json::Value::Null
    }

    // the one test a coordinate has to pass, on the way in and on the way
    // out: a place that is not a place never enters the world and never
    // reaches the screen.
    fn card_place_sound(lat: f64, lon: f64) -> bool {
        lat.is_finite() && lon.is_finite()
            && lat >= -90.0 && lat <= 90.0
            && lon >= -180.0 && lon <= 180.0
    }

    // ---- the event -------------------------------------------------------
    // CardPlace {id, lat, lon, acc, t}: one card carries at most one place,
    // so a second fix replaces the first. Read and written through /cards'
    // own cards_read / cards_write, so the var's address stays in one place
    // and cards.rs is never edited.

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "CardPlace" {
            return state;
        }
        let id = e["data"]["id"].as_str().unwrap_or("").to_string();
        let lat = e["data"]["lat"].as_f64().unwrap_or(f64::NAN);
        let lon = e["data"]["lon"].as_f64().unwrap_or(f64::NAN);
        if id.is_empty() || !card_place_sound(lat, lon) {
            return state;
        }
        let acc = e["data"]["acc"].as_f64().unwrap_or(0.0);
        let acc = if acc.is_finite() && acc >= 0.0 { acc } else { 0.0 };
        let now = e["data"]["t"].as_u64().unwrap_or(0);
        let mut list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        if !list.is_array() {
            return state;
        }
        let block = serde_json::json!({
            "kind": "location",
            "lat": lat,
            "lon": lon,
            "acc": acc,
            "t": now,
            "source": "device"
        });
        let mut changed = false;
        for c in list.as_array_mut().expect("cards is an array").iter_mut() {
            if c["id"].as_str().unwrap_or("") != id {
                continue;
            }
            if !c["blocks"].is_array() {
                continue;
            }
            let blocks = c["blocks"].as_array_mut().expect("blocks is an array");
            let mut at = blocks.len();
            for i in 0..blocks.len() {
                if blocks[i]["kind"].as_str().unwrap_or("") == "location" {
                    at = i;
                    break;
                }
            }
            if at < blocks.len() {
                blocks[at] = block.clone();
            } else {
                blocks.push(block.clone());
            }
            c["edited"] = serde_json::json!(now);
            changed = true;
        }
        if changed {
            cards_write(list.to_string());
        }
        state
    }

    // ---- the pill --------------------------------------------------------
    // spliced in before the page's LAST closing div: inside the card's own
    // scrolling box, after the blocks, and before whatever me_under puts
    // under the card. No data-ev on it, so the loop's delegated click never
    // fires and a tap cannot repaint #app out from under itself.

    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card.clone());
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        let id = card_esc(c["id"].as_str().unwrap_or("").to_string());
        let at = card_place_of(card.clone());
        let pill = if at.is_null() {
            format!(
                "<span class=\"card-place dim\" data-card=\"{}\">map location</span>",
                id)
        } else {
            format!(
                "<span class=\"card-place\" data-card=\"{}\" data-lat=\"{:.5}\" data-lon=\"{:.5}\" data-acc=\"{}\" data-t=\"{}\">map location</span>",
                id,
                at["lat"].as_f64().unwrap_or(0.0),
                at["lon"].as_f64().unwrap_or(0.0),
                at["acc"].as_f64().unwrap_or(0.0).round() as i64,
                at["t"].as_u64().unwrap_or(0))
        };
        match html.rfind("</div>") {
            Some(i) => format!("{}{}{}", &html[..i], pill, &html[i..]),
            None => format!("{}{}", html, pill),
        }
    }
}
