struct feature_Urgency;
impl feature_Urgency {
    // an ask says how urgent it is: the entry /ask files gains `urgency`
    // ("urgent" | "whenever") from the event, after the chain beneath has
    // filed it. Absent means whenever.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "Ask" {
            return state;
        }
        let urgency = e["data"]["urgency"].as_str().unwrap_or("whenever").to_string();
        let t = e["data"]["t"].as_u64().unwrap_or(0);
        let mut asks: serde_json::Value = serde_json::from_str(&asks_read())
            .unwrap_or(serde_json::json!([]));
        let mut hit = false;
        if let Some(arr) = asks.as_array_mut() {
            for a in arr.iter_mut() {
                if a["t"].as_u64().unwrap_or(0) == t {
                    a["urgency"] = serde_json::json!(urgency);
                    hit = true;
                }
            }
        }
        if hit {
            asks_write(asks.to_string());
        }
        state
    }
}
