struct feature_Payload;
impl feature_Payload {
    // fm:context-bridge — the linker's hook: this token asks for
    // Context::republish(), one line per var that named a page key. Untick this
    // node and no var may claim one.
    //
    // the paint's turn. The update that just ran was protected by rung 3's
    // frozen view — that is what makes it replayable — but the render that
    // follows it should show what is TRUE now, including a CtxUpdate that
    // arrived during this very event. So the views are re-frozen here, after
    // the update and before the paint: determinism where it is needed,
    // freshness where it is seen.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event);
        context_turn_begin();
        context_layer_begin();
        ctx_republish(state)
    }

    // the first paint. boot() runs init() and then render(), so a bridged var
    // is in the state before anything has rendered — the page never sees a
    // frame with the key missing.
    fn init() -> String {
        ctx_republish(existing.init())
    }

    fn ctx_republish(state: String) -> String {
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        with_context(|c| c.republish(&mut s));
        s.to_string()
    }
}
