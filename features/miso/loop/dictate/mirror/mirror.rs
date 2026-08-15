struct feature_Mirror;
impl feature_Mirror {
    // ---- server half: the exchange stores blobs + a per-user index

    fn blob_root() -> String {
        format!("{}/.miso-blobs", std::env::var("HOME").unwrap_or(".".to_string()))
    }

    fn blob_user(cookie: String, tunnel: bool) -> String {
        let _ = tunnel;
        let who = sender_of(cookie);
        if who.is_empty() { "_local".to_string() } else { who }
    }

    fn blob_id_ok(id: &String) -> bool {
        !id.is_empty() && id.len() < 80
            && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.' || c == '_')
    }

    fn route(r: request) -> response {
        let p = r.path.clone();
        if let Some(id) = p.strip_prefix("blob/") {
            if !msg_guarded(&r) {
                return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
            }
            let id = id.to_string();
            if !blob_id_ok(&id) {
                return json_response(400, "{\"ok\":false,\"error\":\"bad id\"}".to_string());
            }
            let dir = format!("{}/{}", blob_root(), blob_user(r.cookie.clone(), r.tunnel));
            let file = format!("{}/{}", dir, id);
            if r.method == "POST" {
                let _ = std::fs::create_dir_all(dir);
                let _ = std::fs::write(file, &r.raw);
                return json_response(200, "{\"ok\":true}".to_string());
            }
            return match std::fs::read(file) {
                Ok(bytes) => response { status: 200,
                                        ctype: "application/octet-stream".to_string(),
                                        body: bytes, set_cookie: String::new(),
                                        cache: "no-store".to_string() },
                Err(_) => json_response(404, "{\"ok\":false}".to_string()),
            };
        }
        existing.route(r)
    }

    fn index_file(user: String) -> String {
        format!("{}/{}/index.json", blob_root(), user)
    }

    fn read_index(user: String) -> serde_json::Value {
        let raw = std::fs::read_to_string(index_file(user)).unwrap_or_default();
        serde_json::from_str(&raw).unwrap_or(serde_json::json!([]))
    }

    fn handle_msg(msg: String) -> String {
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        let t = m["type"].as_str().unwrap_or("").to_string();
        if t != "RecShared" && t != "RecIndex" {
            return existing.handle_msg(msg);
        }
        let from = m["_from"].as_str().unwrap_or("").to_string();
        let user = if from.is_empty() { "_local".to_string() } else { from.clone() };
        if t == "RecIndex" {
            let items = read_index(user);
            return serde_json::json!({ "type": "RecIndexed", "data": { "items": items } })
                .to_string();
        }
        let meta = m["data"].clone();
        let id = meta["id"].as_str().unwrap_or("").to_string();
        if !blob_id_ok(&id) {
            return "{\"ok\":false}".to_string();
        }
        let mut index = read_index(user.clone());
        let known = index.as_array().map(|a| a.iter()
            .any(|e| e["id"].as_str() == Some(id.as_str()))).unwrap_or(false);
        if !known {
            if let Some(arr) = index.as_array_mut() {
                arr.push(meta.clone());
            }
            let _ = std::fs::create_dir_all(format!("{}/{}", blob_root(), user.clone()));
            let _ = std::fs::write(index_file(user.clone()), index.to_string());
            if !from.is_empty() {
                publish(format!("user.{}", from),
                        serde_json::json!({ "type": "RecShared", "data": meta }).to_string());
            }
        }
        "{\"ok\":true}".to_string()
    }

    // ---- client half: merge remote metadata; render here-awareness

    fn merge_remote(state: String, meta: serde_json::Value) -> String {
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if !s["dict_files"].is_array() {
            s["dict_files"] = serde_json::json!([]);
        }
        let id = meta["id"].as_str().unwrap_or("").to_string();
        let files = s["dict_files"].as_array_mut().expect("dict_files is array");
        if id.is_empty() || files.iter().any(|f| f["id"].as_str() == Some(id.as_str())) {
            return s.to_string();   // the origin instance ignores its own echo
        }
        let mut entry = meta;
        entry["here"] = serde_json::json!(false);
        files.push(entry);
        s.to_string()
    }

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        let t = e["type"].as_str().unwrap_or("").to_string();
        if t == "RecShared" {
            return merge_remote(state, e["data"].clone());
        }
        if t == "RecIndexed" {
            let mut s = state;
            let empty: Vec<serde_json::Value> = Vec::new();
            for item in e["data"]["items"].as_array().unwrap_or(&empty) {
                s = merge_remote(s, item.clone());
            }
            return s;
        }
        if t == "RecFetched" {
            let mut s: serde_json::Value = serde_json::from_str(&state)
                .unwrap_or(serde_json::json!({}));
            let id = e["data"]["id"].as_str().unwrap_or("").to_string();
            if let Some(files) = s["dict_files"].as_array_mut() {
                for f in files {
                    if f["id"].as_str() == Some(id.as_str()) {
                        f["here"] = serde_json::json!(true);
                    }
                }
            }
            return s.to_string();
        }
        state
    }

    // here-aware grid, replacing /dictate's: remote tiles dim until played
    fn render_files(state: String) -> String {
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let empty: Vec<serde_json::Value> = Vec::new();
        let files = s["dict_files"].as_array().unwrap_or(&empty);
        let playing = s["dict_playing"].as_str().unwrap_or("").to_string();
        let mut grid = String::from("<div class=\"file-grid\">");
        for f in files {
            let id = f["id"].as_str().unwrap_or("");
            let label = f["label"].as_str().unwrap_or("note");
            let absent = f["here"].as_bool() == Some(false);
            let mut cls = String::new();
            if playing == id { cls.push_str(" playing"); }
            if absent { cls.push_str(" remote"); }
            grid.push_str(&format!(
                "<div class=\"file-icon{}\" data-ev=\"dict_play_{}\"><span class=\"icon\">🔊</span><div class=\"file-label\">{}</div>{}</div>",
                cls, id, label, dict_file_extra(f.to_string())));
        }
        grid.push_str("</div>");
        grid
    }
}
