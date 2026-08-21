struct feature_Queue;
impl feature_Queue {
    // a tick click toggles that build's entry in the user-scoped choice map.
    // Absent key = the default (ticked); we store only explicit choices, so
    // the map stays small and the default can evolve without rewriting it.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return state;
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        let build = match ev.strip_prefix("qtick_") {
            Some(b) => b.to_string(),
            None => return state,
        };
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let raw = SyncVar::<String>::user("update_ticks").get(&s);
        let mut ticks: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::json!({}));
        if !ticks.is_object() {
            ticks = serde_json::json!({});
        }
        let now_on = ticks[&build].as_bool().unwrap_or(true);
        ticks[&build] = serde_json::json!(!now_on);
        SyncVar::<String>::user("update_ticks").set(&mut s, &ticks.to_string());
        s.to_string()
    }
}
