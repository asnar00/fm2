struct feature_DoubleTaps;
impl feature_DoubleTaps {
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["ev"].as_str().unwrap_or("") != "tap_double" {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        // read the count the user can see, write twice it — register
        // semantics, the fleet converges like reset's zero does
        let doubled = Var::<u64>::local("tap_count").get(&s) * 2;
        Var::<u64>::global("tap_count").set(&mut s, &doubled);
        s.to_string()
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
