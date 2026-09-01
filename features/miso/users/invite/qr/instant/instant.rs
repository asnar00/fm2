struct feature_Instant;
impl feature_Instant {
    // ---- the token store -----------------------------------------------------
    // one row per live code, beside users.json under the same lock, with /qr's
    // three-way read: a list, or JSON null meaning "something is wrong — do not
    // write". A MISSING file is an empty list; a present but broken one is null.

    fn instant_file() -> String {
        format!("{}/invite-instant.json", auth_dir())
    }

    // the PIN's own window, which is what the ask asked for
    fn instant_ttl_ms() -> u64 {
        300000
    }

    fn instant_list() -> serde_json::Value {
        if !std::path::Path::new(&instant_file()).exists() {
            return serde_json::json!([]);
        }
        let raw = match std::fs::read_to_string(instant_file()) {
            Ok(r) => r,
            Err(e) => {
                println!("instant: the codes can't be read: {}", e);
                return serde_json::Value::Null;
            }
        };
        let v: serde_json::Value = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                println!("instant: the codes are not valid JSON ({}) — refusing to write", e);
                return serde_json::Value::Null;
            }
        };
        if !v.is_array() {
            println!("instant: the codes are not a JSON array — refusing to write");
            return serde_json::Value::Null;
        }
        v
    }

    // temp file, owner-only, rename — /invite's discipline, for its reasons
    fn instant_save(list: serde_json::Value) -> bool {
        if !list.is_array() {
            return false;
        }
        let body = match serde_json::to_string_pretty(&list) {
            Ok(b) => b,
            Err(_) => return false,
        };
        let _ = std::fs::create_dir_all(auth_dir());
        let tmp = format!("{}.tmp", instant_file());
        if std::fs::write(&tmp, body.as_bytes()).is_err() {
            println!("instant: could not write the codes");
            return false;
        }
        fm_own_only(&tmp);
        if std::fs::rename(&tmp, instant_file()).is_err() {
            println!("instant: could not replace the codes");
            let _ = std::fs::remove_file(&tmp);
            return false;
        }
        true
    }

    // a spent code is kept until it expires, so a second scan can be TOLD it was
    // used rather than being called invalid — the difference matters at a door
    fn instant_prune(list: serde_json::Value) -> serde_json::Value {
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for row in list.as_array().unwrap_or(&empty) {
            if row["expires"].as_u64().unwrap_or(0) > now_ms() {
                out.push(row.clone());
            }
        }
        serde_json::Value::Array(out)
    }

    fn instant_index_of(list: serde_json::Value, token: String) -> usize {
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

    // ---- tokens and keys -----------------------------------------------------

    fn instant_new_token() -> String {
        let bytes = random_bytes(16);
        let mut out = String::new();
        for b in bytes {
            out = format!("{}{:02x}", out, b);
        }
        out
    }

    // shape before store: anything not 32 hex characters never reaches the file
    fn instant_token_ok(t: String) -> bool {
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

    fn instant_short(t: String) -> String {
        t.chars().take(6).collect::<String>()
    }

    // a real E.164 number is at most 15 digits, so 17 can never be one. The test
    // answers from a bare string, which is why it is the test everywhere.
    fn instant_is_synthetic(phone: String) -> bool {
        let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
        digits.len() > 15
    }

    // 17 digits, leading 9. The last four are kept clear of every existing
    // entry's last four: nothing depends on that today (/to-owner discards the
    // `_from` that exchange truncates) but the truncation is one untick away,
    // and a mint is a cheap place to be certain.
    fn instant_new_number(list: serde_json::Value) -> String {
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut taken: Vec<String> = Vec::new();
        for u in list.as_array().unwrap_or(&empty) {
            let p = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
            if p.len() >= 4 {
                taken.push(p[p.len() - 4..].to_string());
            }
        }
        let mut tries = 0;
        loop {
            let bytes = random_bytes(17);
            let mut digits = String::from("9");
            for b in bytes.iter().skip(1) {
                digits = format!("{}{}", digits, (b % 10).to_string());
            }
            let last4 = digits[digits.len() - 4..].to_string();
            tries = tries + 1;
            if !taken.contains(&last4) || tries > 50 {
                return format!("+{}", digits);
            }
        }
    }

    fn instant_say(status: u16, words: String) -> response {
        invite_say(status, words)
    }

    // "a=1&b=2" -> the value of a key. Tokens are hex, so nothing needs decoding.
    fn instant_param(query: String, key: String) -> String {
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

    // ---- the routes ----------------------------------------------------------
    // newest node in the tree = outermost on the route chain, so the claim and
    // the page answer a device with no cookie, before /gate can turn it away.

    fn route(r: request) -> response {
        if r.path == "users/invite/instant/mint" && r.method == "POST" {
            return instant_mint(r);
        }
        if r.path == "users/invite/instant/claim" && r.method == "POST" {
            return instant_claim(r);
        }
        if r.path == "go" && r.method == "GET" {
            return instant_page();
        }
        existing.route(r)
    }

    // the scanned page IS the login page: its script scope already holds `post`,
    // `$` and /enrol, so the claim finishes down the same road a PIN does. Served
    // from here rather than left to /gate's 401, which only fires on the tunnel.
    fn instant_page() -> response {
        let html = std::fs::read("site/login.html").unwrap_or_default();
        response { status: 200, ctype: "text/html; charset=utf-8".to_string(),
                   body: html, set_cookie: String::new(),
                   cache: "no-store, must-revalidate".to_string() }
    }

    // mint: the account is made FIRST and the token bound to it, so the link
    // carries no user details — only a pointer to an entry that already exists.
    fn instant_mint(r: request) -> response {
        let who = invite_caller(r.cookie.clone());
        if !invite_may(who.clone()) {
            println!("instant: refused a code to {}",
                     if who.is_empty() { "nobody".to_string() } else { who.clone() });
            return invite_denied();
        }
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let name = v["name"].as_str().unwrap_or("").trim().to_string();
        if name.is_empty() {
            return instant_say(400, "that needs a name".to_string());
        }
        // a `_` name mints a test user whose codes go to the server log; nobody
        // should be able to mint one from here (/pretend lets an admin type one)
        if name.starts_with('_') {
            return instant_say(400, "a name can't start with _".to_string());
        }
        if name.chars().count() > 60 {
            return instant_say(400, "that name is too long".to_string());
        }
        with_store_lock(|| {
            let codes = instant_list();
            if codes.is_null() {
                return instant_say(500, "the codes can't be read".to_string());
            }
            let mut users = invite_list();
            if users.is_null() {
                return invite_unreadable();
            }
            let number = instant_new_number(users.clone());
            let entry = serde_json::json!({
                "name": name.clone(),
                "phone": number.clone(),
                "instant": true,
                "invited_by": who.clone(),
                "invited": now_ms()
            });
            match users.as_array_mut() {
                Some(a) => {
                    a.push(entry);
                }
                None => {
                    return invite_unreadable();
                }
            }
            if !invite_save(users) {
                return instant_say(500, "that didn't save — try again".to_string());
            }
            let row = serde_json::json!({
                "token": instant_new_token(),
                "key": number.clone(),
                "name": name.clone(),
                "by": who.clone(),
                "made": now_ms(),
                "expires": now_ms() + instant_ttl_ms(),
                "used": false
            });
            let mut codes = instant_prune(codes);
            match codes.as_array_mut() {
                Some(a) => {
                    a.push(row.clone());
                }
                None => {
                    return instant_say(500, "the codes can't be read".to_string());
                }
            }
            if !instant_save(codes) {
                return instant_say(500, "that code couldn't be saved".to_string());
            }
            println!("instant: {} minted {} a code {}…", who, name,
                     instant_short(row["token"].as_str().unwrap_or("").to_string()));
            json_response(200, serde_json::json!({
                "ok": true,
                "token": row["token"].as_str().unwrap_or(""),
                "name": name,
                "expires": row["expires"].as_u64().unwrap_or(0)
            }).to_string())
        })
    }

    // claim: the crossing. Read and spend happen in ONE locked section, so two
    // devices racing the same code cannot both be let in — the loser is told the
    // code was used, which is the true answer.
    fn instant_claim(r: request) -> response {
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let token = v["t"].as_str().unwrap_or("").to_string();
        if !instant_token_ok(token.clone()) {
            return instant_say(403, "this link isn't valid".to_string());
        }
        with_store_lock(|| {
            let codes = instant_list();
            if codes.is_null() {
                return instant_say(500, "the codes can't be read".to_string());
            }
            let at = instant_index_of(codes.clone(), token.clone());
            if at == usize::MAX {
                return instant_say(403, "this link isn't valid".to_string());
            }
            let row = codes[at].clone();
            if row["expires"].as_u64().unwrap_or(0) <= now_ms() {
                return instant_say(403,
                    "this code has expired — ask for a fresh one".to_string());
            }
            if row["used"].as_bool().unwrap_or(false) {
                return instant_say(403,
                    "this code was already used — ask for a fresh one".to_string());
            }
            let key = row["key"].as_str().unwrap_or("").to_string();
            // the account must still be there: an entry removed between mint and
            // scan must not hand out a cookie for nobody
            let name = find_user(key.clone());
            if name.is_empty() {
                return instant_say(403, "this link isn't valid".to_string());
            }
            let mut codes = codes;
            codes[at]["used"] = serde_json::json!(true);
            codes[at]["claimed"] = serde_json::json!(now_ms());
            if !instant_save(codes) {
                return instant_say(500, "that didn't save — try again".to_string());
            }
            println!("instant: {} came in on code {}…", name, instant_short(token));
            let mut resp = json_response(200,
                format!("{{\"ok\":true,\"name\":\"{}\"}}", name.replace('"', "'")));
            resp.set_cookie = format!(
                "miso_auth={}; Max-Age=31536000; Path=/; Secure; HttpOnly; SameSite=Lax",
                make_token(key));
            resp
        })
    }

    // ---- the two things that must not leak -----------------------------------

    // a synthetic key is not a number anyone can be reached on. Without this the
    // base would find the account and cheerfully text seventeen digits.
    fn auth_request(r: request) -> response {
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        if instant_is_synthetic(phone) {
            return json_response(403,
                "{\"ok\":false,\"error\":\"that number isn't on the guest list\"}".to_string());
        }
        existing.auth_request(r)
    }

    // the invited list already showed names only; this keeps the synthetic
    // string out of the row data as well, so it never reaches a page at all.
    fn invite_invited(r: request) -> response {
        let resp = existing.invite_invited(r);
        if resp.status != 200 {
            return resp;
        }
        let raw = String::from_utf8(resp.body.clone()).unwrap_or_default();
        let mut v: serde_json::Value = match serde_json::from_str(&raw) {
            Ok(x) => x,
            Err(_) => return resp,
        };
        let mut changed = false;
        if let Some(list) = v["list"].as_array_mut() {
            for u in list.iter_mut() {
                let p = u["phone"].as_str().unwrap_or("").to_string();
                if instant_is_synthetic(p) {
                    u["phone"] = serde_json::json!("");
                    u["instant"] = serde_json::json!(true);
                    changed = true;
                }
            }
        }
        if !changed {
            return resp;
        }
        json_response(200, v.to_string())
    }

    // ---- the page ------------------------------------------------------------

    // the second pill, beside /qr's own
    fn invite_rows_html(inv: serde_json::Value) -> String {
        let base = existing.invite_rows_html(inv.clone());
        format!("<div class=\"ins-lead\"><span class=\"ins-open\" data-ins=\"open\">add someone now</span></div>{}",
                base)
    }

    // the sheet: a name box, then the code. At a door the screen IS the code.
    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let q = s["invite_instant"].clone();
        if !q["open"].as_bool().unwrap_or(false) {
            return base;
        }
        let err = q["error"].as_str().unwrap_or("").to_string();
        let token = q["token"].as_str().unwrap_or("").to_string();
        let name = q["name"].as_str().unwrap_or("").to_string();
        let middle = if !token.is_empty() {
            format!("<div class=\"ins-frame\"><div class=\"ins-code\" data-ins-token=\"{}\"></div></div><div class=\"ins-word\">{} — scan to join</div>",
                    card_esc(token), card_esc(name))
        } else {
            let say = if err.is_empty() {
                String::new()
            } else {
                format!("<div class=\"ins-say\">{}</div>", card_esc(err))
            };
            // the sheet takes the whole screen, so it says what it is for in one
            // word the way /qr's does — a bare field on black is a puzzle
            format!("<div class=\"ins-word\">who's joining?</div><div class=\"ins-ask\"><input class=\"ins-name\" placeholder=\"name\" autocomplete=\"off\"><span class=\"ins-btn ins-go\" data-ins=\"mint\">show the code</span></div>{}",
                    say)
        };
        format!("{}<div class=\"ins-sheet\">{}<div class=\"ins-controls\"><span class=\"ins-btn\" data-ins=\"done\">done</span></div></div>",
                base, middle)
    }

    // the mint's answer under this node's own transient key — a code belongs to
    // the server and has no business syncing to devices as world state
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "InstantSheet" {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        s["invite_instant"] = e["data"].clone();
        s.to_string()
    }
}
