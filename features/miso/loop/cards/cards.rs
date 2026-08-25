struct feature_Cards;
impl feature_Cards {
    // ---- the store -------------------------------------------------------
    // the cards list is a declared /var: a JSON list in a string, user-scoped
    // and last-write, exactly as /ask's `asks` is. The address is written
    // once here so a subnode never has to know where a card lives.

    fn cards_read() -> String {
        with_context(|c| c.cards_cards_get())
    }

    fn cards_write(list: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/loop/cards", "cards", serde_json::json!(list));
        });
    }

    fn cards_tiles_read() -> bool {
        with_context(|c| c.cards_tiles_get())
    }

    fn cards_tiles_write(on: bool) {
        edit_context(|c| {
            let _ = c.edit_op("miso/loop/cards", "tiles", serde_json::json!(on));
        });
    }

    // ---- the object ------------------------------------------------------
    // a fresh card of a type: id is <owner>.<created ms> so a copy is
    // recognisable as the same card wherever it lands. A profile's default
    // body is the three blocks the 👤 page asks for.

    fn card_new(owner: String, kind: String, now: u64) -> serde_json::Value {
        serde_json::json!({
            "id": format!("{}.{}", owner, now),
            "owner": owner,
            "type": kind,
            "created": now,
            "edited": now,
            "blocks": [
                { "kind": "title", "text": owner },
                { "kind": "picture", "data": "" },
                { "kind": "text", "text": "" }
            ],
            "links": []
        })
    }

    // the lookup a consumer asks with: the first card of this type, owned by
    // this owner — an empty owner means any. Returns the card as JSON, or an
    // empty string for "you do not hold one".
    fn card_of_type(list: String, owner: String, kind: String) -> String {
        let v: serde_json::Value = serde_json::from_str(&list)
            .unwrap_or(serde_json::Value::Null);
        let empty: Vec<serde_json::Value> = Vec::new();
        for c in v.as_array().unwrap_or(&empty) {
            if c["type"].as_str().unwrap_or("") != kind {
                continue;
            }
            if !owner.is_empty() && c["owner"].as_str().unwrap_or("") != owner {
                continue;
            }
            return c.to_string();
        }
        String::new()
    }

    // every stored string passes through here on its way to the screen: a
    // card's text is the user's, and a picture's data URL is arbitrary bytes.
    fn card_esc(s: String) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }

    // ---- the events ------------------------------------------------------
    // three, and each one reads the list, changes one thing, and writes it
    // back. A malformed var reads as the empty list rather than throwing.

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let kind = e["type"].as_str().unwrap_or("").to_string();
        if kind == "CardTiles" {
            cards_tiles_write(e["data"]["on"].as_bool().unwrap_or(false));
            return state;
        }
        if kind != "CardEnsure" && kind != "CardEdit" && kind != "CardPic" {
            return state;
        }
        let mut list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        if !list.is_array() {
            list = serde_json::json!([]);
        }
        let now = e["data"]["t"].as_u64().unwrap_or(0);
        let mut changed = false;
        if kind == "CardEnsure" {
            let owner = e["data"]["owner"].as_str().unwrap_or("").trim().to_string();
            let owner = if owner.is_empty() { "you".to_string() } else { owner };
            let ty = e["data"]["type"].as_str().unwrap_or("profile").to_string();
            if card_of_type(list.to_string(), owner.clone(), ty.clone()).is_empty() {
                list.as_array_mut().expect("cards is an array")
                    .push(card_new(owner, ty, now));
                changed = true;
            }
        } else {
            let id = e["data"]["id"].as_str().unwrap_or("").to_string();
            let at = e["data"]["i"].as_u64().unwrap_or(0) as usize;
            let text = e["data"]["text"].as_str().unwrap_or("").to_string();
            let data = e["data"]["data"].as_str().unwrap_or("").to_string();
            for c in list.as_array_mut().expect("cards is an array").iter_mut() {
                if c["id"].as_str().unwrap_or("") != id {
                    continue;
                }
                let mut hit = false;
                if let Some(blocks) = c["blocks"].as_array_mut() {
                    if at < blocks.len() {
                        if kind == "CardEdit" {
                            blocks[at]["text"] = serde_json::json!(text);
                        } else {
                            blocks[at]["data"] = serde_json::json!(data);
                        }
                        hit = true;
                    }
                }
                if hit {
                    c["edited"] = serde_json::json!(now);
                    changed = true;
                }
            }
        }
        if changed {
            cards_write(list.to_string());
        }
        state
    }

    // ---- the two renderings ----------------------------------------------
    // the page: one to five phone screens of blocks, scrolled and edited in
    // place. Editable blocks carry no data-ev on purpose — a click that ran
    // the loop would repaint #app out from under the caret.

    fn card_page_html(card: String) -> String {
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        let id = card_esc(c["id"].as_str().unwrap_or("").to_string());
        let empty: Vec<serde_json::Value> = Vec::new();
        let blocks = c["blocks"].as_array().unwrap_or(&empty);
        let mut out = format!("<div class=\"card-page\" data-card=\"{}\">", id);
        let mut at = 0usize;
        for b in blocks {
            let kind = b["kind"].as_str().unwrap_or("");
            let text = card_esc(b["text"].as_str().unwrap_or("").to_string());
            let data = card_esc(b["data"].as_str().unwrap_or("").to_string());
            if kind == "title" {
                out.push_str(&format!(
                    "<div class=\"card-title\" contenteditable=\"true\" data-card=\"{}\" data-block=\"{}\" data-ph=\"your name\">{}</div>",
                    id, at, text));
            } else if kind == "picture" {
                if data.is_empty() {
                    out.push_str(&format!(
                        "<div class=\"card-pic empty\" data-card=\"{}\" data-block=\"{}\">add a picture</div>",
                        id, at));
                } else {
                    out.push_str(&format!(
                        "<div class=\"card-pic\" data-card=\"{}\" data-block=\"{}\"><img src=\"{}\" alt=\"\"></div>",
                        id, at, data));
                }
            } else if kind == "text" {
                out.push_str(&format!(
                    "<div class=\"card-text\" contenteditable=\"true\" data-card=\"{}\" data-block=\"{}\" data-ph=\"say what you are here to do\">{}</div>",
                    id, at, text));
            }
            at += 1;
        }
        out.push_str("</div>");
        out
    }

    // the tile: picture and title, the thumbnail a grid or a list is made of.
    // The dictaphone's grid of recordings is its named future consumer (#p11);
    // nothing in /dictate is touched here.
    fn card_tile_html(card: String) -> String {
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        let id = card_esc(c["id"].as_str().unwrap_or("").to_string());
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut title = String::new();
        let mut pic = String::new();
        for b in c["blocks"].as_array().unwrap_or(&empty) {
            let kind = b["kind"].as_str().unwrap_or("");
            if kind == "title" && title.is_empty() {
                title = card_esc(b["text"].as_str().unwrap_or("").to_string());
            }
            if kind == "picture" && pic.is_empty() {
                pic = card_esc(b["data"].as_str().unwrap_or("").to_string());
            }
        }
        let face = if pic.is_empty() {
            let initial: String = title.chars().take(1).collect();
            format!("<div class=\"card-tile-face empty\">{}</div>", initial)
        } else {
            format!("<div class=\"card-tile-face\"><img src=\"{}\" alt=\"\"></div>", pic)
        };
        format!(
            "<div class=\"card-tile\" data-card=\"{}\">{}<div class=\"card-tile-title\">{}</div></div>",
            id, face, title)
    }

    // the tile's dev mount, and this node's only claim on the screen: with the
    // `tiles` var set — which the page half does only when the URL carries
    // ?cardtiles=1 — every held card's tile is drawn as a grid. The ?readout=
    // convention: nothing until it is asked for.
    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        if !cards_tiles_read() {
            return base;
        }
        let list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut grid = String::from("<div class=\"card-tiles\">");
        for c in list.as_array().unwrap_or(&empty) {
            grid.push_str(&card_tile_html(c.to_string()));
        }
        grid.push_str("</div>");
        format!("{}{}", base, grid)
    }
}
