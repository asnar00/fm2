struct feature_StoredWords;
impl feature_StoredWords {
    // ---- server: the index reply carries the words -------------------------
    // words.json sits beside the blobs, one {text, rung, grade} per recording,
    // written by an earlier build; index.json entries may carry a transcript
    // of their own. The reply is filled, the files are not rewritten.

    fn handle_msg(msg: String) -> String {
        let reply = existing.handle_msg(msg.clone());
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        if m["type"].as_str().unwrap_or("") != "RecIndex" {
            return reply;
        }
        let mut r: serde_json::Value = serde_json::from_str(&reply)
            .unwrap_or(serde_json::Value::Null);
        if r["type"].as_str().unwrap_or("") != "RecIndexed" {
            return reply;
        }
        let from = m["_from"].as_str().unwrap_or("").to_string();
        let user = if from.is_empty() { "_local".to_string() } else { from };
        let words = stored_words_read(user);
        if let Some(items) = r["data"]["items"].as_array_mut() {
            for it in items.iter_mut() {
                stored_words_fill(it, &words);
            }
        }
        r.to_string()
    }

    fn stored_words_read(user: String) -> serde_json::Value {
        let raw = std::fs::read_to_string(format!("{}/{}/words.json", blob_root(), user))
            .unwrap_or_default();
        serde_json::from_str(&raw).unwrap_or(serde_json::json!({}))
    }

    fn stored_words_fill(item: &mut serde_json::Value, words: &serde_json::Value) {
        if !item["transcript"].as_str().unwrap_or("").is_empty() {
            return;
        }
        let id = item["id"].as_str().unwrap_or("").to_string();
        let w = &words[id.as_str()];
        let text = w["text"].as_str().unwrap_or("").to_string();
        if text.is_empty() {
            return;
        }
        item["transcript"] = serde_json::json!(text);
        item["t_rung"] = if w["rung"].is_null() { serde_json::json!("local") } else { w["rung"].clone() };
        item["t_grade"] = if w["grade"].is_null() { serde_json::json!(1) } else { w["grade"].clone() };
    }

    // ---- client: a listed file learns its words -----------------------------
    // /mirror's merge adds files the device did not know and leaves the known
    // ones alone; this stamps the known ones, then lands the words in the post
    // on this same event.

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "RecIndexed" {
            return state;
        }
        let state = stored_words_stamp(state, &e["data"]["items"]);
        as_posts_sync(state)
    }

    fn stored_words_stamp(state: String, items: &serde_json::Value) -> String {
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut changed = false;
        if let Some(files) = s["dict_files"].as_array_mut() {
            for it in items.as_array().unwrap_or(&empty) {
                let text = it["transcript"].as_str().unwrap_or("");
                if text.is_empty() {
                    continue;
                }
                for f in files.iter_mut() {
                    if f["id"].as_str().unwrap_or("") != it["id"].as_str().unwrap_or("") {
                        continue;
                    }
                    if !f["transcript"].as_str().unwrap_or("").is_empty() {
                        continue;
                    }
                    f["transcript"] = serde_json::json!(text);
                    f["t_rung"] = it["t_rung"].clone();
                    f["t_grade"] = it["t_grade"].clone();
                    changed = true;
                }
            }
        }
        if changed { s.to_string() } else { state }
    }
}
