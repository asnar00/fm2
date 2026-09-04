struct feature_OwnRole;
impl feature_OwnRole {
    // ---- six levels, and yours is the one already lit ------------------------
    // "same as me" was a seventh entry that said what the six already say: it
    // resolved to the author's own role, so the list held a name for a thing
    // that was also in the list. Ash asked for it gone and for the default to
    // BE the user's own level (#p109). The floor logic does not move an inch —
    // an unset `post_level` still stamps the author's own grade, which is what
    // "same as me" always meant. What changes is that the list now shows you
    // which one that is.
    //
    // `armed_level_row` is redefined whole rather than wrapped because every
    // line of it changes: one entry leaves and the lit test stops asking
    // "is nothing chosen" and starts asking "is this the one you would get".
    // It still builds its entries with `armed_pill`, so /explained's sentences
    // and /plain-words' wording are all still on them.
    fn armed_level_row() -> String {
        let chosen = armed_level_read();
        // an unset var — including a device left on the old "same as me" —
        // reads as the role you hold. That is the same answer it always gave;
        // it is now visible.
        let lit = if chosen.is_empty() { own_role_mine() } else { chosen };
        armed_level_box("publish level".to_string(),
                        armed_level_entries("armed_lvl_".to_string(), lit))
    }

    // the seventh row goes here rather than in `armed_level_row`, so that
    // every surface drawing this list loses it together — the visibility
    // picker on a post's page has no "same as me" either, and gets that for
    // nothing. Whole redefinition rather than a wrapper: the base's answer is
    // the seven-row list this replaces, not a fallback to fall through to.
    fn armed_level_entries(prefix: String, lit: String) -> String {
        let mut pills = String::new();
        for g in armed_levels().iter() {
            pills.push_str(&armed_pill(format!("{}{}", prefix, g), g.clone(),
                                       &lit == g));
        }
        pills
    }

    // your own role in the project you are working in, read the way the floor
    // itself is read — `audience_grade_in` off the project card — so the row
    // that is lit is exactly the floor an unset choice would stamp. Anything
    // missing answers empty, and an empty answer lights nothing: with no
    // project selected there is no role to be, no floor is stamped (`card_new`
    // returns before the floor line) and the list says so by lighting none of
    // its rows.
    //
    // "my card" is the profile card with no `from` — /exchange's rule, asked
    // of its own `card_of_type` rather than re-tested here, because a copy
    // heading the list is exactly the bug that rule exists for (misses.md,
    // "the first profile card").
    fn own_role_mine() -> String {
        let card = card_of_type(cards_read(), String::new(), "profile".to_string());
        if card.is_empty() {
            return String::new();
        }
        let c: serde_json::Value = serde_json::from_str(&card)
            .unwrap_or(serde_json::Value::Null);
        let name = c["owner"].as_str().unwrap_or("").to_string();
        if name.is_empty() {
            return String::new();
        }
        let pid = current_project_read();
        if pid.is_empty() {
            return String::new();
        }
        let proj = audience_project_in(cards_read(), pid);
        if proj.is_null() {
            return String::new();
        }
        audience_grade_in(&proj, name)
    }
}
