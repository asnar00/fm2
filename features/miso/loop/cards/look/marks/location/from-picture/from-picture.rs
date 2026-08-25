struct feature_FromPicture;
impl feature_FromPicture {
    // ---- precedence ------------------------------------------------------
    // /location's own update stamps every place "device", because when it was
    // written that was the only source there was. This link sits outside it:
    // it notes what the card held BEFORE the chain below runs, lets that chain
    // do the write, and then has the last word — a place the event said came
    // from a picture is re-stamped "picture", and a device fix that has just
    // landed on top of a picture's tag is undone. That second case is the real
    // collision: /location asks the phone once per card per page load with a
    // ten second timeout, so a fix asked for before the photo was chosen can
    // easily answer after it.

    fn update(state: String, event: String) -> String {
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let is_place = e["type"].as_str().unwrap_or("") == "CardPlace";
        let id = e["data"]["id"].as_str().unwrap_or("").to_string();
        let lat = e["data"]["lat"].as_f64().unwrap_or(f64::NAN);
        let lon = e["data"]["lon"].as_f64().unwrap_or(f64::NAN);
        let live = is_place && !id.is_empty() && card_place_sound(lat, lon);
        let before = if live {
            card_pic_place_of(id.clone())
        } else {
            serde_json::Value::Null
        };
        let state = existing.update(state, event.clone());
        if !live {
            return state;
        }
        let after = card_pic_place_of(id.clone());
        if after.is_null() {
            return state;
        }
        if e["data"]["source"].as_str().unwrap_or("") == "picture" {
            let mut block = after;
            block["source"] = serde_json::json!("picture");
            // EXIF records a place, never how sure of it the camera was
            block["acc"] = serde_json::json!(0.0);
            card_pic_place_put(id, block);
        } else if before["source"].as_str().unwrap_or("") == "picture" {
            card_pic_place_put(id, before);
        }
        state
    }

    // the card's location block by id, through /cards' own cards_read, so the
    // var's address stays in one place and cards.rs is never edited. A card
    // with no block, or no such card, is Null.
    fn card_pic_place_of(id: String) -> serde_json::Value {
        let list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        for c in list.as_array().unwrap_or(&empty) {
            if c["id"].as_str().unwrap_or("") != id {
                continue;
            }
            for b in c["blocks"].as_array().unwrap_or(&empty) {
                if b["kind"].as_str().unwrap_or("") == "location" {
                    return b.clone();
                }
            }
        }
        serde_json::Value::Null
    }

    // put one block back where the card's location block sits. Only a card
    // that already carries one is touched: this never creates a place, it only
    // corrects the one /location's write just made.
    fn card_pic_place_put(id: String, block: serde_json::Value) {
        let mut list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        if !list.is_array() {
            return;
        }
        let mut changed = false;
        for c in list.as_array_mut().expect("cards is an array").iter_mut() {
            if c["id"].as_str().unwrap_or("") != id {
                continue;
            }
            if !c["blocks"].is_array() {
                continue;
            }
            let blocks = c["blocks"].as_array_mut().expect("blocks is an array");
            for i in 0..blocks.len() {
                if blocks[i]["kind"].as_str().unwrap_or("") == "location" {
                    blocks[i] = block.clone();
                    changed = true;
                    break;
                }
            }
        }
        if changed {
            cards_write(list.to_string());
        }
    }

    // ---- the pill knows its source ---------------------------------------
    // spliced into the opening tag of the lit pill /location drew, so the page
    // half can say where the place came from without a second read of the
    // store. A dim pill has no place and so has no source.

    fn card_page_html(card: String) -> String {
        let html = existing.card_page_html(card.clone());
        let at = card_place_of(card);
        if at.is_null() {
            return html;
        }
        let src = if at["source"].as_str().unwrap_or("") == "picture" {
            "picture"
        } else {
            "device"
        };
        let mark = "<span class=\"card-place\" data-card=";
        match html.find(mark) {
            Some(i) => format!(
                "{}<span class=\"card-place\" data-source=\"{}\" data-card={}",
                &html[..i], src, &html[i + mark.len()..]),
            None => html,
        }
    }
}
