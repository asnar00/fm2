struct feature_Policy;
impl feature_Policy {
    // a panel picker click writes the user-scoped policy /var; the declared
    // merge ships it to the user's other devices, and the `js:update_policy`
    // column republishes it at the key policy.index.js already reads.
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
        update_policy_write(choice);
        state
    }

    // the address, written once. The closure clones because `edit_context`
    // replays it against this turn's frozen view and therefore runs it twice.
    fn update_policy_write(choice: String) {
        edit_context(|c| {
            let _ = c.edit_op("miso/shell/update/policy", "update_policy",
                              serde_json::json!(choice.clone()));
        });
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
