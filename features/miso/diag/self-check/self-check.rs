struct feature_SelfCheck;
impl feature_SelfCheck {
    // the latest self-check per device is kept beside the diag log, so
    // reading it does not mean tailing; the report line itself still lands in
    // the log through /diag's own handler (existing.route).
    fn route(r: request) -> response {
        if r.path == "diag/report" && r.method == "POST" {
            self_check_keep(&r.body);
            return existing.route(r);
        }
        if r.path == "diag/self-check" && r.method == "GET" {
            return self_check_get(r);
        }
        existing.route(r)
    }

    fn self_check_file() -> String {
        let log = diag_file();
        let dir = std::path::Path::new(&log).parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or("/tmp".to_string());
        format!("{}/miso-self-check.json", dir)
    }

    fn self_check_read() -> serde_json::Map<String, serde_json::Value> {
        let raw = std::fs::read_to_string(self_check_file()).unwrap_or_default();
        match serde_json::from_str::<serde_json::Value>(&raw) {
            Ok(serde_json::Value::Object(m)) => m,
            _ => serde_json::Map::new(),
        }
    }

    // a report of kind "self-check" replaces its device's entry; anything
    // else passes untouched. Capped at 200 devices, oldest evicted.
    fn self_check_keep(body: &str) {
        if body.len() > 2048 || !body.contains("\"self-check\"") {
            return;
        }
        let v: serde_json::Value = match serde_json::from_str(body) {
            Ok(v) => v,
            Err(_) => return,
        };
        if v["kind"].as_str() != Some("self-check") {
            return;
        }
        let device = v["device"].as_str().unwrap_or("unknown").to_string();
        let mut map = self_check_read();
        let mut entry = v.clone();
        entry["at"] = serde_json::Value::from(now_ms());
        map.insert(device, entry);
        while map.len() > 200 {
            let oldest = map.iter()
                .min_by_key(|(_, e)| e["at"].as_u64().unwrap_or(0))
                .map(|(k, _)| k.clone());
            match oldest {
                Some(k) => { map.remove(&k); }
                None => break,
            }
        }
        let _ = std::fs::write(self_check_file(),
            serde_json::to_string(&serde_json::Value::Object(map)).unwrap_or_default());
    }

    // owner-only through the tunnel (admin authority); open on localhost for
    // the builder's tooling, as the rest of /diag's readers are
    fn self_check_get(r: request) -> response {
        if r.tunnel {
            let who = context_user_of(r.cookie.clone(), r.tunnel, r.query.clone());
            if !authed(r.cookie.clone()) || authority_rank(who) < 3 {
                return json_response(401, "{\"ok\":false,\"error\":\"owner only\"}".to_string());
            }
        }
        let map = self_check_read();
        let mut list: Vec<serde_json::Value> = map.into_iter().map(|(_, e)| e).collect();
        list.sort_by_key(|e| std::cmp::Reverse(e["at"].as_u64().unwrap_or(0)));
        let n = r.query.split('&')
            .find_map(|kv| kv.strip_prefix("n=").and_then(|s| s.parse::<usize>().ok()))
            .unwrap_or(20);
        list.truncate(n);
        json_response(200, serde_json::to_string(&list).unwrap_or("[]".to_string()))
    }
}
