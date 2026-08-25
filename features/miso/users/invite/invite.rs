struct feature_Invite;
impl feature_Invite {
    // ---- the store: the guest list itself --------------------------------
    // users.json is the one file the app must never wipe, so reading it is a
    // three-way answer and not two: a list, or JSON null meaning "something is
    // wrong — do not write". Missing, unreadable, not JSON, not an array all
    // fold into null, loudly. Nothing in this node writes on a null.

    fn invite_file() -> String {
        format!("{}/users.json", auth_dir())
    }

    fn invite_list() -> serde_json::Value {
        let raw = match std::fs::read_to_string(invite_file()) {
            Ok(r) => r,
            Err(e) => {
                println!("invite: the guest list can't be read: {}", e);
                return serde_json::Value::Null;
            }
        };
        let v: serde_json::Value = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                println!("invite: the guest list is not valid JSON ({}) — refusing to write it", e);
                return serde_json::Value::Null;
            }
        };
        if !v.is_array() {
            println!("invite: the guest list is not a JSON array — refusing to write it");
            return serde_json::Value::Null;
        }
        v
    }

    // temp file, owner-only, rename: a crash mid-write leaves the old list
    // whole, and a freshly written list is never born world-readable (rename
    // carries the temp file's permissions, not the old file's).
    fn invite_save(list: serde_json::Value) -> bool {
        if !list.is_array() {
            return false;
        }
        let body = match serde_json::to_string_pretty(&list) {
            Ok(b) => b,
            Err(_) => return false,
        };
        let tmp = format!("{}.tmp", invite_file());
        if std::fs::write(&tmp, body.as_bytes()).is_err() {
            println!("invite: could not write the guest list");
            return false;
        }
        fm_own_only(&tmp);
        if std::fs::rename(&tmp, invite_file()).is_err() {
            println!("invite: could not replace the guest list");
            let _ = std::fs::remove_file(&tmp);
            return false;
        }
        true
    }

    // ---- who is asking ----------------------------------------------------
    // these routes are OUTSIDE /gate's wall (this node is newer, so it is
    // outermost on the route chain and sees a request first), which means the
    // cookie check is this node's own job and not an inherited one.

    fn invite_caller(cookie: String) -> String {
        let t = cookie_token(cookie);
        if !t.is_empty() && token_valid(t.clone()) {
            format!("phone:{}", token_phone(t))
        } else {
            String::new()
        }
    }

    // inviting is a shared-reach act — the guest list is the one list everyone
    // is on — so it takes the same rung /authority's may_write_shared takes.
    fn invite_may(who: String) -> bool {
        !who.is_empty() && authority_rank(who) >= 2
    }

    fn invite_admin(who: String) -> bool {
        authority_rank(who) >= 3
    }

    fn invite_say(status: u16, words: String) -> response {
        json_response(status, format!("{{\"ok\":false,\"error\":\"{}\"}}",
                                      words.replace('"', "'")))
    }

    fn invite_denied() -> response {
        invite_say(403, "you can't invite people".to_string())
    }

    fn invite_unreadable() -> response {
        invite_say(500, "the guest list can't be read".to_string())
    }

    // ---- the three routes -------------------------------------------------

    // the store's health is the first question these three routes ask, before
    // the cookie: with users.json unreadable NOBODY is authed (token_valid
    // re-checks the guest list), so an authority-first order would answer a
    // broken box with "you can't invite people" and log nothing true. Rig-found.
    fn route(r: request) -> response {
        if r.path == "users/invite" || r.path == "users/uninvite"
            || r.path == "users/invited" {
            if invite_list().is_null() {
                return invite_unreadable();
            }
        }
        if r.path == "users/invite" && r.method == "POST" {
            return invite_add(r);
        }
        if r.path == "users/uninvite" && r.method == "POST" {
            return invite_remove(r);
        }
        if r.path == "users/invited" && r.method == "GET" {
            return invite_invited(r);
        }
        existing.route(r)
    }

    // add: authority, then shape, then the lock. The duplicate check is inside
    // the lock with the append, so two sends at once cannot both pass it.
    fn invite_add(r: request) -> response {
        let who = invite_caller(r.cookie.clone());
        if !invite_may(who.clone()) {
            println!("invite: refused an invite from {}",
                     if who.is_empty() { "nobody".to_string() } else { who.clone() });
            return invite_denied();
        }
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let name = v["name"].as_str().unwrap_or("").trim().to_string();
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        if name.is_empty() {
            return invite_say(400, "that invite needs a name".to_string());
        }
        if phone.len() < 8 {
            return invite_say(400, "that doesn't look like a phone number".to_string());
        }
        // "07700 900003" normalises to +07700900003, which is a DIFFERENT
        // number to the whole tree — the guest list has always wanted the
        // country code (users.md), and a person typed in without one could
        // never log in. No country code begins with a zero, so this catches
        // the trunk prefix without pretending to know which country you meant.
        if phone.starts_with("+0") {
            return invite_say(400, "that number needs its country code".to_string());
        }
        with_store_lock(|| {
            let mut list = invite_list();
            if list.is_null() {
                return invite_unreadable();
            }
            let mut seen = false;
            let empty: Vec<serde_json::Value> = Vec::new();
            for u in list.as_array().unwrap_or(&empty) {
                let up = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
                if !up.is_empty() && up == phone {
                    seen = true;
                }
            }
            if seen {
                return invite_say(400, "they're already on the list".to_string());
            }
            let entry = serde_json::json!({
                "name": name.clone(),
                "phone": phone.clone(),
                "invited_by": who.clone(),
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
                return invite_say(500, "that invite couldn't be saved".to_string());
            }
            println!("invite: {} invited {} ({})", who, name, tag(phone));
            json_response(200, "{\"ok\":true}".to_string())
        })
    }

    // remove: only an invite (it carries `invited`), only one nobody has used,
    // and only your own unless you are admin. A hand-written guest-list entry
    // can never be deleted through the app.
    fn invite_remove(r: request) -> response {
        let who = invite_caller(r.cookie.clone());
        if !invite_may(who.clone()) {
            return invite_denied();
        }
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        if phone.is_empty() {
            return invite_say(400, "that doesn't look like a phone number".to_string());
        }
        with_store_lock(|| {
            let mut list = invite_list();
            if list.is_null() {
                return invite_unreadable();
            }
            let empty: Vec<serde_json::Value> = Vec::new();
            let mut at = usize::MAX;
            let mut i = 0usize;
            for u in list.as_array().unwrap_or(&empty) {
                let up = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
                if !up.is_empty() && up == phone {
                    at = i;
                }
                i = i + 1;
            }
            if at == usize::MAX {
                return invite_say(404, "they're not on the list".to_string());
            }
            let entry = list[at].clone();
            if !entry["invited"].is_u64() {
                return invite_say(403, "that one wasn't invited from here".to_string());
            }
            if entry["joined"].is_u64() {
                return invite_say(403, "they've already joined".to_string());
            }
            let by = entry["invited_by"].as_str().unwrap_or("").to_string();
            if by != who && !invite_admin(who.clone()) {
                return invite_say(403, "that isn't your invite".to_string());
            }
            match list.as_array_mut() {
                Some(a) => {
                    a.remove(at);
                }
                None => {
                    return invite_unreadable();
                }
            }
            if !invite_save(list) {
                return invite_say(500, "that invite couldn't be removed".to_string());
            }
            println!("invite: {} removed the invite for {}", who, tag(phone));
            json_response(200, "{\"ok\":true}".to_string())
        })
    }

    // the list, and the whole of the member seam: a member is told `may:false`
    // and given nothing, so the page has nothing to draw and no secret to keep.
    fn invite_invited(r: request) -> response {
        let who = invite_caller(r.cookie.clone());
        if who.is_empty() {
            return invite_denied();
        }
        if !invite_may(who.clone()) {
            return json_response(200, "{\"ok\":true,\"may\":false,\"list\":[]}".to_string());
        }
        let list = invite_list();
        if list.is_null() {
            return invite_unreadable();
        }
        let empty: Vec<serde_json::Value> = Vec::new();
        let mut out: Vec<serde_json::Value> = Vec::new();
        for u in list.as_array().unwrap_or(&empty) {
            let invited = u["invited"].as_u64().unwrap_or(0);
            if invited == 0 {
                continue;
            }
            if u["invited_by"].as_str().unwrap_or("") != who {
                continue;
            }
            let joined = u["joined"].as_u64().unwrap_or(0);
            out.push(serde_json::json!({
                "name": u["name"].as_str().unwrap_or(""),
                "phone": u["phone"].as_str().unwrap_or(""),
                "joined": joined >= invited && joined > 0
            }));
        }
        json_response(200, serde_json::json!({
            "ok": true, "may": true, "list": out
        }).to_string())
    }

    // ---- joined ------------------------------------------------------------
    // the cheapest honest signal that a phone has logged in is the login
    // itself. The stamp is taken AFTER the inner chain returns, because
    // /code-guard already holds the store lock for the whole of a verify and
    // the lock is not reentrant.

    fn auth_verify(r: request) -> response {
        let body = r.body.clone();
        let resp = existing.auth_verify(r);
        if resp.status != 200 {
            return resp;
        }
        let v: serde_json::Value = serde_json::from_str(&body)
            .unwrap_or(serde_json::Value::Null);
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        if !phone.is_empty() {
            invite_stamp_joined(phone);
        }
        resp
    }

    fn invite_stamp_joined(phone: String) {
        with_store_lock(|| {
            let list = invite_list();
            if list.is_null() {
                return;
            }
            let mut changed = false;
            let mut arr = match list.as_array() {
                Some(a) => a.clone(),
                None => {
                    return;
                }
            };
            for u in arr.iter_mut() {
                let up = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
                if !up.is_empty() && up == phone {
                    u["joined"] = serde_json::json!(now_ms());
                    changed = true;
                }
            }
            if changed {
                let _ = invite_save(serde_json::Value::Array(arr));
            }
        })
    }

    // ---- the page ----------------------------------------------------------
    // /me's under-the-card seam. Nothing is drawn until the server has spoken:
    // no `invite` key yet means the fetch is still in flight, and `may:false`
    // means the caller is a member and there is no row to see.

    fn me_under(state: String) -> String {
        let base = existing.me_under(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let inv = s["invite"].clone();
        if !inv["may"].as_bool().unwrap_or(false) {
            return base;
        }
        let mut out = String::from("<div class=\"invite\">");
        out.push_str("<div class=\"crow invite-new\">");
        out.push_str("<input class=\"invite-name\" placeholder=\"name\" autocomplete=\"off\">");
        out.push_str("<input class=\"invite-phone\" placeholder=\"phone\" inputmode=\"tel\" autocomplete=\"off\">");
        out.push_str("<span class=\"invite-send\" data-invite=\"send\">invite</span>");
        out.push_str("</div>");
        let err = inv["error"].as_str().unwrap_or("").to_string();
        if !err.is_empty() {
            out.push_str(&format!("<div class=\"invite-say\">{}</div>", card_esc(err)));
        }
        let empty: Vec<serde_json::Value> = Vec::new();
        for p in inv["list"].as_array().unwrap_or(&empty) {
            let name = card_esc(p["name"].as_str().unwrap_or("").to_string());
            let phone = card_esc(p["phone"].as_str().unwrap_or("").to_string());
            let joined = p["joined"].as_bool().unwrap_or(false);
            let status = if joined { "joined" } else { "invited" };
            let x = if joined {
                String::new()
            } else {
                format!("<span class=\"invite-x\" data-invite=\"x\" data-phone=\"{}\">✕</span>",
                        phone)
            };
            out.push_str(&format!(
                "<div class=\"crow invite-row\"><span class=\"cnum invite-status\">{}</span><div class=\"ctext\">{}</div>{}</div>",
                status, name, x));
        }
        out.push_str("</div>");
        format!("{}{}", base, out)
    }

    // the fetched answer, straight into the loop state under this node's own
    // key. It is state and not a /var on purpose: the guest list is the
    // server's, and syncing it to devices as world state would be a lie.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "InviteList" {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        s["invite"] = e["data"].clone();
        s.to_string()
    }
}
