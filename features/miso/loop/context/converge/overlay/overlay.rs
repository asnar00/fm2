struct feature_Overlay;
impl feature_Overlay {
    // the server's layer boundary. This link is outermost, so the layer freezes
    // BEFORE rung 3's turn freezes the user's own world and thaws after it: one
    // turn, two frozen views, and a resolved read that cannot change its mind
    // halfway through a request.
    fn route(r: request) -> response {
        let _ = context_reside(context_layer_key(), &context_layer_cell());
        context_layer_begin();
        if r.path == "diag/context/layer" {
            let body = context_layer(|g| Some(g.snapshot().to_string()))
                .unwrap_or("[]".to_string());
            context_layer_end();
            return json_response(200, body);
        }
        let resp = existing.route(r);
        context_layer_end();
        resp
    }

    // may this caller write the shared layer? Only the localhost tooling
    // identity, which rung 5 namespaced as `local:`. A logged-in user reaching
    // through the tunnel may set their OWN vars and no one else's — setting the
    // layer changes what everybody who never overrode it sees, which is a
    // privilege this ladder has not been asked to grant (see overlay.md).
    fn ctx_may_write_layer() -> bool {
        context_user_now().starts_with("local:")
    }

    // the client's layer boundary, around the same event rung 3 makes a turn.
    // No residency here: the wasm place has no clock and no log, so its layer
    // is whatever ops have reached it.
    fn on_event(input: String) -> String {
        context_layer_begin();
        let out = existing.on_event(input);
        context_layer_end();
        out
    }

    // say hello at boot, so this instance can be told who it is. Queued through
    // the state outbox like every other message, so it survives being offline
    // and rides the same retry.
    fn init() -> String {
        let state = existing.init();
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if !s["_send"].is_array() {
            s["_send"] = serde_json::json!([]);
        }
        s["_send"].as_array_mut().expect("_send is array")
            .push(serde_json::json!({ "type": "CtxHello" }));
        s.to_string()
    }

    // the client's two jobs on the update chain: retype a record that belongs
    // to the layer, and apply it. Stamping the outbox used to be the third and
    // had to be last, which it stopped being when nodes newer than this one
    // arrived; it now runs in `/turn-end`'s phase, straight after the drain it
    // has always had to follow. `ctx_stamp_outbox` below is unchanged.
    fn update(state: String, event: String) -> String {
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let kind = e["type"].as_str().unwrap_or("").to_string();
        // a record that belongs to the LAYER is retyped before it enters the
        // chain beneath. Otherwise rung 6's link would write it into this
        // user's own world, which for an inherit var would silently detach
        // every user the layer was meant to reach. Renaming rather than
        // suppressing keeps every other link seeing exactly what it did.
        let mut passed = event.clone();
        if kind == "CtxUpdate" && e["data"]["at"].as_str().unwrap_or("user") == "global" {
            let mut hidden = e.clone();
            hidden["type"] = serde_json::json!("CtxUpdateAtLayer");
            passed = hidden.to_string();
        }
        let state = existing.update(state, passed);
        if kind == "CtxNonce" {
            context_instance_set(e["data"]["nonce"].as_str().unwrap_or("").to_string());
        }
        if kind == "CtxUpdate" {
            ctx_apply_update(e["data"].clone());
        }
        state
    }

    // an arriving CtxUpdate carries the resolved value, which layer it belongs
    // to, and whether the var is present there at all. rung 6's link has
    // already written the value into this user's own world; when the record
    // belongs to the layer that write is dead ballast (a global var's resolver
    // never reads the user's field), and the live one happens here.
    fn ctx_apply_update(data: serde_json::Value) -> String {
        let path = data["path"].as_str().unwrap_or("").to_string();
        let name = data["name"].as_str().unwrap_or("").to_string();
        let at_layer = data["at"].as_str().unwrap_or("user") == "global";
        let present = data["present"].as_bool().unwrap_or(true);
        let value = data["value"].clone();
        if at_layer {
            let cell = context_layer_cell();
            let mut layer = cell.write().unwrap_or_else(|p| p.into_inner());
            if present {
                let _ = layer.set_from_json(&path, &name, value);
            } else {
                let _ = layer.apply_op(&path, &name, "clear", serde_json::Value::Null);
            }
            return "layer".to_string();
        }
        if !present {
            let _ = edit_context(|c| c.apply_op(&path, &name, "clear",
                                                serde_json::Value::Null));
        }
        "own".to_string()
    }

    // stamp every unstamped CtxOp in the outbox with this instance's next id.
    // Doing it here rather than where the op is minted means one place knows
    // about identity, and a message the transport has to re-send carries the
    // id it was stamped with the first time.
    fn ctx_stamp_outbox(state: String) -> String {
        if context_instance_now().is_empty() {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let mut touched = false;
        if let Some(outbox) = s["_send"].as_array_mut() {
            for m in outbox.iter_mut() {
                if m["type"].as_str().unwrap_or("") == "CtxOp"
                    && !m["data"]["id"].is_string() {
                    m["data"]["id"] = serde_json::json!(context_op_next_id());
                    touched = true;
                }
            }
        }
        if touched {
            return s.to_string();
        }
        state
    }

    // the server's half: identity for a new instance, duplicate suppression,
    // and routing a global var's op to the layer instead of the sender's world.
    fn handle_msg(msg: String) -> String {
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        let kind = m["type"].as_str().unwrap_or("").to_string();
        if kind == "CtxHello" {
            return serde_json::json!({
                "type": "CtxNonce", "data": { "nonce": context_mint_nonce() }
            }).to_string();
        }
        if kind != "CtxOp" {
            return existing.handle_msg(msg);
        }
        let path = m["data"]["path"].as_str().unwrap_or("").to_string();
        let name = m["data"]["name"].as_str().unwrap_or("").to_string();
        let op = m["data"]["op"].as_str().unwrap_or("").to_string();
        let id = m["data"]["id"].as_str().unwrap_or("").to_string();
        // a global-scoped var's authority is always the layer; any other var's
        // LAYER value — the fallback a user who never overrode it inherits — is
        // addressable by asking for it, and only by the tooling identity.
        let asked = m["data"]["at"].as_str().unwrap_or("user") == "global";
        if asked && !ctx_may_write_layer() {
            return serde_json::json!({ "ok": false, "error":
                "only local tooling may write the shared layer" }).to_string();
        }
        let global = Context::scope_of(&path, &name) == Some("global") || asked;
        let who = if global {
            context_layer_key().to_string()
        } else {
            context_user_now()
        };
        context_seen_prime(&who);
        if !context_seen_mark(&who, &id) {
            // acknowledged and skipped: the sender is told the op is in, so its
            // outbox stops retrying, and nothing is applied a second time.
            return serde_json::json!({
                "ok": true, "duplicate": id
            }).to_string();
        }
        if op == "clear" {
            return ctx_op_clear(m, global);
        }
        if !global {
            return existing.handle_msg(msg);
        }
        // a global var's authority is the layer, so the request acts as the
        // layer for the duration of the call: rung 6 applies the op and rung 6a
        // logs it, both addressing `_global` because that is who this thread
        // now is. No interception, no second apply, no second log format.
        //
        // The sender is anonymised for the same reason and by the same move:
        // rung 6's link relays to `user.<_from>`, and this op is on its way to
        // EVERYONE — the sender included, since they are in "everyone". Leaving
        // the identity on would put a second copy of the same record in the
        // 50-entry backlog, ageing it out faster for every other instance.
        let was = context_user_now();
        context_user_set(context_layer_key().to_string());
        let inner = existing.handle_msg(anonymised(msg));
        context_user_set(was);
        ctx_after_layer(m, inner)
    }

    // an evicted user's dedupe state goes with them. Both halves are rebuilt
    // from their log by `context_seen_prime` the next time this process meets
    // them, so this costs one re-read and stops an evicted user costing memory
    // for the life of the process.
    fn context_evicted(users: Vec<String>) {
        for user in &users {
            context_seen_forget(user);
        }
        existing.context_evicted(users)
    }

    // the same message with no sender on it. The identity that matters has
    // already been used — it was checked for the privilege to write the layer —
    // and what remains of it downstream is only an audience.
    fn anonymised(msg: String) -> String {
        let mut m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        if m.is_object() {
            m["_from"] = serde_json::json!("");
        }
        m.to_string()
    }

    // the relay half of a layer op: what came back from rung 6 carries the
    // resolved value; everybody connected hears it.
    fn ctx_after_layer(m: serde_json::Value, inner: String) -> String {
        let answered: serde_json::Value = serde_json::from_str(&inner)
            .unwrap_or(serde_json::Value::Null);
        if answered["type"].as_str().unwrap_or("") != "CtxUpdate" {
            return inner;
        }
        ctx_relay(m["data"]["path"].clone(), m["data"]["name"].clone(),
                  answered["data"]["value"].clone(), true, true)
    }

    // `clear` is this rung's verb, so it is applied here rather than through
    // rung 6's link — which would reject it, since its merge column knows only
    // set and add.
    fn ctx_op_clear(m: serde_json::Value, at_layer: bool) -> String {
        let path = m["data"]["path"].as_str().unwrap_or("").to_string();
        let name = m["data"]["name"].as_str().unwrap_or("").to_string();
        let was = context_user_now();
        if at_layer {
            context_user_set(context_layer_key().to_string());
        }
        let who = context_user_now();
        let outcome = edit_context(|c| c.apply_op(&path, &name, "clear",
                                                  serde_json::Value::Null));
        if !who.is_empty() {
            if outcome.is_ok() {
                context_log_append(&who, serde_json::json!({
                    "path": path.clone(), "name": name.clone(), "op": "clear",
                    "value": serde_json::Value::Bool(true),
                    "id": m["data"]["id"].clone(),
                }));
            }
        }
        if at_layer {
            context_user_set(was.clone());
        }
        let resolved = match outcome {
            Ok(v) => v,
            Err(e) => return serde_json::json!({ "ok": false, "error": e }).to_string(),
        };
        let update = ctx_relay(serde_json::json!(path), serde_json::json!(name),
                               resolved, at_layer, false);
        if !at_layer {
            let from = m["_from"].as_str().unwrap_or("").to_string();
            if !from.is_empty() {
                publish(format!("user.{}", from), update.clone());
            }
        }
        update
    }

    // one CtxUpdate, and who hears it. A layer op reaches everybody through the
    // audience /messaging already filters as "global"; a user's own op is rung
    // 6's business and is already on its way to that user's instances.
    fn ctx_relay(path: serde_json::Value, name: serde_json::Value,
                 value: serde_json::Value, at_layer: bool, present: bool) -> String {
        let update = serde_json::json!({
            "type": "CtxUpdate",
            "data": {
                "path": path, "name": name, "value": value,
                "at": if at_layer { "global" } else { "user" },
                "present": present,
            }
        }).to_string();
        if at_layer {
            publish("global".to_string(), update.clone());
        }
        update
    }
}
