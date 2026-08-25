struct feature_New;
impl feature_New {
    // the one door for making a card of a type: `CardNew {owner, type,
    // title, t}` appends a card with the profile's three-block body — a
    // title, an empty picture, an empty paragraph — and opens it, so the
    // surface that asked lands on the page ready to write. Projects and
    // posts both come through here; a later type needs no new event.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "CardNew" {
            return state;
        }
        let owner = e["data"]["owner"].as_str().unwrap_or("").trim().to_string();
        let owner = if owner.is_empty() { "you".to_string() } else { owner };
        let kind = e["data"]["type"].as_str().unwrap_or("post").to_string();
        let title = e["data"]["title"].as_str().unwrap_or("").to_string();
        let now = e["data"]["t"].as_u64().unwrap_or(0);
        let mut list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        if !list.is_array() {
            list = serde_json::json!([]);
        }
        let mut card = card_new(owner, kind, now);
        card["blocks"][0]["text"] = serde_json::json!(title);
        let id = card["id"].as_str().unwrap_or("").to_string();
        list.as_array_mut().expect("cards is an array").push(card);
        cards_write(list.to_string());
        browse_open_write(id);
        state
    }
}
