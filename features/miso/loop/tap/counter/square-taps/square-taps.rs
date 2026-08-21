struct feature_SquareTaps;
impl feature_SquareTaps {
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["ev"].as_str().unwrap_or("") != "tap_square" {
            return state;
        }
        // read the count the user can see, write its square — a reset to a
        // computed number, so the fleet converges the way zero does
        let n = tap_count_read();
        tap_count_reset(n.saturating_mul(n));
        state
    }

    // the sub-tool idiom: n² rides the toolbar while taps is open
    fn tool_controls(state: String) -> String {
        let prev = existing.tool_controls(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["open_tool"].as_str().unwrap_or("") != "taps" {
            return prev;
        }
        format!("{}<div class=\"tool-button ctrl\" data-ev=\"tap_square\" title=\"square\">n\u{00b2}</div>", prev)
    }
}
