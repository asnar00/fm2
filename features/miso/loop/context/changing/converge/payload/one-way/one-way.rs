struct feature_OneWay;
impl feature_OneWay {
    // the bridge writes page keys and never reads them, so a fragment that
    // writes one is writing into the sea: the next republish puts the context's
    // value back and nothing says a word. This link notices and says it.
    //
    // The bridged key SET comes free: `republish` writes its keys
    // unconditionally, so republishing into an empty object is a list of them
    // and of what the context would publish this time.
    fn ctx_republish(state: String) -> String {
        let mut probe = serde_json::json!({});
        with_context(|c| c.republish(&mut probe));
        let before: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let lost = bridge_lost(&before, &probe);
        let out = existing.ctx_republish(state);
        bridge_announce(out, lost)
    }

    // a key is lost when the page's copy differs from what WE published into it
    // last time. Comparing against the context's current value instead would
    // accuse the context of every change it legitimately made.
    fn bridge_lost(before: &serde_json::Value,
                   probe: &serde_json::Value) -> Vec<String> {
        let mut lost: Vec<String> = Vec::new();
        let empty = serde_json::Map::new();
        for (key, fresh) in probe.as_object().unwrap_or(&empty) {
            let page = before.get(key).cloned().unwrap_or(serde_json::Value::Null);
            if let Some(last) = bridge_shadow(key.clone(), fresh.clone()) {
                if !page.is_null() && page != last {
                    lost.push(key.clone());
                }
            }
        }
        lost
    }

    // what this world last had published into it, key by key: the previous
    // value is answered and the new one remembered in the same call. Keyed by
    // user because the server composes this code too, where one process
    // republishes for everybody.
    fn bridge_shadow(key: String, fresh: serde_json::Value) -> Option<serde_json::Value> {
        static SHADOW: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<String, String>>>
            = std::sync::OnceLock::new();
        let map = SHADOW.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
        let mut held = map.lock().unwrap_or_else(|p| p.into_inner());
        let seat = format!("{}\u{1}{}", context_user_now(), key);
        let was = held.insert(seat, fresh.to_string());
        match was {
            Some(text) => serde_json::from_str(&text).ok(),
            None => None,
        }
    }

    // the complaint rides the payload, because the place that can print it is
    // the page. The key is absent when there is nothing to say, so a warning
    // never outlives what caused it.
    fn bridge_announce(state: String, lost: Vec<String>) -> String {
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if lost.is_empty() {
            if s.is_object() {
                s.as_object_mut().expect("state is an object").remove("_bridge_lost");
            }
            return s.to_string();
        }
        s["_bridge_lost"] = serde_json::json!(lost);
        s.to_string()
    }
}
