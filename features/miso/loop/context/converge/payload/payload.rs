struct feature_Payload;
impl feature_Payload {
    // fm:context-bridge — the linker's hook: this token asks for
    // Context::republish(), one line per var that named a page key. Untick this
    // node and no var may claim one.
    //
    // the paint's freshness. The update that just ran was protected by rung 3's
    // frozen view — that is what makes it replayable — but the render that
    // follows it should show what is TRUE now, including a CtxUpdate that
    // arrived during this very event. An arriving record for the LAYER is
    // written straight to the live layer cell, so the layer's view is re-taken
    // here, before the paint: determinism where it is needed, freshness where
    // it is seen.
    //
    // The user's own view is NOT re-taken, and the `context_turn_begin()` that
    // used to be on the line above was a mistake rather than a mechanism: with
    // `/first-turn`'s depth counter in place it re-froze nothing, and having no
    // matching end it left the depth one higher after every event — so the
    // client's own view was taken at the first event and never again, and the
    // boundary law survived only because `edit_context` mirrors a turn's own
    // writes into it. It is gone. Everything that reaches the user's own world
    // during a turn goes through `edit_context`, whose read-your-own-writes
    // already shows it to the rest of the turn, paint included.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event);
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
