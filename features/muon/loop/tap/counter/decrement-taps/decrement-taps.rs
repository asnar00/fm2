struct feature_DecrementTaps;
impl feature_DecrementTaps {
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["ev"].as_str().unwrap_or("") != "tap_dec" {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let count = Var::<u64>::local("tap_count").get(&s);
        if count == 0 {
            return state; // the asked-for guard, and the u64 floor
        }
        let lowered = count - 1;
        Var::<u64>::global("tap_count").set(&mut s, &lowered);
        s.to_string()
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
