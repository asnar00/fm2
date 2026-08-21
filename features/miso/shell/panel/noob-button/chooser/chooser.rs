struct feature_Chooser;
impl feature_Chooser {
    // a tick click toggles that node path in the user-scoped choice map.
    // Absent key = the default (on); only explicit choices are stored, so
    // the default can evolve without rewriting anyone's map. /queue's
    // pattern, with node paths as keys.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return state;
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        let path = match ev.strip_prefix("ftick_") {
            Some(p) => p.to_string(),
            None => return state,
        };
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let raw = SyncVar::<String>::user("feature_ticks").get(&s);
        let mut ticks: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::json!({}));
        if !ticks.is_object() {
            ticks = serde_json::json!({});
        }
        let now_on = ticks[&path].as_bool().unwrap_or(true);
        ticks[&path] = serde_json::json!(!now_on);
        SyncVar::<String>::user("feature_ticks").set(&mut s, &ticks.to_string());
        s.to_string()
    }
}
