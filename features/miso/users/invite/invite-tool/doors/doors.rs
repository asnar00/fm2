struct feature_Doors;
impl feature_Doors {
    // ---- the page ----------------------------------------------------------
    // two buttons and nothing else (#p69). No `existing` call on purpose: the
    // send row, the invited list and /qr's pill all leave with it, and untick
    // brings every one of them back. The selected project rides on the block
    // so the sheet can say where the person is going without a fetch.
    fn invite_rows_html(inv: serde_json::Value) -> String {
        let _ = inv;
        let proj = current_project_card();
        let id = card_esc(proj["id"].as_str().unwrap_or("").to_string());
        let title = if proj.is_null() { String::new() } else { browse_title_of(&proj) };
        format!(
            "<div class=\"doors\" data-project=\"{}\" data-project-title=\"{}\"><div class=\"door\" data-door=\"qr\">show QR code</div><div class=\"door\" data-door=\"name\">invite by name</div></div>",
            id, title)
    }

    // ---- the name road ------------------------------------------------------
    // the sheet sends `rank` and `project` beside name and phone. Checked
    // BEFORE the inner add so a refused rank writes nothing; stamped AFTER a
    // 200, outside the inner lock (it is not reentrant — /invite's own
    // `joined` stamp takes the same care).
    fn invite_add(r: request) -> response {
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let project = v["project"].as_str().unwrap_or("").trim().to_string();
        let rank = v["rank"].as_str().unwrap_or("").trim().to_string();
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        let who = invite_caller(r.cookie.clone());
        if !project.is_empty() {
            let bad = invite_into_ok(who.clone(), project.clone(), rank.clone());
            if !bad.is_empty() {
                return invite_say(400, bad);
            }
        }
        let resp = existing.invite_add(r);
        if resp.status != 200 || project.is_empty() {
            return resp;
        }
        invite_into_stamp(phone, project, rank);
        resp
    }

    // the three checks, as one seam so the QR road (/ranked) obeys exactly the
    // same ones: the inviter holds the project (own or copy, not deleted),
    // stands in it, and is not giving a rank above their own. Empty means
    // fine, else the sentence to show. Rank is project standing (/audience),
    // never /authority's app standing.
    fn invite_into_ok(who: String, project: String, rank: String) -> String {
        if who.is_empty() {
            return "you can't invite people".to_string();
        }
        if !audience_is_grade(rank.clone()) {
            return "that isn't a role".to_string();
        }
        let proj = audience_project_in(exchange_cards_of(who.clone()), project);
        if proj.is_null() {
            return "you don't hold that project".to_string();
        }
        let mine = audience_grade_in(&proj, exchange_name_of(who));
        if mine.is_empty() {
            return "you're not in that project".to_string();
        }
        if audience_rank(rank) < audience_rank(mine.clone()) {
            return format!("you're {} there — you can't invite someone in above that",
                           invite_into_article(mine));
        }
        String::new()
    }

    fn invite_into_article(rank: String) -> String {
        if rank == "admin" {
            return "an admin".to_string();
        }
        if rank == "team" {
            return "team".to_string();
        }
        if rank == "public" {
            return "public".to_string();
        }
        format!("a {}", rank)
    }

    // two fields beside `authority` on the guest-list entry, never overloading
    // it. `added` is cleared so a re-invite (a taken-back invite sent again)
    // is a fresh promise. Only an entry that has not joined: a member who
    // re-scans a code (/ranked's road answers that as a success) is not
    // re-invited anywhere. Under the store lock, temp-write and rename, as
    // every guest-list write is.
    fn invite_into_stamp(phone: String, project: String, rank: String) {
        if phone.is_empty() {
            return;
        }
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
                let up = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
                if !up.is_empty() && up == phone && !u["joined"].is_u64() {
                    u["project"] = serde_json::json!(project.clone());
                    u["rank"] = serde_json::json!(rank.clone());
                    if let Some(o) = u.as_object_mut() {
                        o.remove("added");
                    }
                    changed = true;
                }
            }
            if changed {
                let _ = invite_save(serde_json::Value::Array(arr));
                println!("doors: {} is invited into {} as {}", tag(phone.clone()), project, rank);
            }
        })
    }
}
