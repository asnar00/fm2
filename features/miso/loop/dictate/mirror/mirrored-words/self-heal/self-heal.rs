struct feature_SelfHeal;
impl feature_SelfHeal {
    // an empty result may never replace a non-empty transcript: a failed
    // attempt reports a failure, not an erasure (#p21a)
    fn update(state: String, event: String) -> String {
        let before: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let after = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "Transcribed" {
            return after;
        }
        if !e["data"]["text"].as_str().unwrap_or("").is_empty() {
            return after;   // a real result: nothing to protect
        }
        let id = e["data"]["id"].as_str().unwrap_or("").to_string();
        // what did this device know a moment ago?
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut kept = serde_json::Value::Null;
        for f in before["dict_files"].as_array().unwrap_or(&empty) {
            if f["id"].as_str() == Some(id.as_str())
                && !f["transcript"].as_str().unwrap_or("").is_empty() {
                kept = f.clone();
            }
        }
        if kept.is_null() {
            return after;   // nothing was lost: an honest first failure
        }
        let mut s: serde_json::Value = serde_json::from_str(&after)
            .unwrap_or(serde_json::json!({}));
        if let Some(files) = s["dict_files"].as_array_mut() {
            for f in files {
                if f["id"].as_str() == Some(id.as_str()) {
                    f["transcript"] = kept["transcript"].clone();
                    f["t_rung"] = kept["t_rung"].clone();
                    f["t_grade"] = kept["t_grade"].clone();
                    if let Some(o) = f.as_object_mut() {
                        o.remove("t_err");   // not a failed note; not this device's work
                    }
                }
            }
        }
        transcribe(s.to_string())
    }
}
