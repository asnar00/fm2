struct feature_InvitedInto;
impl feature_InvitedInto {
    // ---- the watch ---------------------------------------------------------
    // a role link needs the newcomer's profile card id, and the phone mints
    // that card on the first paint of 👤 — so the join happens on their first
    // cards write that carries it, not at login. Watched from `route`,
    // outside the turn, for /exchange's reason: only out here may another
    // world be named, read and written.
    fn route(r: request) -> response {
        let watch = r.path == "msg" && r.method == "POST"
            && exchange_is_cards_op(&r.body);
        if !watch {
            return existing.route(r);
        }
        let who = exchange_who(&r);
        let resp = existing.route(r);
        if resp.status == 200 && !who.is_empty() {
            invited_into_try(who);
        }
        resp
    }

    fn invited_into_entry(key: String) -> serde_json::Value {
        let list = exchange_users();
        let empty: Vec<serde_json::Value> = Vec::new();
        for u in list.as_array().unwrap_or(&empty) {
            if exchange_key_of(u) == key {
                return u.clone();
            }
        }
        serde_json::Value::Null
    }

    // the newcomer's own profile card id, or empty for "not yet"
    fn invited_into_profile(who: String, name: String) -> String {
        let held: serde_json::Value = serde_json::from_str(&exchange_cards_of(who))
            .unwrap_or(serde_json::json!([]));
        let empty: Vec<serde_json::Value> = Vec::new();
        for c in held.as_array().unwrap_or(&empty) {
            if c["type"].as_str().unwrap_or("") != "profile" {
                continue;
            }
            if !c["from"].is_null() {
                continue;
            }
            if c["owner"].as_str().unwrap_or("") != name {
                continue;
            }
            if !exchange_owns_id(c, name.clone()) {
                continue;
            }
            return c["id"].as_str().unwrap_or("").to_string();
        }
        String::new()
    }

    // ---- the join ----------------------------------------------------------
    fn invited_into_try(who: String) {
        let entry = invited_into_entry(who.clone());
        let project = entry["project"].as_str().unwrap_or("").to_string();
        if project.is_empty() || entry["added"].is_u64() {
            return;
        }
        let name = exchange_name_of(who.clone());
        if name.is_empty() {
            return;
        }
        let profile = invited_into_profile(who.clone(), name.clone());
        if profile.is_empty() {
            return;   // no card of their own yet: the write that makes one is next
        }
        let asked = entry["rank"].as_str().unwrap_or("").to_string();
        let rank = if audience_is_grade(asked.clone()) { asked } else { audience_default_grade() };
        // the inviter's held copy names the owner; the owner's world holds
        // the original, which is the only card a role may be written on
        let inviter = entry["invited_by"].as_str().unwrap_or("").to_string();
        let held = audience_project_in(exchange_cards_of(inviter.clone()), project.clone());
        if held.is_null() {
            println!("invited-into: {} no longer holds {} — {} joins nothing", inviter, project, name);
            invited_into_clear(who);
            return;
        }
        let owner = held["owner"].as_str().unwrap_or("").to_string();
        let owner_key = projects_key_for_name(owner.clone());
        if owner_key.is_empty() {
            println!("invited-into: {}'s owner {} has no world — {} joins nothing", project, owner, name);
            invited_into_clear(who);
            return;
        }
        if owner_key == who {
            invited_into_stamp(who);   // an owner is admin by being the owner
            return;
        }
        let mut card = audience_project_in(exchange_cards_of(owner_key.clone()), project.clone());
        if card.is_null() || !card["from"].is_null() {
            println!("invited-into: {} is gone from {}'s world — {} joins nothing", project, owner, name);
            invited_into_clear(who);
            return;
        }
        // past the card's own stamp, so /guard's merge takes this write even
        // from a phone whose clock runs ahead of the server's
        let now = std::cmp::max(now_ms(), card["edited"].as_u64().unwrap_or(0) + 1);
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut links: Vec<serde_json::Value> = Vec::new();
        for l in card["links"].as_array().unwrap_or(&empty) {
            let same = l["kind"].as_str().unwrap_or("") == "role"
                && (l["to"].as_str().unwrap_or("") == profile
                    || projects_link_name(l) == name
                    || l["name"].as_str().unwrap_or("") == name);
            if same {
                continue;   // the same person again REPLACES — /projects' rule
            }
            links.push(l.clone());
        }
        let d = serde_json::json!({
            "card": project.clone(), "to": profile.clone(), "name": name.clone(),
            "role": rank.clone(), "grade": rank.clone(), "t": now });
        links.push(projects_role_link(d, profile.clone(), name.clone(), rank.clone(), now));
        card["links"] = serde_json::Value::Array(links);
        card["edited"] = serde_json::json!(now);
        if !invited_into_put(owner_key.clone(), card.clone()) {
            println!("invited-into: {}'s world would not take the link for {} — next write retries",
                     owner, name);
            return;
        }
        // the changed project to everyone in it, the newcomer included: a
        // server write is not a POST /msg, so /projects' hand-over never sees
        // it. Its filter admits the newcomer because the link is on the card.
        let copy = exchange_copy(&card, owner.clone(), owner_key.clone());
        for l in projects_members(&card).iter() {
            let key = projects_key_for_name(projects_link_name(l));
            if key.is_empty() || key == owner_key {
                continue;
            }
            let mut one: Vec<serde_json::Value> = Vec::new();
            one.push(copy.clone());
            exchange_give(key, one);
        }
        invited_into_select(who.clone(), project.clone());
        invited_into_stamp(who);
        println!("invited-into: {} joined {} as {}", name, project, rank);
    }

    // ---- the door ----------------------------------------------------------
    // /exchange's door restated for a card that is not a copy: one card, a
    // `set`, signed with the recipient's audience so /converge repaints their
    // open pages, handed to `handle_msg` as them. /guard merges it — the
    // union keeps every other card and takes the newer edit of this one.
    fn invited_into_put(to: String, card: serde_json::Value) -> bool {
        let mut cards: Vec<serde_json::Value> = Vec::new();
        cards.push(card);
        let value = serde_json::Value::Array(cards).to_string();
        let msg = serde_json::json!({
            "type": "CtxOp",
            "_from": exchange_audience_of(to.clone()),
            "data": {
                "path": "miso/loop/cards",
                "name": "cards",
                "op": "set",
                "value": value
            }
        }).to_string();
        let saved = context_user_now();
        context_user_set(to.clone());
        let reply = handle_msg(msg);
        context_user_set(saved);
        let r: serde_json::Value = serde_json::from_str(&reply)
            .unwrap_or(serde_json::Value::Null);
        if r["type"].as_str().unwrap_or("") != "CtxUpdate" {
            println!("invited-into: {} would not take the project ({})",
                     tag(to), r["error"].as_str().unwrap_or("no reason given"));
            return false;
        }
        true
    }

    // the newcomer's current project, if they have none: "joins your current
    // project" means it is theirs to work in, and /audience files their first
    // post there. A choice they already made is left alone.
    fn invited_into_select(to: String, pid: String) {
        let saved = context_user_now();
        context_user_set(to.clone());
        let cur = current_project_read();
        if cur.is_empty() {
            let msg = serde_json::json!({
                "type": "CtxOp",
                "_from": exchange_audience_of(to.clone()),
                "data": {
                    "path": "miso/loop/cards/kinds/projects/current-project",
                    "name": "current",
                    "op": "set",
                    "value": pid
                }
            }).to_string();
            let reply = handle_msg(msg);
            let r: serde_json::Value = serde_json::from_str(&reply)
                .unwrap_or(serde_json::Value::Null);
            if r["type"].as_str().unwrap_or("") != "CtxUpdate" {
                println!("invited-into: {} would not take the project as current ({})",
                         tag(to.clone()), r["error"].as_str().unwrap_or("no reason given"));
            }
        }
        context_user_set(saved);
    }

    // ---- the entry ---------------------------------------------------------
    fn invited_into_stamp(who: String) {
        invited_into_mark(who, true);
    }

    fn invited_into_clear(who: String) {
        invited_into_mark(who, false);
    }

    // `added` stamped, or the two fields dropped: under the store lock,
    // through /invite's own temp-write and rename
    fn invited_into_mark(who: String, done: bool) {
        with_store_lock(|| {
            let list = invite_list();
            if list.is_null() {
                return;
            }
            let mut arr = match list.as_array() {
                Some(a) => a.clone(),
                None => {
                    return;
                }
            };
            let mut changed = false;
            for u in arr.iter_mut() {
                if exchange_key_of(u) != who {
                    continue;
                }
                if done {
                    u["added"] = serde_json::json!(now_ms());
                } else if let Some(o) = u.as_object_mut() {
                    o.remove("project");
                    o.remove("rank");
                    o.remove("added");
                }
                changed = true;
            }
            if changed {
                let _ = invite_save(serde_json::Value::Array(arr));
            }
        })
    }
}
