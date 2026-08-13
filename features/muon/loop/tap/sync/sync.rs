struct feature_Sync;
impl feature_Sync {
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if e["ev"].as_str().unwrap_or("") == "tap" {
            // send the tap through the pipe (drained by /messaging)
            if !s["_send"].is_array() {
                s["_send"] = serde_json::json!([]);
            }
            s["_send"].as_array_mut().expect("_send is array").push(
                serde_json::json!({ "type": "TapSync", "data": {} }));
        }
        if e["type"].as_str().unwrap_or("") == "TapTotal" {
            // the authoritative total overwrites the local count
            s["tap_count"] = e["data"]["total"].clone();
        }
        s.to_string()
    }

    fn handle_msg(msg: String) -> String {
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        if m["type"].as_str().unwrap_or("") != "TapSync" {
            return existing.handle_msg(msg);
        }
        let total = taps_total() + 1;
        let _ = std::fs::write(taps_file(), format!("{}", total));
        let reply = serde_json::json!(
            { "type": "TapTotal", "data": { "total": total } }).to_string();
        publish(reply.clone());
        reply
    }

    fn taps_file() -> String {
        "/tmp/muon-taps.txt".to_string()
    }

    fn taps_total() -> u64 {
        std::fs::read_to_string(taps_file())
            .unwrap_or_default().trim().parse().unwrap_or(0)
    }
}
