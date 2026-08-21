struct feature_Enforced;
impl feature_Enforced {
    // the tickbox means it now.
    //
    // Three things happen in this one link, and the ORDER is load-bearing.
    // This node's provenance ties with the whole `/context` subtree and its
    // depth is greater, so it linearises last: this link is the OUTERMOST of
    // the update chain, outside `payload`'s re-freeze and outside `overlay`'s
    // apply.
    //
    // 1. the edit happens BEFORE the chain beneath runs, for two reasons that
    //    agree. `converge`'s link drains the turn's queued ops into `_send`,
    //    and it is deep inside this one — an op queued after it would sit in
    //    the outbox until the next event. And `payload` re-freezes both worlds
    //    on its way out, so an edit made first is the one the paint and its
    //    gates see.
    // 2. the chain runs.
    // 3. the map is published from the re-frozen view, so it carries this
    //    turn's own edit AND any `CtxUpdate` that arrived from another device
    //    during it.
    fn update(state: String, event: String) -> String {
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") == "click" {
            let ev = e["ev"].as_str().unwrap_or("").to_string();
            if let Some(path) = ev.strip_prefix("ftick_") {
                ftick(path.to_string());
            }
        }
        let state = existing.update(state, event);
        publish_ticks(state)
    }

    // the first paint. `boot()` runs init() then render(), so the map is in the
    // state before the chooser's page half has ever looked for it.
    fn init() -> String {
        publish_ticks(existing.init())
    }

    // one tick click, and the two verbs it chooses between.
    //
    // UNTICK is an explicit false: this user's own answer, which is what a
    // person means when they switch something off.
    //
    // RE-TICK of your own untick is a `clear`, not a set-true, and that is the
    // whole argument of
    // this node in one line. The old map stored only explicit choices and an
    // absent key meant on, so re-ticking removed the key and put the user back
    // under whatever the build decided. `enabled` is declared `inherit`, and
    // `clear` is exactly that: the var becomes absent again and resolves
    // through the shared layer to its default. Writing `true` would look
    // identical today and would silently detach that user from the layer
    // forever — the next thing an admin switched off for everyone would leave
    // them the only person still running it, with no way to tell.
    //
    // The local half and the wire half are separate on purpose. `apply_op` is
    // the arriving door and deliberately queues nothing, so the op is queued
    // beside it; `edit_op` is the local door and queues its own. Both give the
    // optimistic update the loop has always had — the tick moves before the
    // server has heard.
    fn ftick(path: String) {
        // this world's OWN answer, if it has one. A node switched off on the
        // shared layer is off here too, but not BY this user — and the two
        // cases want opposite verbs, which is why presence is asked for rather
        // than assumed.
        if ftick_own(path.clone()) == serde_json::json!(false) {
            edit_context(|c| {
                let _ = c.apply_op(&path, "enabled", "clear", serde_json::Value::Null);
            });
            context_op_queue("clear", &path, "enabled", serde_json::json!(true));
            return;
        }
        let on = with_context(|c| c.enabled_off_map())[&path] != serde_json::json!(false);
        edit_context(|c| {
            let _ = c.edit_op(&path, "enabled", serde_json::json!(!on));
        });
    }

    // fm:context-snapshot — this node reads the generated walker, once per
    // click, to ask one question the tick map cannot answer: is this world's
    // own `enabled` PRESENT, and what does it say? Null when the var is absent
    // and the answer is coming from the layer or the default.
    fn ftick_own(path: String) -> serde_json::Value {
        with_context(|c| {
            let empty: Vec<serde_json::Value> = Vec::new();
            let snap = c.snapshot();
            let mut found = serde_json::Value::Null;
            for v in snap.as_array().unwrap_or(&empty) {
                if v["name"].as_str() != Some("enabled") {
                    continue;
                }
                if v["path"].as_str() != Some(path.as_str()) {
                    continue;
                }
                if v["present"].as_bool().unwrap_or(false) {
                    found = v["value"].clone();
                }
            }
            found
        })
    }

    // the map, at the key the page has always read. Same format, new truth: it
    // is DERIVED from the context rather than stored, so it cannot disagree
    // with the gates — they are two readings of one field.
    fn publish_ticks(state: String) -> String {
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        s["feature_ticks"] = serde_json::json!(
            with_context(|c| c.enabled_off_map()).to_string());
        s.to_string()
    }
}
