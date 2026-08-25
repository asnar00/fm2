struct feature_Parity;
impl feature_Parity {
    // fm:context-snapshot — the linker's hook this node reads through: the
    // generated walker over every declared var is what a join answers with, so
    // adding a var to the composition needs no line here. (`alive` asks for it
    // too; the token is a presence test, so two askers cost nothing.)
    //
    // the server's half: a Join reply grows a `ctx` list.
    //
    // The trigger is the one that already exists. `/join` queues `{"type":
    // "Join"}` at boot, and `/resume` re-queues it on foreground return and on
    // the browser's `online` event — boot, reconnect and resume are already one
    // act, and the reason `/resume` exists is the same fifty-entry hole this
    // node exists to close. Inventing a second trigger would have meant a
    // second thing to remember to fire.
    //
    // The reply is one message, so the records ride the existing envelope as a
    // sibling of `values` rather than as a second reply that cannot exist.
    // `/join` reads `data.values`, this node reads `data.ctx`, and neither
    // knows about the other's field.
    fn handle_msg(msg: String) -> String {
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        if m["type"].as_str().unwrap_or("") != "Join" {
            return existing.handle_msg(msg);
        }
        let mut reply: serde_json::Value =
            serde_json::from_str(&existing.handle_msg(msg))
                .unwrap_or(serde_json::Value::Null);
        // with /join unticked the chain answers `{}`, which has no envelope to
        // ride: make one, so the context's join does not depend on SyncVar's.
        if !reply.is_object() || reply["type"].as_str().unwrap_or("").is_empty() {
            reply = serde_json::json!({ "type": "VarJoin", "data": {} });
        }
        reply["data"]["ctx"] = ctx_join_records();
        reply.to_string()
    }

    // what a joining instance is owed: this user's world, and the layer's.
    //
    // Both are read through the generated snapshot — the same walker, the same
    // two addressing strings, the same serialisation `GET /diag/context` prints
    // — so there is no second format and nothing to keep in step with the
    // declarations. Both reads are frozen for this request, so the parcel is
    // one consistent moment rather than a mix of two.
    fn ctx_join_records() -> serde_json::Value {
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        let mine = with_context(|c| c.snapshot());
        for v in mine.as_array().unwrap_or(&empty) {
            if ctx_join_skip(v.clone(), false) {
                continue;
            }
            out.push(ctx_join_record(v.clone(), false));
        }
        let theirs = context_layer(|g| Some(g.snapshot()))
            .unwrap_or(serde_json::json!([]));
        for v in theirs.as_array().unwrap_or(&empty) {
            if ctx_join_skip(v.clone(), true) {
                continue;
            }
            out.push(ctx_join_record(v.clone(), true));
        }
        serde_json::Value::Array(out)
    }

    // three reasons a var says nothing at join, and each one is a rule rather
    // than an optimisation.
    //
    // DEVICE scope never left the device it was set on, so the server's copy is
    // not an authority about anybody's phone and sending it would be a lie.
    //
    // ABSENT means never touched — the whole point of the presence bit — and a
    // joiner that received it would have `present` set by the write path and
    // would stop inheriting. Silence is what keeps a fresh instance resolving
    // through the layer like everyone else. This is also what bounds the
    // parcel: it is one record per var this user has actually touched, not one
    // per var declared.
    //
    // A GLOBAL var's authority is the layer, so its record comes from the layer
    // half; the field every user carries for it is unread ballast.
    //
    // And the mirror of that, which is the ballast in the other direction: the
    // layer is a `Context` like any other, so it carries a field — and a
    // present bit — for vars whose resolver will never look at it. An `own`
    // var that is not `global`-scoped answers from its own field and nothing
    // else, so its layer entry is unreadable by construction. Sending it would
    // have put five records of nobody's value in every parcel.
    fn ctx_join_skip(v: serde_json::Value, at_layer: bool) -> bool {
        let scope = v["scope"].as_str().unwrap_or("");
        if scope == "device" {
            return true;
        }
        if !v["present"].as_bool().unwrap_or(false) {
            return true;
        }
        if scope == "global" && !at_layer {
            return true;
        }
        if at_layer && scope != "global" && v["inherit"].as_str().unwrap_or("") == "own" {
            return true;
        }
        false
    }

    // one record, in the shape an arriving CtxUpdate already has — so the
    // client applies it through the door that exists rather than a new one.
    fn ctx_join_record(v: serde_json::Value, at_layer: bool) -> serde_json::Value {
        serde_json::json!({
            "path": v["path"].clone(),
            "name": v["name"].clone(),
            "value": v["value"].clone(),
            "at": if at_layer { "global" } else { "user" },
            "present": true,
        })
    }

    // the client's half: apply the parcel, inside the paint's turn.
    //
    // This link sits between `overlay`'s and `payload`'s, which is load-bearing
    // in one direction: `payload` re-freezes both worlds and republishes AFTER
    // the chain beneath it, so a record applied here is in the state key a
    // fragment reads before this very paint. Applied outside that link the
    // values would be true and invisible until the next event.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "VarJoin" {
            return state;
        }
        let empty: Vec<serde_json::Value> = Vec::new();
        for rec in e["data"]["ctx"].as_array().unwrap_or(&empty) {
            ctx_join_apply(rec.clone());
        }
        state
    }

    // a record carries the RESOLVED value of a var that is present, so it is
    // applied by assignment — idempotent by construction, which is why a join
    // needs no op id and no seen-set: applying the same parcel twice is
    // applying it once. Assignment also queues nothing, so a join puts nothing
    // on the wire and cannot echo.
    fn ctx_join_apply(rec: serde_json::Value) {
        if rec["at"].as_str().unwrap_or("user") == "global" {
            let _ = ctx_apply_update(rec);
            return;
        }
        let path = rec["path"].as_str().unwrap_or("").to_string();
        let name = rec["name"].as_str().unwrap_or("").to_string();
        let value = rec["value"].clone();
        let _ = edit_context(|c| c.set_from_json(&path, &name, value.clone()));
    }
}
