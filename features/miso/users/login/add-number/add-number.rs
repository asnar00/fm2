struct feature_AddNumber;
impl feature_AddNumber {
    // ---- who is asking -------------------------------------------------------
    // the account's own key, off the cookie and nothing else: adding a number is
    // the owner's own act, so the localhost tooling door has no say in it.

    fn addnum_who(cookie: String) -> String {
        let t = cookie_token(cookie);
        if !t.is_empty() && token_valid(t.clone()) {
            token_phone(t)
        } else {
            String::new()
        }
    }

    fn addnum_users() -> serde_json::Value {
        let raw = std::fs::read_to_string(format!("{}/users.json", auth_dir()))
            .unwrap_or_default();
        let v: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::Value::Null);
        if v.is_array() {
            v
        } else {
            serde_json::Value::Null
        }
    }

    // the alias recorded on one account, if any
    fn addnum_alias_of(key: String) -> String {
        let list = addnum_users();
        let empty: Vec<serde_json::Value> = Vec::new();
        for u in list.as_array().unwrap_or(&empty) {
            let p = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
            if !p.is_empty() && p == key {
                return normalise_phone(u["alias"].as_str().unwrap_or("").to_string());
            }
        }
        String::new()
    }

    // the account a number is an alias FOR — the other direction, and the one
    // the login road asks
    fn addnum_account_for_alias(phone: String) -> String {
        if phone.is_empty() {
            return String::new();
        }
        let list = addnum_users();
        let empty: Vec<serde_json::Value> = Vec::new();
        for u in list.as_array().unwrap_or(&empty) {
            let a = normalise_phone(u["alias"].as_str().unwrap_or("").to_string());
            if !a.is_empty() && a == phone {
                return normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
            }
        }
        String::new()
    }

    // one number, one account: its own number or anybody's alias both count
    fn addnum_taken(phone: String) -> bool {
        if phone.is_empty() {
            return false;
        }
        let list = addnum_users();
        let empty: Vec<serde_json::Value> = Vec::new();
        for u in list.as_array().unwrap_or(&empty) {
            let p = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
            if !p.is_empty() && p == phone {
                return true;
            }
            let a = normalise_phone(u["alias"].as_str().unwrap_or("").to_string());
            if !a.is_empty() && a == phone {
                return true;
            }
        }
        false
    }

    fn addnum_say(status: u16, words: String) -> response {
        json_response(status, format!("{{\"ok\":false,\"error\":\"{}\"}}",
                                      words.replace('"', "'")))
    }

    // the same two rules the guest list has always had for a typed number
    fn addnum_shape_ok(phone: String) -> String {
        if phone.len() < 8 {
            return "that doesn't look like a phone number".to_string();
        }
        if phone.starts_with("+0") {
            return "that number needs its country code".to_string();
        }
        String::new()
    }

    // ---- the routes ----------------------------------------------------------

    fn route(r: request) -> response {
        if r.path == "users/number" && r.method == "GET" {
            return addnum_state(r);
        }
        if r.path == "users/number/request" && r.method == "POST" {
            return addnum_request(r);
        }
        if r.path == "users/number/confirm" && r.method == "POST" {
            return addnum_confirm(r);
        }
        existing.route(r)
    }

    // what the slot draws from: the number if there is one, and nothing about
    // anybody else
    fn addnum_state(r: request) -> response {
        let who = addnum_who(r.cookie.clone());
        if who.is_empty() {
            return addnum_say(403, "log in first".to_string());
        }
        json_response(200, serde_json::json!({
            "ok": true,
            "number": addnum_alias_of(who)
        }).to_string())
    }

    // request: the caller, the shape, the one-account rule, then the code —
    // through the PIN machinery's own send, cap and pending file
    fn addnum_request(r: request) -> response {
        let who = addnum_who(r.cookie.clone());
        if who.is_empty() {
            return addnum_say(403, "log in first".to_string());
        }
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        let bad = addnum_shape_ok(phone.clone());
        if !bad.is_empty() {
            return addnum_say(400, bad);
        }
        if addnum_taken(phone.clone()) {
            return addnum_say(400, "that number is already on the campaign".to_string());
        }
        if sms_count_last_hour(phone.clone()) >= 5 {
            return addnum_say(429, "too many codes sent — try later".to_string());
        }
        let pin = make_pin();
        save_pending(phone.clone(), pin.clone());
        let name = find_user(who.clone());
        if name.starts_with("_") {
            println!("addnum: test user {} pin {}", name, pin);
        } else {
            let err = send_sms(phone.clone(), format!("miso code: {}", pin));
            if !err.is_empty() {
                clear_pending(phone.clone());
                println!("addnum: sms send failed: {}", err);
                return addnum_say(500, "couldn't send the code — try again".to_string());
            }
            record_sms(phone.clone());
        }
        println!("addnum: {} asked to add {}", name, tag(phone));
        json_response(200, "{\"ok\":true,\"sent\":true}".to_string())
    }

    // confirm: the code, checked exactly as auth_verify checks one, and then the
    // alias written under the lock with the one-account rule asked again
    fn addnum_confirm(r: request) -> response {
        let who = addnum_who(r.cookie.clone());
        if who.is_empty() {
            return addnum_say(403, "log in first".to_string());
        }
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        let pin = v["pin"].as_str().unwrap_or("").trim().to_string();
        let p = load_pending(phone.clone());
        if p.is_empty() {
            return addnum_say(403, "no code pending — ask for one".to_string());
        }
        let parts: Vec<&str> = p.split(' ').collect();
        let want = parts[0].to_string();
        let expires: u64 = parts[1].parse().unwrap_or(0);
        let attempts: u64 = parts[2].parse().unwrap_or(0);
        if expires < now_ms() {
            clear_pending(phone);
            return addnum_say(403, "code expired — ask for another".to_string());
        }
        if attempts + 1 > 3 {
            clear_pending(phone);
            return addnum_say(403, "too many tries — ask for another code".to_string());
        }
        if !constant_eq(want.clone(), pin) {
            set_pending_line(phone.clone(),
                             format!("{} {} {}", want, expires, attempts + 1));
            return addnum_say(401, "wrong code".to_string());
        }
        clear_pending(phone.clone());
        with_store_lock(|| {
            if addnum_taken(phone.clone()) {
                return addnum_say(400,
                    "that number is already on the campaign".to_string());
            }
            let mut list = invite_list();
            if list.is_null() {
                return addnum_say(500, "the guest list can't be read".to_string());
            }
            let mut arr = match list.as_array() {
                Some(a) => a.clone(),
                None => {
                    return addnum_say(500, "the guest list can't be read".to_string());
                }
            };
            let mut changed = false;
            for u in arr.iter_mut() {
                let p = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
                if !p.is_empty() && p == who {
                    u["alias"] = serde_json::json!(phone.clone());
                    u["alias_added"] = serde_json::json!(now_ms());
                    changed = true;
                }
            }
            if !changed {
                return addnum_say(404, "we couldn't find your account".to_string());
            }
            list = serde_json::Value::Array(arr);
            if !invite_save(list) {
                return addnum_say(500, "that didn't save — try again".to_string());
            }
            println!("addnum: {} added {} as a login alias",
                     find_user(who.clone()), tag(phone));
            json_response(200, "{\"ok\":true}".to_string())
        })
    }

    // ---- the two points the alias reaches into the login road ----------------

    // an alias answers as its account's name, which is what lets auth/request
    // find the account and text a code to it
    fn find_user(phone: String) -> String {
        let n = existing.find_user(phone.clone());
        if !n.is_empty() {
            return n;
        }
        let key = addnum_account_for_alias(phone);
        if key.is_empty() {
            return String::new();
        }
        existing.find_user(key)
    }

    // the swap that keeps the world key still: the code was checked against the
    // number that was typed, but the SESSION is issued for the account's own
    // key. Without this an alias login would land in a second, empty world.
    fn auth_verify(r: request) -> response {
        let body = r.body.clone();
        let mut resp = existing.auth_verify(r);
        if resp.status != 200 || resp.set_cookie.is_empty() {
            return resp;
        }
        let v: serde_json::Value = serde_json::from_str(&body)
            .unwrap_or(serde_json::Value::Null);
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        let key = addnum_account_for_alias(phone.clone());
        if key.is_empty() {
            return resp;
        }
        println!("auth: {} logged in on their added number", tag(key.clone()));
        resp.set_cookie = format!(
            "miso_auth={}; Max-Age=31536000; Path=/; Secure; HttpOnly; SameSite=Lax",
            make_token(key));
        resp
    }

    // ---- the slot ------------------------------------------------------------
    // one quiet row under your own card, in its .crow grammar. Nothing is drawn
    // until the server has spoken, so the row never flickers a wrong state.

    fn me_under(state: String) -> String {
        let base = existing.me_under(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let n = s["mynumber"].clone();
        if !n["ok"].as_bool().unwrap_or(false) {
            return base;
        }
        let have = n["number"].as_str().unwrap_or("").to_string();
        let body = if !have.is_empty() {
            format!("<div class=\"crow addnum-row\"><span class=\"cnum addnum-status\">number</span><div class=\"ctext\">{}</div></div>",
                    card_esc(have.clone()))
        } else if n["sent"].as_bool().unwrap_or(false) {
            String::from("<div class=\"crow addnum-new\"><input class=\"addnum-pin\" placeholder=\"code\" inputmode=\"numeric\" autocomplete=\"one-time-code\"><span class=\"addnum-do\" data-addnum=\"confirm\">confirm</span></div>")
        } else {
            String::from("<div class=\"crow addnum-new\"><input class=\"addnum-phone\" placeholder=\"add your number\" inputmode=\"tel\" autocomplete=\"tel\"><span class=\"addnum-do\" data-addnum=\"send\">add</span></div>")
        };
        let err = n["error"].as_str().unwrap_or("").to_string();
        let say = if err.is_empty() {
            String::new()
        } else {
            format!("<div class=\"addnum-say\">{}</div>", card_esc(err))
        };
        let note = if have.is_empty() && !n["sent"].as_bool().unwrap_or(false) {
            String::from("<div class=\"addnum-note\">we'll text a code</div>")
        } else {
            String::new()
        };
        format!("{}<div class=\"addnum\">{}{}{}</div>", base, body, say, note)
    }

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "MyNumber" {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        s["mynumber"] = e["data"].clone();
        s.to_string()
    }
}
