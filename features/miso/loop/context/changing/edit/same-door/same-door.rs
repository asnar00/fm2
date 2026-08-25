struct feature_SameDoor;
impl feature_SameDoor {
    // the tooling POST stops being a second door. Rung 3 wrote this route
    // before ops existed, so it assigned straight into the world: no merge
    // discipline, no id, no log record of the shape the log replays, and no
    // word to the caller's other devices. It now mints the same `CtxOp` a
    // client sends and hands it to the same handler, so everything that
    // happens to an edit happens to this one.
    //
    // `existing.context_set` is deliberately NOT called: the chain beneath is
    // the old door (and `remember`'s link on it, which would log the edit a
    // second time). Untick this node and that door is what answers again.
    fn context_set(r: request) -> response {
        if r.tunnel && !authed(r.cookie.clone()) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        let parsed: Result<serde_json::Value, serde_json::Error> =
            serde_json::from_str(&r.body);
        let body = match parsed {
            Ok(v) => v,
            Err(e) => return context_edit_error(400, format!("body is not JSON: {}", e)),
        };
        let path = body["path"].as_str().unwrap_or("").to_string();
        let name = body["name"].as_str().unwrap_or("").to_string();
        if path.is_empty() || name.is_empty() {
            return context_edit_error(400,
                "expected a body of {\"path\": .., \"name\": .., \"value\": ..}".to_string());
        }
        let op = body["op"].as_str().unwrap_or("set").to_string();
        // a clear names no value, but the op carries one anyway so that every
        // record on the wire and in the log has the same shape
        let value = if op == "clear" {
            serde_json::json!(true)
        } else if body.get("value").is_none() {
            return context_edit_error(400,
                format!("no \"value\" given for {}/{}", path, name));
        } else {
            body["value"].clone()
        };
        if let Some(refusal) = counter_refusal(&path, &name, &op, &value) {
            return context_edit_error(400, refusal);
        }
        context_op_post(&body, path, name, op, value, r.cookie.clone())
    }

    // mint the op and hand it to the ordinary handler. `_from` is the same
    // cookie-proven identity `/messaging` stamps, so a repair made through the
    // tunnel reaches that person's other instances; localhost tooling editing a
    // `local:` world has no audience to reach and gets none.
    fn context_op_post(body: &serde_json::Value, path: String, name: String,
                       op: String, value: serde_json::Value,
                       cookie: String) -> response {
        let mut data = serde_json::json!({
            "path": path, "name": name, "op": op, "value": value,
            "id": context_tool_op_id(),
        });
        // the shared layer is addressable here for the same reason it is over
        // /msg, and refused by the same privilege check
        if let Some(at) = body["at"].as_str() {
            data["at"] = serde_json::json!(at);
        }
        let msg = serde_json::json!({
            "type": "CtxOp", "_from": sender_of(cookie), "data": data,
        });
        let reply: serde_json::Value =
            serde_json::from_str(&handle_msg(msg.to_string()))
                .unwrap_or(serde_json::Value::Null);
        if reply["type"].as_str() == Some("CtxUpdate") {
            return json_response(200, serde_json::json!({
                "ok": true, "value": reply["data"]["value"].clone()
            }).to_string());
        }
        if reply["ok"].as_bool() == Some(true) {
            return json_response(200, reply.to_string());   // an absorbed repeat
        }
        context_edit_error(400,
            reply["error"].as_str().unwrap_or("the edit was refused").to_string())
    }

    // a counter's absolute value is not an assignment: it is a reset, which
    // carries the epoch that makes every add minted before it stale. A bare
    // number would deserialise-fail deep inside the merge with a serde message
    // about tuples, so it is refused here, in the caller's vocabulary.
    fn counter_refusal(path: &String, name: &String, op: &String,
                       value: &serde_json::Value) -> Option<String> {
        if context_merge_of(path, name) != "counter" {
            return None;
        }
        if op != "set" || value.is_array() {
            return None;
        }
        Some(format!(
            "{}/{}: merge 'counter' — an absolute set is a RESET and needs the \
             epoch the log replays it under, so a bare value cannot be \
             accepted. Send {{\"op\":\"add\",\"value\":[<epoch>,<delta>]}} to \
             count, or {{\"op\":\"set\",\"value\":[<epoch+1>,<n>]}} to reset. \
             It reads {} now.",
            path, name, context_value_of(path, name)))
    }

    // fm:context-snapshot — the declared attributes, read the way `enforced`
    // reads presence: from the generated walker, so nothing here has to know
    // which vars exist.
    fn context_merge_of(path: &String, name: &String) -> String {
        context_var_field(path, name, "merge")
    }

    fn context_value_of(path: &String, name: &String) -> String {
        context_value_field(path, name)
    }

    fn context_var_field(path: &String, name: &String, field: &str) -> String {
        with_context(|c| {
            let empty: Vec<serde_json::Value> = Vec::new();
            let snap = c.snapshot();
            let mut found = String::new();
            for v in snap.as_array().unwrap_or(&empty) {
                if v["path"].as_str() == Some(path.as_str())
                    && v["name"].as_str() == Some(name.as_str()) {
                    found = v[field].as_str().unwrap_or("").to_string();
                }
            }
            found
        })
    }

    fn context_value_field(path: &String, name: &String) -> String {
        with_context(|c| {
            let empty: Vec<serde_json::Value> = Vec::new();
            let snap = c.snapshot();
            let mut found = "nothing".to_string();
            for v in snap.as_array().unwrap_or(&empty) {
                if v["path"].as_str() == Some(path.as_str())
                    && v["name"].as_str() == Some(name.as_str()) {
                    found = v["value"].to_string();
                }
            }
            found
        })
    }

    // ids only have to be unique within a world's seen-set, and this door is
    // one thread at a time: the clock plus a counter is enough, and says where
    // it came from when a human reads the log.
    fn context_tool_op_id() -> String {
        static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("tool-{}-{}", now_ms(), n)
    }
}
