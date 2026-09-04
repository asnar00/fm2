struct feature_ByActivity;
impl feature_ByActivity {
    // ---- the order ---------------------------------------------------------
    // /people sorted by how near you are in the invite tree. Ash asked for
    // yourself first and then most recently active (#p162). The proximity word
    // stays on the row — `existing` is called first, so its `near` decoration
    // is intact — and only the ORDER is this node's.

    fn people_order(cards: String, state: String) -> String {
        let decorated = existing.people_order(cards, state.clone());
        let list: serde_json::Value = serde_json::from_str(&decorated)
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let own = people_own_id();
        let frozen = by_activity_frozen(state);
        let mut out: Vec<serde_json::Value> = Vec::new();
        for c in list.as_array().unwrap_or(&empty) {
            out.push(c.clone());
        }
        out.sort_by(|a: &serde_json::Value, b: &serde_json::Value| {
            let ka = by_activity_key(a, own.clone(), &frozen);
            let kb = by_activity_key(b, own.clone(), &frozen);
            ka.cmp(&kb)
        });
        serde_json::Value::Array(out).to_string()
    }

    // (you, then where the freeze put them, then the name). A person the
    // freeze does not name sorts after everyone it does — a newcomer joins at
    // the end until the next open — and the name breaks every tie, so two
    // people with the same activity, or none at all, have a stable place.
    fn by_activity_key(card: &serde_json::Value, own: String, frozen: &Vec<String>) -> String {
        let id = card["id"].as_str().unwrap_or("").to_string();
        let mine = if !own.is_empty() && id == own { "0" } else { "1" };
        let mut at = frozen.len();
        for i in 0..frozen.len() {
            if frozen[i] == id {
                at = i;
                break;
            }
        }
        let name = card["owner"].as_str().unwrap_or("").to_lowercase();
        format!("{}{:06}{}", mine, at, name)
    }

    // ---- the freeze --------------------------------------------------------
    // The hold ash asked for: the order is worked out when the surface is
    // OPENED and not again while it is on screen, however many syncs arrive —
    // a live tick every few seconds must not make the list ping around under
    // the eye (/keep's stance: nothing moves under a reader).
    //
    // It lives on the turn's state, /in-place's idiom for exactly this shape:
    // no op on the wire, nothing stored, and a relaunch simply works it out
    // again. Leaving the tool clears it, so the next open re-sorts.

    fn by_activity_frozen(state: String) -> Vec<String> {
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let raw = s["people_shown"].as_str().unwrap_or("").to_string();
        let mut out: Vec<String> = Vec::new();
        for part in raw.split(',') {
            if !part.is_empty() {
                out.push(part.to_string());
            }
        }
        out
    }

    // every profile you hold, most recently active first — the order that is
    // then frozen. Ties and unknowns fall to the name, so it is total.
    fn by_activity_fresh(state: String) -> String {
        let all: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut people: Vec<serde_json::Value> = Vec::new();
        for c in all.as_array().unwrap_or(&empty) {
            if c["type"].as_str().unwrap_or("") == "profile" {
                let mut c = c.clone();
                c["_at"] = serde_json::json!(by_activity_at(&c, state.clone()));
                people.push(c);
            }
        }
        people.sort_by(|a: &serde_json::Value, b: &serde_json::Value| {
            let ta = a["_at"].as_u64().unwrap_or(0);
            let tb = b["_at"].as_u64().unwrap_or(0);
            let na = a["owner"].as_str().unwrap_or("").to_lowercase();
            let nb = b["owner"].as_str().unwrap_or("").to_lowercase();
            tb.cmp(&ta).then(na.cmp(&nb))
        });
        let mut ids: Vec<String> = Vec::new();
        for c in people.iter() {
            ids.push(c["id"].as_str().unwrap_or("").to_string());
        }
        ids.join(",")
    }

    // when a person was last active: the latest of their last post's own
    // moment, their card's last edit, and the last time their phone said where
    // it was. The first two are in the world; the third is the page half's,
    // handed in as `PeopleActive` — /people's own idiom for the distances,
    // which are the server's and not the world's.
    fn by_activity_at(card: &serde_json::Value, state: String) -> u64 {
        let mut best = card["edited"].as_u64().unwrap_or(0);
        let owner = card["owner"].as_str().unwrap_or("").to_string();
        let all: serde_json::Value = serde_json::from_str(&cards_read())
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        for c in all.as_array().unwrap_or(&empty) {
            if c["type"].as_str().unwrap_or("") != "post" {
                continue;
            }
            if c["owner"].as_str().unwrap_or("").to_string() != owner {
                continue;
            }
            let w = c["when"].as_u64().unwrap_or(0);
            let t = if w > 0 { w } else { c["created"].as_u64().unwrap_or(0) };
            if t > best {
                best = t;
            }
        }
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let id = card["id"].as_str().unwrap_or("");
        let seen = s["active"][id].as_u64().unwrap_or(0);
        if seen > best {
            best = seen;
        }
        best
    }

    // ---- the two events ----------------------------------------------------

    fn update(state: String, event: String) -> String {
        let was = open_tool_read();
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        // the last time each phone said where it was, from the page half
        if e["type"].as_str().unwrap_or("") == "PeopleActive" {
            let mut s: serde_json::Value = serde_json::from_str(&state)
                .unwrap_or(serde_json::json!({}));
            s["active"] = e["data"].clone();
            return s.to_string();
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if open_tool_read() != "account" {
            // away from the people: the freeze goes, so the next open re-sorts
            if s["people_shown"].is_string() {
                s["people_shown"] = serde_json::json!(null);
                return s.to_string();
            }
            return state;
        }
        // opened just now, or the first turn of a relaunch that landed here
        let held = s["people_shown"].as_str().unwrap_or("").to_string();
        if was != "account" || held.is_empty() {
            s["people_shown"] = serde_json::json!(by_activity_fresh(s.to_string()));
            return s.to_string();
        }
        state
    }
}
