struct feature_Review;
impl feature_Review {
    // the one OK: stamp the accepted build on the user; /scope ships it to
    // every instance, whose page halves apply the build on arrival.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "AcceptUpdate" {
            return state;
        }
        let build = e["data"]["build"].as_i64().unwrap_or(0);
        if build <= 0 {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let prev: i64 = SyncVar::<String>::user("update_accepted").get(&s)
            .parse().unwrap_or(0);
        if build > prev {
            SyncVar::<String>::user("update_accepted").set(&mut s, &build.to_string());
        }
        s.to_string()
    }
}
