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
        let mut m: serde_json::Value = serde_json::from_str(&body)
            .unwrap_or(serde_json::Value::Null);
        if m["type"].as_str().unwrap_or("").is_empty() {
            return json_response(400, "{\"ok\":false,\"error\":\"untyped message\"}".to_string());
        }
        // stamp the cookie-proven sender identity: handlers key user-scoped
        // data by it, and cannot be lied to by the payload
        m["_from"] = serde_json::json!(sender_of(r.cookie.clone()));
        json_response(200, handle_msg(m.to_string()))
    }

    // ---- broadcast: a versioned slot every client long-polls

    // server-side scope filtering: a wait only ever receives global entries
    // and entries addressed to its own user — other users' values cannot leak
    fn wait_filter(b: serde_json::Value, since: u64, me: String) -> String {
        let v = b["v"].as_u64().unwrap_or(0);
        let mut msgs: Vec<serde_json::Value> = Vec::new();
        let empty: Vec<serde_json::Value> = Vec::new();
        let entries = b["entries"].as_array().unwrap_or(&empty);
        for e in entries {
            if e["v"].as_u64().unwrap_or(0) <= since {
                continue;
            }
            let aud = e["aud"].as_str().unwrap_or("global");
            let mine = format!("user.{}", me);
            if aud == "global" || (!me.is_empty() && aud == mine) {
                msgs.push(e["msg"].clone());
            }
        }
        serde_json::json!({ "v": v, "msgs": msgs }).to_string()
    }

    fn broadcast_file() -> String {
        "/tmp/muon-broadcast.json".to_string()
    }

    fn broadcast_now() -> serde_json::Value {
        let raw = std::fs::read_to_string(broadcast_file()).unwrap_or_default();
        serde_json::from_str(&raw).unwrap_or(serde_json::json!({ "v": 0, "entries": [] }))
    }

    // who a request may hear broadcasts for: everyone's, and their own
    fn sender_of(cookie: String) -> String {
        let t = cookie_token(cookie);
        if !t.is_empty() && token_valid(t.clone()) {
            tag(token_phone(t))
        } else {
            String::new()
        }
    }

    fn publish(audience: String, msg: String) {
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        if m.is_null() {
            return;
        }
        let mut b = broadcast_now();
        let v = b["v"].as_u64().unwrap_or(0) + 1;
        b["v"] = serde_json::json!(v);
        if !b["entries"].is_array() {
            b["entries"] = serde_json::json!([]);
        }
        let entries = b["entries"].as_array_mut().expect("entries is array");
        entries.push(serde_json::json!({ "v": v, "aud": audience, "msg": m }));
        while entries.len() > 50 {
            entries.remove(0);
        }
        let _ = std::fs::write(broadcast_file(), b.to_string());
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
        let me = sender_of(r.cookie.clone());
        let mut i = 0;
        while i < 125 {
            let now = broadcast_now();
            if now["v"].as_u64().unwrap_or(0) > since {
                let hearable = wait_filter(now, since, me.clone());
                return json_response(200, hearable);
            }
            std::thread::sleep(std::time::Duration::from_millis(200));
            i = i + 1;
        }
        json_response(200, format!("{{\"v\":{},\"msgs\":[]}}", since))
    }
}
