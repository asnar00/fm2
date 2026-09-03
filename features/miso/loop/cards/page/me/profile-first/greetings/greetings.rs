struct feature_Greetings;
impl feature_Greetings {
    fn greetings_read() -> i64 {
        with_context(|c| c.greetings_greeted_get())
    }

    fn greetings_write(n: i64) {
        edit_context(|c| {
            let _ = c.edit_op("miso/loop/cards/page/me/profile-first/greetings",
                              "greeted", serde_json::json!(n));
        });
    }

    // the two moments: the gate standing (first page), the gate lifted on a
    // joined world after the first page (second page)
    fn render(state: String) -> String {
        let base = existing.render(state.clone());
        let seen = greetings_read();
        if seen >= 2 {
            return base;
        }
        let gated = profile_first_gated(state.clone());
        if gated && seen == 0 {
            return format!("{}{}", base, greetings_sheet(1));
        }
        let s: serde_json::Value = serde_json::from_str(&state)
            .unwrap_or(serde_json::json!({}));
        let joined = s["_joined"].as_bool().unwrap_or(false);
        if !gated && joined && seen == 1 {
            return format!("{}{}", base, greetings_sheet(2));
        }
        base
    }

    fn greetings_sheet(n: i64) -> String {
        if n == 1 {
            let proj = current_project_card();
            let name = if proj.is_null() { String::new() } else { card_esc(browse_title_of(&proj)) };
            let hello = if name.is_empty() {
                "welcome to miso!".to_string()
            } else {
                format!("welcome to the {} project on miso!", name)
            };
            return format!(concat!(
                "<div id=\"greetSheet\" class=\"greet\">",
                "<div class=\"greet-hello\">{}</div>",
                "<div class=\"greet-say\">first, your profile: a picture, and a line about what you're here to do.</div>",
                "<div class=\"greet-go\" data-ev=\"greet_next\">let's go</div>",
                "</div>"), hello);
        }
        String::from(concat!(
            "<div id=\"greetSheet\" class=\"greet\">",
            "<div class=\"greet-hello\">that's you.</div>",
            "<div class=\"greet-say\">hold any button for two seconds and it tells you what it does.</div>",
            "<div class=\"greet-go\" data-ev=\"greet_next\">got it</div>",
            "</div>"))
    }

    fn update(state: String, event: String) -> String {
        let state = existing.update(state, event.clone());
        let e: serde_json::Value = serde_json::from_str(&event)
            .unwrap_or(serde_json::Value::Null);
        if e["type"].as_str().unwrap_or("") == "click"
            && e["ev"].as_str().unwrap_or("") == "greet_next" {
            let n = greetings_read();
            if n < 2 {
                greetings_write(n + 1);
            }
        }
        state
    }
}
