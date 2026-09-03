struct feature_Revert;
impl feature_Revert {
    // an undo of a cards edit puts back the PRIOR list — whose cards carry
    // their older `edited` stamps, which /guard reads as a stale device and
    // refuses. A deliberate revert is a new edit in time: restamp every
    // card in the prior list to now before the inverse op is issued, so the
    // guard takes it and the phone's revert is not overwritten by the reply.
    fn undo_apply(step: serde_json::Value) {
        existing.undo_apply(cards_revert_restamp(step));
    }

    fn cards_revert_restamp(step: serde_json::Value) -> serde_json::Value {
        let mut step = step;
        let now = revert_stamp();
        if let Some(changes) = step["changes"].as_array_mut() {
            for ch in changes.iter_mut() {
                if ch["path"].as_str().unwrap_or("") != "miso/loop/cards"
                    || ch["name"].as_str().unwrap_or("") != "cards" {
                    continue;
                }
                let raw = ch["prior"].as_str().unwrap_or("").to_string();
                let mut list: serde_json::Value = serde_json::from_str(&raw)
                    .unwrap_or(serde_json::Value::Null);
                if let Some(cards) = list.as_array_mut() {
                    for c in cards.iter_mut() {
                        c["edited"] = serde_json::json!(now);
                    }
                    ch["prior"] = serde_json::json!(list.to_string());
                }
            }
        }
        step
    }

    // one past the newest `edited` this world holds: newer than anything
    // the guard will compare against, without needing a wall clock in wasm
    fn revert_stamp() -> u64 {
        let list: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut top: u64 = 0;
        for c in list.as_array().unwrap_or(&empty) {
            let e = c["edited"].as_u64().unwrap_or(0);
            if e > top {
                top = e;
            }
        }
        top + 1
    }
}
