struct feature_Converge;
impl feature_Converge {
    // fm:context-op — the linker's hook: this token is what asks for
    // Context::edit_op() and Context::apply_op(), the two generated halves of
    // the merge discipline. Untick this node and neither is emitted, and no
    // var type has to be anything it was not already.
    //
    // the client's two jobs, both at the outermost link of the update chain.
    // An arriving CtxUpdate is written to the LIVE context, which by rung 3's
    // construction is invisible to the turn now running and visible to the
    // next one — so a gate never changes its mind halfway through an event.
    // Then the ops this turn produced are handed to the state's `_send`
    // outbox, the same path every message on this system takes.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") == "CtxUpdate" {
            let path = e["data"]["path"].as_str().unwrap_or("").to_string();
            let name = e["data"]["name"].as_str().unwrap_or("").to_string();
            let value = e["data"]["value"].clone();
            // a CtxUpdate carries the RESOLVED value, so it is applied by
            // assignment — rung 3's write path, unchanged. Applying it as an
            // op would re-add a delta and re-queue an echo.
            let _ = edit_context(|c| c.set_from_json(&path, &name, value.clone()));
        }
        let ops = context_op_drain();
        if ops.is_empty() {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if !s["_send"].is_array() {
            s["_send"] = serde_json::json!([]);
        }
        for op in ops {
            s["_send"].as_array_mut().expect("_send is array").push(op);
        }
        s.to_string()
    }

    // the server half. A CtxOp applies to the SENDER's world without asking
    // who that is: rung 5's route link has already put the cookie-proven
    // identity on this thread, so edit_context addresses their table entry and
    // no payload can name someone else's.
    fn handle_msg(msg: String) -> String {
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        if m["type"].as_str().unwrap_or("") != "CtxOp" {
            return existing.handle_msg(msg);
        }
        let path = m["data"]["path"].as_str().unwrap_or("").to_string();
        let name = m["data"]["name"].as_str().unwrap_or("").to_string();
        let op = m["data"]["op"].as_str().unwrap_or("").to_string();
        if path.is_empty() || name.is_empty() || op.is_empty()
            || m["data"]["value"].is_null() {
            return ctx_op_error(
                "a CtxOp needs data.path, data.name, data.op and data.value".to_string());
        }
        let value = m["data"]["value"].clone();
        let resolved = match edit_context(|c| c.apply_op(&path, &name, &op, value.clone())) {
            Ok(v) => v,
            Err(e) => return ctx_op_error(e),
        };
        let update = serde_json::json!({
            "type": "CtxUpdate",
            "data": { "path": path, "name": name, "value": resolved }
        }).to_string();
        // relay to this user's other instances over the audience /messaging
        // already filters on. The reply carries the same record, so the
        // originator confirms immediately and the relay is a duplicate it can
        // absorb — a resolved value applied twice is the value.
        let from = m["_from"].as_str().unwrap_or("").to_string();
        if !from.is_empty() {
            publish(format!("user.{}", from), update.clone());
        }
        update
    }

    fn ctx_op_error(msg: String) -> String {
        serde_json::json!({ "ok": false, "error": msg }).to_string()
    }
}
