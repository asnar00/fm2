struct feature_DoubleTaps;
impl feature_DoubleTaps {
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["ev"].as_str().unwrap_or("") != "tap_double" {
            return state;
        }
        // read the count the user can see, write twice it — a reset to a
        // computed number, so the fleet converges the way zero does
        let doubled = tap_count_read() * 2;
        tap_count_reset(doubled);
        state
    }

    // the sub-tool idiom: ×2 rides the toolbar while taps is open
    fn tool_controls(state: String) -> String {
        let prev = existing.tool_controls(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["open_tool"].as_str().unwrap_or("") != "taps" {
            return prev;
        }
        format!("{}<div class=\"tool-button ctrl\" data-ev=\"tap_double\" title=\"double\">\u{00d7}2</div>", prev)
    }
}
