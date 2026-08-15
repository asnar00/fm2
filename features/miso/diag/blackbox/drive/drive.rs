struct feature_Drive;
impl feature_Drive {
    // commanded interactions: a file-backed queue the page polls. same trust
    // stance as /readout — localhost free for tooling, tunnel cookie-gated.
    fn route(r: request) -> response {
        if r.path == "diag/drive" && r.method == "POST" {
            return drive_enqueue(r);
        }
        if r.path == "diag/drive/next" {
            return drive_next(r);
        }
        existing.route(r)
    }

    fn drive_file() -> String {
        "/tmp/miso-drive.json".to_string()
    }

    fn drive_queue() -> serde_json::Value {
        let raw = std::fs::read_to_string(drive_file()).unwrap_or_default();
        let q: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::json!([]));
        if q.is_array() {
            q
        } else {
            serde_json::json!([])
        }
    }

    fn drive_enqueue(r: request) -> response {
        if !readout_guarded(&r) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        let mut body = r.body;
        if body.len() > 8192 {
            body = body.chars().take(8192).collect();
        }
        let cmd: serde_json::Value = serde_json::from_str(&body)
            .unwrap_or(serde_json::Value::Null);
        if cmd.is_null() {
            return json_response(400, "{\"ok\":false,\"error\":\"bad command\"}".to_string());
        }
        let mut q = drive_queue();
        q.as_array_mut().expect("queue is array").push(cmd);
        let _ = std::fs::write(drive_file(), q.to_string());
        json_response(200, "{\"ok\":true}".to_string())
    }

    fn drive_next(r: request) -> response {
        if !readout_guarded(&r) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        let mut q = drive_queue();
        let arr = q.as_array_mut().expect("queue is array");
        if arr.is_empty() {
            return json_response(200, "{}".to_string());
        }
        let cmd = arr.remove(0);
        let _ = std::fs::write(drive_file(), q.to_string());
        json_response(200, cmd.to_string())
    }
}
