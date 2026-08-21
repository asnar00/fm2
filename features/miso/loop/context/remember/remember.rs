struct feature_Remember;
impl feature_Remember {
    // the load seam. Rung 5's link decides WHICH cell; this one decides whether
    // that cell is current, and rebuilds it from the user's log if it is not.
    // An empty identity — the whole wasm place, and startup — takes neither
    // branch, so nothing here ever runs on the client.
    fn held_context() -> std::sync::Arc<std::sync::RwLock<Context>> {
        let cell = existing.held_context();
        let who = context_user_now();
        if who.is_empty() {
            return cell;
        }
        context_reside(&who, &cell);
        cell
    }

    // the sweep runs once per request, before anything routes: O(resident
    // users) timestamp comparisons, at human request rate. A background thread
    // would need a shutdown story and would evict worlds nobody was asking
    // about; doing it on the way in costs nothing and can never evict the world
    // this request is for.
    fn route(r: request) -> response {
        let who = context_user_of(r.cookie.clone(), r.tunnel, r.query.clone());
        context_evicted(context_evict_idle(&who));
        if r.path == "diag/context/log" {
            return json_response(200, context_log_status());
        }
        existing.route(r)
    }

    // seam: what a world's departure takes with it. This link says it out
    // loud; a later node may have per-user state of its own to drop, and the
    // rule for anything hung here is that it must be rebuildable from the log,
    // because that is all an evicted user leaves behind.
    fn context_evicted(users: Vec<String>) {
        for user in users {
            eprintln!("miso: context evicted (idle): {}", user);
        }
    }

    // what persistence is doing, for the agent's instrument and for a human
    // reading the log after a bad night. Deliberately not user-scoped: it
    // describes the process, not a world.
    fn context_log_status() -> String {
        let h = context_log_health().lock().unwrap_or_else(|p| p.into_inner());
        serde_json::json!({
            "dir": context_dir(),
            "resident": context_resident_count(),
            "known": context_user_count(),
            "idle_ms": context_idle_ms(),
            "log_max": context_log_max(),
            "failures": h.0,
            "last_failure": h.1.clone(),
        }).to_string()
    }

    // the write seam for the op path. rung 6's link has already applied the op
    // to the sender's world and answered; if it answered with a CtxUpdate the
    // op was accepted, and the log records it EXACTLY as applied — the value
    // for a set, the delta for an add.
    fn handle_msg(msg: String) -> String {
        let reply = existing.handle_msg(msg.clone());
        let m: serde_json::Value = serde_json::from_str(&msg)
            .unwrap_or(serde_json::Value::Null);
        if m["type"].as_str().unwrap_or("") != "CtxOp" {
            return reply;
        }
        let answered: serde_json::Value = serde_json::from_str(&reply)
            .unwrap_or(serde_json::Value::Null);
        if answered["type"].as_str().unwrap_or("") != "CtxUpdate" {
            return reply;   // rejected: nothing happened, nothing is recorded
        }
        let who = context_user_now();
        if !who.is_empty() {
            let mut record = serde_json::json!({
                "path": m["data"]["path"].clone(),
                "name": m["data"]["name"].clone(),
                "op": m["data"]["op"].clone(),
                "value": m["data"]["value"].clone(),
            });
            // an op that carries an identity carries it into the log too, so a
            // restart can prime a seen-set from it. A composition whose ops have
            // no ids writes exactly the records it wrote before.
            if m["data"]["id"].is_string() {
                record["id"] = m["data"]["id"].clone();
            }
            context_log_append(&who, record);
        }
        reply
    }

    // the write seam for rung 3's tooling POST. That path assigns through
    // set_from_json rather than through a merge, so the record it produces says
    // "set" — true for every last-write var, and refused loudly at replay for a
    // crdt-sum one, which is the honest report of a path that predates the
    // merge column (see remember.md).
    fn context_set(r: request) -> response {
        let body = r.body.clone();
        let resp = existing.context_set(r);
        if resp.status != 200 {
            return resp;
        }
        let b: serde_json::Value = serde_json::from_str(&body)
            .unwrap_or(serde_json::Value::Null);
        let who = context_user_now();
        if !who.is_empty() && b["path"].is_string() && b["name"].is_string() {
            context_log_append(&who, serde_json::json!({
                "path": b["path"].clone(),
                "name": b["name"].clone(),
                "op": "set",
                "value": b["value"].clone(),
            }));
        }
        resp
    }
}
