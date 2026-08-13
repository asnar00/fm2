struct feature_Blackbox;
impl feature_Blackbox {
    // server half: ingest event batches from devices. cookie-gated — event
    // streams are user data, unlike /diag's anonymous launch reports.
    fn route(r: request) -> response {
        if r.path == "blackbox/events" && r.method == "POST" {
            return blackbox_ingest(r);
        }
        existing.route(r)
    }

    fn blackbox_ingest(r: request) -> response {
        let t = cookie_token(r.cookie.clone());
        if t.is_empty() || !token_valid(t.clone()) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        let who = tag(token_phone(t));
        let mut body = r.body;
        if body.len() > 65536 {
            body = body.chars().take(65536).collect();
        }
        rotate_blackbox_log();
        append_blackbox(format!("{} {} {}\n", now_ms(), who, body.replace("\n", " ")));
        json_response(200, "{\"ok\":true}".to_string())
    }

    fn blackbox_file() -> String {
        "/tmp/muon-blackbox.log".to_string()
    }

    fn rotate_blackbox_log() {
        let size = std::fs::metadata(blackbox_file()).map(|m| m.len()).unwrap_or(0);
        if size > 5000000 {
            let _ = std::fs::rename(blackbox_file(), format!("{}.old", blackbox_file()));
        }
    }

    fn append_blackbox(line: String) {
        use std::io::Write;
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(blackbox_file());
        if let Ok(f) = file {
            let mut f = f;
            let _ = f.write_all(line.as_bytes());
        }
    }
}
