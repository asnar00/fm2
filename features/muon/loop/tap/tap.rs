struct feature_Tap;
impl feature_Tap {
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["ev"].as_str().unwrap_or("") != "tap" {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let count = s["tap_count"].as_u64().unwrap_or(0) + 1;
        s["tap_count"] = serde_json::json!(count);
        s.to_string()
    }

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let count = s["tap_count"].as_u64().unwrap_or(0);
        let label = if count == 0 {
            "tap".to_string()
        } else {
            format!("taps: {}", count)
        };
        format!("{}<div class=\"tap\" data-ev=\"tap\">{}</div>", base, label)
    }
}
