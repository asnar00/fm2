struct feature_Events;
impl feature_Events {
    // base state: an empty JSON object. features extending init merge their
    // defaults into it.
    fn init() -> String {
        "{}".to_string()
    }

    // base update: state unchanged. features extend this chain, reacting to
    // their own events and transforming their own state keys.
    fn update(state: String, event: String) -> String {
        let _ = event;
        state
    }

    // the wasm entry: initial state -> initial html, as a {state, html} payload
    fn boot() -> String {
        let state = init();
        let html = render(state.clone());
        event_payload(state, html)
    }

    // one turn of the loop: unwrap {state, event}, update, render, re-wrap
    fn on_event(input: String) -> String {
        let v: serde_json::Value = serde_json::from_str(&input)
            .unwrap_or(serde_json::Value::Null);
        let state = v["state"].as_str().unwrap_or("{}").to_string();
        let event = v["event"].to_string();
        let new_state = update(state, event);
        let html = render(new_state.clone());
        event_payload(new_state, html)
    }

    fn event_payload(state: String, html: String) -> String {
        serde_json::json!({ "state": state, "html": html }).to_string()
    }
}
