struct feature_Birthplace;
impl feature_Birthplace {
    // stamp the birthplace into the entry /ask just appended (matched by t)
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "Ask" {
            return state;
        }
        let tool = e["data"]["tool"].as_str().unwrap_or("").to_string();
        let at = e["data"]["at"].as_str().unwrap_or("").to_string();
        if tool.is_empty() && at.is_empty() {
            return state;
        }
        let t = e["data"]["t"].as_u64().unwrap_or(0);
        // the asks list is a declared /var now: read it resolved, write it
        // back through the merge column. The `js:asks` column republishes it
        // into the payload, so the panel fragments read `s.asks` unchanged.
        let raw = asks_read();
        let mut asks: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::json!([]));
        if let Some(arr) = asks.as_array_mut() {
            for entry in arr.iter_mut() {
                if entry["t"].as_u64().unwrap_or(0) == t {
                    if !tool.is_empty() {
                        entry["tool"] = serde_json::json!(tool);
                    }
                    if !at.is_empty() {
                        entry["at"] = serde_json::json!(at);
                    }
                }
            }
        }
        asks_write(asks.to_string());
        state
    }
}
