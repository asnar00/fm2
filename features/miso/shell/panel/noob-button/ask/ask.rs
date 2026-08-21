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
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let raw = SyncVar::<String>::user("asks").get(&s);
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
        SyncVar::<String>::user("asks").set(&mut s, &asks.to_string());
        s.to_string()
    }
}
