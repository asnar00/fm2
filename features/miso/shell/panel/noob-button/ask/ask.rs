struct feature_Ask;
impl feature_Ask {
    // an Ask event appends the wish to the user-scoped asks list — the
    // feature_ticks pattern: a JSON list in a string var, /scope carries it
    // to the user's instances and persists it on the server for the dev loop
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "Ask" {
            return state;
        }
        let text = e["data"]["text"].as_str().unwrap_or("").trim().to_string();
        if text.is_empty() {
            return state;
        }
        // the asks list is a declared /var now: read it resolved, write it
        // back through the merge column. The `js:asks` column republishes it
        // into the payload, so the panel fragments read `s.asks` unchanged.
        let raw = asks_read();
        let mut asks: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::json!([]));
        if !asks.is_array() {
            asks = serde_json::json!([]);
        }
        if let Some(arr) = asks.as_array_mut() {
            arr.push(serde_json::json!({
                "t": e["data"]["t"].as_u64().unwrap_or(0),
                "text": text,
                "status": "asked"
            }));
        }
        asks_write(asks.to_string());
        state
    }

    // the two accessors this feature's subnodes share, so the /var's address is
    // written once and its shape stays this node's business.
    fn asks_read() -> String {
        with_context(|c| c.ask_asks_get())
    }

    fn asks_write(list: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/shell/panel/noob-button/ask", "asks",
                              serde_json::json!(list));
        });
    }
}
