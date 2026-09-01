struct feature_OneClaim;
impl feature_OneClaim {
    // one number, one account — enforced at the invite door too. /add-number
    // owns the test; this wrapper only asks it and stands aside.
    fn invite_add(r: request) -> response {
        let v: serde_json::Value = serde_json::from_str(&r.body)
            .unwrap_or(serde_json::Value::Null);
        let phone = normalise_phone(v["phone"].as_str().unwrap_or("").to_string());
        if addnum_taken(phone) {
            return invite_say(400, "that number belongs to someone already".to_string());
        }
        existing.invite_add(r)
    }
}
