struct feature_Logging;
impl feature_Logging {
    // the turn is the unit: arm the switches from the incoming state, let
    // the chain run (logging as it goes), then hand the gathered lines to
    // the page half through state — the same route `_send` takes.
    fn on_event(input: String) -> String {
        let v: serde_json::Value = serde_json::from_str(&input)
            .unwrap_or(serde_json::Value::Null);
        fm_log_arm(v["state"].as_str().unwrap_or("{}"));
        fm_log(format!("event {}", v["event"]));
        let out = existing.on_event(input);
        let lines = fm_log_drain();
        if lines.is_empty() {
            return out;
        }
        let mut o: serde_json::Value = serde_json::from_str(&out)
            .unwrap_or(serde_json::json!({}));
        let inner = o["state"].as_str().unwrap_or("{}").to_string();
        let mut s: serde_json::Value = serde_json::from_str(&inner)
            .unwrap_or(serde_json::json!({}));
        if !s["_log"].is_array() {
            s["_log"] = serde_json::json!([]);
        }
        for l in lines {
            s["_log"].as_array_mut().expect("_log is array").push(l);
        }
        o["state"] = serde_json::json!(s.to_string());
        o.to_string()
    }
}
