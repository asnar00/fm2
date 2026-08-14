struct feature_Veil;
impl feature_Veil {
    // stamp the applied snapshot: gate linearises after /join, so by the time
    // this runs the values are already in state. _joined is page-local and
    // never shipped (only explicit _send ops leave the instance).
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "VarJoin" {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        s["_joined"] = serde_json::json!(true);
        s.to_string()
    }
}
