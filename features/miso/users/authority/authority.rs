struct feature_Authority;
impl feature_Authority {
    // a guest's authority level, read from their users.json entry. Localhost
    // tooling (the mini itself, `local:`) is always full authority; a tunnel
    // user is whatever their entry's "authority" says, defaulting to the
    // least-privilege "member"; anyone not on the list has none.
    fn authority_of(who: String) -> String {
        if who.starts_with("local:") {
            return "admin".to_string();
        }
        let phone = match who.strip_prefix("phone:") {
            Some(p) => normalise_phone(p.to_string()),
            None => return String::new(),
        };
        let raw = std::fs::read_to_string(format!("{}/users.json", auth_dir()))
            .unwrap_or_default();
        let v: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or(serde_json::Value::Null);
        let list = v.as_array().cloned().unwrap_or_default();
        for u in list {
            let uphone = normalise_phone(u["phone"].as_str().unwrap_or("").to_string());
            if !uphone.is_empty() && uphone == phone {
                return u["authority"].as_str().unwrap_or("member").to_string();
            }
        }
        String::new()
    }

    // the ladder, as a rank so checks are comparisons: none < member < support
    // < admin. New rungs slot in by number without touching the call sites.
    fn authority_rank(who: String) -> u8 {
        match authority_of(who).as_str() {
            "admin" => 3,
            "support" => 2,
            "member" => 1,
            _ => 0,
        }
    }

    // may this identity act on SHARED state (everyone's world), as opposed to
    // only their own? Support and above. This is the blast-radius check in its
    // first, coarsest form: "shared" is the whole radius, and support ⊇ shared.
    fn may_write_shared(who: String) -> bool {
        authority_rank(who) >= 2
    }

    // the shared-layer gate, widened from "localhost only" to "localhost OR a
    // sufficiently authorised user". A plain member still cannot touch the
    // shared layer — the default is unchanged and least-privilege — but a
    // support person working through the app now can, without being ash on the
    // mini. This is the one enforcement point the tree had; it is now graded.
    fn ctx_may_write_layer() -> bool {
        existing.ctx_may_write_layer() || may_write_shared(context_user_now())
    }
}
