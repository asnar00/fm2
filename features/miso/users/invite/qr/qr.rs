struct feature_Qr;
impl feature_Qr {
    // ---- the token store ---------------------------------------------------
    // one row per inviter, beside users.json, under the same lock. Read is a
    // three-way answer like /invite's: a list, or JSON null meaning "something
    // is wrong — do not write". A MISSING file is an empty list (the first mint
    // on a fresh box is not a fault); a present but broken one is null.

    fn qr_file() -> String {
        format!("{}/invite-qr.json", auth_dir())
    }

    // a canvassing session, in milliseconds
    fn qr_ttl_ms() -> u64 {
        86400000
    }

    // how many people one code may put on the guest list before it is spent
    fn qr_cap() -> u64 {
        25
    }

    // the floor between two claims on one code: a door takes longer than this,
    // a script does not
    fn qr_gap_ms() -> u64 {
        2000
    }

    fn qr_list() -> serde_json::Value {
        if !std::path::Path::new(&qr_file()).exists() {
            return serde_json::json!([]);
        }
        let raw = match std::fs::read_to_string(qr_file()) {
            Ok(r) => r,
            Err(e) => {
                println!("qr: the invite codes can't be read: {}", e);
                return serde_json::Value::Null;
            }
        };
        let v: serde_json::Value = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                println!("qr: the invite codes are not valid JSON ({}) — refusing to write", e);
                return serde_json::Value::Null;
            }
        };
        if !v.is_array() {
            println!("qr: the invite codes are not a JSON array — refusing to write");
            return serde_json::Value::Null;
        }
        v
    }

    // temp file, owner-only, rename — /invite's discipline, for the same reason:
    // a crash mid-write must leave the old file whole and a fresh one must never
    // be born world-readable.
    fn qr_save(list: serde_json::Value) -> bool {
        if !list.is_array() {
            return false;
        }
        let body = match serde_json::to_string_pretty(&list) {
            Ok(b) => b,
            Err(_) => return false,
        };
        let _ = std::fs::create_dir_all(auth_dir());
        let tmp = format!("{}.tmp", qr_file());
        if std::fs::write(&tmp, body.as_bytes()).is_err() {
            println!("qr: could not write the invite codes");
            return false;
        }
        fm_own_only(&tmp);
        if std::fs::rename(&tmp, qr_file()).is_err() {
            println!("qr: could not replace the invite codes");
            let _ = std::fs::remove_file(&tmp);
            return false;
        }
        true
    }

    // ---- tokens ------------------------------------------------------------

    fn qr_new_token() -> String {
        let bytes = random_bytes(16);
        let mut out = String::new();
        for b in bytes {
            out = format!("{}{:02x}", out, b);
        }
        out
    }

    // shape before store: anything that is not 32 hex characters never reaches
    // the file, so a probe cannot make the server open it
    fn qr_token_ok(t: String) -> bool {
        if t.len() != 32 {
            return false;
        }
        for c in t.chars() {
            if !c.is_ascii_hexdigit() {
                return false;
            }
        }
        true
    }

    // never log a whole code — the log is the one place it could leak from
    fn qr_short(t: String) -> String {
        t.chars().take(6).collect::<String>()
    }

    fn qr_owner_phone(by: String) -> String {
        by.trim_start_matches("phone:").to_string()
    }

    // drop everything already dead; the file is one row per inviter, so this is
    // what keeps it from filling
    fn qr_prune(list: serde_json::Value) -> serde_json::Value {
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for row in list.as_array().unwrap_or(&empty) {
            if row["expires"].as_u64().unwrap_or(0) > now_ms() {
                out.push(row.clone());
            }
        }
        serde_json::Value::Array(out)
    }

    fn qr_index_of(list: serde_json::Value, token: String) -> usize {
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut at = usize::MAX;
        let mut i = 0usize;
        for row in list.as_array().unwrap_or(&empty) {
            if row["token"].as_str().unwrap_or("") == token {
                at = i;
            }
            i = i + 1;
        }
        at
    }

    fn qr_index_by(list: serde_json::Value, who: String) -> usize {
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut at = usize::MAX;
        let mut i = 0usize;
        for row in list.as_array().unwrap_or(&empty) {
            if row["by"].as_str().unwrap_or("") == who {
                at = i;
            }
            i = i + 1;
        }
        at
    }

    fn qr_unreadable() -> response {
        invite_say(500, "the invite codes can't be read".to_string())
    }

    // "a=1&b=2" -> the value of a key, or empty. Tokens are hex, so nothing
    // needs percent-decoding here.
    fn qr_param(query: String, key: String) -> String {
        for part in query.split('&') {
            let mut bits = part.splitn(2, '=');
            let k = bits.next().unwrap_or("");
            let v = bits.next().unwrap_or("");
            if key == k {
                return v.to_string();
            }
        }
        String::new()
    }

    // ---- the routes --------------------------------------------------------
    // this node is the newest in the tree, so it is the OUTERMOST link of the
    // route chain and sees a request before /gate does. That is what lets the
    // three public routes work for a stranger at all — and why each one states
    // its own gate here rather than inheriting one.

    fn route(r: request) -> response {
        if r.path == "users/invite/qr/mint" && r.method == "POST" {
            return qr_mint(r);
        }
        if r.path == "users/invite/qr/revoke" && r.method == "POST" {
            return qr_revoke(r);
        }
        if r.path == "users/invite/qr/check" && r.method == "GET" {
            return qr_check(r);
        }
        if r.path == "users/invite/qr/claim" && r.method == "POST" {
            return qr_claim(r);
        }
        if r.path == "join" && r.method == "GET" {
            return qr_page();
        }
        existing.route(r)
    }

    // the stranger's page, served under its own short path so the code encodes
    // "…/join?t=<token>" and nothing has to redirect
    fn qr_page() -> response {
        let html = std::fs::read("site/join.html").unwrap_or_default();
        response { status: 200, ctype: "text/html; charset=utf-8".to_string(),
                   body: html, set_cookie: String::new(),
                   cache: "no-store, must-revalidate".to_string() }
    }

    fn qr_row_json(row: serde_json::Value) -> response {
        json_response(200, serde_json::json!({
            "ok": true,
            "token": row["token"].as_str().unwrap_or(""),
            "expires": row["expires"].as_u64().unwrap_or(0),
            "uses": row["uses"].as_u64().unwrap_or(0),
            "cap": row["cap"].as_u64().unwrap_or(qr_cap())
        }).to_string())
    }

    // mint: the caller's live code, or a new one. `fresh` is the "new code" tap
    // — it REPLACES the row, which is what makes the old code dead at once.
    fn qr_mint(r: request) -> response {
        let who = invite_caller(r.cookie.clone());
        if !invite_may(who.clone()) {
            println!("qr: refused a code to {}",
                     if who.is_empty() { "nobody".to_string() } else { who.clone() });
            return invite_denied();
        }
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let fresh = v["fresh"].as_bool().unwrap_or(false);
        with_store_lock(|| {
            let list = qr_list();
            if list.is_null() {
                return qr_unreadable();
            }
            let mut list = qr_prune(list);
            let at = qr_index_by(list.clone(), who.clone());
            if at != usize::MAX && !fresh {
                let row = list[at].clone();
                if !qr_save(list) {
                    return invite_say(500, "that code couldn't be saved".to_string());
                }
                return qr_row_json(row);
            }
            if at != usize::MAX {
                match list.as_array_mut() {
                    Some(a) => {
                        a.remove(at);
                    }
                    None => {
                        return qr_unreadable();
                    }
                }
            }
            let row = serde_json::json!({
                "token": qr_new_token(),
                "by": who.clone(),
                "made": now_ms(),
                "expires": now_ms() + qr_ttl_ms(),
                "uses": 0,
                "cap": qr_cap(),
                "last": 0
            });
            match list.as_array_mut() {
                Some(a) => {
                    a.push(row.clone());
                }
                None => {
                    return qr_unreadable();
                }
            }
            if !qr_save(list) {
                return invite_say(500, "that code couldn't be saved".to_string());
            }
            println!("qr: {} minted code {}…", who,
                     qr_short(row["token"].as_str().unwrap_or("").to_string()));
            qr_row_json(row)
        })
    }

    // revoke: the caller's own row, gone. Not an admin power over other people's
    // codes — a code is revoked by its owner or by time.
    fn qr_revoke(r: request) -> response {
        let who = invite_caller(r.cookie.clone());
        if !invite_may(who.clone()) {
            return invite_denied();
        }
        with_store_lock(|| {
            let list = qr_list();
            if list.is_null() {
                return qr_unreadable();
            }
            let mut list = qr_prune(list);
            let at = qr_index_by(list.clone(), who.clone());
            if at != usize::MAX {
                match list.as_array_mut() {
                    Some(a) => {
                        a.remove(at);
                    }
                    None => {
                        return qr_unreadable();
                    }
                }
            }
            if !qr_save(list) {
                return invite_say(500, "that code couldn't be put away".to_string());
            }
            println!("qr: {} put their code away", who);
            json_response(200, "{\"ok\":true}".to_string())
        })
    }

    // the one sentence a dead code gets, in the two flavours a person can tell
    // apart: one is "you are too late", the other is "this isn't a miso link".
    fn qr_look_up(token: String) -> serde_json::Value {
        if !qr_token_ok(token.clone()) {
            return serde_json::json!({ "error": "this invite link isn't valid" });
        }
        let list = qr_list();
        if list.is_null() {
            return serde_json::json!({ "error": "the invite codes can't be read" });
        }
        let at = qr_index_of(list.clone(), token);
        if at == usize::MAX {
            return serde_json::json!({ "error": "this invite link isn't valid" });
        }
        let row = list[at].clone();
        if row["expires"].as_u64().unwrap_or(0) <= now_ms() {
            return serde_json::json!({ "error": "this invite has expired" });
        }
        let by = row["by"].as_str().unwrap_or("").to_string();
        if !invite_may(by.clone()) {
            return serde_json::json!({ "error": "this invite has expired" });
        }
        if row["uses"].as_u64().unwrap_or(0) >= row["cap"].as_u64().unwrap_or(qr_cap()) {
            return serde_json::json!({ "error": "this invite has been used up" });
        }
        row
    }

    // check: what the claim page asks before it draws anything. It answers with
    // the inviter's NAME, which is the point — the person at the door should see
    // who is asking them in — and with nothing else about anybody.
    fn qr_check(r: request) -> response {
        let token = qr_param(r.query.clone(), "t".to_string());
        let row = qr_look_up(token);
        let bad = row["error"].as_str().unwrap_or("").to_string();
        if !bad.is_empty() {
            return json_response(200, serde_json::json!({
                "ok": false, "error": bad
            }).to_string());
        }
        let name = find_user(qr_owner_phone(row["by"].as_str().unwrap_or("").to_string()));
        json_response(200, serde_json::json!({
            "ok": true, "by": name
        }).to_string())
    }

    // claim: the crossing. Everything a typed invite checks is checked here too,
    // with the token standing in for the authority — plus the three bounds that
    // are this feature's security (cap, gap, expiry), all server-side.
    fn qr_claim(r: request) -> response {
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let token = v["t"].as_str().unwrap_or("").to_string();
        let name = v["name"].as_str().unwrap_or("").trim().to_string();
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        // a `_` name mints a test user whose codes go to the server log
        // (/users); /pretend lets an admin TYPE one, but nobody should be able
        // to mint one by scanning something
        if name.starts_with('_') {
            return invite_say(400, "a name can't start with _".to_string());
        }
        let bad = invite_shape_ok(name.clone(), phone.clone());
        if !bad.is_empty() {
            return invite_say(400, bad);
        }
        with_store_lock(|| {
            let codes = qr_list();
            if codes.is_null() {
                return qr_unreadable();
            }
            let row = qr_look_up(token.clone());
            let dead = row["error"].as_str().unwrap_or("").to_string();
            if !dead.is_empty() {
                return invite_say(403, dead);
            }
            if now_ms() < row["last"].as_u64().unwrap_or(0) + qr_gap_ms() {
                return invite_say(429, "one moment — try that again".to_string());
            }
            let by = row["by"].as_str().unwrap_or("").to_string();
            let mut list = invite_list();
            if list.is_null() {
                return invite_unreadable();
            }
            // a number already on the list is answered exactly as a fresh one:
            // a leaked code must not become a way to ask "is this number in the
            // campaign?", and a person re-scanning should simply carry on to
            // their code. Nothing is written and no use is spent.
            let empty: Vec<serde_json::Value> = Vec::new();
            let mut seen = false;
            for u in list.as_array().unwrap_or(&empty) {
                let up = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
                if !up.is_empty() && up == phone {
                    seen = true;
                }
            }
            if seen {
                println!("qr: a claim on {}… was already on the list", qr_short(token));
                return json_response(200, "{\"ok\":true}".to_string());
            }
            let entry = serde_json::json!({
                "name": name.clone(),
                "phone": phone.clone(),
                "invited_by": by.clone(),
                "invited": now_ms()
            });
            match list.as_array_mut() {
                Some(a) => {
                    a.push(entry);
                }
                None => {
                    return invite_unreadable();
                }
            }
            if !invite_save(list) {
                return invite_say(500, "that didn't save — try again".to_string());
            }
            // the use is spent only once the guest list actually took them
            let mut codes = qr_prune(codes);
            let at = qr_index_of(codes.clone(), token.clone());
            if at != usize::MAX {
                codes[at]["uses"] = serde_json::json!(row["uses"].as_u64().unwrap_or(0) + 1);
                codes[at]["last"] = serde_json::json!(now_ms());
                let _ = qr_save(codes);
            }
            println!("qr: {} joined on {}'s code {}… ({})",
                     name, by, qr_short(token), tag(phone));
            json_response(200, "{\"ok\":true}".to_string())
        })
    }

    // ---- the page ----------------------------------------------------------

    // the way in: one pill above the invite rows, wherever they are drawn
    fn invite_rows_html(inv: serde_json::Value) -> String {
        let base = existing.invite_rows_html(inv.clone());
        format!("<div class=\"qr-lead\"><span class=\"qr-open\" data-qr=\"open\">show a QR code</span></div>{}",
                base)
    }

    // the sheet itself: everything else on the screen goes away, because at a
    // door the screen IS the code
    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let q = s["invite_qr"].clone();
        if !q["open"].as_bool().unwrap_or(false) {
            return base;
        }
        let err = q["error"].as_str().unwrap_or("").to_string();
        let token = q["token"].as_str().unwrap_or("").to_string();
        let middle = if !err.is_empty() {
            format!("<div class=\"qr-say\">{}</div>", card_esc(err))
        } else {
            format!("<div class=\"qr-frame\"><div class=\"qr-code\" data-qr-token=\"{}\"></div></div>",
                    card_esc(token))
        };
        let uses = q["uses"].as_u64().unwrap_or(0);
        let count = if uses == 0 {
            String::new()
        } else if uses == 1 {
            "<div class=\"qr-count\">1 signed up</div>".to_string()
        } else {
            format!("<div class=\"qr-count\">{} signed up</div>", uses)
        };
        format!("{}<div class=\"qr-sheet\">{}<div class=\"qr-word\">join miso</div>{}<div class=\"qr-controls\"><span class=\"qr-btn\" data-qr=\"new\">new code</span><span class=\"qr-btn\" data-qr=\"done\">done</span></div></div>",
                base, middle, count)
    }

    // the mint's answer, verbatim, under this node's own state key — server
    // state, like /invite's list, and for the same reason: a code belongs to
    // the server and has no business syncing to devices as world state
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "QrSheet" {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        s["invite_qr"] = e["data"].clone();
        s.to_string()
    }
}
