struct feature_LastWord;
impl feature_LastWord {
    // page two says what it is for; page three is the last word
    fn greetings_sheet(n: i64) -> String {
        if n == 3 {
            return String::from(concat!(
                "<div id=\"greetSheet\" class=\"greet\">",
                "<div class=\"greet-hello\">that's it!</div>",
                "<div class=\"greet-say\">hold any button to find out what it does.</div>",
                "<div class=\"greet-go\" data-ev=\"greet_next\">done</div>",
                "</div>"));
        }
        let base = existing.greetings_sheet(n);
        if n != 2 {
            return base;
        }
        base.replacen("<div class=\"greet-hello\">that's you.</div>",
                      "<div class=\"greet-hello\">two things to switch on</div>", 1)
            .replacen("<div class=\"greet-say\">hold any button for two seconds and it tells you what it does.</div>",
                      "<div class=\"greet-say\">Face ID to log in, and notifications so the team can reach you.</div>", 1)
    }

    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        if greetings_read() != 2 {
            return base;
        }
        if profile_first_gated(state.clone()) {
            return base;
        }
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        if !s["_joined"].as_bool().unwrap_or(false) {
            return base;
        }
        format!("{}{}", base, greetings_sheet(3))
    }

    // the base steps greeted to 2 and stops; this steps it to 3 and marks the
    // tour seen, so it never starts
    fn update(state: String, event: String) -> String {
        let before = greetings_read();
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") == "click"
            && e["ev"].as_str().unwrap_or("") == "greet_next"
            && before == 2 {
            greetings_write(3);
            tour_seen_write(true);
        }
        state
    }
}
