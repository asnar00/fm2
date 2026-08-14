struct feature_Policy;
impl feature_Policy {
    // a panel picker click writes the user-scoped policy var; /scope ships it
    // to the user's other devices and /join restores it on boot.
    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") != "click" {
            return state;
        }
        let choice = policy_choice(e["ev"].as_str().unwrap_or("").to_string());
        if choice.is_empty() {
            return state;
        }
        let mut s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        Var::<String>::user("update_policy").set(&mut s, &choice);
        s.to_string()
    }

    fn policy_choice(ev: String) -> String {
        match ev.as_str() {
            "policy_auto" => "auto".to_string(),
            "policy_fixes" => "fixes".to_string(),
            "policy_consent" => "consent".to_string(),
            _ => String::new(),
        }
    }
}
