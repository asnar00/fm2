struct feature_Attention;
impl feature_Attention {
    // the backgrounded rung of the attention rule. A builder edit arrives
    // through the diag door with no sender, and the person whose world it
    // changed may not have the app in front of them at all — so their push
    // subscriptions are told. The other two rungs are the page's business.
    //
    // The op is applied by the chain beneath either way: this link only reads
    // the world on both sides of it and decides whether there is news. Reading
    // is safe here because `edit_context`'s replay makes the turn's own writes
    // visible to the rest of the turn, so `before` and `after` really differ
    // when the edit changed something and really match when it did not.
    fn handle_msg(msg: String) -> String {
        if !attention_is_bench_ask_op(&msg) {
            return existing.handle_msg(msg);
        }
        let owner = context_user_now();
        let before = asks_read();
        let reply = existing.handle_msg(msg);
        let after = asks_read();
        if after == before {
            return reply;   // a restamp that changed nothing is not news
        }
        let body = attention_news(&before, &after);
        if body.is_empty() {
            return reply;   // nothing to say, so nothing is said (#p19)
        }
        attention_push_to_user(owner, body);
        reply
    }

    // which ops are the builder speaking to a user: a `CtxOp` on /ask's `asks`
    // var with no signer. A user's own edit carries `_from` and is their own
    // doing, so it never notifies them about themselves.
    //
    // `asks` is the only attention-worthy var today. The rule is general; the
    // registry is not built until a second var wants one.
    fn attention_is_bench_ask_op(msg: &String) -> bool {
        let m: serde_json::Value = serde_json::from_str(msg)
            .unwrap_or(serde_json::Value::Null);
        m["type"].as_str().unwrap_or("") == "CtxOp"
            && m["_from"].as_str().unwrap_or("").is_empty()
            && m["data"]["name"].as_str().unwrap_or("") == "asks"
            && m["data"]["path"].as_str().unwrap_or("")
               == "miso/shell/panel/noob-button/ask"
    }

    // the notification's body, in the entry's own words. Only an entry the
    // edit actually changed may speak, and only if it has something to say:
    // the question it now carries, else the builder's note. A bare status flip
    // says nothing, and silence beats a notification that reads "miso".
    fn attention_news(before: &String, after: &String) -> String {
        let old: serde_json::Value = serde_json::from_str(before)
            .unwrap_or(serde_json::Value::Null);
        let new: serde_json::Value = serde_json::from_str(after)
            .unwrap_or(serde_json::Value::Null);
        let empty: Vec<serde_json::Value> = Vec::new();
        let olds = old.as_array().unwrap_or(&empty).clone();
        for e in new.as_array().unwrap_or(&empty) {
            let t = e["t"].clone();
            let mut was = serde_json::Value::Null;
            for o in olds.iter() {
                if o["t"] == t {
                    was = o.clone();
                }
            }
            if &was == e {
                continue;
            }
            let question = e["question"]["text"].as_str().unwrap_or("").to_string();
            if !question.is_empty() {
                return question;
            }
            let note = e["note"].as_str().unwrap_or("").to_string();
            if !note.is_empty() {
                return note;
            }
        }
        String::new()
    }

    // the wire half, over /push's own path. A subscription line already ends
    // with the subscriber's phone, and a world key is `phone:<number>`, so the
    // targeted walk is the deploy announcement's walk with one match added —
    // including its expired-subscription cleanup, which is the only way a dead
    // endpoint ever leaves the file. A user with no subscriptions matches no
    // line and nothing happens.
    fn attention_push_to_user(owner: String, body: String) {
        let payload = serde_json::json!({ "title": "miso", "body": body }).to_string();
        let raw = std::fs::read_to_string(subs_file()).unwrap_or_default();
        for line in raw.lines() {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() != 4 {
                continue;
            }
            if format!("phone:{}", parts[3]) != owner {
                continue;
            }
            let status = send_push(parts[0].to_string(), parts[1].to_string(),
                                   parts[2].to_string(), payload.clone());
            println!("attention: {} -> {} (status {})", tag(parts[3].to_string()),
                     endpoint_origin(parts[0].to_string()), status);
            if status == 404 || status == 410 {
                remove_sub(parts[0].to_string());
            }
        }
    }
}
