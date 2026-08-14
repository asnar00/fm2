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
        // a device-local counter: /sync escalates it to a shared one
        Var::<u64>::local("tap_count").add(&mut s, 1);
        s.to_string()
    }

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let count = Var::<u64>::local("tap_count").get(&s);
        let label = if count == 0 {
            "tap".to_string()
        } else {
            format!("taps: {}", count)
        };
        format!("{}<div class=\"tap\" data-ev=\"tap\">{}</div>", base, label)
    }
}
