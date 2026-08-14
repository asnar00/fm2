struct feature_Join;
impl feature_Join {
    // boot half: queue the Join through the state outbox — the canonical
    // send path, so a replayed boot re-queues it and replay-mode messaging
    // correctly declines to deliver it.
    fn init() -> String {
        let state = existing.init();
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if !s["_send"].is_array() {
            s["_send"] = serde_json::json!([]);
        }
        s["_send"].as_array_mut().expect("_send is array")
            .push(serde_json::json!({ "type": "Join" }));
        s.to_string()
    }

    // server half: a Join reply is a snapshot of every var the sender may
    // hear — its /scope audience: global, plus its own user scope.
    fn handle_msg(msg: String) -> String {
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        if m["type"].as_str().unwrap_or("") != "Join" {
            return existing.handle_msg(msg);
        }
        let from = m["_from"].as_str().unwrap_or("").to_string();
        let values = snapshot_vars(from);
        serde_json::json!({ "type": "VarJoin", "data": { "values": values } })
            .to_string()
    }

    // read the var store, keeping entries in the sender's hearable scopes and
    // stripping the scope prefix back to the bare state key. user entries
    // overwrite same-named global ones (more specific wins).
    fn snapshot_vars(from: String) -> serde_json::Value {
        let mut values = serde_json::json!({});
        let user_prefix = format!("user.{}.", from);
        for prefix in ["global.".to_string(), user_prefix] {
            if prefix == "user.." {
                continue; // unauthenticated sender: global only
            }
            for (store_key, value) in stored_vars() {
                if let Some(bare) = store_key.strip_prefix(&prefix) {
                    values[bare] = value;
                }
            }
        }
        values
    }

    fn stored_vars() -> Vec<(String, serde_json::Value)> {
        let mut vars = Vec::new();
        let entries = match std::fs::read_dir(var_dir()) {
            Ok(e) => e,
            Err(_) => return vars,
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(store_key) = name.strip_suffix(".json") {
                vars.push((store_key.to_string(),
                           var_read(store_key.to_string())["v"].clone()));
            }
        }
        vars
    }

    // client half: VarJoin is the plural of /scope's VarUpdate — write every
    // snapshot entry into state under its bare key, then render repaints.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "VarJoin" {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if let Some(values) = e["data"]["values"].as_object() {
            for (key, value) in values {
                s[key] = value.clone();
            }
        }
        s.to_string()
    }
}
