struct feature_StampStands;
impl feature_StampStands {
    // the server's last word on an `asks` set. The list has two authors — the
    // asker's device (the words, the urgency, the birthplace, the approved
    // paragraph, the answer to a did-you-mean) and the builder's bench
    // (`status`, `build`, `note`, `question`) — and both send the WHOLE list,
    // so under last-write whichever arrived second wrote over the other. This
    // link merges the arriving list into the world's, per ask, keyed by `t`:
    // each side keeps the fields it owns, entries the sender never saw
    // survive, and a device may not move a status it did not just earn.
    fn handle_msg(msg: String) -> String {
        let mut m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        if !asks_merge_applies(&m) {
            return existing.handle_msg(msg);
        }
        let incoming: serde_json::Value = serde_json::from_str(
            m["data"]["value"].as_str().unwrap_or("")).unwrap_or(serde_json::Value::Null);
        if !incoming.is_array() {
            return existing.handle_msg(msg);
        }
        let current: serde_json::Value = serde_json::from_str(&asks_read())
            .unwrap_or(serde_json::json!([]));
        // who wrote it. `/messaging` stamps `_from` with the cookie-proven
        // sender and cannot be lied to by the payload; the builder's bench
        // reaches `/diag/context` over localhost with no cookie, so its op
        // carries no sender at all. No sender is the bench; a sender is the
        // asker's own device.
        let bench = m["_from"].as_str().unwrap_or("").is_empty();
        let merged = asks_stand_merge(current, incoming, bench);
        m["data"]["value"] = serde_json::json!(merged.to_string());
        existing.handle_msg(m.to_string())
    }

    fn asks_merge_applies(m: &serde_json::Value) -> bool {
        m["type"].as_str().unwrap_or("") == "CtxOp"
            && m["data"]["path"].as_str().unwrap_or("")
                == "miso/shell/panel/noob-button/ask"
            && m["data"]["name"].as_str().unwrap_or("") == "asks"
            && m["data"]["op"].as_str().unwrap_or("set") == "set"
            // a layered write addresses something other than the user's own
            // value, which is not what this merge reads: leave it alone
            && m["data"]["at"].as_str().unwrap_or("").is_empty()
    }

    // union by `t`: every ask the world holds survives (a `set` cannot delete
    // an ask, as it cannot delete a card — /guard), an ask both sides carry is
    // merged field by field, and an ask only the sender has is appended.
    fn asks_stand_merge(current: serde_json::Value, incoming: serde_json::Value, bench: bool) -> serde_json::Value {
        let cur = asks_fold_dupes(current);
        let inc = asks_fold_dupes(incoming);
        let mut out: Vec<serde_json::Value> = Vec::new();
        for h in cur.iter() {
            let t = h["t"].as_u64().unwrap_or(0);
            let mut keep = h.clone();
            for i in inc.iter() {
                if i["t"].as_u64().unwrap_or(0) == t {
                    keep = asks_merge_entry(h, i, bench);
                }
            }
            out.push(keep);
        }
        for i in inc.iter() {
            let t = i["t"].as_u64().unwrap_or(0);
            if !cur.iter().any(|h| h["t"].as_u64().unwrap_or(0) == t) {
                out.push(i.clone());
            }
        }
        serde_json::Value::Array(out)
    }

    // one entry from each side, same `t`. The owner of a field is whoever can
    // write it: the asker's device owns the ask itself, the bench owns the
    // lifecycle stamp. The side that does not own a field can only FILL it —
    // never change it, never clear it — which is what stops a copy that has
    // not caught up from undoing the other side's work.
    fn asks_merge_entry(held: &serde_json::Value, inc: &serde_json::Value, bench: bool) -> serde_json::Value {
        let mut out = held.clone();
        // the asker's own fields travel with the device
        for k in asks_asker_fields() {
            asks_put(&mut out, k.clone(), asks_pick(held, inc, k.clone(), !bench));
        }
        // everything else — the builder's stamps, and any field a later node
        // adds without filing it above — travels with the bench
        for k in asks_bench_fields(held, inc) {
            asks_put(&mut out, k.clone(), asks_pick(held, inc, k.clone(), bench));
        }
        asks_put(&mut out, "status".to_string(), asks_merge_status(held, inc, bench));
        out
    }

    // an absent field stays absent: the merge fills and keeps, it never plants
    // a null where neither side wrote anything.
    fn asks_put(entry: &mut serde_json::Value, key: String, value: serde_json::Value) {
        if value.is_null() {
            return;
        }
        entry[key] = value;
    }

    fn asks_asker_fields() -> Vec<String> {
        vec!["text".to_string(), "urgency".to_string(), "tool".to_string(),
             "at".to_string(), "proposal".to_string(), "answer".to_string()]
    }

    // every key either side carries that the asker does not own and that is
    // not the key or the status (both settled separately)
    fn asks_bench_fields(held: &serde_json::Value, inc: &serde_json::Value) -> Vec<String> {
        let mut ks: Vec<String> = Vec::new();
        for src in [held, inc] {
            if let Some(o) = src.as_object() {
                for k in o.keys() {
                    if k == "t" || k == "status" {
                        continue;
                    }
                    if asks_asker_fields().contains(k) {
                        continue;
                    }
                    if !ks.contains(k) {
                        ks.push(k.clone());
                    }
                }
            }
        }
        ks
    }

    // the owner's value when the owner has one; otherwise the other side's, so
    // a field is only ever filled in, never dropped.
    fn asks_pick(held: &serde_json::Value, inc: &serde_json::Value, key: String, incoming_owns: bool) -> serde_json::Value {
        let h = held.get(key.clone()).cloned().unwrap_or(serde_json::Value::Null);
        let i = inc.get(key.clone()).cloned().unwrap_or(serde_json::Value::Null);
        let owner = if incoming_owns { i.clone() } else { h.clone() };
        let other = if incoming_owns { h } else { i };
        if asks_has(&owner) { owner } else { other }
    }

    fn asks_has(v: &serde_json::Value) -> bool {
        if v.is_null() {
            return false;
        }
        if let Some(s) = v.as_str() {
            return !s.trim().is_empty();
        }
        true
    }

    // the contested field. The bench is the last word on where an ask has got
    // to — that is what a stamp IS. A device may move the status only when it
    // carries the thing that earns the move: a new `answer` settles a
    // did-you-mean (which walks the ask back to `asked`, the one backwards
    // step on the ladder), a new `proposal` upgrades it to `proposed`.
    // Anything else from a device is a copy that has not caught up, and the
    // held status stands.
    fn asks_merge_status(held: &serde_json::Value, inc: &serde_json::Value, bench: bool) -> serde_json::Value {
        let h = held.get("status").cloned().unwrap_or(serde_json::Value::Null);
        let i = inc.get("status").cloned().unwrap_or(serde_json::Value::Null);
        if !asks_has(&i) {
            return h;
        }
        if !asks_has(&h) {
            return i;
        }
        if h == i {
            return h;
        }
        if bench {
            return i;
        }
        if asks_field_earned(held, inc, "answer".to_string())
            || asks_field_earned(held, inc, "proposal".to_string()) {
            return i;
        }
        println!("asks: kept {} over {} for ask {} — the device's list is behind",
                 h.as_str().unwrap_or("?"), i.as_str().unwrap_or("?"),
                 held["t"].as_u64().unwrap_or(0));
        h
    }

    // does the arriving entry carry a value for this field that the world has
    // not seen? That is the asker having just done something, rather than
    // having sent back what they were given.
    fn asks_field_earned(held: &serde_json::Value, inc: &serde_json::Value, key: String) -> bool {
        let i = inc.get(key.clone()).cloned().unwrap_or(serde_json::Value::Null);
        if !asks_has(&i) {
            return false;
        }
        let h = held.get(key.clone()).cloned().unwrap_or(serde_json::Value::Null);
        h != i
    }

    // two entries under one `t` in the same list are one ask written twice:
    // fold them, later fields winning, so the merge has a single entry per key
    // to reason about and no duplicate can shadow the one that was merged.
    fn asks_fold_dupes(list: serde_json::Value) -> Vec<serde_json::Value> {
        let empty: Vec<serde_json::Value> = Vec::new();
        let items = list.as_array().unwrap_or(&empty);
        let mut out: Vec<serde_json::Value> = Vec::new();
        for it in items {
            let t = it["t"].as_u64().unwrap_or(0);
            let mut at = out.len();
            for n in 0..out.len() {
                if out[n]["t"].as_u64().unwrap_or(0) == t {
                    at = n;
                }
            }
            if at == out.len() {
                out.push(it.clone());
                continue;
            }
            println!("asks: folded a second entry for ask {}", t);
            let mut merged = out[at].clone();
            if let Some(o) = it.as_object() {
                for k in o.keys() {
                    if asks_has(&it[k.clone()]) {
                        merged[k.clone()] = it[k.clone()].clone();
                    }
                }
            }
            out[at] = merged;
        }
        out
    }
}
