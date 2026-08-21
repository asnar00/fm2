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

    // server half: the reply. It carried a snapshot of the var store until
    // rung 8 deleted the store; what it carries now is `/parity`'s `ctx`, hung
    // on by the newer link outside this one. The envelope stays because two
    // nodes ride it — `/parity` reads `data.ctx`, `/veil` waits for the type —
    // and because "the joining moment" is this node's job whatever the payload
    // turns out to be. `values` stays as an empty object rather than
    // disappearing, so a client from before this build reads it as "nothing to
    // apply" instead of crashing on an absent field.
    fn handle_msg(msg: String) -> String {
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        if m["type"].as_str().unwrap_or("") != "Join" {
            return existing.handle_msg(msg);
        }
        serde_json::json!({ "type": "VarJoin", "data": { "values": {} } })
            .to_string()
    }
}
