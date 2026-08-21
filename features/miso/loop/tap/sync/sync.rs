struct feature_Sync;
impl feature_Sync {
    // the whole shared-counter feature, post-generic: on a tap, ship the
    // already-applied increment as an op on the global counter. arrival,
    // server keying, storage and broadcast are /scope's generic machinery.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["ev"].as_str().unwrap_or("") != "tap" {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        SyncVar::<u64>::global("tap_count").add_op(&mut s, 1);
        s.to_string()
    }
}
