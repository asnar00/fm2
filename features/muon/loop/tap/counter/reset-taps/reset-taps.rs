struct feature_ResetTaps;
impl feature_ResetTaps {
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["ev"].as_str().unwrap_or("") != "tap_reset" {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        // register semantics: zero locally now, VarSet sweeps the fleet
        Var::<u64>::global("tap_count").set(&mut s, &0);
        s.to_string()
    }

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        // the pill's own visibility condition, and only with something to reset
        if s["open_tool"].is_string() && s["open_tool"].as_str() != Some("taps") {
            return base;
        }
        let count = Var::<u64>::local("tap_count").get(&s);
        if count == 0 {
            return base;
        }
        format!("{}<div class=\"tap reset\" data-ev=\"tap_reset\">reset</div>", base)
    }
}
