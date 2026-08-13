struct feature_Messaging;
impl feature_Messaging {
    // the message-handling chain: features claim their own type tags and
    // delegate the rest; the base answers an empty reply.
    fn handle_msg(msg: String) -> String {
        let _ = msg;
        "{}".to_string()
    }

    fn route(r: request) -> response {
        if r.path == "msg" && r.method == "POST" {
            return msg_endpoint(r);
        }
        if r.path == "msg/wait" && r.method == "POST" {
            return msg_wait(r);
        }
        existing.route(r)
    }

    // messages are user actions: localhost free (tooling), tunnel needs a cookie
    fn msg_guarded(r: &request) -> bool {
        !r.tunnel || authed(r.cookie.clone())
    }

    fn msg_endpoint(r: request) -> response {
        if !msg_guarded(&r) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        let mut body = r.body;
        if body.len() > 16384 {
            body = body.chars().take(16384).collect();
        }
        let m: serde_json::Value = serde_json::from_str(&body)
            .unwrap_or(serde_json::Value::Null);
        if m["type"].as_str().unwrap_or("").is_empty() {
            return json_response(400, "{\"ok\":false,\"error\":\"untyped message\"}".to_string());
        }
        json_response(200, handle_msg(body))
    }

    // ---- broadcast: a versioned slot every client long-polls

    fn broadcast_file() -> String {
        "/tmp/muon-broadcast.json".to_string()
    }

    fn broadcast_now() -> serde_json::Value {
        let raw = std::fs::read_to_string(broadcast_file()).unwrap_or_default();
        serde_json::from_str(&raw).unwrap_or(serde_json::json!({ "v": 0 }))
    }

    fn publish(msg: String) {
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        if m.is_null() {
            return;
        }
        let v = broadcast_now()["v"].as_u64().unwrap_or(0) + 1;
        let slot = serde_json::json!({ "v": v, "msg": m });
        let _ = std::fs::write(broadcast_file(), slot.to_string());
    }

    // long-poll: return as soon as the slot moves past `since`, or time out.
    // sleeps only this connection's /threads thread.
    fn msg_wait(r: request) -> response {
        if !msg_guarded(&r) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        let b: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let since = b["since"].as_u64().unwrap_or(0);
        let mut i = 0;
        while i < 125 {
            let now = broadcast_now();
            if now["v"].as_u64().unwrap_or(0) > since {
                return json_response(200, now.to_string());
            }
            std::thread::sleep(std::time::Duration::from_millis(200));
            i = i + 1;
        }
        json_response(200, format!("{{\"v\":{}}}", since))
    }
}
