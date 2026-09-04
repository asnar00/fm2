struct feature_ToOne;
impl feature_ToOne {
    // /push rings the whole team, because until now every notification was
    // news for everyone: a deploy. An answer to one person's request is news
    // for that person only. This adds the one road that was missing — the
    // same wire, one recipient — and lives under /push so it cannot compose
    // without the protocol it uses.
    fn route(r: request) -> response {
        if r.path == "push/one" {
            return push_one_route(r);
        }
        existing.route(r)
    }

    // Screened as `POST pic/retrofit` is: the bench on the box may call it,
    // anyone reaching it through the tunnel must be logged in. It is a
    // builder's door, not a way for one phone to ring another — a phone
    // could otherwise name any number and any words.
    fn push_one_route(r: request) -> response {
        if r.method != "POST" {
            return json_response(405, "{\"ok\":false}".to_string());
        }
        if r.tunnel && !authed(r.cookie.clone()) {
            return json_response(401, "{\"ok\":false,\"error\":\"log in first\"}".to_string());
        }
        let body: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::json!({}));
        let phone = body["phone"].as_str().unwrap_or("").to_string();
        let title = body["title"].as_str().unwrap_or("miso").to_string();
        let text = body["body"].as_str().unwrap_or("").to_string();
        if phone.is_empty() || text.is_empty() {
            return json_response(400,
                "{\"ok\":false,\"error\":\"phone and body are both needed\"}".to_string());
        }
        let payload = format!("{{\"title\":{},\"body\":{}}}",
                              serde_json::Value::String(title),
                              serde_json::Value::String(text));
        let sent = send_one(phone.clone(), payload);
        json_response(200, format!("{{\"ok\":true,\"sent\":{}}}", sent))
    }

    // every subscription this person holds — a phone and a laptop are two
    // lines and both ring. Numbers are compared by their digits alone, the
    // way `/ask_ack`'s guest-list lookup does, so +44… and 44… are one
    // person. An expired subscription is pruned exactly as `send_all` prunes
    // it; a device that never subscribed simply yields nothing, and the
    // caller is told `sent: 0` rather than an error — the sheet is the
    // record, the push is the courtesy.
    fn send_one(phone: String, payload: String) -> u32 {
        let want = digits_only(phone);
        if want.is_empty() {
            return 0;
        }
        let raw = std::fs::read_to_string(subs_file()).unwrap_or_default();
        let mut sent = 0;
        for line in raw.lines() {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() != 4 {
                continue;
            }
            if digits_only(parts[3].to_string()) != want {
                continue;
            }
            let status = send_push(parts[0].to_string(), parts[1].to_string(),
                                   parts[2].to_string(), payload.clone());
            println!("push one: {} -> {} (status {})", tag(parts[3].to_string()),
                     endpoint_origin(parts[0].to_string()), status);
            if status == 404 || status == 410 {
                remove_sub(parts[0].to_string());
            } else {
                sent = sent + 1;
            }
        }
        sent
    }

    fn digits_only(s: String) -> String {
        s.chars().filter(|c| c.is_ascii_digit()).collect()
    }
}
