struct feature_Ranked;
impl feature_Ranked {
    // ---- mint --------------------------------------------------------------
    // the rank and the project ride on the caller's token row. Checked before
    // the inner mint so a refused rank mints nothing; stamped after it, under
    // a second take of the lock (the inner one is not reentrant). With
    // neither field in the body — "new code" sends `fresh` alone — the row
    // that is about to be replaced lends its fields to the new one.
    fn qr_mint(r: request) -> response {
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let asked_project = v["project"].as_str().unwrap_or("").trim().to_string();
        let asked_rank = v["rank"].as_str().unwrap_or("").trim().to_string();
        let who = invite_caller(r.cookie.clone());
        if !asked_project.is_empty() {
            let bad = invite_into_ok(who.clone(), asked_project.clone(), asked_rank.clone());
            if !bad.is_empty() {
                return invite_say(400, bad);
            }
        }
        let old = ranked_row_of(who.clone());
        let resp = existing.qr_mint(r);
        if resp.status != 200 {
            return resp;
        }
        let project = if asked_project.is_empty() {
            old["project"].as_str().unwrap_or("").to_string()
        } else {
            asked_project
        };
        let rank = if asked_rank.is_empty() {
            old["rank"].as_str().unwrap_or("").to_string()
        } else {
            asked_rank
        };
        if rank.is_empty() {
            return resp;
        }
        ranked_stamp_row(who, project.clone(), rank.clone());
        // the answer says what the code leads to, so the sheet can say it too
        let mut body: serde_json::Value = serde_json::from_slice(&resp.body)
            .unwrap_or(serde_json::Value::Null);
        if !body.is_object() {
            return resp;
        }
        body["rank"] = serde_json::json!(rank);
        body["project"] = serde_json::json!(project);
        json_response(200, body.to_string())
    }

    fn ranked_row_of(who: String) -> serde_json::Value {
        let list = qr_list();
        if list.is_null() {
            return serde_json::json!({});
        }
        let at = qr_index_by(list.clone(), who);
        if at == usize::MAX {
            return serde_json::json!({});
        }
        list[at].clone()
    }

    fn ranked_stamp_row(who: String, project: String, rank: String) {
        with_store_lock(|| {
            let mut list = qr_list();
            if list.is_null() {
                return;
            }
            let at = qr_index_by(list.clone(), who.clone());
            if at == usize::MAX {
                return;
            }
            list[at]["project"] = serde_json::json!(project.clone());
            list[at]["rank"] = serde_json::json!(rank.clone());
            let _ = qr_save(list);
        })
    }

    // ---- claim -------------------------------------------------------------
    // after a claim the guest-list entry is stamped with the row's fields —
    // through /doors' own stamp, which skips an entry that has joined, so a
    // member re-scanning a code (a success, by /qr's design) is not
    // re-invited anywhere.
    fn qr_claim(r: request) -> response {
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let token = v["t"].as_str().unwrap_or("").to_string();
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        let resp = existing.qr_claim(r);
        if resp.status != 200 {
            return resp;
        }
        let list = qr_list();
        if list.is_null() {
            return resp;
        }
        let at = qr_index_of(list.clone(), token);
        if at == usize::MAX {
            return resp;
        }
        let project = list[at]["project"].as_str().unwrap_or("").to_string();
        let rank = list[at]["rank"].as_str().unwrap_or("").to_string();
        if project.is_empty() || rank.is_empty() {
            return resp;
        }
        invite_into_stamp(phone, project, rank);
        resp
    }

    // ---- the sheet ---------------------------------------------------------
    // one quiet line under "join miso": where the code leads. The state is
    // the mint's answer verbatim, which now carries the two fields; the
    // title is read from the canvasser's own world, where the project is.
    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let q = s["invite_qr"].clone();
        if !q["open"].as_bool().unwrap_or(false) {
            return base;
        }
        let rank = q["rank"].as_str().unwrap_or("").to_string();
        if rank.is_empty() {
            return base;
        }
        let project = q["project"].as_str().unwrap_or("").to_string();
        let title = if project.is_empty() {
            String::new()
        } else {
            browse_title_of(&audience_card_by_id(project))
        };
        let line = if title.is_empty() {
            format!("as {}", card_esc(rank))
        } else {
            format!("into {} as {}", title, card_esc(rank))
        };
        let mark = "<div class=\"qr-word\">join miso</div>";
        base.replacen(mark, &format!("{}<div class=\"qr-rank\">{}</div>", mark, line), 1)
    }
}
