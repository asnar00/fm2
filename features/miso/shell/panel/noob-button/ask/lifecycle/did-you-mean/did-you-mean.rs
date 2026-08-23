struct feature_DidYouMean;
impl feature_DidYouMean {
    // one tap settles the intent: the answer is stamped on the ask and the
    // status goes back to `asked`, so the ask is actionable again and the
    // bench's monitor fires on the restamp without knowing this node exists.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "AskAnswer" {
            return state;
        }
        let choice = e["data"]["choice"].as_str().unwrap_or("").to_string();
        if choice.is_empty() {
            return state;
        }
        let t = e["data"]["t"].as_u64().unwrap_or(0);
        let raw = asks_read();
        let mut asks: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::json!([]));
        let mut hit = false;
        if let Some(arr) = asks.as_array_mut() {
            for entry in arr.iter_mut() {
                if entry["t"].as_u64().unwrap_or(0) != t {
                    continue;
                }
                // a chip the entry never offered is not an answer: a stale
                // page, or a hand-made event, changes nothing here
                if !answer_offered(entry.clone(), choice.clone()) {
                    continue;
                }
                // the same tap twice is the same answer: a stale page may
                // repeat it, and repeating it must cost the log nothing
                if entry["answer"].as_str().unwrap_or("") == choice
                    && entry["status"].as_str().unwrap_or("") == "asked" {
                    continue;
                }
                entry["answer"] = serde_json::json!(choice);
                entry["status"] = serde_json::json!("asked");
                hit = true;
            }
        }
        // no matching entry, no write: an answer for a timestamp this world
        // does not carry must not grow the op log
        if hit {
            asks_write(asks.to_string());
        }
        state
    }

    // is this choice one of the readings the question actually offered?
    fn answer_offered(entry: serde_json::Value, choice: String) -> bool {
        let empty: Vec<serde_json::Value> = Vec::new();
        let opts = entry["question"]["options"].as_array()
            .unwrap_or(&empty).clone();
        for o in opts {
            if o["key"].as_str().unwrap_or("") == choice {
                return true;
            }
        }
        false
    }
}
