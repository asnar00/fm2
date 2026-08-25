struct feature_Converge;
impl feature_Converge {
    // fm:context-op — the linker's hook: this token is what asks for
    // Context::edit_op() and Context::apply_op(), the two generated halves of
    // the merge discipline. Untick this node and neither is emitted, and no
    // var type has to be anything it was not already.
    //
    // the client's job on the update chain: an arriving CtxUpdate is written
    // to the LIVE context, which by rung 3's construction is invisible to the
    // turn now running and visible to the next one — so a gate never changes
    // its mind halfway through an event.
    //
    // Shipping this turn's ops used to happen here too, at what was then the
    // outermost link. It is not outermost any more and has not been for some
    // time, so an op minted by a newer node was minted after the drain and
    // waited for the next event (notes.md, "the late link's ops"). The drain
    // now runs in `/turn-end`'s phase, which is after every update link by
    // construction; `ctx_ship_ops` below is unchanged and is what that phase
    // calls.
    //
    // fm:turn-end-required — that move is why this node may not be composed
    // without the phase. Nothing here would fail to build; the ops would simply
    // queue and never leave.
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
        state
    }

    // the drain: everything the turn queued, appended to the state's `_send`
    // outbox — the same path every message on this system takes. It is called
    // from `/turn-end`'s phase now rather than from the link above, so an op
    // minted at any depth ships in the turn that minted it. Draining an empty
    // outbox is a no-op, so calling it twice in a turn changes nothing.
    fn ctx_ship_ops(state: String) -> String {
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
        let audience = ctx_relay_audience(m["_from"].as_str().unwrap_or("").to_string());
        if !audience.is_empty() {
            publish(audience, update.clone());
        }
        update
    }

    // who hears a relayed edit — a seam, so a later node can answer it
    // differently without this file changing its mind. The base answers the
    // SENDER's own audience, which is exactly what the relay above always did,
    // and an unsigned edit reaches nobody.
    fn ctx_relay_audience(from: String) -> String {
        if from.is_empty() {
            String::new()
        } else {
            format!("user.{}", from)
        }
    }

    fn ctx_op_error(msg: String) -> String {
        serde_json::json!({ "ok": false, "error": msg }).to_string()
    }
}
