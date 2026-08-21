struct feature_DecrementTaps;
impl feature_DecrementTaps {
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["ev"].as_str().unwrap_or("") != "tap_dec" {
            return state;
        }
        let count = tap_count_read();
        if count == 0 {
            return state; // the asked-for guard, and the u64 floor
        }
        // -1 is a reset to a computed number, like ×2: it opens a new epoch,
        // so a tap in flight cannot land on top of the lowered count
        tap_count_reset(count - 1);
        state
    }

    // the sub-tool idiom: -1 rides the toolbar while taps is open
    fn tool_controls(state: String) -> String {
        let prev = existing.tool_controls(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if s["open_tool"].as_str().unwrap_or("") != "taps" {
            return prev;
        }
        format!("{}<div class=\"tool-button ctrl\" data-ev=\"tap_dec\" title=\"decrement\">\u{2212}1</div>", prev)
    }
}
