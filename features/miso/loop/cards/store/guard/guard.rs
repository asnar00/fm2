struct feature_Guard;
impl feature_Guard {
    // the server's last word on a `cards` set: merge it into what the user's
    // world already holds before the op is applied. A set that dropped a card
    // the server holds is a stale write from a device that had not joined —
    // the card stays. Per card, the newer `edited` wins. A blank profile
    // arriving beside an existing one is an ensure that ran against an empty
    // world — it is discarded. Nothing here can lose a card that existed.
    fn handle_msg(msg: String) -> String {
        let mut m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        if !cards_guard_applies(&m) {
            return existing.handle_msg(msg);
        }
        let incoming: serde_json::Value = serde_json::from_str(
            m["data"]["value"].as_str().unwrap_or("")).unwrap_or(serde_json::Value::Null);
        if !incoming.is_array() {
            return existing.handle_msg(msg);
        }
        let current: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        let merged = cards_guard_merge(current, incoming);
        m["data"]["value"] = serde_json::json!(merged.to_string());
        existing.handle_msg(m.to_string())
    }

    fn cards_guard_applies(m: &serde_json::Value) -> bool {
        m["type"].as_str().unwrap_or("") == "CtxOp"
            && m["data"]["path"].as_str().unwrap_or("") == "miso/loop/cards"
            && m["data"]["name"].as_str().unwrap_or("") == "cards"
            && m["data"]["op"].as_str().unwrap_or("set") == "set"
    }

    // union by id: every held card survives, the newer edit of a shared id
    // wins (ties to the incoming), new cards append — except a blank profile
    // for an owner who already has one.
    fn cards_guard_merge(current: serde_json::Value, incoming: serde_json::Value) -> serde_json::Value {
        let empty: Vec<serde_json::Value> = Vec::new();
        let cur = current.as_array().unwrap_or(&empty);
        let inc = incoming.as_array().unwrap_or(&empty);
        let mut out: Vec<serde_json::Value> = Vec::new();
        for c in cur {
            let id = c["id"].as_str().unwrap_or("");
            let mut keep = c.clone();
            for i in inc {
                if i["id"].as_str().unwrap_or("") == id
                    && i["edited"].as_u64().unwrap_or(0) >= c["edited"].as_u64().unwrap_or(0) {
                    keep = i.clone();
                }
            }
            out.push(keep);
        }
        for i in inc {
            let id = i["id"].as_str().unwrap_or("");
            if cur.iter().any(|c| c["id"].as_str().unwrap_or("") == id) {
                continue;
            }
            if card_is_blank(i) && cards_guard_has_type(cur, i) {
                println!("cards: dropped a blank {} card for {} — one already exists",
                         i["type"].as_str().unwrap_or("?"), i["owner"].as_str().unwrap_or("?"));
                continue;
            }
            out.push(i.clone());
        }
        serde_json::Value::Array(out)
    }

    fn cards_guard_has_type(cur: &Vec<serde_json::Value>, card: &serde_json::Value) -> bool {
        cur.iter().any(|c| c["type"] == card["type"] && c["owner"] == card["owner"])
    }

    // blank: no block carries text or picture data beyond the seeded title
    fn card_is_blank(card: &serde_json::Value) -> bool {
        let empty: Vec<serde_json::Value> = Vec::new();
        for b in card["blocks"].as_array().unwrap_or(&empty) {
            let kind = b["kind"].as_str().unwrap_or("");
            if kind == "title" {
                continue;
            }
            if !b["text"].as_str().unwrap_or("").trim().is_empty()
                || !b["data"].as_str().unwrap_or("").is_empty() {
                return false;
            }
        }
        true
    }
}
