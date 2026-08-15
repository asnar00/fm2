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

    // a sub-tool of taps: the reset control rides the toolbar while the
    // taps tool is open, the way record rides dictate
    fn tool_controls(state: String) -> String {
        let prev = existing.tool_controls(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["open_tool"].as_str().unwrap_or("") != "taps" {
            return prev;
        }
        format!("{}<div class=\"tool-button ctrl\" data-ev=\"tap_reset\" title=\"reset\">\u{21ba}</div>", prev)
    }
}
