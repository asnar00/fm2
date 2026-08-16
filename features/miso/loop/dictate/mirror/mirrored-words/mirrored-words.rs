struct feature_MirroredWords;
impl feature_MirroredWords {
    // ---- server half: the words store, beside the blob index

    fn words_file(user: String) -> String {
        format!("{}/{}/words.json", blob_root(), user)
    }

    fn read_words(user: String) -> serde_json::Value {
        let raw = std::fs::read_to_string(words_file(user)).unwrap_or_default();
        serde_json::from_str(&raw).unwrap_or(serde_json::json!({}))
    }

    fn handle_msg(msg: String) -> String {
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        let t = m["type"].as_str().unwrap_or("").to_string();
        let from = m["_from"].as_str().unwrap_or("").to_string();
        let user = if from.is_empty() { "_local".to_string() } else { from.clone() };
        if t == "TranscriptShared" {
            let w = m["data"].clone();
            let id = w["id"].as_str().unwrap_or("").to_string();
            if !blob_id_ok(&id) || w["text"].as_str().unwrap_or("").is_empty() {
                return "{\"ok\":false}".to_string();
            }
            let mut words = read_words(user.clone());
            let have = words[id.as_str()]["grade"].as_i64().unwrap_or(0);
            if w["grade"].as_i64().unwrap_or(0) <= have {
                return "{\"ok\":true}".to_string();   // echo or stale: keep what's there
            }
            words[id.as_str()] = serde_json::json!({
                "text": w["text"], "rung": w["rung"], "grade": w["grade"] });
            let _ = std::fs::create_dir_all(format!("{}/{}", blob_root(), user.clone()));
            let _ = std::fs::write(words_file(user.clone()), words.to_string());
            if !from.is_empty() {
                publish(format!("user.{}", from),
                        serde_json::json!({ "type": "TranscriptShared", "data": w }).to_string());
            }
            return "{\"ok\":true}".to_string();
        }
        if t == "RecShared" {
            // a recording announced after its words arrived: the metadata
            // carries them into the index and the broadcast
            let mut m2 = m.clone();
            let id = m2["data"]["id"].as_str().unwrap_or("").to_string();
            let w = read_words(user.clone())[id.as_str()].clone();
            if w["text"].as_str().unwrap_or("") != "" {
                m2["data"]["transcript"] = w["text"].clone();
                m2["data"]["t_rung"] = w["rung"].clone();
                m2["data"]["t_grade"] = w["grade"].clone();
            }
            return existing.handle_msg(m2.to_string());
        }
        if t == "RecIndex" {
            // boot catch-up arrives with words already stamped on its items
            let reply = existing.handle_msg(msg);
            let mut r: serde_json::Value = serde_json::from_str(&reply)
                .unwrap_or(serde_json::Value::Null);
            if r["type"].as_str() != Some("RecIndexed") {
                return reply;
            }
            let words = read_words(user.clone());
            if let Some(items) = r["data"]["items"].as_array_mut() {
                for item in items {
                    let id = item["id"].as_str().unwrap_or("").to_string();
                    let w = words[id.as_str()].clone();
                    let have = item["t_grade"].as_i64().unwrap_or(0);
                    if w["text"].as_str().unwrap_or("") != ""
                        && w["grade"].as_i64().unwrap_or(0) > have {
                        item["transcript"] = w["text"].clone();
                        item["t_rung"] = w["rung"].clone();
                        item["t_grade"] = w["grade"].clone();
                    }
                }
            }
            return r.to_string();
        }
        existing.handle_msg(msg)
    }

    // ---- client half: mirrored words land only when they beat what's here

    // better grade replaces rougher; at equal grade, real words beat the
    // empty stamp a failed local attempt leaves behind
    fn adopt_words(f: &mut serde_json::Value, w: &serde_json::Value) {
        let have = f["t_grade"].as_i64().unwrap_or(0);
        let grade = w["grade"].as_i64().unwrap_or(0);
        let empty_here = f["transcript"].as_str().unwrap_or("").is_empty();
        if w["text"].as_str().unwrap_or("") != ""
            && (grade > have || (grade == have && empty_here)) {
            f["transcript"] = w["text"].clone();
            f["t_rung"] = w["rung"].clone();
            f["t_grade"] = w["grade"].clone();
        }
    }

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let t = e["type"].as_str().unwrap_or("").to_string();
        if t == "TranscriptShared" {
            let w = e["data"].clone();
            let id = w["id"].as_str().unwrap_or("").to_string();
            let mut s: serde_json::Value = serde_json::from_str(&state)
                .unwrap_or(serde_json::json!({}));
            if let Some(files) = s["dict_files"].as_array_mut() {
                for f in files {
                    if f["id"].as_str() == Some(id.as_str()) {
                        adopt_words(f, &w);
                    }
                }
            }
            return transcribe(s.to_string());
        }
        if t == "RecIndexed" {
            // /mirror's merge skips files already here — their words land here
            let mut s: serde_json::Value = serde_json::from_str(&state)
                .unwrap_or(serde_json::json!({}));
            let empty: Vec<serde_json::Value> = Vec::new();
            let items = e["data"]["items"].as_array().unwrap_or(&empty).clone();
            if let Some(files) = s["dict_files"].as_array_mut() {
                for f in files {
                    for item in &items {
                        if f["id"] == item["id"] {
                            let w = serde_json::json!({
                                "text": item["transcript"], "rung": item["t_rung"],
                                "grade": item["t_grade"] });
                            adopt_words(f, &w);
                        }
                    }
                }
            }
            return transcribe(s.to_string());
        }
        state
    }
}
