struct feature_Queue;
impl feature_Queue {
    // a tick click toggles that build's entry in the user-scoped choice /var.
    // Absent key = the default (ticked); we store only explicit choices, so
    // the map stays small and the default can evolve without rewriting it.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return state;
        }
        let ev = e["ev"].as_str().unwrap_or("").to_string();
        let build = match ev.strip_prefix("qtick_") {
            Some(b) => b.to_string(),
            None => return state,
        };
        let raw = update_ticks_read();
        let mut ticks: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::json!({}));
        if !ticks.is_object() {
            ticks = serde_json::json!({});
        }
        let now_on = ticks[&build].as_bool().unwrap_or(true);
        ticks[&build] = serde_json::json!(!now_on);
        update_ticks_write(ticks.to_string());
        state
    }

    // the address, written once. The closure clones because `edit_context`
    // replays it against this turn's frozen view and therefore runs it twice.
    fn update_ticks_read() -> String {
        with_context(|c| c.queue_update_ticks_get())
    }

    fn update_ticks_write(ticks: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/shell/update/policy/queue", "update_ticks",
                              serde_json::json!(ticks.clone()));
        });
    }
}
